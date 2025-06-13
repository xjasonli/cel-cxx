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

inline Status RuntimeBuilder_new_standard(
    std::shared_ptr<google::protobuf::DescriptorPool> descriptor_pool,
    const RuntimeOptions& options,
    std::unique_ptr<RuntimeBuilder>& result
) {
    auto status_or = cel::CreateStandardRuntimeBuilder(
        descriptor_pool,
        options
    );
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

inline Str RuntimeOptions_get_container(const RuntimeOptions& options) {
    return Str(options.container.data(), options.container.size());
}
inline void RuntimeOptions_set_container(RuntimeOptions& options, Str container) {
    options.container = std::string(container);
}

inline cel::UnknownProcessingOptions RuntimeOptions_get_unknown_processing(const RuntimeOptions& options) {
    return options.unknown_processing;
}

inline void RuntimeOptions_set_unknown_processing(RuntimeOptions& options, cel::UnknownProcessingOptions value) {
    options.unknown_processing = value;
}

inline bool RuntimeOptions_get_enable_missing_attribute_errors(const RuntimeOptions& options) {
    return options.enable_missing_attribute_errors;
}

inline void RuntimeOptions_set_enable_missing_attribute_errors(RuntimeOptions& options, bool value) {
    options.enable_missing_attribute_errors = value;
}

inline bool RuntimeOptions_get_enable_timestamp_duration_overflow_errors(const RuntimeOptions& options) {
    return options.enable_timestamp_duration_overflow_errors;
}

inline void RuntimeOptions_set_enable_timestamp_duration_overflow_errors(RuntimeOptions& options, bool value) {
    options.enable_timestamp_duration_overflow_errors = value;
}

inline bool RuntimeOptions_get_short_circuiting(const RuntimeOptions& options) {
    return options.short_circuiting;
}

inline void RuntimeOptions_set_short_circuiting(RuntimeOptions& options, bool value) {
    options.short_circuiting = value;
}

inline bool RuntimeOptions_get_enable_comprehension(const RuntimeOptions& options) {
    return options.enable_comprehension;
}

inline void RuntimeOptions_set_enable_comprehension(RuntimeOptions& options, bool value) {
    options.enable_comprehension = value;
}

inline int RuntimeOptions_get_comprehension_max_iterations(const RuntimeOptions& options) {
    return options.comprehension_max_iterations;
}

inline void RuntimeOptions_set_comprehension_max_iterations(RuntimeOptions& options, int value) {
    options.comprehension_max_iterations = value;
}

inline bool RuntimeOptions_get_enable_comprehension_list_append(const RuntimeOptions& options) {
    return options.enable_comprehension_list_append;
}

inline void RuntimeOptions_set_enable_comprehension_list_append(RuntimeOptions& options, bool value) {
    options.enable_comprehension_list_append = value;
}

inline bool RuntimeOptions_get_enable_regex(const RuntimeOptions& options) {
    return options.enable_regex;
}

inline void RuntimeOptions_set_enable_regex(RuntimeOptions& options, bool value) {
    options.enable_regex = value;
}

inline int RuntimeOptions_get_regex_max_program_size(const RuntimeOptions& options) {
    return options.regex_max_program_size;
}

inline void RuntimeOptions_set_regex_max_program_size(RuntimeOptions& options, int value) {
    options.regex_max_program_size = value;
}

inline bool RuntimeOptions_get_enable_string_conversion(const RuntimeOptions& options) {
    return options.enable_string_conversion;
}

inline void RuntimeOptions_set_enable_string_conversion(RuntimeOptions& options, bool value) {
    options.enable_string_conversion = value;
}

inline bool RuntimeOptions_get_enable_string_concat(const RuntimeOptions& options) {
    return options.enable_string_concat;
}

inline void RuntimeOptions_set_enable_string_concat(RuntimeOptions& options, bool value) {
    options.enable_string_concat = value;
}

inline bool RuntimeOptions_get_enable_list_concat(const RuntimeOptions& options) {
    return options.enable_list_concat;
}

inline void RuntimeOptions_set_enable_list_concat(RuntimeOptions& options, bool value) {
    options.enable_list_concat = value;
}

inline bool RuntimeOptions_get_enable_list_contains(const RuntimeOptions& options) {
    return options.enable_list_contains;
}

inline void RuntimeOptions_set_enable_list_contains(RuntimeOptions& options, bool value) {
    options.enable_list_contains = value;
}

inline bool RuntimeOptions_get_fail_on_warnings(const RuntimeOptions& options) {
    return options.fail_on_warnings;
}

inline void RuntimeOptions_set_fail_on_warnings(RuntimeOptions& options, bool value) {
    options.fail_on_warnings = value;
}

inline bool RuntimeOptions_get_enable_qualified_type_identifiers(const RuntimeOptions& options) {
    return options.enable_qualified_type_identifiers;
}

inline void RuntimeOptions_set_enable_qualified_type_identifiers(RuntimeOptions& options, bool value) {
    options.enable_qualified_type_identifiers = value;
}

inline bool RuntimeOptions_get_enable_heterogeneous_equality(const RuntimeOptions& options) {
    return options.enable_heterogeneous_equality;
}

inline void RuntimeOptions_set_enable_heterogeneous_equality(RuntimeOptions& options, bool value) {
    options.enable_heterogeneous_equality = value;
}

inline bool RuntimeOptions_get_enable_empty_wrapper_null_unboxing(const RuntimeOptions& options) {
    return options.enable_empty_wrapper_null_unboxing;
}

inline void RuntimeOptions_set_enable_empty_wrapper_null_unboxing(RuntimeOptions& options, bool value) {
    options.enable_empty_wrapper_null_unboxing = value;
}

inline bool RuntimeOptions_get_enable_lazy_bind_initialization(const RuntimeOptions& options) {
    return options.enable_lazy_bind_initialization;
}

inline void RuntimeOptions_set_enable_lazy_bind_initialization(RuntimeOptions& options, bool value) {
    options.enable_lazy_bind_initialization = value;
}

inline int RuntimeOptions_get_max_recursion_depth(const RuntimeOptions& options) {
    return options.max_recursion_depth;
}

inline void RuntimeOptions_set_max_recursion_depth(RuntimeOptions& options, int value) {
    options.max_recursion_depth = value;
}

inline bool RuntimeOptions_get_enable_recursive_tracing(const RuntimeOptions& options) {
    return options.enable_recursive_tracing;
}

inline void RuntimeOptions_set_enable_recursive_tracing(RuntimeOptions& options, bool value) {
    options.enable_recursive_tracing = value;
}

inline bool RuntimeOptions_get_enable_fast_builtins(const RuntimeOptions& options) {
    return options.enable_fast_builtins;
}

inline void RuntimeOptions_set_enable_fast_builtins(RuntimeOptions& options, bool value) {
    options.enable_fast_builtins = value;
}


// FunctionDescriptor
inline std::unique_ptr<FunctionDescriptor> FunctionDescriptor_new(
    Str name, bool receiver_style, Slice<const Kind> types, bool is_strict
) {
    auto name_str = std::string_view(name);
    auto types_vec = std::vector<Kind>(types.data(), types.data() + types.size());
    return std::make_unique<FunctionDescriptor>(name_str, receiver_style, types_vec, is_strict);
}

inline std::shared_ptr<FunctionDescriptor> FunctionDescriptor_new_shared(
    Str name, bool receiver_style, Slice<const Kind> types, bool is_strict
) {
    auto name_str = std::string_view(name);
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
