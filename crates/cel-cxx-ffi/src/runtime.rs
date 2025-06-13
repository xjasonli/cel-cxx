use std::pin::Pin;

use crate::absl::{Status, Span, StringView};
use crate::protobuf::{Arena, MessageFactory, DescriptorPool};
use crate::common::{Kind, Value, Ast, OpaqueType};

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!("absl/status/status.h");
        type Status = super::Status;
        include!("absl/strings/string_view.h");
        type string_view<'a> = super::StringView<'a>;
    }

    #[namespace = "google::protobuf"]
    unsafe extern "C++" {
        include!("google/protobuf/descriptor.h");
        type Arena = super::Arena;
        type MessageFactory = super::MessageFactory;
        type DescriptorPool = super::DescriptorPool;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!("runtime/runtime.h");
        include!("runtime/function.h");
        type Kind = super::Kind;
        type Value<'a> = super::Value<'a>;
        type OpaqueType<'a> = super::OpaqueType<'a>;
        type Ast = super::Ast;

        type UnknownProcessingOptions = super::UnknownProcessingOptions;
        #[rust_name = "Activation"]
        type ActivationInterface<'f>;
        type Runtime<'a, 'f>;
        type RuntimeBuilder<'a, 'f>;
        fn type_registry<'this, 'a, 'f>(
            self: Pin<&'this mut RuntimeBuilder<'a, 'f>>
        ) -> Pin<&'this mut TypeRegistry<'this, 'a>>;
        fn function_registry<'this, 'a, 'f>(
            self: Pin<&'this mut RuntimeBuilder<'a, 'f>>
        ) -> Pin<&'this mut FunctionRegistry<'f>>;

        type RuntimeOptions;

        type Function<'a>;
        type FunctionDescriptor;
        fn name(self: &FunctionDescriptor) -> &CxxString;
        fn receiver_style(self: &FunctionDescriptor) -> bool;
        fn types(self: &FunctionDescriptor) -> &CxxVector<Kind>;
        fn is_strict(self: &FunctionDescriptor) -> bool;
        #[rust_name = "shape_matches"]
        fn ShapeMatches(self: &FunctionDescriptor, other: &FunctionDescriptor) -> bool;

        type FunctionOverloadReference<'a, 'f> = super::FunctionOverloadReference<'a, 'f>;

        type FunctionRegistry<'f>;
        #[rust_name = "register"]
        fn Register<'f>(
            self: Pin<&mut FunctionRegistry<'f>>,
            descriptor: &FunctionDescriptor,
            function: UniquePtr<Function<'f>>,
        ) -> Status;
        #[rust_name = "register_lazy"]
        fn RegisterLazyFunction<'f>(
            self: Pin<&mut FunctionRegistry<'f>>,
            descriptor: &FunctionDescriptor,
        ) -> Status;

        include!(<runtime/standard_functions.h>);
        fn RegisterStandardFunctions(
            function_registry: Pin<&mut FunctionRegistry>,
            options: &RuntimeOptions,
        ) -> Status;

        type Program<'a, 'f>;
        type TypeRegistry<'d, 'a>;
        #[rust_name = "register_type"]
        fn RegisterType<'d, 'a>(
            self: Pin<&mut TypeRegistry<'d, 'a>>,
            opaque_type: &OpaqueType<'a>,
        ) -> Status;
    }

    #[namespace = "cel::runtime_internal"]
    unsafe extern "C++" {
        include!(<runtime/function_provider.h>);
        type FunctionProvider;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!("cel-cxx-ffi/include/runtime.h");
        type LazyOverload<'a> = super::LazyOverload<'a>;

        type Span_Value<'a, 'v> = super::Span<'a, super::Value<'v>>;
        type Span_CxxString<'a> = super::Span<'a, cxx::CxxString>;
        type Span_Kind<'a> = super::Span<'a, super::Kind>;
        
        type FunctionRegistryIterator<'a> = super::FunctionRegistryIterator<'a>;
        fn FunctionRegistryIterator_next<'a>(
            iter: &mut FunctionRegistryIterator<'a>,
            key: &mut UniquePtr<CxxString>,
            value: &mut UniquePtr<CxxVector<FunctionDescriptor>>,
        ) -> bool;
        fn FunctionRegistryIterator_drop<'a>(
            iter: &mut FunctionRegistryIterator<'a>,
        );

        // Activation
        fn Activation_new<'f>(
            ffi: Box<AnyFfiActivation<'f>>,
        ) -> UniquePtr<Activation<'f>>;

        // FunctionRegistry
        fn FunctionRegistry_find_static_overloads<'this, 'f>(
            function_registry: &'this FunctionRegistry<'f>,
            name: string_view<'_>,
            receiver_style: bool,
            types: Span_Kind<'_>,
        ) -> Vec<FunctionOverloadReference<'this, 'f>>;
        fn FunctionRegistry_find_static_overloads_by_arity<'this, 'f>(
            function_registry: &'this FunctionRegistry<'f>,
            name: string_view<'_>,
            receiver_style: bool,
            arity: usize,
        ) -> Vec<FunctionOverloadReference<'this, 'f>>;
        fn FunctionRegistry_find_lazy_overloads<'this, 'f>(
            function_registry: &'this FunctionRegistry<'f>,
            name: string_view<'_>,
            receiver_style: bool,
            types: Span_Kind<'_>,
        ) -> Vec<LazyOverload<'this>>;
        fn FunctionRegistry_find_lazy_overloads_by_arity<'this, 'f>(
            function_registry: &'this FunctionRegistry<'f>,
            name: string_view<'_>,
            receiver_style: bool,
            arity: usize,
        ) -> Vec<LazyOverload<'this>>;
        fn FunctionRegistry_list_functions<'this, 'f>(
            function_registry: &'this FunctionRegistry<'f>,
        ) -> FunctionRegistryIterator<'this>;

        // Runtime
        fn Runtime_create_program<'a, 'f>(
            runtime: &Runtime<'a, 'f>,
            ast: UniquePtr<Ast>,
            result: &mut UniquePtr<Program<'a, 'f>>,
        ) -> Status;

        // RuntimeBuilder
        fn RuntimeBuilder_new<'this, 'a, 'f>(
            descriptor_pool: SharedPtr<DescriptorPool>,
            options: &RuntimeOptions,
            result: &'this mut UniquePtr<RuntimeBuilder<'a, 'f>>,
        ) -> Status;
        fn RuntimeBuilder_new_standard<'this, 'a, 'f>(
            descriptor_pool: SharedPtr<DescriptorPool>,
            options: &RuntimeOptions,
            result: &'this mut UniquePtr<RuntimeBuilder<'a, 'f>>,
        ) -> Status;
        fn RuntimeBuilder_build<'this, 'a, 'f>(
            runtime_builder: Pin<&'this mut RuntimeBuilder<'a, 'f>>,
            result: &mut UniquePtr<Runtime<'a, 'f>>,
        ) -> Status;

        // RuntimeOptions
        fn RuntimeOptions_new() -> UniquePtr<RuntimeOptions>;

        // RuntimeOptions getters and setters
        fn RuntimeOptions_get_container(runtime_options: &RuntimeOptions) -> &str;
        fn RuntimeOptions_set_container(runtime_options: Pin<&mut RuntimeOptions>, container: &str);
        fn RuntimeOptions_get_unknown_processing(runtime_options: &RuntimeOptions) -> UnknownProcessingOptions;
        fn RuntimeOptions_set_unknown_processing(runtime_options: Pin<&mut RuntimeOptions>, unknown_processing: UnknownProcessingOptions);
        fn RuntimeOptions_get_enable_missing_attribute_errors(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_missing_attribute_errors(runtime_options: Pin<&mut RuntimeOptions>, enable_missing_attribute_errors: bool);
        fn RuntimeOptions_get_enable_timestamp_duration_overflow_errors(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_timestamp_duration_overflow_errors(runtime_options: Pin<&mut RuntimeOptions>, enable_timestamp_duration_overflow_errors: bool);
        fn RuntimeOptions_get_short_circuiting(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_short_circuiting(runtime_options: Pin<&mut RuntimeOptions>, short_circuiting: bool);
        fn RuntimeOptions_get_enable_comprehension(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_comprehension(runtime_options: Pin<&mut RuntimeOptions>, enable_comprehension: bool);
        fn RuntimeOptions_get_comprehension_max_iterations(runtime_options: &RuntimeOptions) -> i32;
        fn RuntimeOptions_set_comprehension_max_iterations(runtime_options: Pin<&mut RuntimeOptions>, comprehension_max_iterations: i32);
        fn RuntimeOptions_get_enable_comprehension_list_append(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_comprehension_list_append(runtime_options: Pin<&mut RuntimeOptions>, enable_comprehension_list_append: bool);
        fn RuntimeOptions_get_enable_regex(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_regex(runtime_options: Pin<&mut RuntimeOptions>, enable_regex: bool);
        fn RuntimeOptions_get_regex_max_program_size(runtime_options: &RuntimeOptions) -> i32;
        fn RuntimeOptions_set_regex_max_program_size(runtime_options: Pin<&mut RuntimeOptions>, regex_max_program_size: i32);
        fn RuntimeOptions_get_enable_string_conversion(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_string_conversion(runtime_options: Pin<&mut RuntimeOptions>, enable_string_conversion: bool);
        fn RuntimeOptions_get_enable_string_concat(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_string_concat(runtime_options: Pin<&mut RuntimeOptions>, enable_string_concat: bool);
        fn RuntimeOptions_get_enable_list_concat(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_list_concat(runtime_options: Pin<&mut RuntimeOptions>, enable_list_concat: bool);
        fn RuntimeOptions_get_enable_list_contains(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_list_contains(runtime_options: Pin<&mut RuntimeOptions>, enable_list_contains: bool);
        fn RuntimeOptions_get_fail_on_warnings(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_fail_on_warnings(runtime_options: Pin<&mut RuntimeOptions>, fail_on_warnings: bool);
        fn RuntimeOptions_get_enable_qualified_type_identifiers(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_qualified_type_identifiers(runtime_options: Pin<&mut RuntimeOptions>, enable_qualified_type_identifiers: bool);
        fn RuntimeOptions_get_enable_heterogeneous_equality(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_heterogeneous_equality(runtime_options: Pin<&mut RuntimeOptions>, enable_heterogeneous_equality: bool);
        fn RuntimeOptions_get_enable_empty_wrapper_null_unboxing(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_empty_wrapper_null_unboxing(runtime_options: Pin<&mut RuntimeOptions>, enable_empty_wrapper_null_unboxing: bool);
        fn RuntimeOptions_get_enable_lazy_bind_initialization(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_lazy_bind_initialization(runtime_options: Pin<&mut RuntimeOptions>, enable_lazy_bind_initialization: bool);
        fn RuntimeOptions_get_max_recursion_depth(runtime_options: &RuntimeOptions) -> i32;
        fn RuntimeOptions_set_max_recursion_depth(runtime_options: Pin<&mut RuntimeOptions>, max_recursion_depth: i32);
        fn RuntimeOptions_get_enable_recursive_tracing(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_recursive_tracing(runtime_options: Pin<&mut RuntimeOptions>, enable_recursive_tracing: bool);
        fn RuntimeOptions_get_enable_fast_builtins(runtime_options: &RuntimeOptions) -> bool;
        fn RuntimeOptions_set_enable_fast_builtins(runtime_options: Pin<&mut RuntimeOptions>, enable_fast_builtins: bool);

        // FunctionDescriptor
        fn FunctionDescriptor_new(
            name: &str,
            receiver_style: bool,
            types: &[Kind],
            is_strict: bool,
        ) -> UniquePtr<FunctionDescriptor>;

        fn FunctionDescriptor_new_shared(
            name: &str,
            receiver_style: bool,
            types: &[Kind],
            is_strict: bool,
        ) -> SharedPtr<FunctionDescriptor>;

        // Program
        fn Program_evaluate<'a, 'f, 'a2, 'f2>(
            program: &Program<'a, 'f>,
            arena: &'a2 Arena,
            message_factory: &MessageFactory,
            activation: &Activation<'f2>,
            result: &mut UniquePtr<Value<'a2>>,
        ) -> Status;
        fn Program_evaluate2<'a, 'f, 'a2, 'f2>(
            program: &Program<'a, 'f>,
            arena: &'a2 Arena,
            activation: &Activation<'f2>,
            result: &mut UniquePtr<Value<'a2>>,
        ) -> Status;

        // Function
        fn Function_new<'a>(ffi_function: Box<AnyFfiFunction<'a>>) -> UniquePtr<Function<'a>>;
    }
    #[namespace = "rust::cel_cxx"]
    extern "Rust" {
        type AnyFfiActivation<'f>;
        #[cxx_name = "FindVariable"]
        unsafe fn find_variable<'f, 'a>(
            self: &AnyFfiActivation<'f>,
            name: &str,
            descriptor_pool: &'a DescriptorPool,
            message_factory: &'a MessageFactory,
            arena: &'a Arena,
            result: &mut UniquePtr<Value<'a>>,
        ) -> Status;
        #[cxx_name = "FindFunctionOverloads"]
        unsafe fn find_function_overloads<'this, 'f>(
            self: &'this AnyFfiActivation<'f>,
            name: &str,
        ) -> Vec<FunctionOverloadReference<'this, 'f>>;

        type AnyFfiFunction<'f>;
        #[cxx_name = "Invoke"]
        unsafe fn invoke<'f, 'a>(
            self: &AnyFfiFunction<'f>,
            args: Span_Value<'_, 'a>,
            descriptor_pool: &'a DescriptorPool,
            message_factory: &'a MessageFactory,
            arena: &'a Arena,
            overload_id: Span_CxxString<'_>,
            result: Pin<&mut Value<'a>>,
        ) -> Status;
    }

    impl<'a, 'f> Vec<FunctionOverloadReference<'a, 'f>> {}
    impl<'a> Vec<LazyOverload<'a>> {}
}

// Activation
pub use ffi::Activation;
unsafe impl<'f> Send for Activation<'f> {}
unsafe impl<'f> Sync for Activation<'f> {}

impl<'f> Activation<'f> {
    pub fn new<T: FfiActivation<'f> + 'f>(ffi: T) -> cxx::UniquePtr<Self> {
        ffi::Activation_new(Box::new(AnyFfiActivation::new(ffi)))
    }
}

// FunctionOverloadReference
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FunctionOverloadReference<'a, 'f> {
    descriptor: &'a FunctionDescriptor,
    implementation: &'a Function<'f>,
}

unsafe impl<'a, 'f> cxx::ExternType for FunctionOverloadReference<'a, 'f> {
    type Id = cxx::type_id!("cel::FunctionOverloadReference");
    type Kind = cxx::kind::Trivial;
}

impl<'a, 'f> FunctionOverloadReference<'a, 'f> {
    pub fn new(descriptor: &'a FunctionDescriptor, implementation: &'a Function<'f>) -> Self {
        Self { descriptor, implementation }
    }

    pub fn descriptor(&self) -> &'a FunctionDescriptor {
        self.descriptor
    }

    pub fn implementation(&self) -> &'a Function<'f> {
        self.implementation
    }
}

// FunctionProvider
pub use ffi::FunctionProvider;
unsafe impl Send for FunctionProvider {}
unsafe impl Sync for FunctionProvider {}

// LazyOverload
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LazyOverload<'a> {
    descriptor: &'a FunctionDescriptor,
    function_provider: &'a FunctionProvider,
}

unsafe impl<'a> cxx::ExternType for LazyOverload<'a> {
    type Id = cxx::type_id!("rust::cel_cxx::LazyOverload");
    type Kind = cxx::kind::Trivial;
}

impl<'a> LazyOverload<'a> {
    pub fn new(descriptor: &'a FunctionDescriptor, function_provider: &'a FunctionProvider) -> Self {
        Self { descriptor, function_provider }
    }

    pub fn descriptor(&self) -> &'a FunctionDescriptor {
        self.descriptor
    }

    pub fn function_provider(&self) -> &'a FunctionProvider {
        self.function_provider
    }
}

// FfiActivation
pub trait FfiActivation<'f> {
    fn find_variable<'a>(
        &self,
        name: &str,
        descriptor_pool: &'a DescriptorPool,
        message_factory: &'a MessageFactory,
        arena: &'a Arena,
    ) -> Result<Option<cxx::UniquePtr<Value<'a>>>, Status>;

    fn find_function_overloads<'this>(
        &'this self,
        name: &str,
    ) -> Vec<FunctionOverloadReference<'this, 'f>>;
}

// AnyFfiActivation
struct AnyFfiActivation<'f>(Box<dyn FfiActivation<'f> + 'f>);

impl<'f> AnyFfiActivation<'f> {
    fn new<T: FfiActivation<'f> + 'f>(activation: T) -> Self {
        Self(Box::new(activation))
    }

    fn find_variable<'a>(
        &self,
        name: &str,
        descriptor_pool: &'a DescriptorPool,
        message_factory: &'a MessageFactory,
        arena: &'a Arena,
        result: &mut cxx::UniquePtr<Value<'a>>,
    ) -> Status {
        *result = cxx::UniquePtr::null();

        match self.0.find_variable(name, descriptor_pool, message_factory, arena) {
            Ok(Some(value)) => {
                *result = value;
                Status::ok()
            }
            Ok(None) => {
                Status::ok()
            }
            Err(status) => {
                status
            }
        }
    }

    fn find_function_overloads<'this>(
        &'this self,
        name: &str,
    ) -> Vec<FunctionOverloadReference<'this, 'f>> {
        self.0.find_function_overloads(name)
    }
}

// Runtime
pub use ffi::Runtime;
unsafe impl<'a, 'f> Send for Runtime<'a, 'f> {}
unsafe impl<'a, 'f> Sync for Runtime<'a, 'f> {}

impl<'a, 'f> Runtime<'a, 'f> {
    pub fn create_program(
        &self,
        ast: cxx::UniquePtr<Ast>,
    ) -> Result<cxx::UniquePtr<Program<'a, 'f>>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::Runtime_create_program(self, ast, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }
}

impl<'a, 'f> std::fmt::Debug for Runtime<'a, 'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = self as *const Runtime<'a, 'f>;
        write!(f, "Runtime {{ ptr: {:p} }}", ptr)
    }
}

// RuntimeBuilder
pub use ffi::RuntimeBuilder;
unsafe impl<'a, 'f> Send for RuntimeBuilder<'a, 'f> {}
unsafe impl<'a, 'f> Sync for RuntimeBuilder<'a, 'f> {}

impl<'a, 'f> RuntimeBuilder<'a, 'f> {
    pub fn new(
        descriptor_pool: cxx::SharedPtr<DescriptorPool>,
        options: &RuntimeOptions,
    ) -> Result<cxx::UniquePtr<Self>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::RuntimeBuilder_new(descriptor_pool, options, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }
    pub fn new_standard(
        descriptor_pool: cxx::SharedPtr<DescriptorPool>,
        options: &RuntimeOptions,
    ) -> Result<cxx::UniquePtr<Self>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::RuntimeBuilder_new_standard(descriptor_pool, options, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }

    pub fn build(self: Pin<&mut Self>) -> Result<cxx::UniquePtr<Runtime<'a, 'f>>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::RuntimeBuilder_build(self, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }
}

impl<'a, 'f> std::fmt::Debug for RuntimeBuilder<'a, 'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = self as *const RuntimeBuilder<'a, 'f>;
        write!(f, "RuntimeBuilder {{ ptr: {:p} }}", ptr)
    }
}

