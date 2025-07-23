#include <cel-cxx-ffi/include/extensions.h>
#include <cel-cxx-ffi/src/extensions.rs.h>
#include <checker/internal/builtins_arena.h>
#include <runtime/function_adapter.h>
#include <checker/internal/builtins_arena.h>

namespace cel::extensions {

// bindings_ext.h
absl::Status AddBindingsExtensionMacros(ParserBuilder& builder) {
  for (const auto& m : bindings_macros()) {
    CEL_RETURN_IF_ERROR(builder.AddMacro(m));
  }
  return absl::OkStatus();
}

CompilerLibrary BindingsCompilerLibrary() {
    return CompilerLibrary(
        "cel.lib.ext.bindings",
        AddBindingsExtensionMacros);
}


} // namespace cel::extensions

namespace rust::cel_cxx {
using cel::Value;
using cel::StringValue;
using google::protobuf::DescriptorPool;
using google::protobuf::MessageFactory;
using google::protobuf::Arena;
using cel::BinaryFunctionAdapter;
using cel::TernaryFunctionAdapter;
using cel::UnaryFunctionAdapter;
using cel::checker_internal::BuiltinsArena;

// sets_functions.h
absl::Status RegisterSetsDecls(cel::TypeCheckerBuilder& b) {
  cel::ListType list_t(BuiltinsArena(), cel::TypeParamType("T"));
  CEL_ASSIGN_OR_RETURN(
      auto decl,
      MakeFunctionDecl("sets.contains",
                       MakeOverloadDecl("list_sets_contains_list", cel::BoolType(),
                                        list_t, list_t)));
  CEL_RETURN_IF_ERROR(b.AddFunction(decl));

  CEL_ASSIGN_OR_RETURN(
      decl, MakeFunctionDecl("sets.equivalent",
                             MakeOverloadDecl("list_sets_equivalent_list",
                                              cel::BoolType(), list_t, list_t)));
  CEL_RETURN_IF_ERROR(b.AddFunction(decl));

  CEL_ASSIGN_OR_RETURN(
      decl, MakeFunctionDecl("sets.intersects",
                             MakeOverloadDecl("list_sets_intersects_list",
                                              cel::BoolType(), list_t, list_t)));
  CEL_RETURN_IF_ERROR(b.AddFunction(decl));

  return absl::OkStatus();
}

cel::CheckerLibrary SetsCheckerLibrary() {
  return {.id = "cel.lib.ext.sets", .configure = RegisterSetsDecls};
}

// strings.h
absl::StatusOr<StringValue> CharAt(
    const StringValue& string, int64_t index,
    const DescriptorPool* descriptor_pool, MessageFactory* message_factory, Arena* arena) {
  String result;
  String input = String::lossy(string.ToString());
  CEL_RETURN_IF_ERROR(
    CharAtImpl(result, Str(input), index));
  return StringValue::From(std::string(result), arena);
}

absl::StatusOr<int64_t> IndexOfOffset(
    const StringValue& string,
    const StringValue& substring,
    int64_t start_index) {
  int64_t result;
  String input = String::lossy(string.ToString());
  String substr = String::lossy(substring.ToString());
  CEL_RETURN_IF_ERROR(IndexOfImpl(result, Str(input), Str(substr), start_index));
  return result;
}

absl::StatusOr<int64_t> IndexOf(
    const StringValue& string,
    const StringValue& substring) {
  return IndexOfOffset(string, substring, 0);
}

absl::StatusOr<int64_t> LastIndexOfOffset(
    const StringValue& string,
    const StringValue& substring,
    int64_t start_index) {
  int64_t result;
  String input = String::lossy(string.ToString());
  String substr = String::lossy(substring.ToString());
  CEL_RETURN_IF_ERROR(LastIndexOfImpl(result, Str(input), Str(substr), start_index));
  return result;
}

absl::StatusOr<int64_t> LastIndexOf(
    const StringValue& string,
    const StringValue& substring) {
  return LastIndexOfOffset(string, substring, string.Size() - 1);
}

absl::StatusOr<StringValue> StringsQuote(
    const StringValue& string,
    const DescriptorPool* descriptor_pool, MessageFactory* message_factory, Arena* arena) {
  String result;
  String input = String::lossy(string.ToString());
  CEL_RETURN_IF_ERROR(StringsQuoteImpl(result, Str(input)));
  return StringValue::From(std::string(result), arena);
}

absl::StatusOr<StringValue> SubstringRange(
    const StringValue& string,
    int64_t start,
    int64_t end,
    const DescriptorPool* descriptor_pool, MessageFactory* message_factory, Arena* arena) {
  String result;
  String input = String::lossy(string.ToString());
  CEL_RETURN_IF_ERROR(SubstringImpl(result, Str(input), start, end));
  return StringValue::From(std::string(result), arena);
}

absl::StatusOr<StringValue> Substring(
    const StringValue& string,
    int64_t start,
    const DescriptorPool* descriptor_pool, MessageFactory* message_factory, Arena* arena) {
  return SubstringRange(string, start, string.Size(), descriptor_pool, message_factory, arena);
}

absl::StatusOr<StringValue> Trim(
    const StringValue& string,
    const DescriptorPool* descriptor_pool, MessageFactory* message_factory, Arena* arena) {
  String result;
  String input = String::lossy(string.ToString());
  CEL_RETURN_IF_ERROR(TrimImpl(result, Str(input)));
  return StringValue::From(std::string(result), arena);
}

absl::StatusOr<StringValue> Reverse(
    const StringValue& string,
    const DescriptorPool* descriptor_pool, MessageFactory* message_factory, Arena* arena) {
  String result;
  String input = String::lossy(string.ToString());
  CEL_RETURN_IF_ERROR(ReverseImpl(result, Str(input)));
  return StringValue::From(std::string(result), arena);
}

absl::Status RegisterStringsFunctions(cel::FunctionRegistry& function_registry,
    const cel::RuntimeOptions& runtime_options) {
    CEL_RETURN_IF_ERROR(function_registry.Register(
      BinaryFunctionAdapter<absl::StatusOr<Value>, const StringValue&, int64_t>::CreateDescriptor(
        "charAt", true),
      BinaryFunctionAdapter<absl::StatusOr<Value>, const StringValue&, int64_t>::WrapFunction(
        CharAt)));

    CEL_RETURN_IF_ERROR(function_registry.Register(
      BinaryFunctionAdapter<absl::StatusOr<int64_t>, const StringValue&, const StringValue&>::CreateDescriptor(
        "indexOf", true),
      BinaryFunctionAdapter<absl::StatusOr<int64_t>, const StringValue&, const StringValue&>::WrapFunction(
        IndexOf)));

    CEL_RETURN_IF_ERROR(function_registry.Register(
      TernaryFunctionAdapter<absl::StatusOr<int64_t>, const StringValue&, const StringValue&, int64_t>::CreateDescriptor(
        "indexOf", true),
      TernaryFunctionAdapter<absl::StatusOr<int64_t>, const StringValue&, const StringValue&, int64_t>::WrapFunction(
        IndexOfOffset)));

    CEL_RETURN_IF_ERROR(function_registry.Register(
      BinaryFunctionAdapter<absl::StatusOr<int64_t>, const StringValue&, const StringValue&>::CreateDescriptor(
        "lastIndexOf", true),
      BinaryFunctionAdapter<absl::StatusOr<int64_t>, const StringValue&, const StringValue&>::WrapFunction(
        LastIndexOf)));

    CEL_RETURN_IF_ERROR(function_registry.Register(
      TernaryFunctionAdapter<absl::StatusOr<int64_t>, const StringValue&, const StringValue&, int64_t>::CreateDescriptor(
        "lastIndexOf", true),
      TernaryFunctionAdapter<absl::StatusOr<int64_t>, const StringValue&, const StringValue&, int64_t>::WrapFunction(
        LastIndexOfOffset)));

    CEL_RETURN_IF_ERROR(function_registry.Register(
      UnaryFunctionAdapter<absl::StatusOr<StringValue>, const StringValue&>::CreateDescriptor(
        "strings.quote", false),
      UnaryFunctionAdapter<absl::StatusOr<StringValue>, const StringValue&>::WrapFunction(
        StringsQuote)));

    CEL_RETURN_IF_ERROR(function_registry.Register(
      TernaryFunctionAdapter<absl::StatusOr<StringValue>, const StringValue&, int64_t, int64_t>::CreateDescriptor(
        "substring", true),
      TernaryFunctionAdapter<absl::StatusOr<StringValue>, const StringValue&, int64_t, int64_t>::WrapFunction(
        SubstringRange)));

    CEL_RETURN_IF_ERROR(function_registry.Register(
      BinaryFunctionAdapter<absl::StatusOr<StringValue>, const StringValue&, int64_t>::CreateDescriptor(
        "substring", true),
      BinaryFunctionAdapter<absl::StatusOr<StringValue>, const StringValue&, int64_t>::WrapFunction(
        Substring)));

    CEL_RETURN_IF_ERROR(function_registry.Register(
      UnaryFunctionAdapter<absl::StatusOr<StringValue>, const StringValue&>::CreateDescriptor(
        "trim", true),
      UnaryFunctionAdapter<absl::StatusOr<StringValue>, const StringValue&>::WrapFunction(
        Trim)));

    CEL_RETURN_IF_ERROR(function_registry.Register(
      UnaryFunctionAdapter<absl::StatusOr<StringValue>, const StringValue&>::CreateDescriptor(
        "reverse", true),
      UnaryFunctionAdapter<absl::StatusOr<StringValue>, const StringValue&>::WrapFunction(
        Reverse)));

    return cel::extensions::RegisterStringsFunctions(function_registry, runtime_options);
}

} // namespace rust::cel_cxx
