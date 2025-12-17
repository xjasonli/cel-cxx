use crate::absl::{Status, StringView, MutSpan};
use crate::common::{Expr, ListExprElement, StructExprField, MapExprEntry};
use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/status/status.h>);
        type Status = super::Status;

        include!(<absl/strings/string_view.h>);
        type string_view<'a> = super::StringView<'a>;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<parser/parser.h>);
        type GlobalMacroExpander;
        type ReceiverMacroExpander;

        type Macro;
        fn function<'a>(self: &Macro) -> string_view<'a>;
        fn argument_count(self: &Macro) -> usize;
        fn is_receiver_style(self: &Macro) -> bool;
        fn is_variadic(self: &Macro) -> bool;
        fn key<'a>(self: &Macro) -> string_view<'a>;

        type MacroExprFactory;
        #[rust_name = "accu_var_name"]
        fn AccuVarName<'a>(self: Pin<&'a mut MacroExprFactory>) -> string_view<'a>;

        type ParserOptions;
        type ParserBuilderConfigurer;
        type ParserBuilder;
        #[rust_name = "add_macro"]
        fn AddMacro(self: Pin<&mut ParserBuilder>, m: &Macro) -> Status;

        type Parser;
        type ParserLibrary;
        type ParserLibrarySubset;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<common/expr.h>);
        type Expr = super::Expr;
        type ListExprElement = super::ListExprElement;
        type StructExprField = super::StructExprField;
        type MapExprEntry = super::MapExprEntry;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/absl.h>);
        include!(<cel-cxx-ffi/include/parser.h>);

        type MacroPredicate;

        type MutSpan_Expr<'a> = super::MutSpan<'a, Expr>;
        type MutSpan_ListExprElement<'a> = super::MutSpan<'a, ListExprElement>;
        type MutSpan_StructExprField<'a> = super::MutSpan<'a, StructExprField>;
        type MutSpan_MapExprEntry<'a> = super::MutSpan<'a, MapExprEntry>;

        // GlobalMacroExpander
        fn GlobalMacroExpander_new(ffi_expander: Box<AnyFfiGlobalMacroExpander>) -> UniquePtr<GlobalMacroExpander>;

        // ReceiverMacroExpander
        fn ReceiverMacroExpander_new(ffi_expander: Box<AnyFfiReceiverMacroExpander>) -> UniquePtr<ReceiverMacroExpander>;

        // Macro
        fn Macro_new_global<'a, 'f>(
            name: string_view<'a>,
            argument_count: usize,
            expander: UniquePtr<GlobalMacroExpander>,
            result: &mut UniquePtr<Macro>,
        ) -> Status;

        fn Macro_new_global_var_arg<'a, 'f>(
            name: string_view<'a>,
            expander: UniquePtr<GlobalMacroExpander>,
            result: &mut UniquePtr<Macro>,
        ) -> Status;

        fn Macro_new_receiver<'a, 'f>(
            name: string_view<'a>,
            argument_count: usize,
            expander: UniquePtr<ReceiverMacroExpander>,
            result: &mut UniquePtr<Macro>,
        ) -> Status;

        fn Macro_new_receiver_var_arg<'a, 'f>(
            name: string_view<'a>,
            expander: UniquePtr<ReceiverMacroExpander>,
            result: &mut UniquePtr<Macro>,
        ) -> Status;

        // MacroExprFactory
        fn MacroExprFactory_copy(
            factory: Pin<&mut MacroExprFactory>,
            expr: &Expr,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_copy_list_element(
            factory: Pin<&mut MacroExprFactory>,
            list_element: &ListExprElement,
        ) -> UniquePtr<ListExprElement>;
        fn MacroExprFactory_copy_struct_field(
            factory: Pin<&mut MacroExprFactory>,
            struct_field: &StructExprField,
        ) -> UniquePtr<StructExprField>;
        fn MacroExprFactory_copy_map_entry(
            factory: Pin<&mut MacroExprFactory>,
            map_entry: &MapExprEntry,
        ) -> UniquePtr<MapExprEntry>;
        fn MacroExprFactory_new_unspecified(
            factory: Pin<&mut MacroExprFactory>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_null_const(
            factory: Pin<&mut MacroExprFactory>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_bool_const(
            factory: Pin<&mut MacroExprFactory>,
            value: bool,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_int_const(
            factory: Pin<&mut MacroExprFactory>,
            value: i64,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_double_const(
            factory: Pin<&mut MacroExprFactory>,
            value: f64,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_bytes_const<'a>(
            factory: Pin<&mut MacroExprFactory>,
            value: string_view<'a>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_string_const<'a>(
            factory: Pin<&mut MacroExprFactory>,
            value: string_view<'a>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_ident<'a>(
            factory: Pin<&mut MacroExprFactory>,
            name: string_view<'a>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_accu_ident(
            factory: Pin<&mut MacroExprFactory>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_select<'a>(
            factory: Pin<&mut MacroExprFactory>,
            operand: UniquePtr<Expr>,
            field: string_view<'a>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_presence_test<'a>(
            factory: Pin<&mut MacroExprFactory>,
            operand: UniquePtr<Expr>,
            field: string_view<'a>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_call<'a>(
            factory: Pin<&mut MacroExprFactory>,
            function: string_view<'a>,
            args: UniquePtr<CxxVector<Expr>>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_member_call<'a>(
            factory: Pin<&mut MacroExprFactory>,
            function: string_view<'a>,
            target: UniquePtr<Expr>,
            args: UniquePtr<CxxVector<Expr>>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_list_element(
            factory: Pin<&mut MacroExprFactory>,
            expr: UniquePtr<Expr>,
            optional: bool,
        ) -> UniquePtr<ListExprElement>;
        fn MacroExprFactory_new_list(
            factory: Pin<&mut MacroExprFactory>,
            elements: UniquePtr<CxxVector<ListExprElement>>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_struct_field<'a>(
            factory: Pin<&mut MacroExprFactory>,
            name: string_view<'a>,
            value: UniquePtr<Expr>,
            optional: bool,
        ) -> UniquePtr<StructExprField>;
        fn MacroExprFactory_new_struct<'a>(
            factory: Pin<&mut MacroExprFactory>,
            name: string_view<'a>,
            fields: UniquePtr<CxxVector<StructExprField>>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_map_entry(
            factory: Pin<&mut MacroExprFactory>,
            key: UniquePtr<Expr>,
            value: UniquePtr<Expr>,
            optional: bool,
        ) -> UniquePtr<MapExprEntry>;
        fn MacroExprFactory_new_map(
            factory: Pin<&mut MacroExprFactory>,
            entries: UniquePtr<CxxVector<MapExprEntry>>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_comprehension<'a>(
            factory: Pin<&mut MacroExprFactory>,
            iter_var: string_view<'a>,
            iter_range: UniquePtr<Expr>,
            accu_var: string_view<'a>,
            accu_init: UniquePtr<Expr>,
            loop_condition: UniquePtr<Expr>,
            loop_step: UniquePtr<Expr>,
            result: UniquePtr<Expr>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_new_comprehension2<'a>(
            factory: Pin<&mut MacroExprFactory>,
            iter_var: string_view<'a>,
            iter_var2: string_view<'a>,
            iter_range: UniquePtr<Expr>,
            accu_var: string_view<'a>,
            accu_init: UniquePtr<Expr>,
            loop_condition: UniquePtr<Expr>,
            loop_step: UniquePtr<Expr>,
            result: UniquePtr<Expr>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_report_error<'a>(
            factory: Pin<&mut MacroExprFactory>,
            message: string_view<'a>,
        ) -> UniquePtr<Expr>;
        fn MacroExprFactory_report_error_at<'a>(
            factory: Pin<&mut MacroExprFactory>,
            expr: &Expr,
            message: string_view<'a>,
        ) -> UniquePtr<Expr>;

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
        fn ParserOptions_add_macro_calls_mut(parser_options: Pin<&mut ParserOptions>) -> &mut bool;
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

        // ParserBuilderConfigurer
        fn ParserBuilderConfigurer_new(ffi_configurer: Box<AnyFfiParserBuilderConfigurer>) -> UniquePtr<ParserBuilderConfigurer>;

        // ParserLibrary
        fn ParserLibrary_new(id: &CxxString, configurer: UniquePtr<ParserBuilderConfigurer>) -> UniquePtr<ParserLibrary>;
        fn ParserLibrary_id<'a>(parser_library: &'a ParserLibrary) -> &'a CxxString;

        // MacroPredicate
        fn MacroPredicate_new(ffi_predicate: Box<AnyFfiMacroPredicate>) -> UniquePtr<MacroPredicate>;

        // ParserLibrarySubset
        fn ParserLibrarySubset_new(library_id: &CxxString, should_include_macro: UniquePtr<MacroPredicate>) -> UniquePtr<ParserLibrarySubset>;
        fn ParserLibrarySubset_library_id<'a>(parser_library_subset: &'a ParserLibrarySubset) -> &'a CxxString;

        // ParserBuilder
        fn ParserBuilder_add_library(
            parser_builder: Pin<&mut ParserBuilder>,
            library: UniquePtr<ParserLibrary>,
        ) -> Status;
        fn ParserBuilder_add_library_subset(
            parser_builder: Pin<&mut ParserBuilder>,
            library_subset: UniquePtr<ParserLibrarySubset>,
        ) -> Status;
    }

    #[namespace = "rust::cel_cxx"]
    extern "Rust" {
        #[derive(ExternType)]
        type AnyFfiParserBuilderConfigurer<'f>;
        #[cxx_name = "Call"]
        unsafe fn call<'f>(
            self: &AnyFfiParserBuilderConfigurer<'f>,
            parser_builder: Pin<&mut ParserBuilder>,
        ) -> Status;

        #[derive(ExternType)]
        type AnyFfiMacroPredicate<'f>;
        #[cxx_name = "Call"]
        unsafe fn call<'f>(
            self: &AnyFfiMacroPredicate<'f>,
            m: &Macro,
        ) -> bool;

        type AnyFfiGlobalMacroExpander<'f>;
        #[cxx_name = "Call"]
        unsafe fn call<'f>(
            self: &AnyFfiGlobalMacroExpander<'f>,
            factory: Pin<&mut MacroExprFactory>,
            args: &mut [UniquePtr<Expr>],
        ) -> UniquePtr<Expr>;

        type AnyFfiReceiverMacroExpander<'f>;
        #[cxx_name = "Call"]
        unsafe fn call<'f>(
            self: &AnyFfiReceiverMacroExpander<'f>,
            factory: Pin<&mut MacroExprFactory>,
            target: UniquePtr<Expr>,
            args: &mut [UniquePtr<Expr>],
        ) -> UniquePtr<Expr>;
    }

}

// GlobalMacroExpander
pub use ffi::GlobalMacroExpander;
unsafe impl Send for GlobalMacroExpander {}
unsafe impl Sync for GlobalMacroExpander {}

impl GlobalMacroExpander {
    pub fn new<F: FfiGlobalMacroExpander + 'static>(expander: F) -> cxx::UniquePtr<Self> {
        ffi::GlobalMacroExpander_new(Box::new(AnyFfiGlobalMacroExpander::new(expander)))
    }
}

