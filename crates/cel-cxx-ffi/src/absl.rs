use std::borrow::Cow;

use crate::Rep;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!("absl/time/time.h");

        type Duration = super::Duration;
        fn Nanoseconds(n: i64) -> Duration;
        fn ToInt64Nanoseconds(duration: Duration) -> i64;

        type Time = super::Timestamp;
        fn ToUnixNanos(time: Time) -> i64;
        fn FromUnixNanos(nanos: i64) -> Time;

        include!("absl/status/status.h");
        type StatusCode = super::StatusCode;

        type Status = super::Status;
        #[rust_name = "is_ok"]
        fn ok(self: &Status) -> bool;
        fn code(self: &Status) -> StatusCode;
        fn message<'a>(self: &'a Status) -> string_view<'a>;

        fn OkStatus() -> Status;
        fn IsAborted(status: &Status) -> bool;
        fn IsAlreadyExists(status: &Status) -> bool;
        fn IsCancelled(status: &Status) -> bool;
        fn IsDataLoss(status: &Status) -> bool;
        fn IsDeadlineExceeded(status: &Status) -> bool;
        fn IsFailedPrecondition(status: &Status) -> bool;
        fn IsInternal(status: &Status) -> bool;
        fn IsInvalidArgument(status: &Status) -> bool;
        fn IsNotFound(status: &Status) -> bool;
        fn IsOutOfRange(status: &Status) -> bool;
        fn IsPermissionDenied(status: &Status) -> bool;
        fn IsResourceExhausted(status: &Status) -> bool;
        fn IsUnauthenticated(status: &Status) -> bool;
        fn IsUnavailable(status: &Status) -> bool;
        fn IsUnimplemented(status: &Status) -> bool;
        fn IsUnknown(status: &Status) -> bool;

        fn AbortedError(msg: string_view) -> Status;
        fn AlreadyExistsError(msg: string_view) -> Status;
        fn CancelledError(msg: string_view) -> Status;
        fn DataLossError(msg: string_view) -> Status;
        fn DeadlineExceededError(msg: string_view) -> Status;
        fn FailedPreconditionError(msg: string_view) -> Status;
        fn InternalError(msg: string_view) -> Status;
        fn InvalidArgumentError(msg: string_view) -> Status;
        fn NotFoundError(msg: string_view) -> Status;
        fn OutOfRangeError(msg: string_view) -> Status;
        fn PermissionDeniedError(msg: string_view) -> Status;
        fn ResourceExhaustedError(msg: string_view) -> Status;
        fn UnauthenticatedError(msg: string_view) -> Status;
        fn UnavailableError(msg: string_view) -> Status;
        fn UnimplementedError(msg: string_view) -> Status;
        fn UnknownError(msg: string_view) -> Status;

        include!("absl/strings/string_view.h");
        #[allow(unused, non_camel_case_types)]
        type string_view<'a> = super::StringView<'a>;

    }

    #[namespace = "absl::time_internal"]
    unsafe extern "C++" {
        fn FromUnixDuration(duration: Duration) -> Time;
        fn ToUnixDuration(time: Time) -> Duration;
        fn GetRepHi(duration: Duration) -> i64;
        fn GetRepLo(duration: Duration) -> u32;
        fn MakeDuration(hi: i64, lo: u32) -> Duration;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!("cel-cxx-ffi/include/absl.h");

        fn StatusCode_to_string(code: StatusCode) -> String;

        fn Status_new(code: StatusCode, msg: &str) -> Status;
        fn Status_clone(status: &Status) -> Status;
        fn Status_drop(status: &mut Status);
        fn Status_to_string(status: &Status) -> String;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Duration(Rep<'static, u32, 3>);

unsafe impl cxx::ExternType for Duration {
    type Id = cxx::type_id!("absl::Duration");
    type Kind = cxx::kind::Trivial;
}

impl Duration {
    pub fn new(seconds: i64, nanos: u32) -> Self {
        ffi::MakeDuration(seconds, nanos)
    }

    pub fn from_nanos(nanos: i64) -> Self {
        ffi::Nanoseconds(nanos)
    }

    pub fn to_nanos(&self) -> i64 {
        ffi::ToInt64Nanoseconds(*self)
    }

    pub fn seconds(&self) -> i64 {
        ffi::GetRepHi(*self)
    }

    pub fn nanos(&self) -> u32 {
        ffi::GetRepLo(*self)
    }
}

impl From<chrono::Duration> for Duration {
    fn from(value: chrono::Duration) -> Self {
        Self::new(value.num_seconds(), value.subsec_nanos() as u32)
    }
}

impl From<Duration> for chrono::Duration {
    fn from(value: Duration) -> Self {
        Self::new(value.seconds(), value.nanos()).unwrap_or_default()
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Timestamp(Rep<'static, u32, 3>);

unsafe impl cxx::ExternType for Timestamp {
    type Id = cxx::type_id!("absl::Time");
    type Kind = cxx::kind::Trivial;
}

impl Timestamp {
    pub fn new(seconds: i64, nanos: u32) -> Self {
        ffi::FromUnixDuration(Duration::new(seconds, nanos))
    }

    pub fn from_unix_nanos(nanos: i64) -> Self {
        ffi::FromUnixNanos(nanos)
    }

    pub fn unix_nanos(&self) -> i64 {
        ffi::ToUnixNanos(*self)
    }

    pub fn seconds(&self) -> i64 {
        ffi::GetRepHi(ffi::ToUnixDuration(*self))
    }

    pub fn nanos(&self) -> u32 {
        ffi::GetRepLo(ffi::ToUnixDuration(*self))
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self::new(value.timestamp(), value.timestamp_subsec_nanos())
    }
}

impl From<Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(value: Timestamp) -> Self {
        Self::from_timestamp(value.seconds(), value.nanos()).unwrap_or_default()
    }
}



// absl::StatusCode
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum StatusCode {
    Ok = 0,
    Cancelled = 1,
    Unknown = 2,
    InvalidArgument = 3,
    DeadlineExceeded = 4,
    NotFound = 5,
    AlreadyExists = 6,
    PermissionDenied = 7,
    ResourceExhausted = 8,
    FailedPrecondition = 9,
    Aborted = 10,
    OutOfRange = 11,
    Unimplemented = 12,
    Internal = 13,
    Unavailable = 14,
    DataLoss = 15,
    Unauthenticated = 16,
}

unsafe impl cxx::ExternType for StatusCode {
    type Id = cxx::type_id!("absl::StatusCode");
    type Kind = cxx::kind::Trivial;
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = ffi::StatusCode_to_string(*self);
        write!(f, "{}", string)
    }
}

// absl::Status
#[repr(transparent)]
pub struct Status(Rep<'static, usize, 1>);

unsafe impl cxx::ExternType for Status {
    type Id = cxx::type_id!("absl::Status");
    type Kind = cxx::kind::Trivial;
}

impl Clone for Status {
    fn clone(&self) -> Self {
        ffi::Status_clone(self)
    }
}

impl Drop for Status {
    fn drop(&mut self) {
        ffi::Status_drop(self)
    }
}

impl Default for Status {
    fn default() -> Self {
        ffi::OkStatus()
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = ffi::Status_to_string(self);
        write!(f, "{}", string)
    }
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = ffi::Status_to_string(self);
        write!(f, "{}", string)
    }
}

impl std::error::Error for Status {}

impl Status {
    pub fn new(code: StatusCode, message: &str) -> Self {
        ffi::Status_new(code, message)
    }

    pub fn ok() -> Self {
        ffi::OkStatus()
    }

    pub fn is_aborted(&self) -> bool {
        ffi::IsAborted(self)
    }

    pub fn is_already_exists(&self) -> bool {
        ffi::IsAlreadyExists(self)
    }
    
    pub fn is_cancelled(&self) -> bool {
        ffi::IsCancelled(self)
    }

    pub fn is_data_loss(&self) -> bool {
        ffi::IsDataLoss(self)
    }
    
    pub fn is_deadline_exceeded(&self) -> bool {
        ffi::IsDeadlineExceeded(self)
    }

    pub fn is_failed_precondition(&self) -> bool {
        ffi::IsFailedPrecondition(self)
    }
    
    pub fn is_internal(&self) -> bool {
        ffi::IsInternal(self)
    }

    pub fn is_invalid_argument(&self) -> bool {
        ffi::IsInvalidArgument(self)
    }
    
    pub fn is_not_found(&self) -> bool {
        ffi::IsNotFound(self)
    }

    pub fn is_out_of_range(&self) -> bool {
        ffi::IsOutOfRange(self)
    }
    
    pub fn is_permission_denied(&self) -> bool {
        ffi::IsPermissionDenied(self)
    }
    
    pub fn is_resource_exhausted(&self) -> bool {
        ffi::IsResourceExhausted(self)
    }
    
    pub fn is_unauthenticated(&self) -> bool {
        ffi::IsUnauthenticated(self)
    }
    
    pub fn is_unavailable(&self) -> bool {
        ffi::IsUnavailable(self)
    }

    pub fn is_unimplemented(&self) -> bool {
        ffi::IsUnimplemented(self)
    }

    pub fn is_unknown(&self) -> bool {
        ffi::IsUnknown(self)
    }

    pub fn aborted(msg: &str) -> Self {
        ffi::AbortedError(StringView::from_str(msg))
    }

    pub fn already_exists(msg: &str) -> Self {
        ffi::AlreadyExistsError(StringView::from_str(msg))
    }
    
    pub fn cancelled(msg: &str) -> Self {
        ffi::CancelledError(StringView::from_str(msg))
    }

    pub fn data_loss(msg: &str) -> Self {
        ffi::DataLossError(StringView::from_str(msg))
    }
    
    pub fn deadline_exceeded(msg: &str) -> Self {
        ffi::DeadlineExceededError(StringView::from_str(msg))
    }

    pub fn failed_precondition(msg: &str) -> Self {
        ffi::FailedPreconditionError(StringView::from_str(msg))
    }
    
    pub fn internal(msg: &str) -> Self {
        ffi::InternalError(StringView::from_str(msg))
    }

    pub fn invalid_argument(msg: &str) -> Self {
        ffi::InvalidArgumentError(StringView::from_str(msg))
    }
    
    pub fn not_found(msg: &str) -> Self {
        ffi::NotFoundError(StringView::from_str(msg))
    }

    pub fn out_of_range(msg: &str) -> Self {
        ffi::OutOfRangeError(StringView::from_str(msg))
    }
    
    pub fn permission_denied(msg: &str) -> Self {
        ffi::PermissionDeniedError(StringView::from_str(msg))
    }
    
    pub fn resource_exhausted(msg: &str) -> Self {
        ffi::ResourceExhaustedError(StringView::from_str(msg))
    }

    pub fn unauthenticated(msg: &str) -> Self {
        ffi::UnauthenticatedError(StringView::from_str(msg))
    }

    pub fn unavailable(msg: &str) -> Self {
        ffi::UnavailableError(StringView::from_str(msg))
    }
    
    pub fn unimplemented(msg: &str) -> Self {
        ffi::UnimplementedError(StringView::from_str(msg))
    }

    pub fn unknown(msg: &str) -> Self {
        ffi::UnknownError(StringView::from_str(msg))
    }
}


// absl::Span
#[repr(C)]
pub struct Span<'a, T: 'a + SpanElement> {
    ptr: *const T,
    len: usize,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T: 'a + SpanElement> Copy for Span<'a, T> {}
impl<'a, T: 'a + SpanElement> Clone for Span<'a, T> {
    fn clone(&self) -> Self { *self }
}

unsafe impl<'a, T: 'a + SpanElement> cxx::ExternType for Span<'a, T> {
    type Id = T::TypeId;
    type Kind = cxx::kind::Trivial;
}

impl<'a, T: 'a + SpanElement> Span<'a, T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&'a T> {
        if index < self.len() {
            let offset = index * T::size_of();
            unsafe {
                self.ptr.byte_add(offset)
                    .as_ref()
            }
        } else {
            None
        }
    }

    pub fn iter(&self) -> SpanIter<'a, T> {
        SpanIter {
            span: *self,
            index: 0,
        }
    }
}

