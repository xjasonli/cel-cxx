use std::borrow::Cow;
use std::pin::Pin;

use crate::Rep;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/time/time.h>);

        type Duration = super::Duration;
        fn Nanoseconds(n: i64) -> Duration;
        fn ToInt64Nanoseconds(duration: Duration) -> i64;

        type Time = super::Timestamp;
        fn ToUnixNanos(time: Time) -> i64;
        fn FromUnixNanos(nanos: i64) -> Time;

        include!(<absl/status/status.h>);
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

        include!(<absl/strings/string_view.h>);
        #[allow(unused, non_camel_case_types)]
        type string_view<'a> = super::StringView<'a>;
        #[rust_name = "len"]
        fn size(self: &string_view) -> usize;
        fn data(self: &string_view) -> *const c_char;
    }

    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/log/log_entry.h>);
        
        type LogSeverity = super::log::LogSeverity;
        type LogSeverityAtLeast = super::log::LogSeverityAtLeast;
        
        type LogEntry;
        // LogEntry accessor methods
        fn log_severity(self: &LogEntry) -> LogSeverity;
        fn text_message<'a>(self: &'a LogEntry) -> string_view<'a>;
        fn source_filename<'a>(self: &'a LogEntry) -> string_view<'a>;
        //fn source_basename<'a>(self: &'a LogEntry) -> string_view<'a>;
        fn source_line(self: &LogEntry) -> i32;

        include!(<absl/log/globals.h>);
        #[rust_name = "set_stderr_threshold"]
        fn SetStderrThreshold(severity: LogSeverityAtLeast);
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/absl.h>);

        fn Duration_new(seconds: i64, nanos: u32) -> Duration;
        fn Duration_seconds(duration: Duration) -> i64;
        fn Duration_nanos(duration: Duration) -> u32;

        fn Timestamp_new(seconds: i64, nanos: u32) -> Time;
        fn Timestamp_seconds(timestamp: Time) -> i64;
        fn Timestamp_nanos(timestamp: Time) -> u32;

        fn StringView_new<'a>(bytes: &'a [u8]) -> string_view<'a>;

        fn StatusCode_to_string(code: StatusCode) -> String;

        fn Status_new(code: StatusCode, msg: &str) -> Status;
        fn Status_clone(status: &Status) -> Status;
        fn Status_drop(status: &mut Status);
        fn Status_to_string(status: &Status) -> String;
        
        #[rust_name = "initialize_log"]
        fn InitializeLog();

        #[rust_name = "set_log_callback"]
        fn SetLogCallback();
    }

    #[namespace = "rust::cel_cxx"]
    extern "Rust" {
        #[cxx_name = "LogCallback"]
        fn log_callback(entry: &LogEntry);
    }
}

use log::log_callback;
pub mod log {
    use super::ffi;
    use std::sync::Once;

    #[allow(dead_code)]
    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub(super) enum LogSeverity {
        Info = 0,
        Warning = 1,
        Error = 2,
        Fatal = 3,
    }
    unsafe impl cxx::ExternType for LogSeverity {
        type Id = cxx::type_id!("absl::LogSeverity");
        type Kind = cxx::kind::Trivial;
    }

    #[allow(dead_code)]
    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub(super) enum LogSeverityAtLeast {
        Info = 0,
        Warning = 1,
        Error = 2,
        Fatal = 3,
        Infinity = 1000,
    }
    unsafe impl cxx::ExternType for LogSeverityAtLeast {
        type Id = cxx::type_id!("absl::LogSeverityAtLeast");
        type Kind = cxx::kind::Trivial;
    }
    
    static ONCE: Once = Once::new();
    
    /// Initialize absl log system and bridge to Rust log
    /// This function is idempotent and can be called multiple times safely
    pub fn init() {
        ONCE.call_once(|| {
            ffi::initialize_log();

            ffi::set_log_callback();

            ffi::set_stderr_threshold(LogSeverityAtLeast::Infinity);
        });
    }
    