pub trait FfiGlobalMacroExpander
    : Fn(Pin<&mut MacroExprFactory>, &mut [cxx::UniquePtr<Expr>]) -> cxx::UniquePtr<Expr>
{}

impl<'f, F> FfiGlobalMacroExpander for F where F
    : Fn(Pin<&mut MacroExprFactory>, &mut [cxx::UniquePtr<Expr>]) -> cxx::UniquePtr<Expr>
    + 'f
{}

struct AnyFfiGlobalMacroExpander<'f>(Box<dyn FfiGlobalMacroExpander + 'f>);
impl<'f> AnyFfiGlobalMacroExpander<'f> {
    fn new<T: FfiGlobalMacroExpander + 'f>(expander: T) -> Self {
        Self(Box::new(expander))
    }

    fn call(&self, factory: Pin<&mut MacroExprFactory>, args: &mut [cxx::UniquePtr<Expr>]) -> cxx::UniquePtr<Expr> {
        (self.0)(factory, args)
    }
}

// ReceiverMacroExpander
pub use ffi::ReceiverMacroExpander;
unsafe impl Send for ReceiverMacroExpander {}
unsafe impl Sync for ReceiverMacroExpander {}

impl ReceiverMacroExpander {
    pub fn new<F: FfiReceiverMacroExpander + 'static>(expander: F) -> cxx::UniquePtr<Self> {
        ffi::ReceiverMacroExpander_new(Box::new(AnyFfiReceiverMacroExpander::new(expander)))
    }
}

