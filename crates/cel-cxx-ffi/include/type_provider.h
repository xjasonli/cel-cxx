#ifndef CEL_CXX_FFI_INCLUDE_TYPE_PROVIDER_H_
#define CEL_CXX_FFI_INCLUDE_TYPE_PROVIDER_H_

#include <rust/cxx.h>
#include <common/type_reflector.h>

namespace rust::cel_cxx {

using EnumConstant = cel::TypeIntrospector::EnumConstant;
using TypeIntrospector = cel::TypeIntrospector;
using TypeReflector = cel::TypeReflector;

// TypeIntrospector
struct AnyFfiTypeIntrospector;
std::unique_ptr<TypeIntrospector> TypeIntrospector_new(Box<AnyFfiTypeIntrospector> ffi);

// TypeReflector
struct AnyFfiTypeReflector;
std::unique_ptr<TypeReflector> TypeReflector_new(Box<AnyFfiTypeReflector> ffi);

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_TYPE_PROVIDER_H_