    /// Log callback function - converts absl::LogEntry to log::Record
    pub(super) fn log_callback(entry: &ffi::LogEntry) {
        // Convert absl log severity to log::Level
        let level = match entry.log_severity() {
            LogSeverity::Info => log::Level::Info,
            LogSeverity::Warning => log::Level::Warn,
            LogSeverity::Error => log::Level::Error,
            LogSeverity::Fatal => log::Level::Error,
        };
        
        // Extract message and file information
        let message = entry.text_message().to_string_lossy();
        let file = entry.source_filename().to_string_lossy();
        let line = entry.source_line();
        
        // Create and log the record
        log::logger().log(
            &log::Record::builder()
                .args(format_args!("{message}"))
                .level(level)
                .file(Some(file.as_ref()))
                .line(if line > 0 { Some(line as u32) } else { None })
                .build()
        );
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
        ffi::Duration_new(seconds, nanos)
    }

    pub fn from_nanos(nanos: i64) -> Self {
        ffi::Nanoseconds(nanos)
    }

    pub fn to_nanos(&self) -> i64 {
        ffi::ToInt64Nanoseconds(*self)
    }

    pub fn seconds(&self) -> i64 {
        ffi::Duration_seconds(*self)
    }

    pub fn nanos(&self) -> u32 {
        ffi::Duration_nanos(*self)
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
        ffi::Timestamp_new(seconds, nanos)
    }

    pub fn from_unix_nanos(nanos: i64) -> Self {
        ffi::FromUnixNanos(nanos)
    }

    pub fn unix_nanos(&self) -> i64 {
        ffi::ToUnixNanos(*self)
    }

    pub fn seconds(&self) -> i64 {
        ffi::Timestamp_seconds(*self)
    }

    pub fn nanos(&self) -> u32 {
        ffi::Timestamp_nanos(*self)
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
        write!(f, "{string}")
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
        write!(f, "{string}")
    }
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = ffi::Status_to_string(self);
        write!(f, "{string}")
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
        ffi::AbortedError(StringView::new_str(msg))
    }

    pub fn already_exists(msg: &str) -> Self {
        ffi::AlreadyExistsError(StringView::new_str(msg))
    }

    pub fn cancelled(msg: &str) -> Self {
        ffi::CancelledError(StringView::new_str(msg))
    }

    pub fn data_loss(msg: &str) -> Self {
        ffi::DataLossError(StringView::new_str(msg))
    }

    pub fn deadline_exceeded(msg: &str) -> Self {
        ffi::DeadlineExceededError(StringView::new_str(msg))
    }

    pub fn failed_precondition(msg: &str) -> Self {
        ffi::FailedPreconditionError(StringView::new_str(msg))
    }

    pub fn internal(msg: &str) -> Self {
        ffi::InternalError(StringView::new_str(msg))
    }

    pub fn invalid_argument(msg: &str) -> Self {
        ffi::InvalidArgumentError(StringView::new_str(msg))
    }

    pub fn not_found(msg: &str) -> Self {
        ffi::NotFoundError(StringView::new_str(msg))
    }

    pub fn out_of_range(msg: &str) -> Self {
        ffi::OutOfRangeError(StringView::new_str(msg))
    }

    pub fn permission_denied(msg: &str) -> Self {
        ffi::PermissionDeniedError(StringView::new_str(msg))
    }

    pub fn resource_exhausted(msg: &str) -> Self {
        ffi::ResourceExhaustedError(StringView::new_str(msg))
    }

    pub fn unauthenticated(msg: &str) -> Self {
        ffi::UnauthenticatedError(StringView::new_str(msg))
    }

    pub fn unavailable(msg: &str) -> Self {
        ffi::UnavailableError(StringView::new_str(msg))
    }

    pub fn unimplemented(msg: &str) -> Self {
        ffi::UnimplementedError(StringView::new_str(msg))
    }

    pub fn unknown(msg: &str) -> Self {
        ffi::UnknownError(StringView::new_str(msg))
    }
}

