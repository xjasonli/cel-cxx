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

        // use our hacked version of RegisterStringsFunctions in extensions.h
        //
        //include!(<extensions/strings.h>);
        //#[rust_name = "register_strings_functions"]
        //fn RegisterStringsFunctions<'f>(
        //    function_registry: Pin<&mut FunctionRegistry<'f>>,
        //    runtime_options: &RuntimeOptions,
        //) -> Status;
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

        #[rust_name = "register_strings_functions"]
        fn RegisterStringsFunctions<'f>(
            function_registry: Pin<&mut FunctionRegistry<'f>>,
            runtime_options: &RuntimeOptions,
        ) -> Status;
    }

    #[namespace = "rust::cel_cxx"]
    extern "Rust" {
        #[cxx_name = "CharAtImpl"]
        fn char_at(
            result: &mut String,
            string: &str,
            index: i64,
        ) -> Status;

        #[cxx_name = "IndexOfImpl"]
        fn index_of(
            result: &mut i64,
            string: &str,
            substring: &str,
            offset: i64,
        ) -> Status;

        #[cxx_name = "LastIndexOfImpl"]
        fn last_index_of(
            result: &mut i64,
            string: &str,
            substring: &str,
            offset: i64,
        ) -> Status;

        #[cxx_name = "StringsQuoteImpl"]
        fn strings_quote(
            result: &mut String,
            string: &str,
        ) -> Status;

        #[cxx_name = "SubstringImpl"]
        fn substring(
            result: &mut String,
            string: &str,
            start: i64,
            end: i64,
        ) -> Status;

        #[cxx_name = "TrimImpl"]
        fn trim(
            result: &mut String,
            string: &str,
        ) -> Status;

        #[cxx_name = "ReverseImpl"]
        fn reverse(
            result: &mut String,
            string: &str,
        ) -> Status;
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

fn char_at(
    result: &mut String,
    string: &str,
    index: i64,
) -> Status {
    let index = index as usize;
    if let Some(char) = string.chars().nth(index) {
        *result = char.to_string();
        Status::ok()
    } else {
        Status::invalid_argument("index out of bounds")
    }
}

fn index_of(
    result: &mut i64,
    string: &str,
    substring: &str,
    offset: i64,
) -> Status {
    if offset < 0 {
        return Status::invalid_argument("index out of range: {offset}");
    }
    let offset = offset as usize;
    if offset >= string.len() {
        *result = -1;
        return Status::ok();
    }

    let target = &string[offset..];
    let index = target.find(substring);
    if let Some(index) = index {
        *result = offset as i64 + index as i64;
    } else {
        *result = -1;
    }
    Status::ok()
}

fn last_index_of(
    result: &mut i64,
    string: &str,
    substring: &str,
    offset: i64,
) -> Status { 
    if offset < 0 {
        return Status::invalid_argument("index out of range: {offset}");
    }
    let offset = offset as usize;
    if offset >= string.len() {
        *result = -1;
        return Status::ok();
    }

    let target = &string[..offset + 1];
    let index = target.rfind(&substring);
    if let Some(index) = index {
        *result = index as i64;
    } else {
        *result = -1;
    }
    Status::ok()
}

// strings.quote
// https://github.com/google/cel-go/blob/master/ext/strings.go#L751
fn strings_quote(
    result: &mut String,
    string: &str,
) -> Status {
    result.clear();
    result.push('"');
    string.chars().for_each(|c| {
        match c {
            '\x07' => result.push_str("\\a"),
            '\x08' => result.push_str("\\b"),
            '\x0C' => result.push_str("\\f"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\x0B' => result.push_str("\\v"),
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            _ => result.push(c),
        }
    });
    result.push('"');
    Status::ok()
}

fn substring(
    result: &mut String,
    string: &str,
    start: i64,
    end: i64,
) -> Status {
    if start > end {
        return Status::invalid_argument("invalid substring range: start: {start}, end: {end}");
    }
    if start < 0 || start >= string.len() as i64 {
        return Status::invalid_argument("index out of range: {start}");
    }
    if end < 0 || end > string.len() as i64 {
        return Status::invalid_argument("index out of range: {end}");
    }
    *result = string[start as usize..end as usize].to_string();
    Status::ok()
}

fn trim(result: &mut String, string: &str) -> Status {
    *result = string.trim().to_string();
    Status::ok()
}

fn reverse(result: &mut String, string: &str) -> Status {
    *result = string.chars().rev().collect();
    Status::ok()
}
