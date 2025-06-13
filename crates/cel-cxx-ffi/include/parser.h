#ifndef CEL_CXX_FFI_INCLUDE_PARSER_H_
#define CEL_CXX_FFI_INCLUDE_PARSER_H_

#include <rust/cxx.h>
#include <parser/parser.h>

namespace rust::cel_cxx {

using MacroRegistry = cel::MacroRegistry;
using ParserOptions = cel::ParserOptions;
using ParserBuilder = cel::ParserBuilder;

// MacroRegistry
inline std::unique_ptr<cel::MacroRegistry> MacroRegistry_new() {
    return std::make_unique<cel::MacroRegistry>();
}

// ParserOptions
inline ParserOptions ParserOptions_default() {
    return ParserOptions();
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_PARSER_H_