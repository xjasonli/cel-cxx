/// Error type for CEL operations.
///
/// This is the main error type used throughout the cel-cxx library. It represents
/// an error condition in CEL expression compilation or evaluation, modeled after
/// the `absl::Status` type from the Google Abseil library.
///
/// # Examples
///
/// ## Creating Errors
///
/// ```rust
/// use cel_cxx::{Error, Code};
///
/// let error = Error::invalid_argument("Expression syntax is invalid");
/// assert_eq!(error.code(), Code::InvalidArgument);
/// ```
///
/// ## Handling Errors
///
/// ```rust,no_run
/// use cel_cxx::*;
///
/// match Env::builder().build()?.compile("invalid expression!!") {
///     Ok(program) => println!("Compiled successfully"),
///     Err(e) => {
///         eprintln!("Error: {}", e);
///         eprintln!("Error code: {:?}", e.code());
///     }
/// }
/// # Ok::<(), cel_cxx::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    /// The error code.
    code: Code,

    /// The error message.
    message: String,
}

/// Status codes for CEL errors.
///
/// These codes are based on the Google RPC status codes and provide
/// standardized error categorization for different types of failures
/// that can occur during CEL operations.
///
/// # Examples
///
/// ```rust
/// use cel_cxx::Code;
///
/// let code = Code::InvalidArgument;
/// println!("Description: {}", code.description());
/// println!("Display: {}", code);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Code {
    /// The operation completed successfully.
    ///
    /// This is not typically used for errors, but may appear in some contexts.
    Ok = 0,

    /// The operation was cancelled.
    ///
    /// This typically indicates that the operation was cancelled by the caller
    /// or due to a timeout.
    Cancelled = 1,

    /// Unknown error.
    ///
    /// This is used when the specific error category cannot be determined,
    /// often when converting from other error types.
    Unknown = 2,

    /// Client specified an invalid argument.
    ///
    /// This indicates that the input provided to the operation was invalid.
    /// For CEL, this often means syntax errors or invalid expression structure.
    InvalidArgument = 3,

    /// Deadline expired before operation could complete.
    ///
    /// This indicates that the operation took too long to complete.
    DeadlineExceeded = 4,

    /// Some requested entity was not found.
    ///
    /// This indicates that a referenced entity (like a variable or function)
    /// was not found in the current context.
    NotFound = 5,

    /// Some entity that we attempted to create already exists.
    ///
    /// This indicates that an operation tried to create something that already exists.
    AlreadyExists = 6,

    /// The caller does not have permission to execute the specified operation.
    ///
    /// This indicates insufficient permissions to perform the operation.
    PermissionDenied = 7,

    /// Some resource has been exhausted.
    ///
    /// This indicates that a resource limit has been exceeded, such as memory
    /// or computation limits.
    ResourceExhausted = 8,

    /// The system is not in a state required for the operation's execution.
    ///
    /// This indicates that the operation cannot be performed in the current state.
    FailedPrecondition = 9,

    /// The operation was aborted.
    ///
    /// This indicates that the operation was aborted, typically due to a
    /// concurrency issue.
    Aborted = 10,

    /// Operation was attempted past the valid range.
    ///
    /// This indicates that the operation exceeded valid bounds, such as
    /// accessing an array out of bounds.
    OutOfRange = 11,

    /// Operation is not implemented or not supported.
    ///
    /// This indicates that the requested operation is not implemented or
    /// not supported in the current context.
    Unimplemented = 12,

    /// Internal error.
    ///
    /// This indicates an internal error that should not normally occur.
    /// It often indicates a bug in the implementation.
    Internal = 13,

    /// The service is currently unavailable.
    ///
    /// This indicates that the service is temporarily unavailable and the
    /// operation should be retried later.
    Unavailable = 14,

    /// Unrecoverable data loss or corruption.
    ///
    /// This indicates that data has been lost or corrupted in an unrecoverable way.
    DataLoss = 15,

    /// The request does not have valid authentication credentials.
    ///
    /// This indicates that the operation requires authentication credentials
    /// that are missing or invalid.
    Unauthenticated = 16,
}

