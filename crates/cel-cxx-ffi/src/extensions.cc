#include <cel-cxx-ffi/include/extensions.h>
#include <cel-cxx-ffi/src/extensions.rs.h>
#include <checker/internal/builtins_arena.h>
#include <runtime/function_adapter.h>
#include <checker/internal/builtins_arena.h>

namespace rust::cel_cxx {
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

} // namespace rust::cel_cxx
