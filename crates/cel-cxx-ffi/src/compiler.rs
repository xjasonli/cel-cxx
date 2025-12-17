use crate::absl::Status;
use crate::checker::{
    CheckerLibrary, CheckerOptions, TypeChecker, TypeCheckerSubset,
    TypeCheckerBuilderConfigurer, FunctionPredicate,
    TypeCheckerBuilder, ValidationResult,
};
use crate::parser::{
    ParserOptions, Parser, ParserBuilder, ParserLibrary, ParserLibrarySubset,
    ParserBuilderConfigurer, MacroPredicate,
};
use crate::protobuf::DescriptorPool;
use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/status/status.h>);
        type Status = super::Status;
    }

    #[namespace = "google::protobuf"]
    unsafe extern "C++" {
        include!(<google/protobuf/descriptor.h>);
        type DescriptorPool = super::DescriptorPool;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<compiler/compiler.h>);
        type ParserOptions = super::ParserOptions;
        type CheckerOptions = super::CheckerOptions;
        type ParserBuilderConfigurer = super::ParserBuilderConfigurer;
        type TypeCheckerBuilderConfigurer = super::TypeCheckerBuilderConfigurer;
        type ParserBuilder = super::ParserBuilder;
        type ParserLibrary = super::ParserLibrary;
        type CheckerLibrary = super::CheckerLibrary;
        type TypeCheckerSubset = super::TypeCheckerSubset;
        type ParserLibrarySubset = super::ParserLibrarySubset;
        type TypeChecker = super::TypeChecker;
        type Parser = super::Parser;
        type TypeCheckerBuilder<'a> = super::TypeCheckerBuilder<'a>;
        type ValidationResult = super::ValidationResult;

        type Compiler<'a>;
        #[rust_name = "type_checker"]
        fn GetTypeChecker<'a>(self: &Compiler<'a>) -> &TypeChecker;
        #[rust_name = "parser"]
        fn GetParser<'a>(self: &Compiler<'a>) -> &Parser;

        type CompilerBuilder<'a>;
        #[rust_name = "checker_builder"]
        fn GetCheckerBuilder<'a>(
            self: Pin<&mut CompilerBuilder<'a>>,
        ) -> Pin<&mut TypeCheckerBuilder<'a>>;
        #[rust_name = "parser_builder"]
        fn GetParserBuilder<'a>(self: Pin<&mut CompilerBuilder<'a>>) -> Pin<&mut ParserBuilder>;

        type CompilerLibrary;
        type CompilerLibrarySubset;
        type CompilerOptions;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/absl.h>);
        include!(<cel-cxx-ffi/include/compiler.h>);

        type MacroPredicate = super::MacroPredicate;
        type FunctionPredicate = super::FunctionPredicate;

        // Compiler
        fn Compiler_compile(
            compiler: &Compiler,
            source: &[u8],
            description: &str,
            result: &mut UniquePtr<ValidationResult>,
        ) -> Status;

        // CompilerBuilder
        fn CompilerBuilder_new<'a>(
            descriptor_pool: SharedPtr<DescriptorPool>,
            options: &CompilerOptions,
            result: &mut UniquePtr<CompilerBuilder<'a>>,
        ) -> Status;

        fn CompilerBuilder_add_library<'a>(
            compiler_builder: Pin<&mut CompilerBuilder<'a>>,
            library: UniquePtr<CompilerLibrary>,
        ) -> Status;

        fn CompilerBuilder_build<'a>(
            compiler_builder: Pin<&mut CompilerBuilder<'a>>,
            result: &mut UniquePtr<Compiler<'a>>,
        ) -> Status;

        // CompilerLibrary
        fn CompilerLibrary_new_standard() -> UniquePtr<CompilerLibrary>;
        fn CompilerLibrary_new_optional() -> UniquePtr<CompilerLibrary>;
        fn CompilerLibrary_from_checker_library(
            checker_library: UniquePtr<CheckerLibrary>,
        ) -> UniquePtr<CompilerLibrary>;
        fn CompilerLibrary_from_parser_library(
            parser_library: UniquePtr<ParserLibrary>,
        ) -> UniquePtr<CompilerLibrary>;

        fn CompilerLibrary_new(id: &CxxString) -> UniquePtr<CompilerLibrary>;
        fn CompilerLibrary_set_parser_configurer(
            compiler_library: Pin<&mut CompilerLibrary>,
            parser_configurer: UniquePtr<ParserBuilderConfigurer>,
        );
        fn CompilerLibrary_set_checker_configurer(
            compiler_library: Pin<&mut CompilerLibrary>,
            checker_configurer: UniquePtr<TypeCheckerBuilderConfigurer>,
        );
        fn CompilerLibrary_id<'a>(compiler_library: &'a CompilerLibrary) -> &'a CxxString;

        // CompilerLibrarySubset
        fn CompilerLibrarySubset_from_parser_library_subset(
            parser_library_subset: UniquePtr<ParserLibrarySubset>,
        ) -> UniquePtr<CompilerLibrarySubset>;
        fn CompilerLibrarySubset_from_checker_library_subset(
            checker_library_subset: UniquePtr<TypeCheckerSubset>,
        ) -> UniquePtr<CompilerLibrarySubset>;

        fn CompilerLibrarySubset_new(library_id: &CxxString) -> UniquePtr<CompilerLibrarySubset>;
        fn CompilerLibrarySubset_set_macro_predicate(
            compiler_library_subset: Pin<&mut CompilerLibrarySubset>,
            should_include_macro: UniquePtr<MacroPredicate>,
        );
        fn CompilerLibrarySubset_set_function_predicate(
            compiler_library_subset: Pin<&mut CompilerLibrarySubset>,
            should_include_overload: UniquePtr<FunctionPredicate>,
        );
        fn CompilerLibrarySubset_library_id<'a>(compiler_library_subset: &'a CompilerLibrarySubset) -> &'a CxxString;

        // CompilerOptions
        fn CompilerOptions_new() -> UniquePtr<CompilerOptions>;

        // CompilerOptions getters and setters
        fn CompilerOptions_parser_options(compiler_options: &CompilerOptions) -> &ParserOptions;
        fn CompilerOptions_parser_options_mut(
            compiler_options: Pin<&mut CompilerOptions>,
        ) -> Pin<&mut ParserOptions>;
        fn CompilerOptions_checker_options(compiler_options: &CompilerOptions) -> &CheckerOptions;
        fn CompilerOptions_checker_options_mut(
            compiler_options: Pin<&mut CompilerOptions>,
        ) -> Pin<&mut CheckerOptions>;
    }
}

