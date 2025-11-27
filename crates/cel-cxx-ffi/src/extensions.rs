use crate::absl::Status;
use crate::compiler::CompilerLibrary;
use crate::runtime::{FunctionRegistry, RuntimeBuilder, RuntimeOptions};

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        type Status = super::Status;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        type FunctionRegistry<'f> = super::FunctionRegistry<'f>;
        type RuntimeBuilder<'a, 'f> = super::RuntimeBuilder<'a, 'f>;
        type RuntimeOptions = super::RuntimeOptions;
        type CompilerLibrary = super::CompilerLibrary;
    }

    #[namespace = "cel::extensions"]
    unsafe extern "C++" {
        include!(<extensions/comprehensions_v2.h>);
        #[rust_name = "register_comprehensions_v2_functions"]
        fn RegisterComprehensionsV2Functions<'f>(
            function_registry: Pin<&mut FunctionRegistry<'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;

        include!(<extensions/encoders.h>);
        #[rust_name = "register_encoders_functions"]
        fn RegisterEncodersFunctions<'f>(
            function_registry: Pin<&mut FunctionRegistry<'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;

        include!(<extensions/lists_functions.h>);
        #[rust_name = "register_lists_functions"]
        fn RegisterListsFunctions<'f>(
            function_registry: Pin<&mut FunctionRegistry<'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;

        include!(<extensions/math_ext.h>);
        #[rust_name = "register_math_functions"]
        fn RegisterMathExtensionFunctions<'f>(
            function_registry: Pin<&mut FunctionRegistry<'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;

        include!(<extensions/regex_ext.h>);
        #[rust_name = "register_regex_extension_functions"]
        fn RegisterRegexExtensionFunctions<'a, 'f>(
            runtime_builder: Pin<&mut RuntimeBuilder<'a, 'f>>,
        ) -> Status;

        include!(<extensions/regex_functions.h>);
        #[rust_name = "register_regex_functions"]
        fn RegisterRegexFunctions<'f>(
            function_registry: Pin<&mut FunctionRegistry<'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;

        include!(<extensions/select_optimization.h>);
        type SelectOptimizationOptions;
        #[rust_name = "enable_select_optimization"]
        fn EnableSelectOptimization<'a, 'f>(
            builder: Pin<&mut RuntimeBuilder<'a, 'f>>,
            options: &SelectOptimizationOptions,
        ) -> Status;

        include!(<extensions/sets_functions.h>);
        #[rust_name = "register_sets_functions"]
        fn RegisterSetsFunctions<'f>(
            function_registry: Pin<&mut FunctionRegistry<'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;

        include!(<extensions/strings.h>);
        #[rust_name = "register_strings_functions"]
        fn RegisterStringsFunctions<'f>(
            function_registry: Pin<&mut FunctionRegistry<'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/absl.h>);
        include!(<cel-cxx-ffi/include/extensions.h>);

        // bindings_ext.h
        #[rust_name = "bindings_compiler_library"]
        fn BindingsCompilerLibrary() -> UniquePtr<CompilerLibrary>;

        // comprehensions_v2.h
        #[rust_name = "comprehensions_v2_compiler_library"]
        fn ComprehensionsV2CompilerLibrary() -> UniquePtr<CompilerLibrary>;

        // encoders.h
        #[rust_name = "encoders_compiler_library"]
        fn EncodersCompilerLibrary() -> UniquePtr<CompilerLibrary>;

        // lists_functions.h
        #[rust_name = "lists_compiler_library"]
        fn ListsCompilerLibrary() -> UniquePtr<CompilerLibrary>;

        // math_ext_decls.h
        #[rust_name = "math_compiler_library"]
        fn MathCompilerLibrary() -> UniquePtr<CompilerLibrary>;

        // proto_ext.h
        #[rust_name = "proto_ext_compiler_library"]
        fn ProtoExtCompilerLibrary() -> UniquePtr<CompilerLibrary>;

        // regex_ext.h
        #[rust_name = "regex_extension_compiler_library"]
        fn RegexExtensionCompilerLibrary() -> UniquePtr<CompilerLibrary>;

        // regex_functions.h
        #[rust_name = "regex_compiler_library"]
        fn RegexCompilerLibrary() -> UniquePtr<CompilerLibrary>;

        // SelectOptimizationOptions
        fn SelectOptimizationOptions_new() -> UniquePtr<SelectOptimizationOptions>;

        // SelectOptimizationOptions getters and setters
        fn SelectOptimizationOptions_force_fallback_implementation(options: &SelectOptimizationOptions) -> bool;
        fn SelectOptimizationOptions_force_fallback_implementation_mut(options: Pin<&mut SelectOptimizationOptions>) -> &mut bool;

        // sets_functions.h
        #[rust_name = "sets_compiler_library"]
        fn SetsCompilerLibrary() -> UniquePtr<CompilerLibrary>;

        // strings.h
        #[rust_name = "strings_compiler_library"]
        fn StringsCompilerLibrary() -> UniquePtr<CompilerLibrary>;
    }
}


