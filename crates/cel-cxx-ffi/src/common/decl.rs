use crate::absl::{Duration, Status, Timestamp};
use crate::common::{Type, Constant};

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
        fn AddOverload<'a>(self: Pin<&mut FunctionDecl<'a>>, overload: &OverloadDecl<'a>)
            -> Status;

        type Constant = super::Constant;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/absl.h>);
        include!(<cel-cxx-ffi/include/decl.h>);

        // VariableDecl
        fn VariableDecl_new<'a>(name: &str, ty: &Type<'a>) -> UniquePtr<VariableDecl<'a>>;
        fn VariableDecl_new_constant<'a>(
            name: &str,
            value: &Constant,
        ) -> UniquePtr<VariableDecl<'a>>;

        // FunctionDecl
        fn FunctionDecl_new<'a>(name: &str) -> UniquePtr<FunctionDecl<'a>>;

        // OverloadDecl
        fn OverloadDecl_new<'a>(
            id: &str,
            member: bool,
            result: &Type<'a>,
            args: &[Type<'a>],
        ) -> UniquePtr<OverloadDecl<'a>>;
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
    pub fn new(
        id: &str,
        member: bool,
        result: &Type<'a>,
        arguments: &[Type<'a>],
    ) -> cxx::UniquePtr<Self> {
        ffi::OverloadDecl_new(id, member, result, arguments)
    }
}
