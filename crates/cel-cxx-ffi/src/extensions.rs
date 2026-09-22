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
        type FunctionRegistry<'a, 'f> = super::FunctionRegistry<'a, 'f>;
        type RuntimeBuilder<'a, 'f> = super::RuntimeBuilder<'a, 'f>;
        type RuntimeOptions = super::RuntimeOptions;
        type CompilerLibrary = super::CompilerLibrary;
    }

    #[namespace = "cel::extensions"]
    unsafe extern "C++" {
        include!(<extensions/comprehensions_v2.h>);
        #[rust_name = "register_comprehensions_v2_functions"]
        fn RegisterComprehensionsV2Functions<'a, 'f>(
            function_registry: Pin<&mut FunctionRegistry<'a, 'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;

        include!(<extensions/encoders.h>);
        #[rust_name = "register_encoders_functions"]
        fn RegisterEncodersFunctions<'a, 'f>(
            function_registry: Pin<&mut FunctionRegistry<'a, 'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;

        include!(<extensions/regex_ext.h>);
        #[rust_name = "register_regex_extension_functions"]
        fn RegisterRegexExtensionFunctions<'a, 'f>(
            runtime_builder: Pin<&mut RuntimeBuilder<'a, 'f>>,
        ) -> Status;

        include!(<extensions/regex_functions.h>);
        #[rust_name = "register_regex_functions"]
        fn RegisterRegexFunctions<'a, 'f>(
            function_registry: Pin<&mut FunctionRegistry<'a, 'f>>,
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
        fn RegisterSetsFunctions<'a, 'f>(
            function_registry: Pin<&mut FunctionRegistry<'a, 'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;

        // CXX requires explicit version arguments because it binds exact
        // function pointer signatures and cannot apply C++ defaults.
        include!(<extensions/math_ext.h>);
        #[rust_name = "register_math_functions"]
        fn RegisterMathExtensionFunctions<'a, 'f>(
            function_registry: Pin<&mut FunctionRegistry<'a, 'f>>,
            runtime_options: &RuntimeOptions,
            version: i32,
        ) -> Status;
        include!(<extensions/lists_functions.h>);
        #[rust_name = "register_lists_functions"]
        fn RegisterListsFunctions<'a, 'f>(
            function_registry: Pin<&mut FunctionRegistry<'a, 'f>>,
            runtime_options: &RuntimeOptions,
            version: i32,
        ) -> Status;

        include!(<extensions/strings.h>);
        type StringsExtensionOptions;
        #[rust_name = "register_strings_functions"]
        fn RegisterStringsFunctions<'a, 'f>(
            function_registry: Pin<&mut FunctionRegistry<'a, 'f>>,
            runtime_options: &RuntimeOptions,
            options: &StringsExtensionOptions,
        ) -> Status;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/absl.h>);
        include!(<cel-cxx-ffi/include/extensions.h>);

        #[rust_name = "math_extension_latest_version"]
        fn MathExtensionLatestVersion() -> i32;
        #[rust_name = "lists_extension_latest_version"]
        fn ListsExtensionLatestVersion() -> i32;
        #[rust_name = "strings_extension_latest_version"]
        fn StringsExtensionLatestVersion() -> i32;
        fn StringsExtensionOptions_new() -> UniquePtr<StringsExtensionOptions>;
        fn StringsExtensionOptions_version(options: &StringsExtensionOptions) -> i32;
        fn StringsExtensionOptions_version_mut(
            options: Pin<&mut StringsExtensionOptions>,
        ) -> &mut i32;
        fn StringsExtensionOptions_max_precision(options: &StringsExtensionOptions) -> i32;
        fn StringsExtensionOptions_max_precision_mut(
            options: Pin<&mut StringsExtensionOptions>,
        ) -> &mut i32;

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
        fn ListsCompilerLibrary(version: i32) -> UniquePtr<CompilerLibrary>;

        // math_ext_decls.h
        #[rust_name = "math_compiler_library"]
        fn MathCompilerLibrary(version: i32) -> UniquePtr<CompilerLibrary>;

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
        fn SelectOptimizationOptions_force_fallback_implementation(
            options: &SelectOptimizationOptions,
        ) -> bool;
        fn SelectOptimizationOptions_force_fallback_implementation_mut(
            options: Pin<&mut SelectOptimizationOptions>,
        ) -> &mut bool;

        // sets_functions.h
        #[rust_name = "sets_compiler_library"]
        fn SetsCompilerLibrary() -> UniquePtr<CompilerLibrary>;

        // strings.h
        #[rust_name = "strings_compiler_library"]
        fn StringsCompilerLibrary(options: &StringsExtensionOptions) -> UniquePtr<CompilerLibrary>;
    }
}

// bindings_ext.h
pub mod bindings {
    pub use super::ffi::bindings_compiler_library as compiler_library;
}

// encoders.h
pub mod encoders {
    pub use super::ffi::encoders_compiler_library as compiler_library;

    pub fn register_functions<'a, 'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'a, 'f>>,
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

    pub fn register_functions<'a, 'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'a, 'f>>,
        runtime_options: &super::RuntimeOptions,
    ) -> Result<(), super::Status> {
        let status =
            super::ffi::register_comprehensions_v2_functions(function_registry, runtime_options);
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

    /// The latest extension version supported by the cel-cpp lists extension.
    pub fn latest_version() -> i32 {
        super::ffi::lists_extension_latest_version()
    }

    pub fn register_functions<'a, 'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'a, 'f>>,
        runtime_options: &super::RuntimeOptions,
        version: i32,
    ) -> Result<(), super::Status> {
        let status =
            super::ffi::register_lists_functions(function_registry, runtime_options, version);
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

    /// The latest extension version supported by the cel-cpp math extension.
    pub fn latest_version() -> i32 {
        super::ffi::math_extension_latest_version()
    }

    pub fn register_functions<'a, 'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'a, 'f>>,
        runtime_options: &super::RuntimeOptions,
        version: i32,
    ) -> Result<(), super::Status> {
        let status =
            super::ffi::register_math_functions(function_registry, runtime_options, version);
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

    pub fn register_functions<'a, 'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'a, 'f>>,
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

    pub fn register_functions<'a, 'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'a, 'f>>,
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
    pub use super::ffi::StringsExtensionOptions;

    impl StringsExtensionOptions {
        pub fn new() -> cxx::UniquePtr<Self> {
            super::ffi::StringsExtensionOptions_new()
        }

        pub fn version(&self) -> i32 {
            super::ffi::StringsExtensionOptions_version(self)
        }

        pub fn version_mut<'a>(self: std::pin::Pin<&'a mut Self>) -> &'a mut i32 {
            super::ffi::StringsExtensionOptions_version_mut(self)
        }

        pub fn max_precision(&self) -> i32 {
            super::ffi::StringsExtensionOptions_max_precision(self)
        }

        pub fn max_precision_mut<'a>(self: std::pin::Pin<&'a mut Self>) -> &'a mut i32 {
            super::ffi::StringsExtensionOptions_max_precision_mut(self)
        }
    }

    /// The latest extension version supported by the cel-cpp strings extension.
    pub fn latest_version() -> i32 {
        super::ffi::strings_extension_latest_version()
    }

    pub fn register_functions<'a, 'f>(
        function_registry: std::pin::Pin<&mut super::FunctionRegistry<'a, 'f>>,
        runtime_options: &super::RuntimeOptions,
        options: &StringsExtensionOptions,
    ) -> Result<(), super::Status> {
        let status =
            super::ffi::register_strings_functions(function_registry, runtime_options, options);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
