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
inline std::unique_ptr<ParserOptions> ParserOptions_new() {
    return std::make_unique<ParserOptions>();
}

// ParserOptions getters and setters
inline int ParserOptions_error_recovery_limit(const ParserOptions& parser_options) {
    return parser_options.error_recovery_limit;
}
inline int& ParserOptions_error_recovery_limit_mut(ParserOptions& parser_options) {
    return parser_options.error_recovery_limit;
}

inline int ParserOptions_max_recursion_depth(const ParserOptions& parser_options) {
    return parser_options.max_recursion_depth;
}
inline int& ParserOptions_max_recursion_depth_mut(ParserOptions& parser_options) {
    return parser_options.max_recursion_depth;
}

inline int ParserOptions_expression_size_codepoint_limit(const ParserOptions& parser_options) {
    return parser_options.expression_size_codepoint_limit;
}
inline int& ParserOptions_expression_size_codepoint_limit_mut(ParserOptions& parser_options) {
    return parser_options.expression_size_codepoint_limit;
}

inline int ParserOptions_error_recovery_token_lookahead_limit(const ParserOptions& parser_options) {
    return parser_options.error_recovery_token_lookahead_limit;
}
inline int& ParserOptions_error_recovery_token_lookahead_limit_mut(ParserOptions& parser_options) {
    return parser_options.error_recovery_token_lookahead_limit;
}

inline bool ParserOptions_add_macro_calls(const ParserOptions& parser_options) {
    return parser_options.add_macro_calls;
}
inline bool& ParserOptions_add_macro_calls_mut(ParserOptions& parser_options) {
    return parser_options.add_macro_calls;
}

inline bool ParserOptions_enable_optional_syntax(const ParserOptions& parser_options) {
    return parser_options.enable_optional_syntax;
}
inline bool& ParserOptions_enable_optional_syntax_mut(ParserOptions& parser_options) {
    return parser_options.enable_optional_syntax;
}

inline bool ParserOptions_disable_standard_macros(const ParserOptions& parser_options) {
    return parser_options.disable_standard_macros;
}
inline bool& ParserOptions_disable_standard_macros_mut(ParserOptions& parser_options) {
    return parser_options.disable_standard_macros;
}

inline bool ParserOptions_enable_hidden_accumulator_var(const ParserOptions& parser_options) {
    return parser_options.enable_hidden_accumulator_var;
}
inline bool& ParserOptions_enable_hidden_accumulator_var_mut(ParserOptions& parser_options) {
    return parser_options.enable_hidden_accumulator_var;
}

inline bool ParserOptions_enable_quoted_identifiers(const ParserOptions& parser_options) {
    return parser_options.enable_quoted_identifiers;
}
inline bool& ParserOptions_enable_quoted_identifiers_mut(ParserOptions& parser_options) {
    return parser_options.enable_quoted_identifiers;
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_PARSER_H_