pub trait FfiReceiverMacroExpander
    : Fn(Pin<&mut MacroExprFactory>, cxx::UniquePtr<Expr>, &mut [cxx::UniquePtr<Expr>]) -> cxx::UniquePtr<Expr>
{}

impl<'f, F> FfiReceiverMacroExpander for F where F
    : Fn(Pin<&mut MacroExprFactory>, cxx::UniquePtr<Expr>, &mut [cxx::UniquePtr<Expr>]) -> cxx::UniquePtr<Expr>
    + 'f
{}

struct AnyFfiReceiverMacroExpander<'f>(Box<dyn FfiReceiverMacroExpander + 'f>);
impl<'f> AnyFfiReceiverMacroExpander<'f> {
    fn new<T: FfiReceiverMacroExpander + 'f>(expander: T) -> Self {
        Self(Box::new(expander))
    }

    fn call(&self, factory: Pin<&mut MacroExprFactory>, target: cxx::UniquePtr<Expr>, args: &mut [cxx::UniquePtr<Expr>]) -> cxx::UniquePtr<Expr> {
        (self.0)(factory, target, args)
    }
}

// Macro
pub use ffi::Macro;
unsafe impl Send for Macro {}
unsafe impl Sync for Macro {}

impl Macro {
    pub fn new_global(
        name: StringView<'_>,
        argument_count: usize,
        expander: cxx::UniquePtr<GlobalMacroExpander>,
    ) -> Result<cxx::UniquePtr<Self>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::Macro_new_global(name, argument_count, expander, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }

