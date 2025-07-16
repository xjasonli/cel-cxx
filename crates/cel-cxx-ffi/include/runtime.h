#ifndef CEL_CXX_FFI_INCLUDE_RUNTIME_H_
#define CEL_CXX_FFI_INCLUDE_RUNTIME_H_

#include <rust/cxx.h>
#include <runtime/runtime.h>
#include <runtime/function.h>
#include <runtime/runtime_builder_factory.h>
#include <runtime/standard_runtime_builder_factory.h>
#include "absl.h"

namespace rust { inline namespace cxxbridge1 {
using LazyOverload = ::cel::FunctionRegistry::LazyOverload;
using FunctionOverloadReference = ::cel::FunctionOverloadReference;

template <>
Vec<FunctionOverloadReference>::Vec() noexcept;
template <>
void Vec<FunctionOverloadReference>::drop() noexcept;
template <>
::std::size_t Vec<FunctionOverloadReference>::size() const noexcept;
template <>
::std::size_t Vec<FunctionOverloadReference>::capacity() const noexcept;
template <>
FunctionOverloadReference const *Vec<FunctionOverloadReference>::data() const noexcept;
template <>
void Vec<FunctionOverloadReference>::reserve_total(::std::size_t new_cap) noexcept;
template <>
void Vec<FunctionOverloadReference>::set_len(::std::size_t len) noexcept;
template <>
void Vec<FunctionOverloadReference>::truncate(::std::size_t len);
template <>
Vec<LazyOverload>::Vec() noexcept;
template <>
void Vec<LazyOverload>::drop() noexcept;
template <>
::std::size_t Vec<LazyOverload>::size() const noexcept;
template <>
::std::size_t Vec<LazyOverload>::capacity() const noexcept;
template <>
LazyOverload const *Vec<LazyOverload>::data() const noexcept;
template <>
void Vec<LazyOverload>::reserve_total(::std::size_t new_cap) noexcept;
template <>
void Vec<LazyOverload>::set_len(::std::size_t len) noexcept;
template <>
void Vec<LazyOverload>::truncate(::std::size_t len);
}}

namespace rust::cel_cxx {

using Function = cel::Function;
using Status = absl::Status;
using Arena = google::protobuf::Arena;
using FunctionOverloadReference = cel::FunctionOverloadReference;
using MessageFactory = google::protobuf::MessageFactory;
using ActivationInterface = cel::ActivationInterface;
using Runtime = cel::Runtime;
using FunctionRegistry = cel::FunctionRegistry;
using RuntimeBuilder = cel::RuntimeBuilder;
using RuntimeOptions = cel::RuntimeOptions;
using Type = cel::Type;
using Value = cel::Value;
using Ast = cel::Ast;
using Program = cel::Program;
using Kind = cel::Kind;
using FunctionDescriptor = cel::FunctionDescriptor;
using TypeRegistry = cel::TypeRegistry;

using LazyOverload = cel::FunctionRegistry::LazyOverload;
using Span_Kind = absl::Span<const Kind>;
using Span_Value = absl::Span<const Value>;
using Span_CxxString = absl::Span<const std::string>;

struct FunctionRegistryIterator {
    using Map = absl::node_hash_map<std::string, std::vector<const cel::FunctionDescriptor*>>;
    using Iter = Map::const_iterator;
    FunctionRegistryIterator(Map&& map): impl_(std::make_unique<Impl>(std::move(map))) {}

    struct Impl {
        Impl(Map&& map): map_(std::move(map)), iter_(map_.begin()) {}
        Map map_;
        Iter iter_;
    };

    std::unique_ptr<Impl> impl_;
};

inline bool FunctionRegistryIterator_next(
    FunctionRegistryIterator& iter,
    std::unique_ptr<std::string>& key,
    std::unique_ptr<std::vector<FunctionDescriptor>>& value
) {
    if (iter.impl_->iter_ == iter.impl_->map_.end()) {
        return false;
    }

    auto key_ptr = std::make_unique<std::string>(iter.impl_->iter_->first);
    auto value_ptr = std::make_unique<std::vector<FunctionDescriptor>>();
    key.swap(key_ptr);
    for (auto descriptor : iter.impl_->iter_->second) {
        value_ptr->push_back(*descriptor);
    }
    value.swap(value_ptr);

    ++iter.impl_->iter_;
    return true;
}
inline void FunctionRegistryIterator_drop(FunctionRegistryIterator& iter) {
    iter.impl_.reset();
}

struct AnyFfiActivation;
class AnyFfiActivationWrapper: public ActivationInterface {
public:
    AnyFfiActivationWrapper(Box<AnyFfiActivation>&& ffi)
        : ffi_(std::move(ffi)) {}
    
