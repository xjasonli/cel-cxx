pub mod absl;
pub mod checker;
pub mod common;
pub mod compiler;
mod cxx;
pub mod extensions;
pub mod parser;
pub mod protobuf;
pub mod runtime;

pub use absl::*;
pub use checker::*;
pub use common::*;
pub use compiler::*;
pub use extensions::*;
pub use cxx::*;
pub use parser::*;
pub use protobuf::*;
pub use runtime::*;

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
