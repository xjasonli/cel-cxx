#ifndef CEL_CXX_FFI_INCLUDE_CHECKER_H_
#define CEL_CXX_FFI_INCLUDE_CHECKER_H_

#include <rust/cxx.h>
#include <common/ast.h>
#include <checker/checker_options.h>
#include <checker/type_checker_builder.h>
#include <checker/validation_result.h>
#include <checker/optional.h>
#include <checker/standard_library.h>
#include <cel-cxx-ffi/include/optional.h>

namespace rust::cel_cxx {

using Ast = cel::Ast;
using Source = cel::Source;
using CheckerOptions = cel::CheckerOptions;
using TypeCheckerBuilderConfigurer = cel::TypeCheckerBuilderConfigurer;
using TypeCheckerBuilder = cel::TypeCheckerBuilder;
using TypeCheckIssue = cel::TypeCheckIssue;
using ValidationResult = cel::ValidationResult;
using CheckerLibrary = cel::CheckerLibrary;
using TypeCheckerSubset = cel::TypeCheckerSubset;
using FunctionPredicate = cel::TypeCheckerSubset::FunctionPredicate;
using Severity = cel::TypeCheckIssue::Severity;

// TypeCheckerBuilderConfigurer
struct AnyFfiTypeCheckerBuilderConfigurer;

std::unique_ptr<TypeCheckerBuilderConfigurer> TypeCheckerBuilderConfigurer_new(
    Box<AnyFfiTypeCheckerBuilderConfigurer> ffi_configurer);

// CheckerLibrary
inline std::unique_ptr<CheckerLibrary> CheckerLibrary_new(
    const std::string& id,
    std::unique_ptr<TypeCheckerBuilderConfigurer> configurer) {
    return std::make_unique<CheckerLibrary>(CheckerLibrary{
        id,
        std::move(*configurer),
    });
}

inline std::unique_ptr<CheckerLibrary> CheckerLibrary_new_optional() {
    return std::make_unique<CheckerLibrary>(
        PatchOptionalCheckerLibrary(cel::OptionalCheckerLibrary()));
}

inline std::unique_ptr<CheckerLibrary> CheckerLibrary_new_standard() {
    return std::make_unique<CheckerLibrary>(cel::StandardCheckerLibrary());
}

inline const std::string& CheckerLibrary_id(const CheckerLibrary& checker_library) {
    return checker_library.id;
}

// FunctionPredicate
struct AnyFfiFunctionPredicate;
std::unique_ptr<FunctionPredicate> FunctionPredicate_new(
    Box<AnyFfiFunctionPredicate> ffi_predicate);

// TypeCheckerSubset
inline std::unique_ptr<TypeCheckerSubset> TypeCheckerSubset_new(
    const std::string& library_id,
    std::unique_ptr<FunctionPredicate> should_include_overload) {
    return std::make_unique<TypeCheckerSubset>(TypeCheckerSubset{
        library_id,
        std::move(*should_include_overload),
    });
}

inline const std::string& TypeCheckerSubset_library_id(const TypeCheckerSubset& type_checker_subset) {
    return type_checker_subset.library_id;
}

// CheckerOptions
inline std::unique_ptr<CheckerOptions> CheckerOptions_new() {
    return std::make_unique<CheckerOptions>();
}

// CheckerOptions getters and setters
inline bool CheckerOptions_enable_cross_numeric_comparisons(const CheckerOptions& checker_options) {
    return checker_options.enable_cross_numeric_comparisons;
}
inline bool& CheckerOptions_enable_cross_numeric_comparisons_mut(CheckerOptions& checker_options) {
    return checker_options.enable_cross_numeric_comparisons;
}

inline bool CheckerOptions_enable_legacy_null_assignment(const CheckerOptions& checker_options) {
    return checker_options.enable_legacy_null_assignment;
}
inline bool& CheckerOptions_enable_legacy_null_assignment_mut(CheckerOptions& checker_options) {
    return checker_options.enable_legacy_null_assignment;
}

inline bool CheckerOptions_update_struct_type_names(const CheckerOptions& checker_options) {
    return checker_options.update_struct_type_names;
}
inline bool& CheckerOptions_update_struct_type_names_mut(CheckerOptions& checker_options) {
    return checker_options.update_struct_type_names;
}

inline bool CheckerOptions_allow_well_known_type_context_declarations(const CheckerOptions& checker_options) {
    return checker_options.allow_well_known_type_context_declarations;
}
inline bool& CheckerOptions_allow_well_known_type_context_declarations_mut(CheckerOptions& checker_options) {
    return checker_options.allow_well_known_type_context_declarations;
}

inline int CheckerOptions_max_expression_node_count(const CheckerOptions& checker_options) {
    return checker_options.max_expression_node_count;
}
inline int& CheckerOptions_max_expression_node_count_mut(CheckerOptions& checker_options) {
    return checker_options.max_expression_node_count;
}

inline int CheckerOptions_max_error_issues(const CheckerOptions& checker_options) {
    return checker_options.max_error_issues;
}
inline int& CheckerOptions_max_error_issues_mut(CheckerOptions& checker_options) {
    return checker_options.max_error_issues;
}


// TypeCheckerBuilder
inline absl::Status TypeCheckerBuilder_add_library(
    TypeCheckerBuilder& type_checker_builder,
    std::unique_ptr<CheckerLibrary> library) {
    return type_checker_builder.AddLibrary(std::move(*library));
}

inline absl::Status TypeCheckerBuilder_add_library_subset(
    TypeCheckerBuilder& type_checker_builder,
    std::unique_ptr<TypeCheckerSubset> library_subset) {
    return type_checker_builder.AddLibrarySubset(std::move(*library_subset));
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
