#include <cel-cxx-ffi/include/parser.h>
#include <cel-cxx-ffi/src/parser.rs.h>

namespace rust::cel_cxx {

std::unique_ptr<GlobalMacroExpander> GlobalMacroExpander_new(
    Box<AnyFfiGlobalMacroExpander> expander) {
    auto expander_function = [expander = std::move(expander)] (
        MacroExprFactory& factory, absl::Span<Expr> args) -> absl::optional<Expr> {
        auto args_vec = std::vector<std::unique_ptr<Expr>>();
        for (auto& arg : args) {
            args_vec.push_back(std::make_unique<Expr>(std::move(arg)));
        }
        auto expr = expander->Call(factory, Slice<std::unique_ptr<Expr>>(args_vec));
        absl::optional<Expr> result = absl::nullopt;
        if (expr) {
            result = std::move(*expr);
        }
        return result;
    };
    return std::make_unique<GlobalMacroExpander>(std::move(expander_function));
}

std::unique_ptr<ReceiverMacroExpander> ReceiverMacroExpander_new(
    Box<AnyFfiReceiverMacroExpander> expander) {
    auto expander_function = [expander = std::move(expander)] (
        MacroExprFactory& factory, Expr& target, absl::Span<Expr> args) -> absl::optional<Expr> {
        auto args_vec = std::vector<std::unique_ptr<Expr>>();
        for (auto& arg : args) {
            args_vec.push_back(std::make_unique<Expr>(std::move(arg)));
        }
        auto expr = expander->Call(factory, std::make_unique<Expr>(std::move(target)), Slice<std::unique_ptr<Expr>>(args_vec));
        absl::optional<Expr> result = absl::nullopt;
        if (expr) {
            result = std::move(*expr);
        }
        return result;
    };
    return std::make_unique<ReceiverMacroExpander>(std::move(expander_function));
}

std::unique_ptr<ParserBuilderConfigurer> ParserBuilderConfigurer_new(
    Box<AnyFfiParserBuilderConfigurer> ffi_configurer) {
    auto configurer = [ffi_configurer = std::move(ffi_configurer)] (ParserBuilder& builder) {
        return ffi_configurer->Call(builder);
    };
    return std::make_unique<ParserBuilderConfigurer>(std::move(configurer));
}

std::unique_ptr<MacroPredicate> MacroPredicate_new(
    Box<AnyFfiMacroPredicate> ffi_predicate) {
    auto predicate = [ffi_predicate = std::move(ffi_predicate)] (const Macro& m) {
        return ffi_predicate->Call(m);
    };
    return std::make_unique<MacroPredicate>(std::move(predicate));
}

} // namespace rust::cel_cxx
