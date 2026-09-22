#ifndef CEL_CXX_FFI_INCLUDE_ABSL_H
#define CEL_CXX_FFI_INCLUDE_ABSL_H

#include <rust/cxx.h>
#include <absl/status/status.h>
#include <absl/log/log_sink.h>
#include <absl/log/initialize.h>
#include <absl/log/internal/globals.h>
#include <absl/time/time.h>

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
    return absl::Seconds(seconds) + absl::Nanoseconds(nanos);
}

inline int64_t Duration_seconds(Duration duration) {
    return absl::ToInt64Seconds(absl::Floor(duration, absl::Seconds(1)));
}

inline uint32_t Duration_nanos(Duration duration) {
    return static_cast<uint32_t>(absl::ToInt64Nanoseconds(
        duration - absl::Floor(duration, absl::Seconds(1))));
}

inline Timestamp Timestamp_new(i64 seconds, u32 nanos) {
    return absl::FromUnixSeconds(seconds) + absl::Nanoseconds(nanos);
}

inline int64_t Timestamp_seconds(Timestamp timestamp) {
    return absl::ToUnixSeconds(timestamp);
}

inline uint32_t Timestamp_nanos(Timestamp timestamp) {
    return static_cast<uint32_t>(absl::ToInt64Nanoseconds(
        timestamp - absl::FromUnixSeconds(absl::ToUnixSeconds(timestamp))));
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

// Abseil error factories take a defaulted SourceLocation argument. CXX binds
// exact function pointer signatures, so call them through one-argument wrappers.
inline Status AbortedError(StringView msg) {
    return absl::AbortedError(msg);
}

inline Status AlreadyExistsError(StringView msg) {
    return absl::AlreadyExistsError(msg);
}

inline Status CancelledError(StringView msg) {
    return absl::CancelledError(msg);
}

inline Status DataLossError(StringView msg) {
    return absl::DataLossError(msg);
}

inline Status DeadlineExceededError(StringView msg) {
    return absl::DeadlineExceededError(msg);
}

inline Status FailedPreconditionError(StringView msg) {
    return absl::FailedPreconditionError(msg);
}

inline Status InternalError(StringView msg) {
    return absl::InternalError(msg);
}

inline Status InvalidArgumentError(StringView msg) {
    return absl::InvalidArgumentError(msg);
}

inline Status NotFoundError(StringView msg) {
    return absl::NotFoundError(msg);
}

inline Status OutOfRangeError(StringView msg) {
    return absl::OutOfRangeError(msg);
}

inline Status PermissionDeniedError(StringView msg) {
    return absl::PermissionDeniedError(msg);
}

inline Status ResourceExhaustedError(StringView msg) {
    return absl::ResourceExhaustedError(msg);
}

inline Status UnauthenticatedError(StringView msg) {
    return absl::UnauthenticatedError(msg);
}

inline Status UnavailableError(StringView msg) {
    return absl::UnavailableError(msg);
}

inline Status UnimplementedError(StringView msg) {
    return absl::UnimplementedError(msg);
}

inline Status UnknownError(StringView msg) {
    return absl::UnknownError(msg);
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
