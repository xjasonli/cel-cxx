#[cxx::bridge]
mod ffi {
    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!("cel-cxx-ffi/include/cxx.h");
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
