use crate::absl::{Duration, Timestamp};

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/status/status.h>);
        include!(<absl/time/time.h>);
        type Time = super::Timestamp;
        type Duration = super::Duration;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<common/constant.h>);
        type ConstantKindCase = super::ConstantKindCase;

        type Constant;
        fn kind_case(self: &Constant) -> ConstantKindCase;
        fn has_value(self: &Constant) -> bool;
        fn has_null_value(self: &Constant) -> bool;
        fn has_bool_value(self: &Constant) -> bool;
        fn has_int_value(self: &Constant) -> bool;
        fn has_uint_value(self: &Constant) -> bool;
        fn has_double_value(self: &Constant) -> bool;
        fn has_bytes_value(self: &Constant) -> bool;
        fn has_string_value(self: &Constant) -> bool;
        fn has_duration_value(self: &Constant) -> bool;
        fn has_timestamp_value(self: &Constant) -> bool;
        fn bool_value(self: &Constant) -> bool;
        fn int_value(self: &Constant) -> i64;
        fn uint_value(self: &Constant) -> u64;
        fn double_value(self: &Constant) -> f64;
        fn bytes_value(self: &Constant) -> &CxxString;
        fn string_value(self: &Constant) -> &CxxString;
        fn duration_value(self: &Constant) -> Duration;
        fn timestamp_value(self: &Constant) -> Time;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/constant.h>);

        // Constnat
        fn Constant_new_null() -> UniquePtr<Constant>;
        fn Constant_new_bool(value: bool) -> UniquePtr<Constant>;
        fn Constant_new_int(value: i64) -> UniquePtr<Constant>;
        fn Constant_new_uint(value: u64) -> UniquePtr<Constant>;
        fn Constant_new_double(value: f64) -> UniquePtr<Constant>;
        fn Constant_new_bytes(value: &[u8]) -> UniquePtr<Constant>;
        fn Constant_new_string(value: &str) -> UniquePtr<Constant>;
        fn Constant_new_duration(value: Duration) -> UniquePtr<Constant>;
        fn Constant_new_timestamp(value: Time) -> UniquePtr<Constant>;
    }
}

// Constant
pub use ffi::Constant;
unsafe impl Send for Constant {}
unsafe impl Sync for Constant {}

impl Constant {
    pub fn new_null() -> cxx::UniquePtr<Self> {
        ffi::Constant_new_null()
    }

    pub fn new_bool(value: bool) -> cxx::UniquePtr<Self> {
        ffi::Constant_new_bool(value)
    }

    pub fn new_int(value: i64) -> cxx::UniquePtr<Self> {
        ffi::Constant_new_int(value)
    }

    pub fn new_uint(value: u64) -> cxx::UniquePtr<Self> {
        ffi::Constant_new_uint(value)
    }

    pub fn new_double(value: f64) -> cxx::UniquePtr<Self> {
        ffi::Constant_new_double(value)
    }

    pub fn new_bytes(value: &[u8]) -> cxx::UniquePtr<Self> {
        ffi::Constant_new_bytes(value)
    }

    pub fn new_string(value: &str) -> cxx::UniquePtr<Self> {
        ffi::Constant_new_string(value)
    }

    pub fn new_duration(value: Duration) -> cxx::UniquePtr<Self> {
        ffi::Constant_new_duration(value)
    }

    pub fn new_timestamp(value: Timestamp) -> cxx::UniquePtr<Self> {
        ffi::Constant_new_timestamp(value)
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConstantKindCase {
    Unspecified = 0,
    Null,
    Bool,
    Int,
    Uint,
    Double,
    Bytes,
    String,
    Duration,
    Timestamp,
}

unsafe impl cxx::ExternType for ConstantKindCase {
    type Id = cxx::type_id!("cel::ConstantKindCase");
    type Kind = cxx::kind::Trivial;
}

impl ConstantKindCase {
    pub fn is_unspecified(&self) -> bool {
        *self == ConstantKindCase::Unspecified
    }

    pub fn is_null(&self) -> bool {
        *self == ConstantKindCase::Null
    }

    pub fn is_bool(&self) -> bool {
        *self == ConstantKindCase::Bool
    }

    pub fn is_int(&self) -> bool {
        *self == ConstantKindCase::Int
    }

    pub fn is_uint(&self) -> bool {
        *self == ConstantKindCase::Uint
    }

    pub fn is_double(&self) -> bool {
        *self == ConstantKindCase::Double
    }
    
    pub fn is_bytes(&self) -> bool {
        *self == ConstantKindCase::Bytes
    }

    pub fn is_string(&self) -> bool {
        *self == ConstantKindCase::String
    }
    
    pub fn is_duration(&self) -> bool {
        *self == ConstantKindCase::Duration
    }

    pub fn is_timestamp(&self) -> bool {
        *self == ConstantKindCase::Timestamp
    }
}