// RuntimeOptions
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum UnknownProcessingOptions {
    #[default]
    /// No unknown processing.
    Disabled,
    /// Only attributes supported.
    AttributeOnly,
    /// Attributes and functions supported. Function results are dependent on the
    /// logic for handling unknown_attributes, so clients must opt in to both.
    AttributeAndFunction,
}

unsafe impl cxx::ExternType for UnknownProcessingOptions {
    type Id = cxx::type_id!("cel::UnknownProcessingOptions");
    type Kind = cxx::kind::Trivial;
}

pub use ffi::RuntimeOptions;
unsafe impl Send for RuntimeOptions {}
unsafe impl Sync for RuntimeOptions {}

impl RuntimeOptions {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::RuntimeOptions_new()
    }

    pub fn container(&self) -> &str {
        ffi::RuntimeOptions_get_container(self)
    }

    pub fn set_container(self: Pin<&mut Self>, container: &str) {
        ffi::RuntimeOptions_set_container(self, container);
    }

    pub fn unknown_processing(&self) -> UnknownProcessingOptions {
        ffi::RuntimeOptions_get_unknown_processing(self)
    }

    pub fn set_unknown_processing(self: Pin<&mut Self>, unknown_processing: UnknownProcessingOptions) {
        ffi::RuntimeOptions_set_unknown_processing(self, unknown_processing);
    }

    pub fn enable_missing_attribute_errors(&self) -> bool {
        ffi::RuntimeOptions_get_enable_missing_attribute_errors(self)
    }

    pub fn set_enable_missing_attribute_errors(self: Pin<&mut Self>, enable_missing_attribute_errors: bool) {
        ffi::RuntimeOptions_set_enable_missing_attribute_errors(self, enable_missing_attribute_errors);
    }

    pub fn enable_timestamp_duration_overflow_errors(&self) -> bool {
        ffi::RuntimeOptions_get_enable_timestamp_duration_overflow_errors(self)
    }

    pub fn set_enable_timestamp_duration_overflow_errors(self: Pin<&mut Self>, enable_timestamp_duration_overflow_errors: bool) {
        ffi::RuntimeOptions_set_enable_timestamp_duration_overflow_errors(self, enable_timestamp_duration_overflow_errors);
    }

    pub fn short_circuiting(&self) -> bool {
        ffi::RuntimeOptions_get_short_circuiting(self)
    }

    pub fn set_short_circuiting(self: Pin<&mut Self>, short_circuiting: bool) {
        ffi::RuntimeOptions_set_short_circuiting(self, short_circuiting);
    }

    pub fn enable_comprehension(&self) -> bool {
        ffi::RuntimeOptions_get_enable_comprehension(self)
    }

    pub fn set_enable_comprehension(self: Pin<&mut Self>, enable_comprehension: bool) {
        ffi::RuntimeOptions_set_enable_comprehension(self, enable_comprehension);
    }

    pub fn comprehension_max_iterations(&self) -> i32 {
        ffi::RuntimeOptions_get_comprehension_max_iterations(self)
    }

    pub fn set_comprehension_max_iterations(self: Pin<&mut Self>, comprehension_max_iterations: i32) {
        ffi::RuntimeOptions_set_comprehension_max_iterations(self, comprehension_max_iterations);
    }

    pub fn enable_comprehension_list_append(&self) -> bool {
        ffi::RuntimeOptions_get_enable_comprehension_list_append(self)
    }

    pub fn set_enable_comprehension_list_append(self: Pin<&mut Self>, enable_comprehension_list_append: bool) {
        ffi::RuntimeOptions_set_enable_comprehension_list_append(self, enable_comprehension_list_append);
    }

    pub fn enable_regex(&self) -> bool {
        ffi::RuntimeOptions_get_enable_regex(self)
    }

    pub fn set_enable_regex(self: Pin<&mut Self>, enable_regex: bool) {
        ffi::RuntimeOptions_set_enable_regex(self, enable_regex);
    }

    pub fn regex_max_program_size(&self) -> i32 {
        ffi::RuntimeOptions_get_regex_max_program_size(self)
    }

    pub fn set_regex_max_program_size(self: Pin<&mut Self>, regex_max_program_size: i32) {
        ffi::RuntimeOptions_set_regex_max_program_size(self, regex_max_program_size);
    }

    pub fn enable_string_conversion(&self) -> bool {
        ffi::RuntimeOptions_get_enable_string_conversion(self)
    }

    pub fn set_enable_string_conversion(self: Pin<&mut Self>, enable_string_conversion: bool) {
        ffi::RuntimeOptions_set_enable_string_conversion(self, enable_string_conversion);
    }

    pub fn enable_string_concat(&self) -> bool {
        ffi::RuntimeOptions_get_enable_string_concat(self)
    }

    pub fn set_enable_string_concat(self: Pin<&mut Self>, enable_string_concat: bool) {
        ffi::RuntimeOptions_set_enable_string_concat(self, enable_string_concat);
    }

    pub fn enable_list_concat(&self) -> bool {
        ffi::RuntimeOptions_get_enable_list_concat(self)
    }

    pub fn set_enable_list_concat(self: Pin<&mut Self>, enable_list_concat: bool) {
        ffi::RuntimeOptions_set_enable_list_concat(self, enable_list_concat);
    }

    pub fn enable_list_contains(&self) -> bool {
        ffi::RuntimeOptions_get_enable_list_contains(self)
    }

    pub fn set_enable_list_contains(self: Pin<&mut Self>, enable_list_contains: bool) {
        ffi::RuntimeOptions_set_enable_list_contains(self, enable_list_contains);
    }

    pub fn fail_on_warnings(&self) -> bool {
        ffi::RuntimeOptions_get_fail_on_warnings(self)
    }

    pub fn set_fail_on_warnings(self: Pin<&mut Self>, fail_on_warnings: bool) {
        ffi::RuntimeOptions_set_fail_on_warnings(self, fail_on_warnings);
    }

    pub fn enable_qualified_type_identifiers(&self) -> bool {
        ffi::RuntimeOptions_get_enable_qualified_type_identifiers(self)
    }

    pub fn set_enable_qualified_type_identifiers(self: Pin<&mut Self>, enable_qualified_type_identifiers: bool) {
        ffi::RuntimeOptions_set_enable_qualified_type_identifiers(self, enable_qualified_type_identifiers);
    }

    pub fn enable_heterogeneous_equality(&self) -> bool {
        ffi::RuntimeOptions_get_enable_heterogeneous_equality(self)
    }

    pub fn set_enable_heterogeneous_equality(self: Pin<&mut Self>, enable_heterogeneous_equality: bool) {
        ffi::RuntimeOptions_set_enable_heterogeneous_equality(self, enable_heterogeneous_equality);
    }

    pub fn enable_empty_wrapper_null_unboxing(&self) -> bool {
        ffi::RuntimeOptions_get_enable_empty_wrapper_null_unboxing(self)
    }

    pub fn set_enable_empty_wrapper_null_unboxing(self: Pin<&mut Self>, enable_empty_wrapper_null_unboxing: bool) {
        ffi::RuntimeOptions_set_enable_empty_wrapper_null_unboxing(self, enable_empty_wrapper_null_unboxing);
    }

    pub fn enable_lazy_bind_initialization(&self) -> bool {
        ffi::RuntimeOptions_get_enable_lazy_bind_initialization(self)
    }

    pub fn set_enable_lazy_bind_initialization(self: Pin<&mut Self>, enable_lazy_bind_initialization: bool) {
        ffi::RuntimeOptions_set_enable_lazy_bind_initialization(self, enable_lazy_bind_initialization);
    }

    pub fn max_recursion_depth(&self) -> i32 {
        ffi::RuntimeOptions_get_max_recursion_depth(self)
    }

    pub fn set_max_recursion_depth(self: Pin<&mut Self>, max_recursion_depth: i32) {
        ffi::RuntimeOptions_set_max_recursion_depth(self, max_recursion_depth);
    }

    pub fn enable_recursive_tracing(&self) -> bool {
        ffi::RuntimeOptions_get_enable_recursive_tracing(self)
    }

    pub fn set_enable_recursive_tracing(self: Pin<&mut Self>, enable_recursive_tracing: bool) {
        ffi::RuntimeOptions_set_enable_recursive_tracing(self, enable_recursive_tracing);
    }

    pub fn enable_fast_builtins(&self) -> bool {
        ffi::RuntimeOptions_get_enable_fast_builtins(self)
    }

    pub fn set_enable_fast_builtins(self: Pin<&mut Self>, enable_fast_builtins: bool) {
        ffi::RuntimeOptions_set_enable_fast_builtins(self, enable_fast_builtins);
    }
}