impl Code {
    /// Returns a human-readable description of this error code.
    ///
    /// This method provides a detailed description of what the error code means.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Code;
    ///
    /// let code = Code::InvalidArgument;
    /// println!("Description: {}", code.description());
    /// // Output: Description: Client specified an invalid argument
    /// ```
    ///
    /// # Note
    ///
    /// If you only need the description for display purposes (such as in
    /// `println!`, `format!`, or logging), you can use the `Display` trait
    /// implementation instead, which is more concise.
    pub fn description(&self) -> &'static str {
        match self {
            Code::Ok => "The operation completed successfully",
            Code::Cancelled => "The operation was cancelled",
            Code::Unknown => "Unknown error",
            Code::InvalidArgument => "Client specified an invalid argument",
            Code::DeadlineExceeded => "Deadline expired before operation could complete",
            Code::NotFound => "Some requested entity was not found",
            Code::AlreadyExists => "Some entity that we attempted to create already exists",
            Code::PermissionDenied => {
                "The caller does not have permission to execute the specified operation"
            }
            Code::ResourceExhausted => "Some resource has been exhausted",
            Code::FailedPrecondition => {
                "The system is not in a state required for the operation's execution"
            }
            Code::Aborted => "The operation was aborted",
            Code::OutOfRange => "Operation was attempted past the valid range",
            Code::Unimplemented => "Operation is not implemented or not supported",
            Code::Internal => "Internal error",
            Code::Unavailable => "The service is currently unavailable",
            Code::DataLoss => "Unrecoverable data loss or corruption",
            Code::Unauthenticated => "The request does not have valid authentication credentials",
        }
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.description(), f)
    }
}

// ===== impl Error =====

impl Error {
    /// Creates a new error with the specified code and message.
    ///
    /// This is the fundamental constructor for creating errors. Most users
    /// should prefer the specific constructor methods like [`invalid_argument`],
    /// [`not_found`], etc., which are more descriptive.
    ///
    /// # Arguments
    ///
    /// * `code` - The error code indicating the type of error
    /// * `message` - A descriptive message explaining the error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::{Error, Code};
    ///
    /// let error = Error::new(Code::InvalidArgument, "Invalid input provided");
    /// assert_eq!(error.code(), Code::InvalidArgument);
    /// assert_eq!(error.message(), "Invalid input provided");
    /// ```
    ///
    /// [`invalid_argument`]: Self::invalid_argument
    /// [`not_found`]: Self::not_found
    pub fn new(code: Code, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Creates an "ok" status with the given message.
    ///
    /// This is rarely used for actual errors, but may be useful in some
    /// contexts where a status needs to indicate success with additional information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let status = Error::ok("Operation completed successfully");
    /// ```
    pub fn ok(message: impl Into<String>) -> Self {
        Self::new(Code::Ok, message)
    }

    /// Creates a "cancelled" error with the given message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::cancelled("Operation was cancelled by user");
    /// ```
    pub fn cancelled(message: impl Into<String>) -> Self {
        Self::new(Code::Cancelled, message)
    }

    /// Creates an "unknown" error with the given message.
    ///
    /// This is useful when converting from other error types where the specific
    /// category cannot be determined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::unknown("An unexpected error occurred");
    /// ```
    pub fn unknown(message: impl Into<String>) -> Self {
        Self::new(Code::Unknown, message)
    }

    /// Creates an "invalid argument" error with the given message.
    ///
    /// This is commonly used for CEL compilation errors due to invalid syntax
    /// or malformed expressions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::invalid_argument("Expression contains invalid syntax");
    /// ```
    pub fn invalid_argument(message: impl Into<String>) -> Self {
        Self::new(Code::InvalidArgument, message)
    }

    /// Creates a "deadline exceeded" error with the given message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::deadline_exceeded("Operation took too long to complete");
    /// ```
    pub fn deadline_exceeded(message: impl Into<String>) -> Self {
        Self::new(Code::DeadlineExceeded, message)
    }

    /// Creates a "not found" error with the given message.
    ///
    /// This is commonly used when a referenced variable or function is not
    /// found in the current environment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::not_found("Variable 'x' is not declared");
    /// ```
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(Code::NotFound, message)
    }