    pub fn new_global_var_arg(
        name: StringView<'_>,
        expander: cxx::UniquePtr<GlobalMacroExpander>,
    ) -> Result<cxx::UniquePtr<Self>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::Macro_new_global_var_arg(name, expander, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }

    pub fn new_receiver(
        name: StringView<'_>,
        argument_count: usize,
        expander: cxx::UniquePtr<ReceiverMacroExpander>,
    ) -> Result<cxx::UniquePtr<Self>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::Macro_new_receiver(name, argument_count, expander, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }

    pub fn new_receiver_var_arg(
        name: StringView<'_>,
        expander: cxx::UniquePtr<ReceiverMacroExpander>,
    ) -> Result<cxx::UniquePtr<Self>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::Macro_new_receiver_var_arg(name, expander, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }
}

// MacroExprFactory
pub use ffi::MacroExprFactory;
unsafe impl Send for MacroExprFactory {}
unsafe impl Sync for MacroExprFactory {}

impl MacroExprFactory {
    pub fn copy(self: Pin<&mut Self>, expr: &Expr) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_copy(self, expr)
    }

    pub fn copy_list_element(self: Pin<&mut Self>, list_element: &ListExprElement) -> cxx::UniquePtr<ListExprElement> {
        ffi::MacroExprFactory_copy_list_element(self, list_element)
    }

    pub fn copy_struct_field(self: Pin<&mut Self>, struct_field: &StructExprField) -> cxx::UniquePtr<StructExprField> {
        ffi::MacroExprFactory_copy_struct_field(self, struct_field)
    }

    pub fn copy_map_entry(self: Pin<&mut Self>, map_entry: &MapExprEntry) -> cxx::UniquePtr<MapExprEntry> {
        ffi::MacroExprFactory_copy_map_entry(self, map_entry)
    }

    pub fn new_unspecified(self: Pin<&mut Self>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_unspecified(self)
    }

    pub fn new_null_const(self: Pin<&mut Self>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_null_const(self)
    }
    
    pub fn new_bool_const(self: Pin<&mut Self>, value: bool) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_bool_const(self, value)
    }

    pub fn new_int_const(self: Pin<&mut Self>, value: i64) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_int_const(self, value)
    }

    pub fn new_double_const(self: Pin<&mut Self>, value: f64) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_double_const(self, value)
    }

    pub fn new_bytes_const(self: Pin<&mut Self>, value: StringView<'_>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_bytes_const(self, value)
    }

    pub fn new_string_const(self: Pin<&mut Self>, value: StringView<'_>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_string_const(self, value)
    }

    pub fn new_ident(self: Pin<&mut Self>, name: StringView<'_>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_ident(self, name)
    }

    pub fn new_accu_ident(self: Pin<&mut Self>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_accu_ident(self)
    }

    pub fn new_select(self: Pin<&mut Self>, operand: cxx::UniquePtr<Expr>, field: StringView<'_>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_select(self, operand, field)
    }

    pub fn new_presence_test(self: Pin<&mut Self>, operand: cxx::UniquePtr<Expr>, field: StringView<'_>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_presence_test(self, operand, field)
    }

    pub fn new_call(self: Pin<&mut Self>, function: StringView<'_>, args: cxx::UniquePtr<cxx::CxxVector<Expr>>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_call(self, function, args)
    }

    pub fn new_member_call(self: Pin<&mut Self>, function: StringView<'_>, target: cxx::UniquePtr<Expr>, args: cxx::UniquePtr<cxx::CxxVector<Expr>>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_member_call(self, function, target, args)
    }

    pub fn new_list_element(self: Pin<&mut Self>, expr: cxx::UniquePtr<Expr>, optional: bool) -> cxx::UniquePtr<ListExprElement> {
        ffi::MacroExprFactory_new_list_element(self, expr, optional)
    }

    pub fn new_list(self: Pin<&mut Self>, elements: cxx::UniquePtr<cxx::CxxVector<ListExprElement>>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_list(self, elements)
    }

    pub fn new_struct_field(self: Pin<&mut Self>, name: StringView<'_>, value: cxx::UniquePtr<Expr>, optional: bool) -> cxx::UniquePtr<StructExprField> {
        ffi::MacroExprFactory_new_struct_field(self, name, value, optional)
    }

    pub fn new_struct(self: Pin<&mut Self>, name: StringView<'_>, fields: cxx::UniquePtr<cxx::CxxVector<StructExprField>>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_struct(self, name, fields)
    }

    pub fn new_map_entry(self: Pin<&mut Self>, key: cxx::UniquePtr<Expr>, value: cxx::UniquePtr<Expr>, optional: bool) -> cxx::UniquePtr<MapExprEntry> {
        ffi::MacroExprFactory_new_map_entry(self, key, value, optional)
    }
    
    pub fn new_map(self: Pin<&mut Self>, entries: cxx::UniquePtr<cxx::CxxVector<MapExprEntry>>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_map(self, entries)
    }

    pub fn new_comprehension(
        self: Pin<&mut Self>,
        iter_var: StringView<'_>,
        iter_range: cxx::UniquePtr<Expr>,
        accu_var: StringView<'_>,
        accu_init: cxx::UniquePtr<Expr>,
        loop_condition: cxx::UniquePtr<Expr>,
        loop_step: cxx::UniquePtr<Expr>,
        result: cxx::UniquePtr<Expr>,
    ) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_comprehension(
            self,
            iter_var,
            iter_range,
            accu_var,
            accu_init,
            loop_condition,
            loop_step,
            result,
        )
    }
    
    pub fn new_comprehension2(
        self: Pin<&mut Self>,
        iter_var: StringView<'_>,
        iter_var2: StringView<'_>,
        iter_range: cxx::UniquePtr<Expr>,
        accu_var: StringView<'_>,
        accu_init: cxx::UniquePtr<Expr>,
        loop_condition: cxx::UniquePtr<Expr>,
        loop_step: cxx::UniquePtr<Expr>,
        result: cxx::UniquePtr<Expr>,
    ) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_new_comprehension2(
            self,
            iter_var,
            iter_var2,
            iter_range,
            accu_var,
            accu_init,
            loop_condition,
            loop_step,
            result,
        )
    }

    pub fn report_error(self: Pin<&mut Self>, message: StringView<'_>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_report_error(self, message)
    }
    
    pub fn report_error_at(self: Pin<&mut Self>, expr: &Expr, message: StringView<'_>) -> cxx::UniquePtr<Expr> {
        ffi::MacroExprFactory_report_error_at(self, expr, message)
    }
}

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

