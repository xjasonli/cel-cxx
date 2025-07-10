#include "cel-cxx-ffi/include/extensions.h"

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
