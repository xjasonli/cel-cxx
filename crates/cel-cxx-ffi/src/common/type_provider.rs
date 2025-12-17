use crate::common::{Type, StructTypeField, ValueBuilder};
use crate::protobuf::{MessageFactory, Arena};
use crate::absl::{StringView, Status};

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/strings/string_view.h>);
        type string_view<'a> = super::StringView<'a>;

        include!(<absl/status/status.h>);
        type Status = super::Status;
    }

    #[namespace = "google::protobuf"]
    unsafe extern "C++" {
        include!(<google/protobuf/arena.h>);
        type Arena = super::Arena;

        include!(<google/protobuf/descriptor.h>);
        type MessageFactory = super::MessageFactory;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<common/type_reflector.h>);

        type Type<'a> = super::Type<'a>;
        type StructTypeField<'a> = super::StructTypeField<'a>;
        type ValueBuilder<'a> = super::ValueBuilder<'a>;

        type TypeIntrospector<'a>;
        type TypeReflector<'a>;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/absl.h>);
        include!(<cel-cxx-ffi/include/type_provider.h>);

        type EnumConstant<'a> = super::EnumConstant<'a>;

        // TypeIntrospector
        fn TypeIntrospector_new<'a>(ffi: Box<AnyFfiTypeIntrospector<'a>>) -> UniquePtr<TypeIntrospector<'a>>;

        // TypeReflector
        fn TypeReflector_new<'a>(ffi: Box<AnyFfiTypeReflector<'a>>) -> UniquePtr<TypeReflector<'a>>;
    }

    #[namespace = "rust::cel_cxx"]
    extern "Rust" {
        type AnyFfiTypeIntrospector<'a>;
        #[cxx_name = "FindTypeImpl"]
        unsafe fn find_type_impl<'a>(
            self: &AnyFfiTypeIntrospector<'a>,
            name: string_view<'_>,
            found: &mut bool,
            result: &mut Type<'a>,
        ) -> Status;

        #[cxx_name = "FindEnumConstantImpl"]
        unsafe fn find_enum_constant_impl<'a>(
            self: &AnyFfiTypeIntrospector<'a>,
            type_name: string_view<'_>,
            value_name: string_view<'_>,
            found: &mut bool,
            result: &mut EnumConstant<'a>,
        ) -> Status;

        #[cxx_name = "FindStructTypeFieldByNameImpl"]
        unsafe fn find_struct_type_field_by_name_impl<'a>(
            self: &AnyFfiTypeIntrospector<'a>,
            type_name: string_view<'_>,
            field_name: string_view<'_>,
            found: &mut bool,
            result: &mut StructTypeField<'a>,
        ) -> Status;

        type AnyFfiTypeReflector<'a>;
        #[cxx_name = "NewValueBuilder"]
        unsafe fn new_value_builder<'a>(
            self: &AnyFfiTypeReflector<'a>,
            name: string_view<'_>,
            message_factory: &MessageFactory,
            arena: &Arena,
            result: &mut UniquePtr<ValueBuilder<'a>>,
        ) -> Status;

        #[cxx_name = "FindTypeImpl"]
        unsafe fn find_type_impl<'a>(
            self: &AnyFfiTypeReflector<'a>,
            name: string_view<'_>,
            found: &mut bool,
            result: &mut Type<'a>,
        ) -> Status;

        #[cxx_name = "FindEnumConstantImpl"]
        unsafe fn find_enum_constant_impl<'a>(
            self: &AnyFfiTypeReflector<'a>,
            type_name: string_view<'_>,
            value_name: string_view<'_>,
            found: &mut bool,
            result: &mut EnumConstant<'a>,
        ) -> Status;

        #[cxx_name = "FindStructTypeFieldByNameImpl"]
        unsafe fn find_struct_type_field_by_name_impl<'a>(
            self: &AnyFfiTypeReflector<'a>,
            type_name: string_view<'_>,
            field_name: string_view<'_>,
            found: &mut bool,
            result: &mut StructTypeField<'a>,
        ) -> Status;
    }
}

// TypeIntrospector
#[allow(unused_variables)]
pub trait FfiTypeIntrospector<'a> {
    fn find_type_impl(&self, name: StringView<'_>) -> Result<Option<Type<'a>>, Status> {
        Ok(None)
    }

    fn find_enum_constant_impl(&self, type_name: StringView<'_>, value_name: StringView<'_>) -> Result<Option<EnumConstant<'a>>, Status> {
        Ok(None)
    }

    fn find_struct_type_field_by_name_impl(&self, type_name: StringView<'_>, field_name: StringView<'_>) -> Result<Option<StructTypeField<'a>>, Status> {
        Ok(None)
    }
}

struct AnyFfiTypeIntrospector<'a>(Box<dyn FfiTypeIntrospector<'a> + 'a>);

impl<'a> AnyFfiTypeIntrospector<'a> {
    fn new<T: FfiTypeIntrospector<'a> + 'a>(ffi: T) -> Self {
        Self(Box::new(ffi))
    }