// FunctionDescriptor
pub use ffi::FunctionDescriptor;
unsafe impl Send for FunctionDescriptor {}
unsafe impl Sync for FunctionDescriptor {}

impl FunctionDescriptor {
    pub fn new(name: &str, receiver_style: bool, types: &[Kind], is_strict: bool) -> cxx::UniquePtr<Self> {
        ffi::FunctionDescriptor_new(name, receiver_style, types, is_strict)
    }
    
    pub fn new_shared(name: &str, receiver_style: bool, types: &[Kind], is_strict: bool) -> cxx::SharedPtr<Self> {
        ffi::FunctionDescriptor_new_shared(name, receiver_style, types, is_strict)
    }
}

// FunctionRegistry
pub use ffi::FunctionRegistry;
unsafe impl<'f> Send for FunctionRegistry<'f> {}
unsafe impl<'f> Sync for FunctionRegistry<'f> {}

impl<'f> FunctionRegistry<'f> {
    pub fn register_standard_functions(self: Pin<&mut Self>, options: &RuntimeOptions) -> Status {
        ffi::RegisterStandardFunctions(
            self,
            &*options,
        )
    }

    pub fn find_static_overloads<'this>(
        &'this self,
        name: StringView<'_>,
        receiver_style: bool,
        types: Span<'_, Kind>,
    ) -> Vec<FunctionOverloadReference<'this, 'f>> {
        ffi::FunctionRegistry_find_static_overloads(self, name, receiver_style, types)
    }

    pub fn find_static_overloads_by_arity<'this>(
        &'this self,
        name: StringView<'_>,
        receiver_style: bool,
        arity: usize,
    ) -> Vec<FunctionOverloadReference<'this, 'f>> {
        ffi::FunctionRegistry_find_static_overloads_by_arity(self, name, receiver_style, arity)
    }

    pub fn find_lazy_overloads<'this>(
        &'this self,
        name: StringView<'_>,
        receiver_style: bool,
        types: Span<'_, Kind>,
    ) -> Vec<LazyOverload<'this>> {
        ffi::FunctionRegistry_find_lazy_overloads(self, name, receiver_style, types)
    }

    pub fn find_lazy_overloads_by_arity<'this>(
        &'this self,
        name: StringView<'_>,
        receiver_style: bool,
        arity: usize,
    ) -> Vec<LazyOverload<'this>> {
        ffi::FunctionRegistry_find_lazy_overloads_by_arity(self, name, receiver_style, arity)
    }

    pub fn list_functions<'this>(&'this self) -> FunctionRegistryIterator<'this> {
        ffi::FunctionRegistry_list_functions(self)
    }
}

