use std::pin::Pin;

use crate::absl::Status;
use crate::common::{Ast, FunctionDecl, Source, Type, VariableDecl};

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/status/status.h>);
        type Status = super::Status;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<checker/type_checker_builder.h>);
        include!(<checker/checker_options.h>);
        type Type<'a> = super::Type<'a>;
        type Ast = super::Ast;
        type Source = super::Source;
        type VariableDecl<'a> = super::VariableDecl<'a>;
        type FunctionDecl<'a> = super::FunctionDecl<'a>;

        type CheckerOptions;
        type CheckerLibrary;

        type TypeChecker;

        type TypeCheckerBuilder<'a>;
        #[rust_name = "add_variable"]
        fn AddVariable<'a>(
            self: Pin<&mut TypeCheckerBuilder<'a>>,
            decl: &VariableDecl<'a>,
        ) -> Status;
        #[rust_name = "add_function"]
        fn AddFunction<'a>(
            self: Pin<&mut TypeCheckerBuilder<'a>>,
            decl: &FunctionDecl<'a>,
        ) -> Status;
        #[rust_name = "merge_function"]
        fn MergeFunction<'a>(
            self: Pin<&mut TypeCheckerBuilder<'a>>,
            decl: &FunctionDecl<'a>,
        ) -> Status;
        #[rust_name = "set_expected_type"]
        fn SetExpectedType<'a>(self: Pin<&mut TypeCheckerBuilder<'a>>, expected_type: &Type<'a>);
        fn options<'this, 'a>(self: &'this TypeCheckerBuilder<'a>) -> &'this CheckerOptions;

        type TypeCheckIssue;
        fn severity(self: &TypeCheckIssue) -> Severity;

        type ValidationResult;
        #[rust_name = "is_valid"]
        fn IsValid(self: &ValidationResult) -> bool;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!("cel-cxx-ffi/include/absl.h");
        include!("cel-cxx-ffi/include/checker.h");
        type Severity = super::Severity;

        // CheckerLibrary
        fn CheckerLibrary_new_optional() -> UniquePtr<CheckerLibrary>;
        fn CheckerLibrary_new_standard() -> UniquePtr<CheckerLibrary>;

        // CheckerOptions
        fn CheckerOptions_new() -> UniquePtr<CheckerOptions>;

        // CheckerOptions getters and setters
        fn CheckerOptions_enable_cross_numeric_comparisons(
            checker_options: &CheckerOptions,
        ) -> bool;
        fn CheckerOptions_enable_cross_numeric_comparisons_mut(
            checker_options: Pin<&mut CheckerOptions>,
        ) -> &mut bool;
        fn CheckerOptions_enable_legacy_null_assignment(checker_options: &CheckerOptions) -> bool;
        fn CheckerOptions_enable_legacy_null_assignment_mut(
            checker_options: Pin<&mut CheckerOptions>,
        ) -> &mut bool;
        fn CheckerOptions_update_struct_type_names(checker_options: &CheckerOptions) -> bool;
        fn CheckerOptions_update_struct_type_names_mut(
            checker_options: Pin<&mut CheckerOptions>,
        ) -> &mut bool;
        fn CheckerOptions_allow_well_known_type_context_declarations(
            checker_options: &CheckerOptions,
        ) -> bool;
        fn CheckerOptions_allow_well_known_type_context_declarations_mut(
            checker_options: Pin<&mut CheckerOptions>,
        ) -> &mut bool;
        fn CheckerOptions_max_expression_node_count(checker_options: &CheckerOptions) -> i32;
        fn CheckerOptions_max_expression_node_count_mut(
            checker_options: Pin<&mut CheckerOptions>,
        ) -> &mut i32;
        fn CheckerOptions_max_error_issues(checker_options: &CheckerOptions) -> i32;
        fn CheckerOptions_max_error_issues_mut(
            checker_options: Pin<&mut CheckerOptions>,
        ) -> &mut i32;

        // TypeCheckerBuilder
        fn TypeCheckerBuilder_add_library<'a>(
            type_checker_builder: Pin<&mut TypeCheckerBuilder<'a>>,
            library: UniquePtr<CheckerLibrary>,
        ) -> Status;

        // TypeCheckIssue
        fn TypeCheckIssue_to_display_string(
            type_check_issue: &TypeCheckIssue,
            source: &Source,
        ) -> String;

        // ValidationResult
        unsafe fn ValidationResult_get_ast(validation_result: &ValidationResult) -> *const Ast;
        fn ValidationResult_release_ast(
            validation_result: Pin<&mut ValidationResult>,
            result: &mut UniquePtr<Ast>,
        ) -> Status;

        fn ValidationResult_format_error(validation_result: &ValidationResult) -> String;
    }

    impl UniquePtr<ValidationResult> {}
}

