use crate::absl::StringView;
use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
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

        type ParserOptions;
        type ParserBuilder;
        type Parser;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!("cel-cxx-ffi/include/absl.h");
        include!("cel-cxx-ffi/include/parser.h");

        // ParserOptions
        fn ParserOptions_new() -> UniquePtr<ParserOptions>;

        // ParserOptions getters and setters
        fn ParserOptions_error_recovery_limit(parser_options: &ParserOptions) -> i32;
        fn ParserOptions_error_recovery_limit_mut(
            parser_options: Pin<&mut ParserOptions>,
        ) -> &mut i32;
        fn ParserOptions_max_recursion_depth(parser_options: &ParserOptions) -> i32;
        fn ParserOptions_max_recursion_depth_mut(
            parser_options: Pin<&mut ParserOptions>,
        ) -> &mut i32;
        fn ParserOptions_expression_size_codepoint_limit(parser_options: &ParserOptions) -> i32;
        fn ParserOptions_expression_size_codepoint_limit_mut(
            parser_options: Pin<&mut ParserOptions>,
        ) -> &mut i32;
        fn ParserOptions_error_recovery_token_lookahead_limit(
            parser_options: &ParserOptions,
        ) -> i32;
        fn ParserOptions_error_recovery_token_lookahead_limit_mut(
            parser_options: Pin<&mut ParserOptions>,
        ) -> &mut i32;
        fn ParserOptions_add_macro_calls(parser_options: &ParserOptions) -> bool;
        fn ParserOptions_add_macro_calls_mut(
            parser_options: Pin<&mut ParserOptions>,
        ) -> &mut bool;
        fn ParserOptions_enable_optional_syntax(parser_options: &ParserOptions) -> bool;
        fn ParserOptions_enable_optional_syntax_mut(
            parser_options: Pin<&mut ParserOptions>,
        ) -> &mut bool;
        fn ParserOptions_disable_standard_macros(parser_options: &ParserOptions) -> bool;
        fn ParserOptions_disable_standard_macros_mut(
            parser_options: Pin<&mut ParserOptions>,
        ) -> &mut bool;
        fn ParserOptions_enable_hidden_accumulator_var(parser_options: &ParserOptions) -> bool;
        fn ParserOptions_enable_hidden_accumulator_var_mut(
            parser_options: Pin<&mut ParserOptions>,
        ) -> &mut bool;
        fn ParserOptions_enable_quoted_identifiers(parser_options: &ParserOptions) -> bool;
        fn ParserOptions_enable_quoted_identifiers_mut(
            parser_options: Pin<&mut ParserOptions>,
        ) -> &mut bool;
    }
}

// Macro
pub use ffi::Macro;
unsafe impl Send for Macro {}
unsafe impl Sync for Macro {}

pub use ffi::ParserOptions;
unsafe impl Send for ParserOptions {}
unsafe impl Sync for ParserOptions {}

impl ParserOptions {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::ParserOptions_new()
    }

    pub fn error_recovery_limit(&self) -> i32 {
        ffi::ParserOptions_error_recovery_limit(self)
    }

    pub fn error_recovery_limit_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
        ffi::ParserOptions_error_recovery_limit_mut(self)
    }

    pub fn max_recursion_depth(&self) -> i32 {
        ffi::ParserOptions_max_recursion_depth(self)
    }

    pub fn max_recursion_depth_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
        ffi::ParserOptions_max_recursion_depth_mut(self)
    }

    pub fn expression_size_codepoint_limit(&self) -> i32 {
        ffi::ParserOptions_expression_size_codepoint_limit(self)
    }

    pub fn expression_size_codepoint_limit_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
        ffi::ParserOptions_expression_size_codepoint_limit_mut(self)
    }

    pub fn error_recovery_token_lookahead_limit(&self) -> i32 {
        ffi::ParserOptions_error_recovery_token_lookahead_limit(self)
    }

    pub fn error_recovery_token_lookahead_limit_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
        ffi::ParserOptions_error_recovery_token_lookahead_limit_mut(self)
    }

    pub fn add_macro_calls(&self) -> bool {
        ffi::ParserOptions_add_macro_calls(self)
    }

    pub fn add_macro_calls_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut bool {
        ffi::ParserOptions_add_macro_calls_mut(self)
    }

    pub fn enable_optional_syntax(&self) -> bool {
        ffi::ParserOptions_enable_optional_syntax(self)
    }

    pub fn enable_optional_syntax_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut bool {
        ffi::ParserOptions_enable_optional_syntax_mut(self)
    }

    pub fn disable_standard_macros(&self) -> bool {
        ffi::ParserOptions_disable_standard_macros(self)
    }

    pub fn disable_standard_macros_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut bool {
        ffi::ParserOptions_disable_standard_macros_mut(self)
    }

    pub fn enable_hidden_accumulator_var(&self) -> bool {
        ffi::ParserOptions_enable_hidden_accumulator_var(self)
    }

    pub fn enable_hidden_accumulator_var_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut bool {
        ffi::ParserOptions_enable_hidden_accumulator_var_mut(self)
    }

    pub fn enable_quoted_identifiers(&self) -> bool {
        ffi::ParserOptions_enable_quoted_identifiers(self)
    }

    pub fn enable_quoted_identifiers_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut bool {
        ffi::ParserOptions_enable_quoted_identifiers_mut(self)
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
        write!(f, "Parser {{ ptr: {ptr:p} }}")
    }
}
