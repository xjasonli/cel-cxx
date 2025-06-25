#ifndef CEL_CXX_FFI_INCLUDE_ABSL_H
#define CEL_CXX_FFI_INCLUDE_ABSL_H

#include <rust/cxx.h>
#include "absl/status/status.h"

namespace rust {
template <> struct IsRelocatable<::absl::Status> : std::true_type {};
} // namespace rust

namespace rust::cel_cxx {

using StringView = absl::string_view;
using StatusCode = absl::StatusCode;
using Status = absl::Status;

inline StringView StringView_new(Slice<const uint8_t> bytes) {
    return StringView((const char*)bytes.data(), bytes.size());
}

inline String StatusCode_to_string(StatusCode code) {
    return String(absl::StatusCodeToString(code));
}

inline Status Status_new(StatusCode code, Str msg) {
    return Status(code, std::string_view(msg));
}

inline Status Status_clone(const Status& status) {
    return Status(status);
}

inline void Status_drop(Status& status) {
    status.~Status();
}

inline String Status_to_string(const Status& status) {
    return String(status.ToString());
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_ABSL_H
