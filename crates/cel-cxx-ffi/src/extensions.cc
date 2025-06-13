#include "cel-cxx-ffi/include/extensions.h"

namespace cel::extensions {

using ::cel::checker_internal::BuiltinsArena;

TypeParamType TypeParamA() { return TypeParamType("A"); }
TypeParamType TypeParamB() { return TypeParamType("B"); }

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

// lists_functions.h
absl::Status RegisterListsDecls(TypeCheckerBuilder& builder) {
    CEL_ASSIGN_OR_RETURN(
        auto distinct_decl,
        MakeFunctionDecl(
            "distinct",
            MakeMemberOverloadDecl(
                "list_distinct",
                ListType(BuiltinsArena(), TypeParamA()),
                ListType(BuiltinsArena(), TypeParamA()))));

    CEL_ASSIGN_OR_RETURN(
        auto flatten_decl,
        MakeFunctionDecl(
            "flatten",
            MakeMemberOverloadDecl(
                "list_flatten",
                ListType(BuiltinsArena(), TypeParamA()),
                ListType(BuiltinsArena(), ListType(BuiltinsArena(), TypeParamA())))));

    CEL_ASSIGN_OR_RETURN(
        auto range_decl,
        MakeFunctionDecl(
            "lists.range",
            MakeOverloadDecl(
                "lists.range",
                ListType(BuiltinsArena(), IntType()),
                IntType())));

    CEL_ASSIGN_OR_RETURN(
        auto reverse_decl,
        MakeFunctionDecl(
            "reverse",
            MakeMemberOverloadDecl(
                "list_reverse",
                ListType(BuiltinsArena(), TypeParamA()),
                ListType(BuiltinsArena(), TypeParamA()))));

    CEL_ASSIGN_OR_RETURN(
        auto slice_decl,
        MakeFunctionDecl(
            "slice",
            MakeMemberOverloadDecl(
                "list_slice",
                ListType(BuiltinsArena(), TypeParamA()),
                ListType(BuiltinsArena(), TypeParamA()),
                IntType(),
                IntType())));
    
    CEL_ASSIGN_OR_RETURN(
        auto sort_decl,
        MakeFunctionDecl(
            "sort",
            MakeMemberOverloadDecl(
                "list_sort",
                ListType(BuiltinsArena(), TypeParamA()),
                ListType(BuiltinsArena(), TypeParamA()))));
    
    CEL_ASSIGN_OR_RETURN(
        auto sort_by_associated_keys_decl,
        MakeFunctionDecl(
            "@sortByAssociatedKeys",
            MakeMemberOverloadDecl(
                "list_@sortByAssociatedKeys",
                ListType(BuiltinsArena(), TypeParamA()),
                ListType(BuiltinsArena(), TypeParamA()),
                ListType(BuiltinsArena(), TypeParamB()))));

    CEL_RETURN_IF_ERROR(builder.MergeFunction(distinct_decl));
    CEL_RETURN_IF_ERROR(builder.MergeFunction(flatten_decl));
    CEL_RETURN_IF_ERROR(builder.MergeFunction(range_decl));
    CEL_RETURN_IF_ERROR(builder.MergeFunction(reverse_decl));
    CEL_RETURN_IF_ERROR(builder.MergeFunction(slice_decl));
    CEL_RETURN_IF_ERROR(builder.MergeFunction(sort_decl));
    CEL_RETURN_IF_ERROR(builder.MergeFunction(sort_by_associated_keys_decl));
    return absl::OkStatus();
}

std::vector<Macro> lists_macros();
absl::Status AddListsExtensionMacros(ParserBuilder& builder) {
  for (const auto& m : lists_macros()) {
    CEL_RETURN_IF_ERROR(builder.AddMacro(m));
  }
  return absl::OkStatus();
}

CompilerLibrary ListsCompilerLibrary() {
    return CompilerLibrary(
        "cel.lib.ext.lists",
        AddListsExtensionMacros,
        RegisterListsDecls);
}


// regex_functions.h
absl::Status RegisterRegexDecls(TypeCheckerBuilder& builder) {
    CEL_ASSIGN_OR_RETURN(
        auto extract_decl,
        MakeFunctionDecl(
            std::string(kRegexExtract),
            MakeOverloadDecl(
                kRegexExtract,
                StringType(),
                StringType(), StringType(), StringType())));

    CEL_ASSIGN_OR_RETURN(
        auto capture_decl,
        MakeFunctionDecl(
            std::string(kRegexCapture),
            MakeOverloadDecl(
                kRegexCapture,
                StringType(),
                StringType(), StringType())));
    
    CEL_ASSIGN_OR_RETURN(
        auto capture_n_decl,
        MakeFunctionDecl(
            std::string(kRegexCaptureN),
            MakeOverloadDecl(
                kRegexCaptureN,
                MapType(BuiltinsArena(), StringType(), StringType()),
                StringType(), StringType())));
    
    CEL_RETURN_IF_ERROR(builder.MergeFunction(extract_decl));
    CEL_RETURN_IF_ERROR(builder.MergeFunction(capture_decl));
    CEL_RETURN_IF_ERROR(builder.MergeFunction(capture_n_decl));
    return absl::OkStatus();
}

CompilerLibrary RegexCompilerLibrary() {
    return CompilerLibrary(
        "cel.lib.ext.regex",
        RegisterRegexDecls);
}

} // namespace cel::extensions