pub use ffi::CheckerLibrary;
unsafe impl Send for CheckerLibrary {}
unsafe impl Sync for CheckerLibrary {}

impl CheckerLibrary {
    pub fn new_optional() -> cxx::UniquePtr<Self> {
        ffi::CheckerLibrary_new_optional()
    }

    pub fn new_standard() -> cxx::UniquePtr<Self> {
        ffi::CheckerLibrary_new_standard()
    }
}

pub use ffi::CheckerOptions;
unsafe impl Send for CheckerOptions {}
unsafe impl Sync for CheckerOptions {}

impl CheckerOptions {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::CheckerOptions_new()
    }

    pub fn enable_cross_numeric_comparisons(&self) -> bool {
        ffi::CheckerOptions_enable_cross_numeric_comparisons(self)
    }

    pub fn enable_cross_numeric_comparisons_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut bool {
        ffi::CheckerOptions_enable_cross_numeric_comparisons_mut(self)
    }

    pub fn enable_legacy_null_assignment(&self) -> bool {
        ffi::CheckerOptions_enable_legacy_null_assignment(self)
    }

    pub fn enable_legacy_null_assignment_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut bool {
        ffi::CheckerOptions_enable_legacy_null_assignment_mut(self)
    }

    pub fn update_struct_type_names(&self) -> bool {
        ffi::CheckerOptions_update_struct_type_names(self)
    }

    pub fn update_struct_type_names_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut bool {
        ffi::CheckerOptions_update_struct_type_names_mut(self)
    }

    pub fn allow_well_known_type_context_declarations(&self) -> bool {
        ffi::CheckerOptions_allow_well_known_type_context_declarations(self)
    }

    pub fn allow_well_known_type_context_declarations_mut<'a>(
        self: Pin<&'a mut Self>,
    ) -> &'a mut bool {
        ffi::CheckerOptions_allow_well_known_type_context_declarations_mut(self)
    }

    pub fn max_expression_node_count(&self) -> i32 {
        ffi::CheckerOptions_max_expression_node_count(self)
    }

    pub fn max_expression_node_count_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
        ffi::CheckerOptions_max_expression_node_count_mut(self)
    }

    pub fn max_error_issues(&self) -> i32 {
        ffi::CheckerOptions_max_error_issues(self)
    }

    pub fn max_error_issues_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
        ffi::CheckerOptions_max_error_issues_mut(self)
    }
}

pub use ffi::ValidationResult;
unsafe impl Send for ValidationResult {}
unsafe impl Sync for ValidationResult {}

impl ValidationResult {
    pub fn ast(&self) -> Option<&Ast> {
        unsafe {
            let ast = ffi::ValidationResult_get_ast(self);
            ast.as_ref()
        }
    }

    pub fn release_ast(self: Pin<&mut Self>) -> Result<cxx::UniquePtr<Ast>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::ValidationResult_release_ast(self, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }

    pub fn format_error(&self) -> String {
        ffi::ValidationResult_format_error(self)
    }
}

// Severity
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Severity {
    #[default]
    Error,
    Warning,
    Information,
    Deprecated,
}

unsafe impl cxx::ExternType for Severity {
    type Id = cxx::type_id!("rust::cel_cxx::Severity");
    type Kind = cxx::kind::Trivial;
}

// TypeCheckIssue
pub use ffi::TypeCheckIssue;
unsafe impl Send for TypeCheckIssue {}
unsafe impl Sync for TypeCheckIssue {}
impl TypeCheckIssue {
    pub fn to_display_string(&self, source: &Source) -> String {
        ffi::TypeCheckIssue_to_display_string(self, source)
    }
}

pub use ffi::TypeChecker;
unsafe impl Send for TypeChecker {}
unsafe impl Sync for TypeChecker {}

impl std::fmt::Debug for TypeChecker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = self as *const TypeChecker;
        write!(f, "TypeChecker {{ ptr: {ptr:p} }}")
    }
}

pub use ffi::TypeCheckerBuilder;
unsafe impl<'a> Send for TypeCheckerBuilder<'a> {}
unsafe impl<'a> Sync for TypeCheckerBuilder<'a> {}

impl<'a> TypeCheckerBuilder<'a> {
    pub fn add_library(self: Pin<&mut Self>, library: cxx::UniquePtr<CheckerLibrary>) -> Status {
        ffi::TypeCheckerBuilder_add_library(self, library)
    }
}