    virtual absl::StatusOr<bool> FindVariable(
        absl::string_view name,
        const google::protobuf::DescriptorPool* ABSL_NONNULL descriptor_pool,
        google::protobuf::MessageFactory* ABSL_NONNULL message_factory,
        google::protobuf::Arena* ABSL_NONNULL arena, Value* ABSL_NONNULL result) const override;

    virtual std::vector<FunctionOverloadReference> FindFunctionOverloads(
        absl::string_view name) const override;

    virtual absl::Span<const cel::AttributePattern> GetUnknownAttributes()
        const override { return {}; }

    virtual absl::Span<const cel::AttributePattern> GetMissingAttributes()
        const override { return {}; }

private:
    Box<AnyFfiActivation> ffi_;
};


// ActivationInterface
inline std::unique_ptr<ActivationInterface> Activation_new(
    Box<AnyFfiActivation> ffi
) {
    return std::make_unique<AnyFfiActivationWrapper>(std::move(ffi));
}

// FunctionRegistry
inline Vec<FunctionOverloadReference> FunctionRegistry_find_static_overloads(
    const FunctionRegistry& function_registry,
    absl::string_view name, bool receiver_style, absl::Span<const Kind> types
) {
    auto overloads = function_registry.FindStaticOverloads(name, receiver_style, types);
    Vec<FunctionOverloadReference> result;
    for (const auto& overload : overloads) {
        result.push_back(overload);
    }
    return result;
}

inline Vec<FunctionOverloadReference> FunctionRegistry_find_static_overloads_by_arity(
    const FunctionRegistry& function_registry,
    absl::string_view name, bool receiver_style, size_t arity
) {
    auto overloads = function_registry.FindStaticOverloadsByArity(name, receiver_style, arity);
    Vec<FunctionOverloadReference> result;
    for (const auto& overload : overloads) {
        result.push_back(overload);
    }
    return result;
}

inline Vec<LazyOverload> FunctionRegistry_find_lazy_overloads(
    const FunctionRegistry& function_registry,
    absl::string_view name, bool receiver_style, absl::Span<const Kind> types
) {
    auto overloads = function_registry.FindLazyOverloads(name, receiver_style, types);
    Vec<LazyOverload> result;
    for (const auto& overload : overloads) {
        result.push_back(overload);
    }
    return result;
}

inline Vec<LazyOverload> FunctionRegistry_find_lazy_overloads_by_arity(
    const FunctionRegistry& function_registry,
    absl::string_view name, bool receiver_style, size_t arity
) {
    auto overloads = function_registry.FindLazyOverloadsByArity(name, receiver_style, arity);
    Vec<LazyOverload> result;
    for (const auto& overload : overloads) {
        result.push_back(overload);
    }
    return result;
}

inline FunctionRegistryIterator FunctionRegistry_list_functions(
    const FunctionRegistry& function_registry
) {
    auto map = function_registry.ListFunctions();
    return FunctionRegistryIterator(std::move(map));
}

// Runtime
inline Status Runtime_create_program(const Runtime& runtime,
    std::unique_ptr<Ast> ast, std::unique_ptr<Program>& result
) {
    auto status_or = runtime.CreateProgram(std::move(ast));
    if (!status_or.ok()) {
        return status_or.status();
    }
    result.reset(status_or.value().release());
    return Status();
}

// RuntimeBuilder
inline Status RuntimeBuilder_new(
    std::shared_ptr<google::protobuf::DescriptorPool> descriptor_pool,
    const RuntimeOptions& options,
    std::unique_ptr<RuntimeBuilder>& result
) {
    auto status_or = cel::CreateRuntimeBuilder(
        descriptor_pool,
        options);
    if (!status_or.ok()) {
        return status_or.status();
    }
    auto builder = std::make_unique<RuntimeBuilder>(std::move(status_or.value()));
    result.swap(builder);
    return Status();
}

inline Status RuntimeBuilder_build(
    RuntimeBuilder& builder,
    std::unique_ptr<Runtime>& result
) {
    auto status_or = std::move(builder).Build();
    if (!status_or.ok()) {
        return status_or.status();
    }
    auto runtime_ptr = status_or.value().release();
    result.reset(const_cast<Runtime*>(runtime_ptr));
    return Status();
}

// RuntimeOptions
inline std::unique_ptr<RuntimeOptions> RuntimeOptions_new() {
    return std::make_unique<RuntimeOptions>();
}

inline Str RuntimeOptions_container(const RuntimeOptions& options) {
    return Str(options.container.data(), options.container.size());
}
inline std::string& RuntimeOptions_container_mut(RuntimeOptions& options) {
    return options.container;
}

inline cel::UnknownProcessingOptions RuntimeOptions_unknown_processing(const RuntimeOptions& options) {
    return options.unknown_processing;
}

inline cel::UnknownProcessingOptions& RuntimeOptions_unknown_processing_mut(RuntimeOptions& options) {
    return options.unknown_processing;
}

inline bool RuntimeOptions_enable_missing_attribute_errors(const RuntimeOptions& options) {
    return options.enable_missing_attribute_errors;
}

inline bool& RuntimeOptions_enable_missing_attribute_errors_mut(RuntimeOptions& options) {
    return options.enable_missing_attribute_errors;
}

inline bool RuntimeOptions_enable_timestamp_duration_overflow_errors(const RuntimeOptions& options) {
    return options.enable_timestamp_duration_overflow_errors;
}

inline bool& RuntimeOptions_enable_timestamp_duration_overflow_errors_mut(RuntimeOptions& options) {
    return options.enable_timestamp_duration_overflow_errors;
}

inline bool RuntimeOptions_short_circuiting(const RuntimeOptions& options) {
    return options.short_circuiting;
}

inline bool& RuntimeOptions_short_circuiting_mut(RuntimeOptions& options) {
    return options.short_circuiting;
}

inline bool RuntimeOptions_enable_comprehension(const RuntimeOptions& options) {
    return options.enable_comprehension;
}

inline bool& RuntimeOptions_enable_comprehension_mut(RuntimeOptions& options) {
    return options.enable_comprehension;
}

inline int RuntimeOptions_comprehension_max_iterations(const RuntimeOptions& options) {
    return options.comprehension_max_iterations;
}

inline int& RuntimeOptions_comprehension_max_iterations_mut(RuntimeOptions& options) {
    return options.comprehension_max_iterations;
}

inline bool RuntimeOptions_enable_comprehension_list_append(const RuntimeOptions& options) {
    return options.enable_comprehension_list_append;
}

inline bool& RuntimeOptions_enable_comprehension_list_append_mut(RuntimeOptions& options) {
    return options.enable_comprehension_list_append;
}

inline bool RuntimeOptions_enable_regex(const RuntimeOptions& options) {
    return options.enable_regex;
}

inline bool& RuntimeOptions_enable_regex_mut(RuntimeOptions& options) {
    return options.enable_regex;
}

inline int RuntimeOptions_regex_max_program_size(const RuntimeOptions& options) {
    return options.regex_max_program_size;
}

inline int& RuntimeOptions_regex_max_program_size_mut(RuntimeOptions& options) {
    return options.regex_max_program_size;
}

inline bool RuntimeOptions_enable_string_conversion(const RuntimeOptions& options) {
    return options.enable_string_conversion;
}

inline bool& RuntimeOptions_enable_string_conversion_mut(RuntimeOptions& options) {
    return options.enable_string_conversion;
}

inline bool RuntimeOptions_enable_string_concat(const RuntimeOptions& options) {
    return options.enable_string_concat;
}

inline bool& RuntimeOptions_enable_string_concat_mut(RuntimeOptions& options) {
    return options.enable_string_concat;
}

inline bool RuntimeOptions_enable_list_concat(const RuntimeOptions& options) {
    return options.enable_list_concat;
}

inline bool& RuntimeOptions_enable_list_concat_mut(RuntimeOptions& options) {
    return options.enable_list_concat;
}

inline bool RuntimeOptions_enable_list_contains(const RuntimeOptions& options) {
    return options.enable_list_contains;
}

inline bool& RuntimeOptions_enable_list_contains_mut(RuntimeOptions& options) {
    return options.enable_list_contains;
}

inline bool RuntimeOptions_fail_on_warnings(const RuntimeOptions& options) {
    return options.fail_on_warnings;
}

inline bool& RuntimeOptions_fail_on_warnings_mut(RuntimeOptions& options) {
    return options.fail_on_warnings;
}

inline bool RuntimeOptions_enable_qualified_type_identifiers(const RuntimeOptions& options) {
    return options.enable_qualified_type_identifiers;
}

inline bool& RuntimeOptions_enable_qualified_type_identifiers_mut(RuntimeOptions& options) {
    return options.enable_qualified_type_identifiers;
}

inline bool RuntimeOptions_enable_heterogeneous_equality(const RuntimeOptions& options) {
    return options.enable_heterogeneous_equality;
}

inline bool& RuntimeOptions_enable_heterogeneous_equality_mut(RuntimeOptions& options) {
    return options.enable_heterogeneous_equality;
}

inline bool RuntimeOptions_enable_empty_wrapper_null_unboxing(const RuntimeOptions& options) {
    return options.enable_empty_wrapper_null_unboxing;
}

inline bool& RuntimeOptions_enable_empty_wrapper_null_unboxing_mut(RuntimeOptions& options) {
    return options.enable_empty_wrapper_null_unboxing;
}

inline bool RuntimeOptions_enable_lazy_bind_initialization(const RuntimeOptions& options) {
    return options.enable_lazy_bind_initialization;
}

inline bool& RuntimeOptions_enable_lazy_bind_initialization_mut(RuntimeOptions& options) {
    return options.enable_lazy_bind_initialization;
}

inline int RuntimeOptions_max_recursion_depth(const RuntimeOptions& options) {
    return options.max_recursion_depth;
}

inline int& RuntimeOptions_max_recursion_depth_mut(RuntimeOptions& options) {
    return options.max_recursion_depth;
}

inline bool RuntimeOptions_enable_recursive_tracing(const RuntimeOptions& options) {
    return options.enable_recursive_tracing;
}

inline bool& RuntimeOptions_enable_recursive_tracing_mut(RuntimeOptions& options) {
    return options.enable_recursive_tracing;
}

inline bool RuntimeOptions_enable_fast_builtins(const RuntimeOptions& options) {
    return options.enable_fast_builtins;
}

inline bool& RuntimeOptions_enable_fast_builtins_mut(RuntimeOptions& options) {
    return options.enable_fast_builtins;
}


// FunctionDescriptor
inline std::unique_ptr<FunctionDescriptor> FunctionDescriptor_new(
    Str name, bool receiver_style, Slice<const Kind> types, bool is_strict
) {
    auto name_str = std::string_view(name.data(), name.size());
    auto types_vec = std::vector<Kind>(types.data(), types.data() + types.size());
    return std::make_unique<FunctionDescriptor>(name_str, receiver_style, types_vec, is_strict);
}

inline std::shared_ptr<FunctionDescriptor> FunctionDescriptor_new_shared(
    Str name, bool receiver_style, Slice<const Kind> types, bool is_strict
) {
    auto name_str = std::string_view(name.data(), name.size());
    auto types_vec = std::vector<Kind>(types.data(), types.data() + types.size());
    return std::make_shared<FunctionDescriptor>(name_str, receiver_style, types_vec, is_strict);
}

// Program
inline Status Program_evaluate(
    const Program& program,
    const Arena& arena,
    const MessageFactory& message_factory,
    const ActivationInterface& activation,
    std::unique_ptr<Value>& result
) {
    auto status_or = program.Evaluate(
        &const_cast<Arena&>(arena),
        &const_cast<MessageFactory&>(message_factory),
        activation
    );

    if (!status_or.ok()) {
        return status_or.status();
    }

    result = std::make_unique<Value>(std::move(status_or.value()));

    return Status();
}

inline Status Program_evaluate2(
    const Program& program,
    const Arena& arena,
    const ActivationInterface& activation,
    std::unique_ptr<Value>& result
) {
    auto status_or = program.Evaluate(
        &const_cast<Arena&>(arena),
        activation
    );
    if (!status_or.ok()) {
        return status_or.status();
    }
    
    result = std::make_unique<Value>(std::move(status_or.value()));

    return Status();
}

struct AnyFfiFunction;

class AnyFfiFunctionWrapper: public cel::Function {
public:
    AnyFfiFunctionWrapper(Box<AnyFfiFunction> ffi)
        : ffi_(std::move(ffi)) {}
        

    virtual absl::StatusOr<Value> Invoke(
        absl::Span<const Value> args,
        const google::protobuf::DescriptorPool* ABSL_NONNULL descriptor_pool,
        google::protobuf::MessageFactory* ABSL_NONNULL message_factory,
        google::protobuf::Arena* ABSL_NONNULL arena,
        absl::Span<const std::string> overload_id) const override;
private:
    Box<AnyFfiFunction> ffi_;
};

inline std::unique_ptr<Function> Function_new(Box<AnyFfiFunction> ffi_function) {
    return std::make_unique<AnyFfiFunctionWrapper>(std::move(ffi_function));
}

} // namespace rust::cel_cxx

namespace rust {
template <> struct IsRelocatable<::rust::cel_cxx::FunctionRegistryIterator> : std::true_type {};
}

#endif // CEL_CXX_FFI_INCLUDE_RUNTIME_H_
