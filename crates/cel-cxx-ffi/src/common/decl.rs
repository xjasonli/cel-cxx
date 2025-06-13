use crate::absl::{Status, Duration, Timestamp};
use crate::common::Type;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/status/status.h>);
        include!(<absl/time/time.h>);
        type Status = super::Status;
        type Time = super::Timestamp;
        type Duration = super::Duration;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<common/type.h>);
        type Type<'a> = super::Type<'a>;

        include!(<common/decl.h>);
        type VariableDecl<'a>;
        type OverloadDecl<'a>;
        type FunctionDecl<'a>;
        #[rust_name = "add_overload"]
        fn AddOverload<'a>(self: Pin<&mut FunctionDecl<'a>>, overload: &OverloadDecl<'a>) -> Status;

        type Constant;
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
        include!("cel-cxx-ffi/include/absl.h");
        include!("cel-cxx-ffi/include/common/decl.h");

        // VariableDecl
        fn VariableDecl_new<'a>(name: &str, ty: &Type<'a>) -> UniquePtr<VariableDecl<'a>>;
        fn VariableDecl_new_constant<'a>(name: &str, value: &Constant) -> UniquePtr<VariableDecl<'a>>;

        // FunctionDecl
        fn FunctionDecl_new<'a>(name: &str) -> UniquePtr<FunctionDecl<'a>>;

        // OverloadDecl
        fn OverloadDecl_new<'a>(id: &str, member: bool, result: &Type<'a>, args: &[Type<'a>]) -> UniquePtr<OverloadDecl<'a>>;

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

pub use ffi::VariableDecl;
unsafe impl Send for VariableDecl<'_> {}
unsafe impl Sync for VariableDecl<'_> {}

impl<'a> VariableDecl<'a> {
    pub fn new(name: &str, ty: &Type<'a>) -> cxx::UniquePtr<Self> {
        ffi::VariableDecl_new(name, ty)
    }

    pub fn new_constant(name: &str, value: &Constant) -> cxx::UniquePtr<Self> {
        ffi::VariableDecl_new_constant(name, value)
    }
}

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

pub use ffi::FunctionDecl;
unsafe impl Send for FunctionDecl<'_> {}
unsafe impl Sync for FunctionDecl<'_> {}

impl<'a> FunctionDecl<'a> {
    pub fn new(name: &str) -> cxx::UniquePtr<Self> {
        ffi::FunctionDecl_new(name)
    }
}

pub use ffi::OverloadDecl;
unsafe impl Send for OverloadDecl<'_> {}
unsafe impl Sync for OverloadDecl<'_> {}

impl<'a> OverloadDecl<'a> {
    pub fn new(id: &str, member: bool, result: &Type<'a>, arguments: &[Type<'a>]) -> cxx::UniquePtr<Self> {
        ffi::OverloadDecl_new(id, member, result, arguments)
    }
}
