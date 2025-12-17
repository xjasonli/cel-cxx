#include <cel-cxx-ffi/include/checker.h>
#include <cel-cxx-ffi/src/checker.rs.h>

namespace rust::cel_cxx {

std::unique_ptr<TypeCheckerBuilderConfigurer> TypeCheckerBuilderConfigurer_new(
    Box<AnyFfiTypeCheckerBuilderConfigurer> ffi_configurer) {
    auto configurer = [ffi_configurer = std::move(ffi_configurer)] (TypeCheckerBuilder& builder) {
        return ffi_configurer->Call(builder);
    };
    return std::make_unique<TypeCheckerBuilderConfigurer>(std::move(configurer));
}

std::unique_ptr<FunctionPredicate> FunctionPredicate_new(
    Box<AnyFfiFunctionPredicate> ffi_predicate) {
    auto predicate = [ffi_predicate = std::move(ffi_predicate)] (absl::string_view function, absl::string_view overload_id) {
        return ffi_predicate->Call(function, overload_id);
    };
    return std::make_unique<FunctionPredicate>(std::move(predicate));
}

} // namespace rust::cel_cxx
