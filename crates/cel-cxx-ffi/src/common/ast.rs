use crate::protobuf::{Arena, DescriptorPool};
use crate::common::Type;

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

        type Ast;
        #[rust_name = "is_checked"]
        fn IsChecked(self: &Ast) -> bool;
    }

    #[namespace = "cel::ast_internal"]
    unsafe extern "C++" {
        include!(<common/ast/ast_impl.h>);

        type AstImpl;
        #[rust_name = "return_type"]
        fn GetReturnType(self: &AstImpl) -> &AstType;

        #[rust_name = "AstType"]
        type Type;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!("cel-cxx-ffi/include/common/ast.h");

        fn AstImpl_cast_from_public_ast(ast: &Ast) -> &AstImpl;
        fn AstType_to_type<'a>(
            ast_type: &AstType,
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
        let ast_impl = ffi::AstImpl_cast_from_public_ast(self);
        let ast_type = ast_impl.return_type();
        ffi::AstType_to_type(ast_type, descriptor_pool, arena)
    }
}