// ParserBuilderConfigurer
pub use ffi::ParserBuilderConfigurer;
unsafe impl Send for ParserBuilderConfigurer {}
unsafe impl Sync for ParserBuilderConfigurer {}

impl ParserBuilderConfigurer {
    pub fn new<F: FfiParserBuilderConfigurer + 'static>(configurer: F) -> cxx::UniquePtr<Self> {
        ffi::ParserBuilderConfigurer_new(Box::new(AnyFfiParserBuilderConfigurer::new(configurer)))
    }
}

pub trait FfiParserBuilderConfigurer: Fn(Pin<&mut ParserBuilder>) -> Status {}
impl<'f, F> FfiParserBuilderConfigurer for F where
    F: 'f + Fn(Pin<&mut ParserBuilder>) -> Status {}

struct AnyFfiParserBuilderConfigurer<'f>(Box<dyn FfiParserBuilderConfigurer + 'f>);
impl<'f> AnyFfiParserBuilderConfigurer<'f> {
    fn new<T: FfiParserBuilderConfigurer + 'f>(configurer: T) -> Self {
        Self(Box::new(configurer))
    }

    fn call(&self, parser_builder: Pin<&mut ParserBuilder>) -> Status {
        (self.0)(parser_builder)
    }
}

// ParserLibrary
pub use ffi::ParserLibrary;
unsafe impl Send for ParserLibrary {}
unsafe impl Sync for ParserLibrary {}

