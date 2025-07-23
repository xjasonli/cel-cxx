use crate::absl::Status;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/status/status.h>);
        type Status = super::Status;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<common/source.h>);
        type Source;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/absl.h>);
        include!(<cel-cxx-ffi/include/source.h>);

        fn Source_new(content: &[u8], description: &str, result: &mut UniquePtr<Source>) -> Status;
    }
}

pub use ffi::Source;
unsafe impl Send for Source {}
unsafe impl Sync for Source {}

impl Source {
    pub fn new(content: &[u8], description: &str) -> Result<cxx::UniquePtr<Self>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::Source_new(content, description, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }
}