    fn find_type_impl(&self, name: StringView<'_>, found: &mut bool, result: &mut Type<'a>) -> Status {
        match self.0.find_type_impl(name) {
            Ok(Some(type_)) => {
                *found = true;
                *result = type_;
                Status::ok()
            }
            Ok(None) => {
                *found = false;
                Status::ok()
            }
            Err(status) => status,
        }
    }

    fn find_enum_constant_impl(&self, type_name: StringView<'_>, value_name: StringView<'_>, found: &mut bool, result: &mut EnumConstant<'a>) -> Status {
        match self.0.find_enum_constant_impl(type_name, value_name) {
            Ok(Some(enum_constant)) => {
                *found = true;
                *result = enum_constant;
                Status::ok()
            }
            Ok(None) => {
                *found = false;
                Status::ok()
            }
            Err(status) => status,
        }
    }

    fn find_struct_type_field_by_name_impl(&self, type_name: StringView<'_>, field_name: StringView<'_>, found: &mut bool, result: &mut StructTypeField<'a>) -> Status {
        match self.0.find_struct_type_field_by_name_impl(type_name, field_name) {
            Ok(Some(struct_type_field)) => {
                *found = true;
                *result = struct_type_field;
                Status::ok()
            }
            Ok(None) => {
                *found = false;
                Status::ok()
            }
            Err(status) => status,
        }
    }
}

pub use ffi::TypeIntrospector;
unsafe impl<'a> Send for TypeIntrospector<'a> {}
unsafe impl<'a> Sync for TypeIntrospector<'a> {}

impl<'a> TypeIntrospector<'a> {
    pub fn new<T: FfiTypeIntrospector<'a> + 'a>(ffi: T) -> cxx::UniquePtr<Self> {
        ffi::TypeIntrospector_new(Box::new(AnyFfiTypeIntrospector::new(ffi)))
    }
}

// TypeReflector
pub trait FfiTypeReflector<'a>: FfiTypeIntrospector<'a> {
    fn new_value_builder(
        &self,
        name: StringView<'_>,
        message_factory: &MessageFactory,
        arena: &Arena,
    ) -> Result<cxx::UniquePtr<ValueBuilder<'a>>, Status>;
}

struct AnyFfiTypeReflector<'a>(Box<dyn FfiTypeReflector<'a> + 'a>);

impl<'a> AnyFfiTypeReflector<'a> {
    fn new<T: FfiTypeReflector<'a> + 'a>(ffi: T) -> Self {
        Self(Box::new(ffi))
    }

    fn new_value_builder(
        &self,
        name: StringView<'_>,
        message_factory: &MessageFactory,
        arena: &Arena,
        result: &mut cxx::UniquePtr<ValueBuilder<'a>>,
    ) -> Status {
        match self.0.new_value_builder(name, message_factory, arena) {
            Ok(value_builder) => {
                *result = value_builder;
                Status::ok()
            }
            Err(status) => status,
        }
    }

    fn find_type_impl(&self, name: StringView<'_>, found: &mut bool, result: &mut Type<'a>) -> Status {
        match self.0.find_type_impl(name) {
            Ok(Some(type_)) => {
                *found = true;
                *result = type_;
                Status::ok()
            }
            Ok(None) => {
                *found = false;
                Status::ok()
            }
            Err(status) => status,
        }
    }

    fn find_enum_constant_impl(&self, type_name: StringView<'_>, value_name: StringView<'_>, found: &mut bool, result: &mut EnumConstant<'a>) -> Status {
        match self.0.find_enum_constant_impl(type_name, value_name) {
            Ok(Some(enum_constant)) => {
                *found = true;
                *result = enum_constant;
                Status::ok()
            }
            Ok(None) => {
                *found = false;
                Status::ok()
            }
            Err(status) => status,
        }
    }

    fn find_struct_type_field_by_name_impl(&self, type_name: StringView<'_>, field_name: StringView<'_>, found: &mut bool, result: &mut StructTypeField<'a>) -> Status {
        match self.0.find_struct_type_field_by_name_impl(type_name, field_name) {
            Ok(Some(struct_type_field)) => {
                *found = true;
                *result = struct_type_field;
                Status::ok()
            }
            Ok(None) => {
                *found = false;
                Status::ok()
            }
            Err(status) => status,
        }
    }
}

pub use ffi::TypeReflector;
unsafe impl<'a> Send for TypeReflector<'a> {}
unsafe impl<'a> Sync for TypeReflector<'a> {}

impl<'a> TypeReflector<'a> {
    pub fn new<T: FfiTypeReflector<'a> + 'a>(ffi: T) -> cxx::UniquePtr<Self> {
        ffi::TypeReflector_new(Box::new(AnyFfiTypeReflector::new(ffi)))
    }
}

// EnumConstant
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EnumConstant<'a> {
    pub type_: Type<'a>,
    pub type_full_name: StringView<'a>,
    pub value_name: StringView<'a>,
    pub number: i32,
}

unsafe impl<'a> cxx::ExternType for EnumConstant<'a> {
    type Id = cxx::type_id!("rust::cel_cxx::EnumConstant");
    type Kind = cxx::kind::Trivial;
}
