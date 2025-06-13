mod cxx;
pub mod absl;
pub mod protobuf;
pub mod common;
pub mod checker;
pub mod parser;
pub mod runtime;
pub mod compiler;
pub mod extensions;

pub use absl::*;
pub use protobuf::*;
pub use common::*;
pub use checker::*;
pub use parser::*;
pub use runtime::*;
pub use compiler::*;
pub use extensions::*;

// Rep is a helper type to represent data layout.
#[repr(C)]
#[derive(Copy, Clone)]
struct Rep<'a, T: Copy + Clone, const N: usize> {
    _space: std::mem::MaybeUninit<[T; N]>,
    _marker: std::marker::PhantomData<&'a ()>,
}


pub trait SizedExternType: ::cxx::ExternType + Sized {
    fn size_of() -> usize {
        std::mem::size_of::<Self>()
    }
}