// FunctionRegistryIterator
#[repr(C)]
pub struct FunctionRegistryIterator<'a>(crate::Rep<'a, usize, 1>);

unsafe impl<'a> cxx::ExternType for FunctionRegistryIterator<'a> {
    type Id = cxx::type_id!("rust::cel_cxx::FunctionRegistryIterator");
    type Kind = cxx::kind::Trivial;
}

impl<'a> Drop for FunctionRegistryIterator<'a> {
    fn drop(&mut self) {
        ffi::FunctionRegistryIterator_drop(self);
    }
}

//unsafe impl<'a> Send for FunctionRegistryIterator<'a> {}
//unsafe impl<'a> Sync for FunctionRegistryIterator<'a> {}

impl<'a> std::iter::Iterator for FunctionRegistryIterator<'a> {
    type Item = (
        cxx::UniquePtr<cxx::CxxString>,
        cxx::UniquePtr<cxx::CxxVector<FunctionDescriptor>>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        let mut key = cxx::UniquePtr::null();
        let mut value = cxx::UniquePtr::null();
        if ffi::FunctionRegistryIterator_next(self, &mut key, &mut value) {
            Some((key, value))
        } else {
            None
        }
    }
}

// Program
pub use ffi::Program;
unsafe impl<'a, 'f> Send for Program<'a, 'f> {}
unsafe impl<'a, 'f> Sync for Program<'a, 'f> {}