// absl::Span<const T>
#[repr(C)]
pub struct Span<'a, T: 'a + SpanElement> {
    ptr: *const T,
    len: usize,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T: 'a + SpanElement> Copy for Span<'a, T> {}
impl<'a, T: 'a + SpanElement> Clone for Span<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
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
            unsafe { self.ptr.byte_add(offset).as_ref() }
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

impl<'a, T: 'a + SpanElement + cxx::vector::VectorElement> Span<'a, T> {
    pub fn from_vector(vector: &'a cxx::CxxVector<T>) -> Self {
        Self {
            ptr: unsafe { vector.get_unchecked(0) as *const T },
            len: vector.len(),
            _marker: std::marker::PhantomData,
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
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
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
        (
            self.span.len() - self.index,
            Some(self.span.len() - self.index),
        )
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
        f.debug_list().entries(self.iter()).finish()
    }
}

pub trait SpanElement: crate::SizedExternType {
    type TypeId;
}

// absl::Span<T>
pub struct MutSpan<'a, T: 'a + MutSpanElement> {
    ptr: *mut T,
    len: usize,
    _marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T: 'a + MutSpanElement> Copy for MutSpan<'a, T> {}
impl<'a, T: 'a + MutSpanElement> Clone for MutSpan<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe impl<'a, T: 'a + MutSpanElement> cxx::ExternType for MutSpan<'a, T> {
    type Id = T::TypeId;
    type Kind = cxx::kind::Trivial;
}

impl<'a, T: 'a + MutSpanElement> MutSpan<'a, T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<Pin<&'a mut T>> {
        if index < self.len() {
            unsafe {
                self.ptr.byte_add(index * T::size_of())
                    .as_mut()
                    .map(|ptr| Pin::new_unchecked(ptr))
            }
        } else {
            None
        }
    }

    pub fn iter(&self) -> MutSpanIter<'a, T> {
        MutSpanIter {
            span: *self,
            index: 0,
        }
    }
}

impl<'a, T: 'a + MutSpanElement + cxx::vector::VectorElement> MutSpan<'a, T> {
    pub fn from_vector(mut vector: Pin<&'a mut cxx::CxxVector<T>>) -> Self {
        Self {
            ptr: unsafe { vector.as_mut().index_unchecked_mut(0).get_unchecked_mut() as *mut T },
            len: vector.len(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T: 'a + MutSpanElement + cxx::ExternType<Kind = cxx::kind::Trivial>> MutSpan<'a, T> {
    pub fn from_slice(slice: &'a mut [T]) -> Self {
        Self {
            ptr: slice.as_mut_ptr(),
            len: slice.len(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn as_slice(&self) -> &'a mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

pub struct MutSpanIter<'a, T: 'a + MutSpanElement> {
    span: MutSpan<'a, T>,
    index: usize,
}

impl<'a, T: 'a + MutSpanElement> std::iter::Iterator for MutSpanIter<'a, T> {
    type Item = Pin<&'a mut T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.span.get(self.index) {
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.span.len() - self.index,
            Some(self.span.len() - self.index),
        )
    }
}
impl<'a, T: 'a + MutSpanElement> std::iter::FusedIterator for MutSpanIter<'a, T> {}
impl<'a, T: 'a + MutSpanElement> std::iter::ExactSizeIterator for MutSpanIter<'a, T> {}
impl<'a, T: 'a + MutSpanElement> std::iter::IntoIterator for &MutSpan<'a, T> {
    type Item = Pin<&'a mut T>;
    type IntoIter = MutSpanIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        MutSpanIter { span: *self, index: 0 }
    }
}

impl<'a, T: 'a + MutSpanElement + std::fmt::Debug> std::fmt::Debug for MutSpan<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub trait MutSpanElement: crate::SizedExternType {
    type TypeId;
}

// absl::string_view
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StringView<'a>(Rep<'a, usize, 2>);

unsafe impl<'a> cxx::ExternType for StringView<'a> {
    type Id = cxx::type_id!("absl::string_view");
    type Kind = cxx::kind::Trivial;
}

impl<'a> crate::SizedExternType for StringView<'a> {}

impl<'a> SpanElement for StringView<'a> {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_StringView");
}

impl<'a> StringView<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        ffi::StringView_new(bytes)
    }

    pub fn new_str(s: &'a str) -> Self {
        Self::new(s.as_bytes())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    pub fn to_str(&self) -> Result<&'a str, std::str::Utf8Error> {
        std::str::from_utf8(self.as_bytes())
    }

    pub fn to_string_lossy(&self) -> Cow<'a, str> {
        String::from_utf8_lossy(self.as_bytes())
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data() as *const u8
    }
}

impl<'a> From<&'a str> for StringView<'a> {
    fn from(value: &'a str) -> Self {
        Self::new_str(value)
    }
}

impl<'a> From<&'a String> for StringView<'a> {
    fn from(value: &'a String) -> Self {
        Self::new_str(value.as_str())
    }
}

impl<'a> From<&'a [u8]> for StringView<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::new(value)
    }
}

impl<'a> From<&'a Vec<u8>> for StringView<'a> {
    fn from(value: &'a Vec<u8>) -> Self {
        Self::new(value.as_slice())
    }
}
