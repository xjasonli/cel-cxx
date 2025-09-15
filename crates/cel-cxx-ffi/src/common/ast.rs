use crate::common::Type;
use crate::protobuf::{Arena, DescriptorPool};

#[cxx::bridge]
mod ffi {
    #[namespace = "google::protobuf"]
    unsafe extern "C++" {
        include!(<google/protobuf/descriptor.h>);

        type Arena = super::Arena;
        type DescriptorPool = super::DescriptorPool;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<common/type.h>);
        type Type<'a> = super::Type<'a>;

        include!(<common/ast.h>);
        type TypeSpec;

        type Ast;
        #[rust_name = "is_checked"]
        fn IsChecked(self: &Ast) -> bool;

        #[rust_name = "return_type_spec"]
        fn GetReturnType(self: &Ast) -> &TypeSpec;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/ast.h>);

        fn TypeSpec_to_type<'a>(
            ast_type: &TypeSpec,
            descriptor_pool: &DescriptorPool,
            arena: &'a Arena,
        ) -> Type<'a>;
    }

    impl UniquePtr<Ast> {}
}

pub use ffi::Ast;
unsafe impl Send for Ast {}
unsafe impl Sync for Ast {}

impl Ast {
    pub fn return_type<'a>(&self, descriptor_pool: &DescriptorPool, arena: &'a Arena) -> Type<'a> {
        let type_spec = self.return_type_spec();
        ffi::TypeSpec_to_type(type_spec, descriptor_pool, arena)
    }
}