impl ParserLibrary {
    pub fn new(id: &cxx::CxxString, configurer: cxx::UniquePtr<ParserBuilderConfigurer>) -> cxx::UniquePtr<Self> {
        ffi::ParserLibrary_new(id, configurer)
    }

    pub fn id(&self) -> &cxx::CxxString {
        ffi::ParserLibrary_id(&self)
    }
}

// MacroPredicate
pub use ffi::MacroPredicate;
unsafe impl Send for MacroPredicate {}
unsafe impl Sync for MacroPredicate {}

impl MacroPredicate {
    pub fn new<F: FfiMacroPredicate + 'static>(predicate: F) -> cxx::UniquePtr<Self> {
        ffi::MacroPredicate_new(Box::new(AnyFfiMacroPredicate::new(predicate)))
    }
}

pub trait FfiMacroPredicate: Fn(&Macro) -> bool {}
impl<'f, F> FfiMacroPredicate for F where
    F: 'f + Fn(&Macro) -> bool {}

struct AnyFfiMacroPredicate<'f>(Box<dyn FfiMacroPredicate + 'f>);
impl<'f> AnyFfiMacroPredicate<'f> {
    fn new<T: FfiMacroPredicate + 'f>(predicate: T) -> Self {
        Self(Box::new(predicate))
    }

    fn call(&self, m: &Macro) -> bool {
        (self.0)(m)
    }
}

// ParserLibrarySubset
pub use ffi::ParserLibrarySubset;
unsafe impl Send for ParserLibrarySubset {}
unsafe impl Sync for ParserLibrarySubset {}

impl ParserLibrarySubset {
    pub fn new(library_id: &cxx::CxxString, should_include_macro: cxx::UniquePtr<MacroPredicate>) -> cxx::UniquePtr<Self> {
        ffi::ParserLibrarySubset_new(library_id, should_include_macro)
    }

    pub fn library_id(&self) -> &cxx::CxxString {
        ffi::ParserLibrarySubset_library_id(&self)
    }
}

// ParserBuilder
pub use ffi::ParserBuilder;
unsafe impl Send for ParserBuilder {}
unsafe impl Sync for ParserBuilder {}

impl ParserBuilder {
    pub fn add_library(self: Pin<&mut Self>, library: cxx::UniquePtr<ParserLibrary>) -> Status {
        ffi::ParserBuilder_add_library(self, library)
    }

    pub fn add_library_subset(self: Pin<&mut Self>, library_subset: cxx::UniquePtr<ParserLibrarySubset>) -> Status {
        ffi::ParserBuilder_add_library_subset(self, library_subset)
    }
}

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
