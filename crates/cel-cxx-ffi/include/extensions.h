#ifndef CEL_CXX_FFI_INCLUDE_EXTENSIONS_H_
#define CEL_CXX_FFI_INCLUDE_EXTENSIONS_H_

#include <rust/cxx.h>
#include <extensions/bindings_ext.h>
#include <extensions/encoders.h>
#include <extensions/lists_functions.h>
#include <extensions/math_ext_decls.h>
#include <extensions/proto_ext.h>
#include <extensions/regex_functions.h>
#include <extensions/regex_ext.h>
#include <extensions/select_optimization.h>
#include <extensions/sets_functions.h>
#include <extensions/strings.h>
#include <checker/internal/builtins_arena.h>
#include <internal/status_macros.h>

namespace cel::extensions {

// bindings_ext.h
cel::CompilerLibrary BindingsCompilerLibrary();

// regex_ext.h
cel::CheckerLibrary RegexExtensionCheckerLibrary();

} // namespace cel::extensions

namespace rust::cel_cxx {

// bindings_ext.h
inline std::unique_ptr<cel::CompilerLibrary> BindingsCompilerLibrary() {
    return std::make_unique<cel::CompilerLibrary>(cel::extensions::BindingsCompilerLibrary());
}

// encoders.h
inline std::unique_ptr<cel::CompilerLibrary> EncodersCompilerLibrary() {
    return std::make_unique<cel::CompilerLibrary>(
        cel::CompilerLibrary::FromCheckerLibrary(
            cel::extensions::EncodersCheckerLibrary()));
}


// lists_functions.h
inline std::unique_ptr<cel::CompilerLibrary> ListsCompilerLibrary() {
    return std::make_unique<cel::CompilerLibrary>(cel::extensions::ListsCompilerLibrary());
}

// math_ext_decls.h
inline std::unique_ptr<cel::CompilerLibrary> MathCompilerLibrary() {
    return std::make_unique<cel::CompilerLibrary>(cel::extensions::MathCompilerLibrary());
}

// proto_ext.h
inline std::unique_ptr<cel::CompilerLibrary> ProtoExtCompilerLibrary() {
    return std::make_unique<cel::CompilerLibrary>(cel::extensions::ProtoExtCompilerLibrary());
}

// regex_functions.h
inline std::unique_ptr<cel::CompilerLibrary> RegexCompilerLibrary() {
    return std::make_unique<cel::CompilerLibrary>(
        cel::CompilerLibrary::FromCheckerLibrary(
            cel::extensions::RegexCheckerLibrary()));
}

inline std::unique_ptr<cel::CompilerLibrary> RegexExtensionCompilerLibrary() {
    return std::make_unique<cel::CompilerLibrary>(
        cel::CompilerLibrary::FromCheckerLibrary(
            cel::extensions::RegexExtensionCheckerLibrary()));
}

// SelectOptimizationOptions
inline std::unique_ptr<cel::extensions::SelectOptimizationOptions> SelectOptimizationOptions_new() {
    return std::make_unique<cel::extensions::SelectOptimizationOptions>();
}

// SelectOptimizationOptions setters and getters
inline bool SelectOptimizationOptions_force_fallback_implementation(
    const cel::extensions::SelectOptimizationOptions& options) {
    return options.force_fallback_implementation;
}

inline bool& SelectOptimizationOptions_force_fallback_implementation_mut(
    cel::extensions::SelectOptimizationOptions& options) {
    return options.force_fallback_implementation;
}

// sets_functions.h
inline std::unique_ptr<cel::CompilerLibrary> SetsCompilerLibrary() {
    return std::make_unique<cel::CompilerLibrary>(cel::extensions::SetsCompilerLibrary());
}

// strings.h
inline std::unique_ptr<cel::CompilerLibrary> StringsCompilerLibrary() {
    cel::CheckerLibrary origin = cel::extensions::StringsCheckerLibrary();
    cel::CheckerLibrary checker_library = {
        .id = origin.id,
        .configure = [origin=std::move(origin)](cel::TypeCheckerBuilder& builder) -> absl::Status {
            CEL_ASSIGN_OR_RETURN(
                auto trim_decl,
                cel::MakeFunctionDecl(
                    "trim",
                    cel::MakeMemberOverloadDecl("string_trim", cel::StringType(), cel::StringType())));

            CEL_RETURN_IF_ERROR(builder.AddFunction(std::move(trim_decl)));
            return origin.configure(builder);
        }
    };

    return std::make_unique<cel::CompilerLibrary>(
        cel::CompilerLibrary::FromCheckerLibrary(std::move(checker_library)));
}

absl::Status RegisterStringsFunctions(cel::FunctionRegistry& function_registry,
    const cel::RuntimeOptions& runtime_options);

} // namespace rust::cel_cxx

#endif  // CEL_CXX_FFI_INCLUDE_EXTENSIONS_H_
