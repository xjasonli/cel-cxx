#ifndef CEL_CXX_FFI_INCLUDE_CONSTANT_H_
#define CEL_CXX_FFI_INCLUDE_CONSTANT_H_

#include <rust/cxx.h>
#include <common/constant.h>

namespace rust::cel_cxx {

using Duration = absl::Duration;
using Timestamp = absl::Time;
using Constant = cel::Constant;

// Constant
inline std::unique_ptr<Constant> Constant_new_null() {
    auto constant = std::make_unique<Constant>();
    constant->set_null_value();
    return constant;
}

inline std::unique_ptr<Constant> Constant_new_bool(bool value) {
    auto constant = std::make_unique<Constant>();
    constant->set_bool_value(value);
    return constant;
}

inline std::unique_ptr<Constant> Constant_new_int(i64 value) {
    auto constant = std::make_unique<Constant>();
    constant->set_int_value(value);
    return constant;
}

inline std::unique_ptr<Constant> Constant_new_uint(u64 value) {
    auto constant = std::make_unique<Constant>();
    constant->set_uint_value(value);
    return constant;
}

inline std::unique_ptr<Constant> Constant_new_double(f64 value) {
    auto constant = std::make_unique<Constant>();
    constant->set_double_value(value);
    return constant;
}

inline std::unique_ptr<Constant> Constant_new_bytes(Slice<const u8> value) {
    auto constant = std::make_unique<Constant>();
    constant->set_bytes_value(std::string(reinterpret_cast<const char*>(value.data()), value.size()));
    return constant;
}

inline std::unique_ptr<Constant> Constant_new_string(Str value) {
    auto constant = std::make_unique<Constant>();
    constant->set_string_value(std::string(value));
    return constant;
}

inline std::unique_ptr<Constant> Constant_new_duration(Duration value) {
    auto constant = std::make_unique<Constant>();
    constant->set_duration_value(value);
    return constant;
}

inline std::unique_ptr<Constant> Constant_new_timestamp(Timestamp value) {
    auto constant = std::make_unique<Constant>();
    constant->set_timestamp_value(value);
    return constant;
}


} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_CONSTANT_H_