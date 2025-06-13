#ifndef CEL_CXX_FFI_INCLUDE_CHECKER_H_
#define CEL_CXX_FFI_INCLUDE_CHECKER_H_

#include <rust/cxx.h>
#include <common/ast.h>
#include <checker/checker_options.h>
#include <checker/type_checker_builder.h>
#include <checker/validation_result.h>
#include <checker/optional.h>
#include <checker/standard_library.h>

namespace rust::cel_cxx {

using Ast = cel::Ast;
using Source = cel::Source;
using CheckerOptions = cel::CheckerOptions;
using TypeCheckerBuilder = cel::TypeCheckerBuilder;
using TypeCheckIssue = cel::TypeCheckIssue;
using ValidationResult = cel::ValidationResult;
using CheckerLibrary = cel::CheckerLibrary;
using Severity = cel::TypeCheckIssue::Severity;

// CheckerLibrary
inline std::unique_ptr<CheckerLibrary> CheckerLibrary_new_optional() {
    return std::make_unique<CheckerLibrary>(cel::OptionalCheckerLibrary());
}

inline std::unique_ptr<CheckerLibrary> CheckerLibrary_new_standard() {
    return std::make_unique<CheckerLibrary>(cel::StandardCheckerLibrary());
}

// CheckerOptions
inline CheckerOptions CheckerOptions_default() {
    return CheckerOptions();
}

// TypeCheckerBuilder
inline absl::Status TypeCheckerBuilder_add_library(
    TypeCheckerBuilder& type_checker_builder,
    std::unique_ptr<CheckerLibrary> library) {
    return type_checker_builder.AddLibrary(std::move(*library));
}

// TypeCheckIssue
inline String TypeCheckIssue_to_display_string(
    const TypeCheckIssue& type_check_issue,
    const Source& source) {
    return type_check_issue.ToDisplayString(source);
}

// ValidationResult
inline const Ast* ValidationResult_get_ast(const ValidationResult& validation_result) {
    return validation_result.GetAst();
}

inline absl::Status ValidationResult_release_ast(
    ValidationResult& validation_result,
    std::unique_ptr<Ast>& result) {
    auto ast = validation_result.ReleaseAst();
    if (ast.ok()) {
        result = std::move(ast.value());
    }
    return ast.status();
}

inline String ValidationResult_format_error(const ValidationResult& validation_result) {
    return validation_result.FormatError();
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_CHECKER_H_