// bindings_ext.h
pub mod bindings {
    pub use super::ffi::bindings_compiler_library as compiler_library;
}

// encoders.h
pub mod encoders {
    pub use super::ffi::encoders_compiler_library as compiler_library;

    pub fn register_functions<'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'f>>,
        runtime_options: &super::RuntimeOptions,
    ) -> Result<(), super::Status> {
        let status = super::ffi::register_encoders_functions(function_registry, runtime_options);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// comprehensions_v2.h
pub mod comprehensions {
    pub use super::ffi::comprehensions_v2_compiler_library as compiler_library;

    pub fn register_functions<'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'f>>,
        runtime_options: &super::RuntimeOptions,
    ) -> Result<(), super::Status> {
        let status = super::ffi::register_comprehensions_v2_functions(function_registry, runtime_options);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// lists_functions.h
pub mod lists {
    pub use super::ffi::lists_compiler_library as compiler_library;

    pub fn register_functions<'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'f>>,
        runtime_options: &super::RuntimeOptions,
    ) -> Result<(), super::Status> {
        let status = super::ffi::register_lists_functions(function_registry, runtime_options);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// math_ext.h
pub mod math {
    pub use super::ffi::math_compiler_library as compiler_library;

    pub fn register_functions<'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'f>>,
        runtime_options: &super::RuntimeOptions,
    ) -> Result<(), super::Status> {
        let status = super::ffi::register_math_functions(function_registry, runtime_options);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// proto_ext.h
pub mod proto {
    pub use super::ffi::proto_ext_compiler_library as compiler_library;
}

// regex_functions.h
pub mod re {
    pub use super::ffi::regex_compiler_library as compiler_library;

    pub fn register_functions<'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'f>>,
        runtime_options: &super::RuntimeOptions,
    ) -> Result<(), super::Status> {
        let status = super::ffi::register_regex_functions(function_registry, runtime_options);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// regex_ext.h
pub mod regex {
    pub use super::ffi::regex_extension_compiler_library as compiler_library;

    pub fn register_functions<'a, 'f>(
        runtime_builder: std::pin::Pin<&mut super::RuntimeBuilder<'a, 'f>>,
    ) -> Result<(), super::Status> {
        let status = super::ffi::register_regex_extension_functions(runtime_builder);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// select_optimization.h
pub mod select_optimization {
    use std::pin::Pin;

    pub use super::ffi::SelectOptimizationOptions as Options;
    unsafe impl Send for Options {}
    unsafe impl Sync for Options {}

    impl Options {
        pub fn new() -> cxx::UniquePtr<Self> {
            super::ffi::SelectOptimizationOptions_new()
        }

        pub fn force_fallback_implementation(&self) -> bool {
            super::ffi::SelectOptimizationOptions_force_fallback_implementation(self)
        }

        pub fn force_fallback_implementation_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut bool {
            super::ffi::SelectOptimizationOptions_force_fallback_implementation_mut(self)
        }
    }

    pub fn enable<'a, 'f>(
        builder: std::pin::Pin<&mut super::RuntimeBuilder<'a, 'f>>,
        options: &Options,
    ) -> Result<(), super::Status> {
        let status = super::ffi::enable_select_optimization(builder, options);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// sets_functions.h
pub mod sets {
    pub use super::ffi::sets_compiler_library as compiler_library;

    pub fn register_functions<'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'f>>,
        runtime_options: &super::RuntimeOptions,
    ) -> Result<(), super::Status> {
        let status = super::ffi::register_sets_functions(function_registry, runtime_options);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// strings.h
pub mod strings {
    pub use super::ffi::strings_compiler_library as compiler_library;

    pub fn register_functions<'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'f>>,
        runtime_options: &super::RuntimeOptions,
    ) -> Result<(), super::Status> {
        let status = super::ffi::register_strings_functions(function_registry, runtime_options);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
