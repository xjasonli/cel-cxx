use crate::absl::Status;
use crate::checker::{CheckerLibrary, CheckerOptions};
use crate::checker::{TypeChecker, TypeCheckerBuilder, ValidationResult};
use crate::parser::ParserOptions;
use crate::parser::{Parser, ParserBuilder};
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
        type ParserBuilder = super::ParserBuilder;
        type CheckerLibrary = super::CheckerLibrary;
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
        type CompilerOptions;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!("cel-cxx-ffi/include/absl.h");
        include!("cel-cxx-ffi/include/compiler.h");

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

        // CompilerOptions
        fn CompilerOptions_new() -> UniquePtr<CompilerOptions>;

        // CompilerOptions getters and setters
        fn CompilerOptions_parser_options<'a>(compiler_options: &'a CompilerOptions) -> &'a ParserOptions;
        fn CompilerOptions_parser_options_mut<'a>(compiler_options: Pin<&'a mut CompilerOptions>) -> Pin<&'a mut ParserOptions>;
        fn CompilerOptions_checker_options<'a>(compiler_options: &'a CompilerOptions) -> &'a CheckerOptions;
        fn CompilerOptions_checker_options_mut<'a>(compiler_options: Pin<&'a mut CompilerOptions>) -> Pin<&'a mut CheckerOptions>;
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
}

pub use ffi::CompilerOptions;
unsafe impl Send for CompilerOptions {}
unsafe impl Sync for CompilerOptions {}

impl CompilerOptions {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::CompilerOptions_new()
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
