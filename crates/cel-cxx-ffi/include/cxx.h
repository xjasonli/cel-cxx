#ifndef CEL_CXX_FFI_INCLUDE_CXX_H_
#define CEL_CXX_FFI_INCLUDE_CXX_H_

#include <string>

namespace rust::cel_cxx {

inline size_t CxxString_size_of() {
    return sizeof(std::string);
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_CXX_H_
