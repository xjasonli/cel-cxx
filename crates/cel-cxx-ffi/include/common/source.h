#ifndef CEL_CXX_FFI_INCLUDE_COMMON_SOURCE_H_
#define CEL_CXX_FFI_INCLUDE_COMMON_SOURCE_H_

#include <rust/cxx.h>
#include <common/source.h>

namespace rust::cel_cxx {

using Source = cel::Source;

inline absl::Status Source_new(Slice<const uint8_t> content, Str description, std::unique_ptr<Source>& result) {
    auto content_view = std::string_view(reinterpret_cast<const char*>(content.data()), content.size());
    auto source = cel::NewSource(content_view, std::string(description));
    if (source.ok()) {
        result = std::move(source.value());
    }
    return source.status();
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_COMMON_SOURCE_H_