#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    missing_docs,
    unreachable_pub
)]

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use cel_cxx_macros::*;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod r#async;

#[cfg(feature = "async-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-std")))]
pub use r#async::AsyncStd;

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
pub use r#async::Tokio;

#[cfg(feature = "smol")]
#[cfg_attr(docsrs, doc(cfg(feature = "smol")))]
pub use r#async::Smol;

/// Marker types and traits for function and runtime polymorphism.
pub mod marker;
pub use marker::*;

/// Environment for compiling CEL expressions.
pub mod env;
pub use env::*;

/// Compiled CEL programs ready for evaluation.
pub mod program;
pub use program::*;

mod ffi;

/// Error types and error handling utilities.
pub mod error;
pub use error::*;

/// Function registration and declaration utilities.
pub mod function;
pub use function::*;

/// CEL macro system for compile-time expression expansion.
pub mod macros;
pub use macros::*;

/// Variable declarations for CEL environments.
pub mod variable;
pub use variable::*;

pub mod activation;
pub use activation::*;

pub mod kind;
pub use kind::*;

pub mod types;
pub use types::*;

pub mod values;
pub use values::*;

pub mod maybe_future;
pub use maybe_future::*;
