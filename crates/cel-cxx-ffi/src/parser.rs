use crate::absl::{Status, StringView};
use std::ffi::c_int;
use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        type Status = super::Status;
        type string_view<'a> = super::StringView<'a>;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<parser/parser.h>);
        type Macro;
        fn function<'a>(self: &Macro) -> string_view<'a>;
        fn argument_count(self: &Macro) -> usize;
        fn is_receiver_style(self: &Macro) -> bool;
        fn is_variadic(self: &Macro) -> bool;
        fn key<'a>(self: &Macro) -> string_view<'a>;

        type MacroRegistry;
        #[rust_name = "register_macro"]
        fn RegisterMacro(self: Pin<&mut MacroRegistry>, macro_: &Macro) -> Status;

        type ParserOptions = super::ParserOptions;
        type ParserBuilder;
        type Parser;

        include!(<parser/standard_macros.h>);
        fn RegisterStandardMacros(
            macro_registry: Pin<&mut MacroRegistry>,
            parser_options: &ParserOptions,
        ) -> Status;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!("cel-cxx-ffi/include/absl.h");
        include!("cel-cxx-ffi/include/parser.h");

        // MacroRegistry
        fn MacroRegistry_new() -> UniquePtr<MacroRegistry>;

        // ParserOptions
        fn ParserOptions_default() -> ParserOptions;
    }
}

// Macro
pub use ffi::Macro;
unsafe impl Send for Macro {}
unsafe impl Sync for Macro {}

// MacroRegistry
pub use ffi::MacroRegistry;
unsafe impl Send for MacroRegistry {}
unsafe impl Sync for MacroRegistry {}

impl MacroRegistry {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::MacroRegistry_new()
    }

    pub fn register_standard(self: Pin<&mut Self>, parser_options: &ParserOptions) -> Status {
        ffi::RegisterStandardMacros(self, parser_options)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ParserOptions {
    pub error_recovery_limit: c_int,
    pub max_recursion_depth: c_int,
    pub expression_size_codepoint_limit: c_int,
    pub error_recovery_token_lookahead_limit: c_int,
    pub add_macro_calls: bool,
    pub enable_optional_syntax: bool,
    pub disable_standard_macros: bool,
    pub enable_hidden_accumulator_var: bool,
    pub enable_quoted_identifiers: bool,
}

unsafe impl cxx::ExternType for ParserOptions {
    type Id = cxx::type_id!("cel::ParserOptions");
    type Kind = cxx::kind::Trivial;
}

impl Default for ParserOptions {
    fn default() -> Self {
        ffi::ParserOptions_default()
    }
}

pub use ffi::ParserBuilder;
unsafe impl Send for ParserBuilder {}
unsafe impl Sync for ParserBuilder {}

// Parser
pub use ffi::Parser;
unsafe impl Send for Parser {}
unsafe impl Sync for Parser {}

impl std::fmt::Debug for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = self as *const Parser;
        write!(f, "Parser {{ ptr: {:p} }}", ptr)
    }
}
