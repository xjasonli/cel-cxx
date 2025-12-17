#ifndef CEL_CXX_FFI_INCLUDE_PARSER_H_
#define CEL_CXX_FFI_INCLUDE_PARSER_H_

#include <rust/cxx.h>
#include <parser/parser.h>

namespace rust::cel_cxx {

using Expr = cel::Expr;
using Macro = cel::Macro;
using ListExprElement = cel::ListExprElement;
using StructExprField = cel::StructExprField;
using MapExprEntry = cel::MapExprEntry;
using GlobalMacroExpander = cel::GlobalMacroExpander;
using ReceiverMacroExpander = cel::ReceiverMacroExpander;

using ParserBuilderConfigurer = cel::ParserBuilderConfigurer;
using ParserLibrary = cel::ParserLibrary;
using MacroPredicate = cel::ParserLibrarySubset::MacroPredicate;
using ParserLibrarySubset = cel::ParserLibrarySubset;
using MacroRegistry = cel::MacroRegistry;
using MacroExprFactory = cel::MacroExprFactory;
using ParserOptions = cel::ParserOptions;
using ParserBuilder = cel::ParserBuilder;

using Span_Expr = absl::Span<const cel::Expr>;
using Span_ListExprElement = absl::Span<const cel::ListExprElement>;
using Span_StructExprField = absl::Span<const cel::StructExprField>;
using Span_MapExprEntry = absl::Span<const cel::MapExprEntry>;

using MutSpan_Expr = absl::Span<cel::Expr>;
using MutSpan_ListExprElement = absl::Span<cel::ListExprElement>;
using MutSpan_StructExprField = absl::Span<cel::StructExprField>;
using MutSpan_MapExprEntry = absl::Span<cel::MapExprEntry>;

struct AnyFfiGlobalMacroExpander;
struct AnyFfiReceiverMacroExpander;
struct AnyFfiParserBuilderConfigurer;
struct AnyFfiMacroPredicate;

// MacroRegistry
inline std::unique_ptr<cel::MacroRegistry> MacroRegistry_new() {
    return std::make_unique<cel::MacroRegistry>();
}

// GlobalMacroExpander
std::unique_ptr<GlobalMacroExpander> GlobalMacroExpander_new(
    Box<AnyFfiGlobalMacroExpander> expander);

// ReceiverMacroExpander
std::unique_ptr<ReceiverMacroExpander> ReceiverMacroExpander_new(
    Box<AnyFfiReceiverMacroExpander> expander);

// Macro
inline absl::Status Macro_new_global(
    absl::string_view name,
    size_t argument_count,
    std::unique_ptr<GlobalMacroExpander> expander,
    std::unique_ptr<Macro>& result) {
    auto status = cel::Macro::Global(name, argument_count, std::move(*expander));
    if (status.ok()) {
        result = std::make_unique<Macro>(std::move(status.value()));
    }
    return status.status();
}

inline absl::Status Macro_new_global_var_arg(
    absl::string_view name,
    std::unique_ptr<GlobalMacroExpander> expander,
    std::unique_ptr<Macro>& result) {
    auto status = cel::Macro::GlobalVarArg(name, std::move(*expander));
    if (status.ok()) {
        result = std::make_unique<Macro>(std::move(status.value()));
    }
    return status.status();
}

inline absl::Status Macro_new_receiver(
    absl::string_view name,
    size_t argument_count,
    std::unique_ptr<ReceiverMacroExpander> expander,
    std::unique_ptr<Macro>& result) {
    auto status = cel::Macro::Receiver(name, argument_count, std::move(*expander));
    if (status.ok()) {
        result = std::make_unique<Macro>(std::move(status.value()));
    }
    return status.status();
}

inline absl::Status Macro_new_receiver_var_arg(
    absl::string_view name,
    std::unique_ptr<ReceiverMacroExpander> expander,
    std::unique_ptr<Macro>& result) {
    auto status = cel::Macro::ReceiverVarArg(name, std::move(*expander));
    if (status.ok()) {
        result = std::make_unique<Macro>(std::move(status.value()));
    }
    return status.status();
}

// MacroExprFactory
inline std::unique_ptr<Expr> MacroExprFactory_copy(MacroExprFactory& factory, const Expr& expr) {
    return std::make_unique<Expr>(factory.Copy(expr));
}

inline std::unique_ptr<ListExprElement> MacroExprFactory_copy_list_element(MacroExprFactory& factory, const ListExprElement& list_element) {
    return std::make_unique<ListExprElement>(factory.Copy(list_element));
}

inline std::unique_ptr<StructExprField> MacroExprFactory_copy_struct_field(MacroExprFactory& factory, const StructExprField& struct_field) {
    return std::make_unique<StructExprField>(factory.Copy(struct_field));
}

inline std::unique_ptr<MapExprEntry> MacroExprFactory_copy_map_entry(MacroExprFactory& factory, const MapExprEntry& map_entry) {
    return std::make_unique<MapExprEntry>(factory.Copy(map_entry));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_unspecified(MacroExprFactory& factory) {
    return std::make_unique<Expr>(factory.NewUnspecified());
}

inline std::unique_ptr<Expr> MacroExprFactory_new_null_const(MacroExprFactory& factory) {
    return std::make_unique<Expr>(factory.NewNullConst());
}

inline std::unique_ptr<Expr> MacroExprFactory_new_bool_const(MacroExprFactory& factory, bool value) {
    return std::make_unique<Expr>(factory.NewBoolConst(value));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_int_const(MacroExprFactory& factory, int64_t value) {
    return std::make_unique<Expr>(factory.NewIntConst(value));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_double_const(MacroExprFactory& factory, double value) {
    return std::make_unique<Expr>(factory.NewDoubleConst(value));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_bytes_const(MacroExprFactory& factory, absl::string_view value) {
    return std::make_unique<Expr>(factory.NewBytesConst(value));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_string_const(MacroExprFactory& factory, absl::string_view value) {
    return std::make_unique<Expr>(factory.NewStringConst(value));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_ident(MacroExprFactory& factory, absl::string_view name) {
    return std::make_unique<Expr>(factory.NewIdent(name));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_accu_ident(MacroExprFactory& factory) {
    return std::make_unique<Expr>(factory.NewAccuIdent());
}

inline std::unique_ptr<Expr> MacroExprFactory_new_select(MacroExprFactory& factory, std::unique_ptr<Expr> operand, absl::string_view field) {
    return std::make_unique<Expr>(factory.NewSelect(std::move(*operand), field));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_presence_test(MacroExprFactory& factory, std::unique_ptr<Expr> operand, absl::string_view field) {
    return std::make_unique<Expr>(factory.NewPresenceTest(std::move(*operand), field));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_call(MacroExprFactory& factory, absl::string_view function, std::unique_ptr<std::vector<Expr>> args) {
    return std::make_unique<Expr>(factory.NewCall(function, std::move(*args)));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_member_call(MacroExprFactory& factory, absl::string_view function, std::unique_ptr<Expr> target, std::unique_ptr<std::vector<Expr>> args) {
    return std::make_unique<Expr>(factory.NewMemberCall(function, std::move(*target), std::move(*args)));
}

inline std::unique_ptr<ListExprElement> MacroExprFactory_new_list_element(MacroExprFactory& factory, std::unique_ptr<Expr> expr, bool optional) {
    return std::make_unique<ListExprElement>(factory.NewListElement(std::move(*expr), optional));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_list(MacroExprFactory& factory, std::unique_ptr<std::vector<ListExprElement>> elements) {
    return std::make_unique<Expr>(factory.NewList(std::move(*elements)));
}

inline std::unique_ptr<StructExprField> MacroExprFactory_new_struct_field(MacroExprFactory& factory, absl::string_view name, std::unique_ptr<Expr> value, bool optional) {
    return std::make_unique<StructExprField>(factory.NewStructField(name, std::move(*value), optional));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_struct(MacroExprFactory& factory, absl::string_view name, std::unique_ptr<std::vector<StructExprField>> fields) {
    return std::make_unique<Expr>(factory.NewStruct(name, std::move(*fields)));
}

inline std::unique_ptr<MapExprEntry> MacroExprFactory_new_map_entry(MacroExprFactory& factory, std::unique_ptr<Expr> key, std::unique_ptr<Expr> value, bool optional) {
    return std::make_unique<MapExprEntry>(factory.NewMapEntry(std::move(*key), std::move(*value), optional));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_map(MacroExprFactory& factory, std::unique_ptr<std::vector<MapExprEntry>> entries) {
    return std::make_unique<Expr>(factory.NewMap(std::move(*entries)));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_comprehension(MacroExprFactory& factory, absl::string_view iter_var, std::unique_ptr<Expr> iter_range, absl::string_view accu_var, std::unique_ptr<Expr> accu_init, std::unique_ptr<Expr> loop_condition, std::unique_ptr<Expr> loop_step, std::unique_ptr<Expr> result) {
    return std::make_unique<Expr>(factory.NewComprehension(iter_var, std::move(*iter_range), accu_var, std::move(*accu_init), std::move(*loop_condition), std::move(*loop_step), std::move(*result)));
}

inline std::unique_ptr<Expr> MacroExprFactory_new_comprehension2(MacroExprFactory& factory, absl::string_view iter_var, absl::string_view iter_var2, std::unique_ptr<Expr> iter_range, absl::string_view accu_var, std::unique_ptr<Expr> accu_init, std::unique_ptr<Expr> loop_condition, std::unique_ptr<Expr> loop_step, std::unique_ptr<Expr> result) {
    return std::make_unique<Expr>(factory.NewComprehension(iter_var, iter_var2, std::move(*iter_range), accu_var, std::move(*accu_init), std::move(*loop_condition), std::move(*loop_step), std::move(*result)));
}

inline std::unique_ptr<Expr> MacroExprFactory_report_error(MacroExprFactory& factory, absl::string_view message) {
    return std::make_unique<Expr>(factory.ReportError(message));
}

inline std::unique_ptr<Expr> MacroExprFactory_report_error_at(MacroExprFactory& factory, const Expr& expr, absl::string_view message) {
    return std::make_unique<Expr>(factory.ReportErrorAt(expr, message));
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

// ParserBuilderConfigurer
std::unique_ptr<ParserBuilderConfigurer> ParserBuilderConfigurer_new(
    Box<AnyFfiParserBuilderConfigurer> ffi_configurer);

// ParserLibrary
inline std::unique_ptr<ParserLibrary> ParserLibrary_new(
    const std::string& id,
    std::unique_ptr<ParserBuilderConfigurer> configurer) {
    return std::make_unique<ParserLibrary>(ParserLibrary{
        id,
        std::move(*configurer),
    });
}

inline const std::string& ParserLibrary_id(const ParserLibrary& parser_library) {
    return parser_library.id;
}

// MacroPredicate
std::unique_ptr<MacroPredicate> MacroPredicate_new(
    Box<AnyFfiMacroPredicate> ffi_predicate);

// ParserLibrarySubset
inline std::unique_ptr<ParserLibrarySubset> ParserLibrarySubset_new(
    const std::string& library_id,
    std::unique_ptr<MacroPredicate> should_include_macro) {
    return std::make_unique<ParserLibrarySubset>(ParserLibrarySubset{
        library_id,
        std::move(*should_include_macro),
    });
}

inline const std::string& ParserLibrarySubset_library_id(const ParserLibrarySubset& parser_library_subset) {
    return parser_library_subset.library_id;
}

// ParserBuilder
inline absl::Status ParserBuilder_add_library(ParserBuilder& parser_builder, std::unique_ptr<ParserLibrary> library) {
    return parser_builder.AddLibrary(std::move(*library));
}

inline absl::Status ParserBuilder_add_library_subset(ParserBuilder& parser_builder, std::unique_ptr<ParserLibrarySubset> library_subset) {
    return parser_builder.AddLibrarySubset(std::move(*library_subset));
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_PARSER_H_