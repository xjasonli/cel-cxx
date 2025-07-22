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

#[derive(Debug)]
pub struct EnvInnerOptions {
    pub container: String,
    pub enable_standard: bool,
    pub enable_optional: bool,
    pub enable_ext_bindings: bool,
    pub enable_ext_encoders: bool,
    pub enable_ext_lists: bool,
    pub enable_ext_math: bool,
    pub enable_ext_proto: bool,
    pub enable_ext_regex: bool,
    pub enable_ext_re: bool,
    pub enable_ext_sets: bool,
    pub enable_ext_strings: bool,
    pub enable_ext_select_optimization: bool,
}

impl Default for EnvInnerOptions {
    fn default() -> Self {
        Self {
            container: "".to_string(),
            enable_standard: true,
            enable_optional: false,
            enable_ext_bindings: false,
            enable_ext_encoders: false,
            enable_ext_lists: false,
            enable_ext_math: false,
            enable_ext_proto: false,
            enable_ext_regex: false,
            enable_ext_re: false,
            enable_ext_sets: false,
            enable_ext_strings: false,
            enable_ext_select_optimization: false,
        }
    }
}

impl EnvInnerOptions {
    pub fn check_consistency(&mut self) {
        // If the regex extension is enabled, we need to enable the optional extension.
        if self.enable_ext_regex && !self.enable_optional {
            self.enable_optional = true;
        }
    }
}

impl<'f> EnvInner<'f> {
    pub fn new_with_registries(
        function_registry: FunctionRegistry<'f>,
        variable_registry: VariableRegistry,
        mut env_options: EnvInnerOptions,
    ) -> Result<Self, ffi::Status> {
        ffi::absl::log::init();

        env_options.check_consistency();

        let function_registry = Arc::new(function_registry);
        let variable_registry = Arc::new(variable_registry);

        EnvInnerTryBuilder {
            function_registry: function_registry.clone(),
            variable_registry: variable_registry.clone(),
            ffi_ctx: ffi::Ctx::new_generated(),
            ffi_compiler_builder: |ffi_ctx: &ffi::Ctx| {
                let options = ffi::CompilerOptions::new();
                let mut builder =
                    ffi::CompilerBuilder::new(ffi::DescriptorPool::generated(), &options)?;
                
                if !env_options.container.is_empty() {
                    builder
                        .pin_mut()
                        .checker_builder()
                        .set_container(ffi::StringView::new_str(&env_options.container));
                }

                if env_options.enable_standard {
                    builder
                        .pin_mut()
                        .add_library(ffi::compiler::CompilerLibrary::new_standard())?;
                }

                if env_options.enable_optional {
                    builder
                        .pin_mut()
                        .add_library(ffi::compiler::CompilerLibrary::new_optional())?;
                }

                if env_options.enable_ext_bindings {
                    builder
                        .pin_mut()
                        .add_library(ffi::extensions::bindings::compiler_library())?;
                }

                if env_options.enable_ext_encoders {
                    builder
                        .pin_mut()
                        .add_library(ffi::extensions::encoders::compiler_library())?;
                }

                if env_options.enable_ext_lists {
                    builder
                        .pin_mut()
                        .add_library(ffi::extensions::lists::compiler_library())?;
                }

                if env_options.enable_ext_math {
                    builder
                        .pin_mut()
                        .add_library(ffi::extensions::math::compiler_library())?;
                }

                if env_options.enable_ext_proto {
                    builder
                        .pin_mut()
                        .add_library(ffi::extensions::proto::compiler_library())?;
                }

                if env_options.enable_ext_regex {
                    builder
                        .pin_mut()
                        .add_library(ffi::extensions::regex::compiler_library())?;
                }

                if env_options.enable_ext_re {
                    builder
                        .pin_mut()
                        .add_library(ffi::extensions::re::compiler_library())?;
                }

                if env_options.enable_ext_sets {
                    builder
                        .pin_mut()
                        .add_library(ffi::extensions::sets::compiler_library())?;
                }

                if env_options.enable_ext_strings {
                    builder
                        .pin_mut()
                        .add_library(ffi::extensions::strings::compiler_library())?;
                }


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
                        .merge_function(&ffi_function_decl);
                }

                let compiler = builder.pin_mut().build()?;
                Ok(compiler)
            },
            ffi_runtime_builder: |ffi_ctx: &ffi::Ctx| {
                let mut options = ffi::RuntimeOptions::new();

                if !env_options.container.is_empty() {
                    options
                        .pin_mut()
                        .container_mut()
                        .push_str(&env_options.container);
                }

                if env_options.enable_optional {
                    *options.pin_mut().enable_qualified_type_identifiers_mut() = true;
                }

                let mut builder =
                    ffi::RuntimeBuilder::new(ffi_ctx.shared_descriptor_pool().clone(), &options)?;

                if env_options.enable_standard {
                    builder.pin_mut().enable_standard(&options)?;
                }
                if env_options.enable_optional {
                    builder.pin_mut().enable_optional(&options)?;
                }

                if env_options.enable_ext_encoders {
                    ffi::extensions::encoders::register_functions(
                        builder.pin_mut().function_registry(),
                        &options,
                    )?;
                }

                if env_options.enable_ext_lists {
                    ffi::extensions::lists::register_functions(
                        builder.pin_mut().function_registry(),
                        &options,
                    )?;
                }

                if env_options.enable_ext_math {
                    ffi::extensions::math::register_functions(
                        builder.pin_mut().function_registry(),
                        &options,
                    )?;
                }

                if env_options.enable_ext_regex {
                    ffi::extensions::regex::register_functions(
                        builder.pin_mut()
                    )?;
                }

                if env_options.enable_ext_re {
                    ffi::extensions::re::register_functions(
                        builder.pin_mut().function_registry(),
                        &options,
                    )?;
                }

                if env_options.enable_ext_sets {
                    ffi::extensions::sets::register_functions(
                        builder.pin_mut().function_registry(),
                        &options,
                    )?;
                }

                if env_options.enable_ext_strings {
                    ffi::extensions::strings::register_functions(
                        builder.pin_mut().function_registry(),
                        &options,
                    )?;
                }

                if env_options.enable_ext_select_optimization {
                    let options = ffi::extensions::select_optimization::Options::new();
                    ffi::extensions::select_optimization::enable(
                        builder.pin_mut(),
                        &options,
                    )?;
                }

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
