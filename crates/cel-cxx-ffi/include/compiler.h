#ifndef CEL_CXX_FFI_INCLUDE_COMPILER_H_
#define CEL_CXX_FFI_INCLUDE_COMPILER_H_

#include <rust/cxx.h>
#include <compiler/compiler.h>
#include <compiler/compiler_factory.h>
#include <compiler/standard_library.h>
#include <compiler/optional.h>
#include <cel-cxx-ffi/include/optional.h>

namespace rust::cel_cxx {

using DescriptorPool = google::protobuf::DescriptorPool;
using ParserOptions = cel::ParserOptions;
using ParserLibrary = cel::ParserLibrary;
using ParserLibrarySubset = cel::ParserLibrarySubset;
using CheckerOptions = cel::CheckerOptions;
using CheckerLibrary = cel::CheckerLibrary;
using TypeCheckerSubset = cel::TypeCheckerSubset;
using Compiler = cel::Compiler;
using CompilerBuilder = cel::CompilerBuilder;
using CompilerOptions = cel::CompilerOptions;
using CompilerLibrary = cel::CompilerLibrary;
using CompilerLibrarySubset = cel::CompilerLibrarySubset;
using ValidationResult = cel::ValidationResult;
using MacroPredicate = cel::ParserLibrarySubset::MacroPredicate;
using FunctionPredicate = cel::TypeCheckerSubset::FunctionPredicate;
using ParserBuilderConfigurer = cel::ParserBuilderConfigurer;
using TypeCheckerBuilderConfigurer = cel::TypeCheckerBuilderConfigurer;

// Compiler
inline absl::Status Compiler_compile(
    const Compiler& compiler,
    Slice<const uint8_t> source,
    Str description,
    std::unique_ptr<ValidationResult>& result)
{
    auto source_view = std::string_view(reinterpret_cast<const char*>(source.data()), source.size());
    auto description_view = std::string_view(description.data(), description.size());
    auto status_or = compiler.Compile(source_view, description_view);
    if (status_or.ok()) {
        result = std::make_unique<ValidationResult>(std::move(status_or.value()));
    }
    return status_or.status();
}

// CompilerBuilder
inline absl::Status CompilerBuilder_new(
    std::shared_ptr<DescriptorPool> descriptor_pool,
    const CompilerOptions& options,
    std::unique_ptr<CompilerBuilder>& result)
{
    auto status_or = cel::NewCompilerBuilder(descriptor_pool, options);
    if (status_or.ok()) {
        result = std::move(status_or.value());
    }
    return status_or.status();
}

inline absl::Status CompilerBuilder_add_library(
    CompilerBuilder& compiler_builder,
    std::unique_ptr<CompilerLibrary> library) {
    return compiler_builder.AddLibrary(std::move(*library));
}

inline absl::Status CompilerBuilder_build(
    CompilerBuilder& compiler_builder,
    std::unique_ptr<Compiler>& result)
{
    auto status_or = compiler_builder.Build();
    if (status_or.ok()) {
        result = std::move(status_or.value());
    }
    return status_or.status();
}

// CompilerLibrary
inline std::unique_ptr<CompilerLibrary> CompilerLibrary_new_standard() {
    return std::make_unique<CompilerLibrary>(cel::StandardCompilerLibrary());
}

inline std::unique_ptr<CompilerLibrary> CompilerLibrary_new_optional() {
    return std::make_unique<CompilerLibrary>(PatchOptionalCompilerLibrary(cel::OptionalCompilerLibrary()));
}

inline std::unique_ptr<CompilerLibrary> CompilerLibrary_from_checker_library(
    std::unique_ptr<CheckerLibrary> checker_library) {
    return std::make_unique<CompilerLibrary>(cel::CompilerLibrary::FromCheckerLibrary(std::move(*checker_library)));
}

inline std::unique_ptr<CompilerLibrary> CompilerLibrary_from_parser_library(
    std::unique_ptr<ParserLibrary> parser_library) {
    return std::make_unique<CompilerLibrary>(
        std::move(parser_library->id),
        std::move(parser_library->configure),
        nullptr
    );
}

inline std::unique_ptr<CompilerLibrary> CompilerLibrary_new(const std::string& id) {
    return std::make_unique<CompilerLibrary>(
        id,
        nullptr,
        nullptr
    );
}

inline void CompilerLibrary_set_parser_configurer(
    CompilerLibrary& compiler_library,
    std::unique_ptr<ParserBuilderConfigurer> parser_configurer) {
    compiler_library.configure_parser = std::move(*parser_configurer);
}

inline void CompilerLibrary_set_checker_configurer(
    CompilerLibrary& compiler_library,
    std::unique_ptr<TypeCheckerBuilderConfigurer> checker_configurer) {
    compiler_library.configure_checker = std::move(*checker_configurer);
}

inline const std::string& CompilerLibrary_id(const CompilerLibrary& compiler_library) {
    return compiler_library.id;
}

// CompilerLibrarySubset
inline std::unique_ptr<CompilerLibrarySubset> CompilerLibrarySubset_from_parser_library_subset(
    std::unique_ptr<ParserLibrarySubset> parser_library_subset) {
    return std::make_unique<CompilerLibrarySubset>(
        CompilerLibrarySubset{
            std::move(parser_library_subset->library_id),
            std::move(parser_library_subset->should_include_macro),
            nullptr
        }
    );
}

inline std::unique_ptr<CompilerLibrarySubset> CompilerLibrarySubset_from_checker_library_subset(
    std::unique_ptr<TypeCheckerSubset> checker_library_subset) {
    return std::make_unique<CompilerLibrarySubset>(
        CompilerLibrarySubset{
            std::move(checker_library_subset->library_id),
            nullptr,
            std::move(checker_library_subset->should_include_overload),
        }
    );
}

inline std::unique_ptr<CompilerLibrarySubset> CompilerLibrarySubset_new(const std::string& library_id) {
    return std::make_unique<CompilerLibrarySubset>(
        CompilerLibrarySubset{
            library_id,
            nullptr,
            nullptr
        }
    );
}

inline void CompilerLibrarySubset_set_macro_predicate(
    CompilerLibrarySubset& compiler_library_subset,
    std::unique_ptr<MacroPredicate> should_include_macro) {
    compiler_library_subset.should_include_macro = std::move(*should_include_macro);
}

inline void CompilerLibrarySubset_set_function_predicate(
    CompilerLibrarySubset& compiler_library_subset,
    std::unique_ptr<FunctionPredicate> should_include_overload) {
    compiler_library_subset.should_include_overload = std::move(*should_include_overload);
}

inline const std::string& CompilerLibrarySubset_library_id(const CompilerLibrarySubset& compiler_library_subset) {
    return compiler_library_subset.library_id;
}

// CompilerOptions
inline std::unique_ptr<CompilerOptions> CompilerOptions_new() {
    return std::make_unique<CompilerOptions>();
}

// CompilerOptions getters and setters
inline const ParserOptions& CompilerOptions_parser_options(const CompilerOptions& compiler_options) {
    return compiler_options.parser_options;
}
inline ParserOptions& CompilerOptions_parser_options_mut(CompilerOptions& compiler_options) {
    return compiler_options.parser_options;
}

inline const CheckerOptions& CompilerOptions_checker_options(const CompilerOptions& compiler_options) {
    return compiler_options.checker_options;
}
inline CheckerOptions& CompilerOptions_checker_options_mut(CompilerOptions& compiler_options) {
    return compiler_options.checker_options;
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_COMPILER_H_