impl<'a, T: 'a + SpanElement + cxx::ExternType<Kind = cxx::kind::Trivial>> Span<'a, T> {
    pub fn from_slice(slice: &'a [T]) -> Self {
        Self {
            ptr: slice.as_ptr(),
            len: slice.len(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn as_slice(&self) -> &'a [T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len)}
    }
}

pub struct SpanIter<'a, T: 'a + SpanElement> {
    span: Span<'a, T>,
    index: usize,
}

impl<'a, T: 'a + SpanElement> std::iter::Iterator for SpanIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.span.get(self.index) {
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.span.len() - self.index, Some(self.span.len() - self.index))
    }
}

impl<'a, T: 'a + SpanElement> std::iter::FusedIterator for SpanIter<'a, T> {}

impl<'a, T: 'a + SpanElement> std::iter::ExactSizeIterator for SpanIter<'a, T> {}

impl<'a, T: 'a + SpanElement> std::iter::IntoIterator for Span<'a, T> {
    type Item = &'a T;
    type IntoIter = SpanIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SpanIter {
            span: self,
            index: 0,
        }
    }
}

impl<'a, T: 'a + SpanElement + std::fmt::Debug> std::fmt::Debug for Span<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.iter())
            .finish()
    }
}

pub trait SpanElement: crate::SizedExternType {
    type TypeId;
}


// absl::string_view
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StringView<'a> {
    len: usize,
    ptr: *const u8,
    _marker: std::marker::PhantomData<&'a u8>,
}

unsafe impl<'a> cxx::ExternType for StringView<'a> {
    type Id = cxx::type_id!("absl::string_view");
    type Kind = cxx::kind::Trivial;
}

impl<'a> crate::SizedExternType for StringView<'a> {}

impl<'a> SpanElement for StringView<'a> {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_StringView");
}

impl<'a> StringView<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            len: bytes.len(),
            ptr: bytes.as_ptr(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn from_str(s: &'a str) -> Self {
        Self::from_bytes(s.as_bytes())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }

    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.as_bytes())
    }

    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.as_bytes())
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }
}
