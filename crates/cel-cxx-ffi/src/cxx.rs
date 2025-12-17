use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/cxx.h>);
        fn CxxString_size_of() -> usize;
    }
}

impl crate::SizedExternType for cxx::CxxString {
    fn size_of() -> usize {
        ffi::CxxString_size_of()
    }
}

impl crate::absl::SpanElement for cxx::CxxString {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_CxxString");
}


pub trait CxxVectorExt<T: cxx::memory::UniquePtrTarget>: private::Sealed {
    fn push_unique(self: Pin<&mut Self>, value: cxx::UniquePtr<T>);
    fn pop_unique(self: Pin<&mut Self>) -> Option<cxx::UniquePtr<T>>;
}

impl<T: UniquePtrVectorElement> CxxVectorExt<T> for cxx::CxxVector<T> {
    fn push_unique(self: Pin<&mut Self>, value: cxx::UniquePtr<T>) {
        T::push_unique(self, value);
    }

    fn pop_unique(self: Pin<&mut Self>) -> Option<cxx::UniquePtr<T>> {
        if self.is_empty() {
            return None;
        }
        Some(T::pop_unique(self))
    }
}
impl<T: cxx::memory::UniquePtrTarget> private::Sealed for cxx::CxxVector<T> {}

pub unsafe trait UniquePtrVectorElement: cxx::vector::VectorElement + cxx::memory::UniquePtrTarget {
    fn push_unique(v: Pin<&mut cxx::CxxVector<Self>>, value: cxx::UniquePtr<Self>);
    fn pop_unique(v: Pin<&mut cxx::CxxVector<Self>>) -> cxx::UniquePtr<Self>;
}

mod private {
    pub trait Sealed {}
}
