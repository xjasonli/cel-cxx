#ifndef CEL_CXX_FFI_INCLUDE_EXTENSIONS_H_
#define CEL_CXX_FFI_INCLUDE_EXTENSIONS_H_

#include <rust/cxx.h>
#include <extensions/bindings_ext.h>
#include <extensions/encoders.h>
#include <extensions/lists_functions.h>
#include <extensions/math_ext_decls.h>
#include <extensions/proto_ext.h>
#include <extensions/regex_functions.h>
#include <extensions/select_optimization.h>
#include <extensions/sets_functions.h>
#include <extensions/strings.h>
#include <checker/internal/builtins_arena.h>

namespace cel::extensions {

// bindings_ext.h
cel::CompilerLibrary BindingsCompilerLibrary();

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

// SelectOptimizationOptions
inline cel::extensions::SelectOptimizationOptions SelectOptimizationOptions_default() {
    return cel::extensions::SelectOptimizationOptions();
}

// sets_functions.h
inline std::unique_ptr<cel::CompilerLibrary> SetsCompilerLibrary() {
    return std::make_unique<cel::CompilerLibrary>(cel::extensions::SetsCompilerLibrary());
}

// strings.h
inline std::unique_ptr<cel::CompilerLibrary> StringsCompilerLibrary() {
    return std::make_unique<cel::CompilerLibrary>(
        cel::CompilerLibrary::FromCheckerLibrary(
            cel::extensions::StringsCheckerLibrary()));
}

} // namespace rust::cel_cxx

#endif  // CEL_CXX_FFI_INCLUDE_EXTENSIONS_H_