impl<'a, 'f> Program<'a, 'f> {
    pub fn evaluate<'a2, 'f2>(
        &self,
        arena: &'a2 Arena,
        message_factory: Option<&MessageFactory>,
        activation: &Activation<'f2>,
    ) -> Result<cxx::UniquePtr<Value<'a2>>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = match message_factory {
            Some(message_factory) => {
                ffi::Program_evaluate(self, arena, message_factory, activation, &mut result)
            }
            None => {
                ffi::Program_evaluate2(self, arena, activation, &mut result)
            }
        };
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }
}

impl<'a, 'f> std::fmt::Debug for Program<'a, 'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = self as *const Program<'a, 'f>;
        write!(f, "Program {{ ptr: {:p} }}", ptr)
    }
}

// Function
pub use ffi::Function;
unsafe impl<'f> Send for Function<'f> {}
unsafe impl<'f> Sync for Function<'f> {}

impl<'f> Function<'f> {
    pub fn new<T: FfiFunction + 'f>(ffi_function: T) -> cxx::UniquePtr<Self> {
        ffi::Function_new(Box::new(AnyFfiFunction::new(ffi_function)))
    }
}

// FfiFunction
pub trait FfiFunction {
    fn invoke<'a>(
        &self,
        args: Span<'_, Value<'a>>,
        descriptor_pool: &'a DescriptorPool,
        message_factory: &'a MessageFactory,
        arena: &'a Arena,
        overload_id: Span<'_, cxx::CxxString>,
    ) -> Result<cxx::UniquePtr<Value<'a>>, Status>;
}

// AnyFfiFunction
struct AnyFfiFunction<'f>(Box<dyn FfiFunction + 'f>);

impl<'f> AnyFfiFunction<'f> {
    fn new<T: FfiFunction + 'f>(function: T) -> Self {
        Self(Box::new(function))
    }

    fn invoke<'a>(
        &self,
        args: Span<'_, Value<'a>>,
        descriptor_pool: &'a DescriptorPool,
        message_factory: &'a MessageFactory,
        arena: &'a Arena,
        overload_id: Span<'_, cxx::CxxString>,
        result: Pin<&mut Value<'a>>,
    ) -> Status {
        match self.0.invoke(args, descriptor_pool, message_factory, arena, overload_id) {
            Ok(mut value) => {
                result.swap(value.pin_mut());
                Status::ok()
            }
            Err(status) => status,
        }
    }
}

// TypeRegistry
pub use ffi::TypeRegistry;
unsafe impl<'d, 'a> Send for TypeRegistry<'d, 'a> {}
unsafe impl<'d, 'a> Sync for TypeRegistry<'d, 'a> {}