    /// Creates an "already exists" error with the given message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::already_exists("Function 'myFunc' is already registered");
    /// ```
    pub fn already_exists(message: impl Into<String>) -> Self {
        Self::new(Code::AlreadyExists, message)
    }

    /// Creates a "permission denied" error with the given message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::permission_denied("Access to this resource is denied");
    /// ```
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::new(Code::PermissionDenied, message)
    }

    /// Creates a "resource exhausted" error with the given message.
    ///
    /// This can be used when evaluation hits resource limits such as memory
    /// or computation time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::resource_exhausted("Memory limit exceeded during evaluation");
    /// ```
    pub fn resource_exhausted(message: impl Into<String>) -> Self {
        Self::new(Code::ResourceExhausted, message)
    }

    /// Creates a "failed precondition" error with the given message.
    ///
    /// This indicates that the operation cannot be performed because the system
    /// is not in the required state. See the documentation for [`Code::FailedPrecondition`]
    /// for guidance on when to use this vs. other error codes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::failed_precondition("Environment must be built before compilation");
    /// ```
    pub fn failed_precondition(message: impl Into<String>) -> Self {
        Self::new(Code::FailedPrecondition, message)
    }

    /// Creates an "aborted" error with the given message.
    ///
    /// This indicates that the operation was aborted, typically due to a
    /// concurrency issue or transaction conflict.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::aborted("Transaction was aborted due to conflict");
    /// ```
    pub fn aborted(message: impl Into<String>) -> Self {
        Self::new(Code::Aborted, message)
    }

    /// Creates an "out of range" error with the given message.
    ///
    /// This indicates that the operation attempted to access something outside
    /// of valid bounds, such as array indices or valid value ranges.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::out_of_range("Index 10 is out of range for array of length 5");
    /// ```
    pub fn out_of_range(message: impl Into<String>) -> Self {
        Self::new(Code::OutOfRange, message)
    }

    /// Creates an "unimplemented" error with the given message.
    ///
    /// This indicates that the requested operation is not implemented or
    /// not supported in the current context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::unimplemented("This feature is not yet implemented");
    /// ```
    pub fn unimplemented(message: impl Into<String>) -> Self {
        Self::new(Code::Unimplemented, message)
    }

    /// Creates an "internal" error with the given message.
    ///
    /// This indicates an internal error that should not normally occur.
    /// It typically indicates a bug in the implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::internal("Internal invariant violated");
    /// ```
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(Code::Internal, message)
    }

    /// Creates an "unavailable" error with the given message.
    ///
    /// This indicates that the service is temporarily unavailable and the
    /// operation should be retried later.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::unavailable("Service is temporarily unavailable");
    /// ```
    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::new(Code::Unavailable, message)
    }

    /// Creates a "data loss" error with the given message.
    ///
    /// This indicates that data has been lost or corrupted in an unrecoverable way.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::data_loss("Critical data has been corrupted");
    /// ```
    pub fn data_loss(message: impl Into<String>) -> Self {
        Self::new(Code::DataLoss, message)
    }

    /// Creates an "unauthenticated" error with the given message.
    ///
    /// This indicates that the operation requires authentication credentials
    /// that are missing or invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::unauthenticated("Valid authentication credentials required");
    /// ```
    pub fn unauthenticated(message: impl Into<String>) -> Self {
        Self::new(Code::Unauthenticated, message)
    }

    /// Returns the error code for this error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::{Error, Code};
    ///
    /// let error = Error::invalid_argument("Bad input");
    /// assert_eq!(error.code(), Code::InvalidArgument);
    /// ```
    pub fn code(&self) -> Code {
        self.code
    }

    /// Returns the error message for this error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    ///
    /// let error = Error::invalid_argument("Bad input");
    /// assert_eq!(error.message(), "Bad input");
    /// ```
    pub fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn from_std_error_generic(
        err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self {
        Self::from_std_error(err.into())
    }

    /// Creates a CEL error from a standard library error.
    ///
    /// This method attempts to extract meaningful error information from
    /// standard library errors and convert them to CEL errors. It inspects
    /// the error source chain for recognizable error types.
    ///
    /// # Arguments
    ///
    /// * `err` - The standard library error to convert
    ///
    /// # Returns
    ///
    /// A CEL `Error` with an appropriate error code and message.
    /// If the error type is not recognized, it will be mapped to `Code::Unknown`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::Error;
    /// use std::io;
    ///
    /// let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    /// let cel_error = Error::from_std_error(Box::new(io_error));
    /// ```
    pub fn from_std_error(err: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self::try_from_std_error(err)
            .unwrap_or_else(|err| Self::new(Code::Unknown, err.to_string()))
    }

    /// Attempts to create a CEL error from a standard library error.
    ///
    /// This method is similar to [`from_std_error`] but returns the original
    /// error if it cannot be converted to a CEL error.
    ///
    /// # Arguments
    ///
    /// * `err` - The standard library error to convert
    ///
    /// # Returns
    ///
    /// `Ok(Error)` if the conversion was successful, or `Err(original_error)`
    /// if the error type could not be recognized.
    ///
    /// # Downcast Stability
    ///
    /// This function does not provide any stability guarantees around how it
    /// will downcast errors into status codes. The conversion logic may change
    /// in future versions.
    ///
    /// [`from_std_error`]: Self::from_std_error
    pub fn try_from_std_error(
        err: Box<dyn std::error::Error + Send + Sync + 'static>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let err = match err.downcast::<Error>() {
            Ok(error) => return Ok(*error),
            Err(err) => err,
        };

        if let Some(error) = Self::find_in_source_chain(&*err) {
            return Ok(error);
        }
        Err(err)
    }

    fn find_in_source_chain(err: &(dyn std::error::Error + 'static)) -> Option<Self> {
        use crate::types::InvalidMapKeyType;

        let mut source = Some(err);

        while let Some(err) = source {
            if let Some(error) = err.downcast_ref::<Error>() {
                return Some((*error).clone());
            }

            if let Some(invalid_mapkey_type) = err.downcast_ref::<InvalidMapKeyType>() {
                return Some(Self::invalid_argument(invalid_mapkey_type.0.to_string()));
            }

            // TODO: Add more error types here

            source = err.source();
        }

        None
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {:?}, message: {:?}", self.code(), self.message(),)
    }
}

impl std::error::Error for Error {}

/// Trait for converting values into CEL errors.
///
/// This trait provides a standardized way to convert different error types
/// into CEL [`Error`] instances. It is automatically implemented for all
/// types that implement the standard library's `Error` trait.
///
/// # Examples
///
/// ```rust
/// use cel_cxx::{Error, IntoError};
/// use std::io;
///
/// let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
/// let cel_error = io_error.into_error();
/// ```
///
/// # Automatic Implementation
///
/// This trait is automatically implemented for any type that implements
/// `std::error::Error + Send + Sync + 'static`, so you typically don't
/// need to implement it manually.
pub trait IntoError {
    /// Converts this value into a CEL error.
    ///
    /// # Returns
    ///
    /// A CEL [`Error`] representing this error condition.
    fn into_error(self) -> Error;
}

impl<E: std::error::Error + Send + Sync + 'static> IntoError for E {
    fn into_error(self) -> Error {
        Error::from_std_error_generic(Box::new(self))
    }
}
