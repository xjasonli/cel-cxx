#ifndef CEL_CXX_FFI_INCLUDE_COMPILER_H_
#define CEL_CXX_FFI_INCLUDE_COMPILER_H_

#include <rust/cxx.h>
#include <compiler/compiler.h>
#include <compiler/compiler_factory.h>
#include <compiler/standard_library.h>
#include <compiler/optional.h>

namespace rust::cel_cxx {

using DescriptorPool = google::protobuf::DescriptorPool;
using ParserOptions = cel::ParserOptions;
using CheckerOptions = cel::CheckerOptions;
using CheckerLibrary = cel::CheckerLibrary;
using Compiler = cel::Compiler;
using CompilerBuilder = cel::CompilerBuilder;
using CompilerOptions = cel::CompilerOptions;
using CompilerLibrary = cel::CompilerLibrary;
using ValidationResult = cel::ValidationResult;

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
    return std::make_unique<CompilerLibrary>(cel::OptionalCompilerLibrary());
}

inline std::unique_ptr<CompilerLibrary> CompilerLibrary_from_checker_library(
    std::unique_ptr<CheckerLibrary> checker_library) {
    return std::make_unique<CompilerLibrary>(cel::CompilerLibrary::FromCheckerLibrary(std::move(*checker_library)));
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
