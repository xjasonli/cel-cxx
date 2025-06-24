use super::*;
use crate::function::FunctionRegistry;
use crate::variable::VariableRegistry;
use crate::{ffi, Constant, Program, ProgramInner, ValueType};
use ouroboros::self_referencing;

#[self_referencing]
#[derive(Debug)]
pub struct EnvInner<'f> {
    function_registry: Arc<FunctionRegistry<'f>>,
    variable_registry: Arc<VariableRegistry>,

    ffi_ctx: ffi::Ctx,

    #[borrows(ffi_ctx)]
    #[covariant]
    ffi_compiler: cxx::UniquePtr<ffi::Compiler<'this>>,

    #[borrows(ffi_ctx)]
    #[covariant]
    ffi_runtime: cxx::UniquePtr<ffi::Runtime<'this, 'static>>,
}

impl<'f> EnvInner<'f> {
    pub fn new_with_registries(
        function_registry: FunctionRegistry<'f>,
        variable_registry: VariableRegistry,
    ) -> Result<Self, ffi::Status> {
        let function_registry = Arc::new(function_registry);
        let variable_registry = Arc::new(variable_registry);

        EnvInnerTryBuilder {
            function_registry: function_registry.clone(),
            variable_registry: variable_registry.clone(),
            ffi_ctx: ffi::Ctx::new_generated(),
            ffi_compiler_builder: |ffi_ctx: &ffi::Ctx| {
                let mut builder = ffi::CompilerBuilder::new(
                    ffi::DescriptorPool::generated(),
                    &ffi::CompilerOptions::default(),
                )?;
                builder
                    .pin_mut()
                    .add_library(ffi::compiler::CompilerLibrary::new_standard())?;
                builder
                    .pin_mut()
                    .add_library(ffi::compiler::CompilerLibrary::new_optional())?;
                builder
                    .pin_mut()
                    .add_library(ffi::extensions::strings::compiler_library())?;
                builder
                    .pin_mut()
                    .add_library(ffi::extensions::bindings_ext::compiler_library())?;
                builder
                    .pin_mut()
                    .add_library(ffi::extensions::encoders::compiler_library())?;
                builder
                    .pin_mut()
                    .add_library(ffi::extensions::lists::compiler_library())?;
                builder
                    .pin_mut()
                    .add_library(ffi::extensions::math_ext::compiler_library())?;
                builder
                    .pin_mut()
                    .add_library(ffi::extensions::proto_ext::compiler_library())?;
                builder
                    .pin_mut()
                    .add_library(ffi::extensions::regex::compiler_library())?;
                builder
                    .pin_mut()
                    .add_library(ffi::extensions::sets::compiler_library())?;

                for (name, decl_or_constant) in variable_registry.entries() {
                    let ffi_variable_decl = match decl_or_constant.constant() {
                        Some(constant) => {
                            let ffi_constant = match constant {
                                Constant::Null => ffi::Constant::new_null(),
                                Constant::Bool(value) => ffi::Constant::new_bool(*value),
                                Constant::Int(value) => ffi::Constant::new_int(*value),
                                Constant::Uint(value) => ffi::Constant::new_uint(*value),
                                Constant::Double(value) => ffi::Constant::new_double(*value),
                                Constant::Bytes(value) => ffi::Constant::new_bytes(value),
                                Constant::String(value) => ffi::Constant::new_string(value),
                                Constant::Duration(value) => {
                                    ffi::Constant::new_duration((*value).into())
                                }
                                Constant::Timestamp(value) => {
                                    ffi::Constant::new_timestamp((*value).into())
                                }
                            };
                            ffi::VariableDecl::new_constant(name, &ffi_constant)
                        }
                        None => {
                            let value_type = decl_or_constant.decl();
                            ffi::VariableDecl::new(
                                name,
                                &ffi::type_from_rust(
                                    value_type,
                                    ffi_ctx.arena(),
                                    ffi_ctx.descriptor_pool(),
                                ),
                            )
                        }
                    };
                    builder
                        .pin_mut()
                        .checker_builder()
                        .add_variable(&ffi_variable_decl);
                }

                for (name, overloads) in function_registry.entries() {
                    let mut ffi_function_decl = ffi::FunctionDecl::new(name);
                    for kind_overload in overloads.entries() {
                        let member = kind_overload.member();

                        for type_overload in kind_overload.entries() {
                            let id = type_overload.decl().id(name, member);
                            let ffi_result = ffi::type_from_rust(
                                type_overload.decl().result(),
                                ffi_ctx.arena(),
                                ffi_ctx.descriptor_pool(),
                            );
                            let ffi_arguments = type_overload
                                .decl()
                                .arguments()
                                .iter()
                                .map(|ty| {
                                    ffi::type_from_rust(
                                        ty,
                                        ffi_ctx.arena(),
                                        ffi_ctx.descriptor_pool(),
                                    )
                                })
                                .collect::<Vec<_>>();
                            let ffi_overload_decl =
                                ffi::OverloadDecl::new(&id, member, &ffi_result, &ffi_arguments);
                            ffi_function_decl.pin_mut().add_overload(&ffi_overload_decl);
                        }
                    }
                    builder
                        .pin_mut()
                        .checker_builder()
                        .add_function(&ffi_function_decl);
                }

                let compiler = builder.pin_mut().build()?;
                Ok(compiler)
            },
            ffi_runtime_builder: |ffi_ctx: &ffi::Ctx| {
                let options = ffi::RuntimeOptions::new();
                let mut builder = ffi::RuntimeBuilder::new_standard(
                    ffi_ctx.shared_descriptor_pool().clone(),
                    &options,
                )?;

                ffi::extensions::strings::register_function(
                    builder.pin_mut().function_registry(),
                    &options,
                )?;
                ffi::extensions::encoders::register_function(
                    builder.pin_mut().function_registry(),
                    &options,
                )?;
                ffi::extensions::lists::register_function(
                    builder.pin_mut().function_registry(),
                    &options,
                )?;
                ffi::extensions::math_ext::register_function(
                    builder.pin_mut().function_registry(),
                    &options,
                )?;
                ffi::extensions::regex::register_function(
                    builder.pin_mut().function_registry(),
                    &options,
                )?;
                ffi::extensions::sets::register_function(
                    builder.pin_mut().function_registry(),
                    &options,
                )?;

                for (name, overloads) in function_registry.entries() {
                    for kind_overload in overloads.entries() {
                        let receiver_style = kind_overload.member();
                        let ffi_types = kind_overload.argument_kinds();
                        let ffi_function_descriptor =
                            ffi::FunctionDescriptor::new(name, receiver_style, ffi_types, true);
                        builder
                            .pin_mut()
                            .function_registry()
                            .register_lazy(&ffi_function_descriptor);
                    }
                }

                for (_, decl) in variable_registry.entries() {
                    if let ValueType::Opaque(opaque) = decl.decl() {
                        let ffi_opaque_type = ffi::opaque_type_from_rust(
                            opaque,
                            ffi_ctx.arena(),
                            ffi_ctx.descriptor_pool(),
                        );
                        builder
                            .pin_mut()
                            .type_registry()
                            .register_type(&ffi_opaque_type);
                    }
                }

                let runtime = builder.pin_mut().build()?;
                Ok(runtime)
            },
        }
        .try_build()
    }

    pub fn function_registry<'this>(&'this self) -> &'this Arc<FunctionRegistry<'f>> {
        self.with_function_registry(|function_registry| function_registry)
    }

    #[allow(unused)]
    pub fn variable_registry(&self) -> &Arc<VariableRegistry> {
        self.with_variable_registry(|variable_registry| variable_registry)
    }

    pub fn ctx(&self) -> &ffi::Ctx {
        self.with_ffi_ctx(|ffi_ctx| ffi_ctx)
    }

    pub fn compiler<'this>(&'this self) -> &'this ffi::Compiler<'this> {
        self.with_ffi_compiler(|ffi_compiler| ffi_compiler)
    }

    pub fn runtime<'this>(&'this self) -> &'this ffi::Runtime<'this, 'static> {
        self.with_ffi_runtime(|ffi_runtime| ffi_runtime)
    }

    pub fn compile<Fm: FnMarker, Rm: RuntimeMarker, S: AsRef<[u8]>>(
        self: Arc<Self>,
        source: S,
    ) -> Result<Program<'f, Fm, Rm>, Error> {
        let inner = ProgramInner::new_from_env_source(self, source)?;
        Ok(Program {
            inner: Arc::new(inner),
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }
}