pub use ffi::Compiler;
unsafe impl<'a> Send for Compiler<'a> {}
unsafe impl<'a> Sync for Compiler<'a> {}

impl<'a> Compiler<'a> {
    pub fn compile(
        &self,
        source: &[u8],
        description: Option<&str>,
    ) -> Result<cxx::UniquePtr<ValidationResult>, Status> {
        let description = description.unwrap_or("<input>");
        let mut result = cxx::UniquePtr::null();
        let status = ffi::Compiler_compile(self, source, description, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }
}

impl<'a> std::fmt::Debug for Compiler<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Compiler {{ type_checker: {:?}, parser: {:?} }}",
            self.type_checker(),
            self.parser(),
        )
    }
}

pub use ffi::CompilerBuilder;
unsafe impl<'a> Send for CompilerBuilder<'a> {}
unsafe impl<'a> Sync for CompilerBuilder<'a> {}

impl<'a> CompilerBuilder<'a> {
    pub fn new(
        descriptor_pool: cxx::SharedPtr<DescriptorPool>,
        options: &CompilerOptions,
    ) -> Result<cxx::UniquePtr<Self>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::CompilerBuilder_new(descriptor_pool, options, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }

    pub fn add_library(
        self: Pin<&mut Self>,
        library: cxx::UniquePtr<CompilerLibrary>,
    ) -> Result<(), Status> {
        let status = ffi::CompilerBuilder_add_library(self, library);
        if status.is_ok() {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn build(self: Pin<&mut Self>) -> Result<cxx::UniquePtr<Compiler<'a>>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::CompilerBuilder_build(self, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }
}

impl<'a> std::fmt::Debug for CompilerBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompilerBuilder {{ ptr: {:p} }}", self as *const Self)
    }
}

// CompilerLibrary
pub use ffi::CompilerLibrary;
unsafe impl Send for CompilerLibrary {}
unsafe impl Sync for CompilerLibrary {}

