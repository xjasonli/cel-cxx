use std::ffi::c_int;
use std::pin::Pin;

use crate::absl::Status;
use crate::common::{VariableDecl, FunctionDecl, Type, Ast, Source};

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

        type CheckerOptions = super::CheckerOptions;
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
        fn SetExpectedType<'a>(
            self: Pin<&mut TypeCheckerBuilder<'a>>,
            expected_type: &Type<'a>,
        );
        fn options<'a>(self: &TypeCheckerBuilder<'a>) -> &CheckerOptions;

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
        fn CheckerOptions_default() -> CheckerOptions;

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

        fn ValidationResult_format_error(
            validation_result: &ValidationResult,
        ) -> String;
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CheckerOptions {
    pub enable_cross_numeric_comparisons: bool,
    pub enable_legacy_null_assignment: bool,
    pub update_struct_type_names: bool,
    pub max_expression_node_count: c_int,
    pub max_error_issues: c_int,
}

unsafe impl cxx::ExternType for CheckerOptions {
    type Id = cxx::type_id!("cel::CheckerOptions");
    type Kind = cxx::kind::Trivial;
}

impl Default for CheckerOptions {
    fn default() -> Self {
        ffi::CheckerOptions_default()
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
    pub fn to_display_string(self: &Self, source: &Source) -> String {
        ffi::TypeCheckIssue_to_display_string(self, source)
    }
}

pub use ffi::TypeChecker;
unsafe impl Send for TypeChecker {}
unsafe impl Sync for TypeChecker {}

impl std::fmt::Debug for TypeChecker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = self as *const TypeChecker;
        write!(f, "TypeChecker {{ ptr: {:p} }}", ptr)
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
