use super::*;

pub(crate) fn error_from_rust(error: &rust::Error) -> Status {
    match error.code() {
        rust::Code::Ok => Status::ok(),
        rust::Code::Cancelled => Status::cancelled(error.message()),
        rust::Code::Unknown => Status::unknown(error.message()),
        rust::Code::InvalidArgument => Status::invalid_argument(error.message()),
        rust::Code::DeadlineExceeded => Status::deadline_exceeded(error.message()),
        rust::Code::NotFound => Status::not_found(error.message()),
        rust::Code::AlreadyExists => Status::already_exists(error.message()),
        rust::Code::PermissionDenied => Status::permission_denied(error.message()),
        rust::Code::ResourceExhausted => Status::resource_exhausted(error.message()),
        rust::Code::FailedPrecondition => Status::failed_precondition(error.message()),
        rust::Code::Aborted => Status::aborted(error.message()),
        rust::Code::OutOfRange => Status::out_of_range(error.message()),
        rust::Code::Unimplemented => Status::unimplemented(error.message()),
        rust::Code::Internal => Status::internal(error.message()),
        rust::Code::Unavailable => Status::unavailable(error.message()),
        rust::Code::DataLoss => Status::data_loss(error.message()),
        rust::Code::Unauthenticated => Status::unauthenticated(error.message()),
    }
}

pub(crate) fn error_to_rust(ffi_status: &Status) -> rust::Error {
    match ffi_status.code() {
        StatusCode::Ok => rust::Error::ok(ffi_status.message().to_string_lossy()),
        StatusCode::Cancelled => rust::Error::cancelled(ffi_status.message().to_string_lossy()),
        StatusCode::Unknown => rust::Error::unknown(ffi_status.message().to_string_lossy()),
        StatusCode::InvalidArgument => {
            rust::Error::invalid_argument(ffi_status.message().to_string_lossy())
        }
        StatusCode::DeadlineExceeded => {
            rust::Error::deadline_exceeded(ffi_status.message().to_string_lossy())
        }
        StatusCode::NotFound => rust::Error::not_found(ffi_status.message().to_string_lossy()),
        StatusCode::AlreadyExists => {
            rust::Error::already_exists(ffi_status.message().to_string_lossy())
        }
        StatusCode::PermissionDenied => {
            rust::Error::permission_denied(ffi_status.message().to_string_lossy())
        }
        StatusCode::ResourceExhausted => {
            rust::Error::resource_exhausted(ffi_status.message().to_string_lossy())
        }
        StatusCode::FailedPrecondition => {
            rust::Error::failed_precondition(ffi_status.message().to_string_lossy())
        }
        StatusCode::Aborted => rust::Error::aborted(ffi_status.message().to_string_lossy()),
        StatusCode::OutOfRange => rust::Error::out_of_range(ffi_status.message().to_string_lossy()),
        StatusCode::Unimplemented => {
            rust::Error::unimplemented(ffi_status.message().to_string_lossy())
        }
        StatusCode::Internal => rust::Error::internal(ffi_status.message().to_string_lossy()),
        StatusCode::Unavailable => rust::Error::unavailable(ffi_status.message().to_string_lossy()),
        StatusCode::DataLoss => rust::Error::data_loss(ffi_status.message().to_string_lossy()),
        StatusCode::Unauthenticated => {
            rust::Error::unauthenticated(ffi_status.message().to_string_lossy())
        }
    }
}