impl CompilerLibrary {
    pub fn new(id: &cxx::CxxString) -> cxx::UniquePtr<Self> {
        ffi::CompilerLibrary_new(id)
    }

    pub fn set_parser_configurer(
        self: Pin<&mut Self>,
        parser_configurer: cxx::UniquePtr<ParserBuilderConfigurer>,
    ) {
        ffi::CompilerLibrary_set_parser_configurer(self, parser_configurer);
    }

    pub fn set_checker_configurer(
        self: Pin<&mut Self>,
        checker_configurer: cxx::UniquePtr<TypeCheckerBuilderConfigurer>,
    ) {
        ffi::CompilerLibrary_set_checker_configurer(self, checker_configurer);
    }

    pub fn new_standard() -> cxx::UniquePtr<Self> {
        ffi::CompilerLibrary_new_standard()
    }

    pub fn new_optional() -> cxx::UniquePtr<Self> {
        ffi::CompilerLibrary_new_optional()
    }

    pub fn from_checker_library(
        checker_library: cxx::UniquePtr<CheckerLibrary>,
    ) -> cxx::UniquePtr<Self> {
        ffi::CompilerLibrary_from_checker_library(checker_library)
    }

    pub fn from_parser_library(
        parser_library: cxx::UniquePtr<ParserLibrary>,
    ) -> cxx::UniquePtr<Self> {
        ffi::CompilerLibrary_from_parser_library(parser_library)
    }

    pub fn id(&self) -> &cxx::CxxString {
        ffi::CompilerLibrary_id(self)
    }
}

// CompilerLibrarySubset
pub use ffi::CompilerLibrarySubset;
unsafe impl Send for CompilerLibrarySubset {}
unsafe impl Sync for CompilerLibrarySubset {}

impl CompilerLibrarySubset {
    pub fn new(library_id: &cxx::CxxString) -> cxx::UniquePtr<Self> {
        ffi::CompilerLibrarySubset_new(library_id)
    }

    pub fn set_macro_predicate(
        self: Pin<&mut Self>,
        macro_predicate: cxx::UniquePtr<MacroPredicate>,
    ) {
        ffi::CompilerLibrarySubset_set_macro_predicate(self, macro_predicate);
    }

    pub fn set_function_predicate(
        self: Pin<&mut Self>,
        function_predicate: cxx::UniquePtr<FunctionPredicate>,
    ) {
        ffi::CompilerLibrarySubset_set_function_predicate(self, function_predicate);
    }

    pub fn library_id(&self) -> &cxx::CxxString {
        ffi::CompilerLibrarySubset_library_id(self)
    }

    pub fn from_parser_library_subset(
        parser_library_subset: cxx::UniquePtr<ParserLibrarySubset>,
    ) -> cxx::UniquePtr<Self> {
        ffi::CompilerLibrarySubset_from_parser_library_subset(parser_library_subset)
    }

    pub fn from_checker_library_subset(
        checker_library_subset: cxx::UniquePtr<TypeCheckerSubset>,
    ) -> cxx::UniquePtr<Self> {
        ffi::CompilerLibrarySubset_from_checker_library_subset(checker_library_subset)
    }
}

// CompilerOptions
pub use ffi::CompilerOptions;
unsafe impl Send for CompilerOptions {}
unsafe impl Sync for CompilerOptions {}

impl CompilerOptions {
    pub fn new() -> cxx::UniquePtr<Self> {
        let mut options = ffi::CompilerOptions_new();
        *options
            .pin_mut()
            .parser_options_mut()
            .enable_optional_syntax_mut() = true;
        options
    }

    pub fn parser_options(&self) -> &ParserOptions {
        ffi::CompilerOptions_parser_options(self)
    }

    pub fn parser_options_mut(self: Pin<&mut Self>) -> Pin<&mut ParserOptions> {
        ffi::CompilerOptions_parser_options_mut(self)
    }

    pub fn checker_options(&self) -> &CheckerOptions {
        ffi::CompilerOptions_checker_options(self)
    }

    pub fn checker_options_mut(self: Pin<&mut Self>) -> Pin<&mut CheckerOptions> {
        ffi::CompilerOptions_checker_options_mut(self)
    }
}
