#ifndef CEL_CXX_FFI_OPTIONAL_H_
#define CEL_CXX_FFI_OPTIONAL_H_

#include <runtime/optional_types.h>
#include <checker/type_checker.h>
#include <compiler/compiler.h>

namespace rust::cel_cxx {

cel::CheckerLibrary PatchOptionalCheckerLibrary(cel::CheckerLibrary origin);
cel::CompilerLibrary PatchOptionalCompilerLibrary(cel::CompilerLibrary origin);

// patched version of cel::EnableOptionalTypes in <runtime/optional_types.h>
absl::Status EnableOptionalTypes(cel::RuntimeBuilder& builder);

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_OPTIONAL_H_
