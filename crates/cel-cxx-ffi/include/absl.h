#ifndef CEL_CXX_FFI_INCLUDE_ABSL_H
#define CEL_CXX_FFI_INCLUDE_ABSL_H

#include <rust/cxx.h>
#include <absl/status/status.h>
#include <absl/log/log_sink.h>
#include <absl/log/initialize.h>
#include <absl/log/internal/globals.h>

namespace rust {
template <> struct IsRelocatable<::absl::Status> : std::true_type {};
} // namespace rust

namespace rust::cel_cxx {

using Duration = absl::Duration;
using Timestamp = absl::Time;
using StringView = absl::string_view;
using StatusCode = absl::StatusCode;
using Status = absl::Status;

inline Duration Duration_new(i64 seconds, u32 nanos) {
    return absl::time_internal::MakeDuration(seconds, nanos);
}

inline int64_t Duration_seconds(Duration duration) {
    return absl::time_internal::GetRepHi(duration);
}

inline uint32_t Duration_nanos(Duration duration) {
    return absl::time_internal::GetRepLo(duration);
}

inline Timestamp Timestamp_new(i64 seconds, u32 nanos) {
    return absl::time_internal::FromUnixDuration(absl::time_internal::MakeDuration(seconds, nanos));
}

inline int64_t Timestamp_seconds(Timestamp timestamp) {
    return absl::time_internal::GetRepHi(absl::time_internal::ToUnixDuration(timestamp));
}

inline uint32_t Timestamp_nanos(Timestamp timestamp) {
    return absl::time_internal::GetRepLo(absl::time_internal::ToUnixDuration(timestamp));
}

inline StringView StringView_new(Slice<const uint8_t> bytes) {
    return StringView((const char*)bytes.data(), bytes.size());
}

inline String StatusCode_to_string(StatusCode code) {
    return String(absl::StatusCodeToString(code));
}

inline Status Status_new(StatusCode code, Str msg) {
    return Status(code, std::string_view(msg.data(), msg.size()));
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

inline void InitializeLog() {
    if (!absl::log_internal::IsInitialized()) {
        absl::InitializeLog();
    }
}

// Set log callback (internal use)
void SetLogCallback();

// Custom log sink for bridging to Rust
class RustLogSink : public absl::LogSink {
public:
    void Send(const absl::LogEntry& entry) override;
};

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_ABSL_H
