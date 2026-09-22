//! Asynchronous runtime support for CEL.
//!
//! This module provides support for asynchronous evaluation of CEL expressions
//! using the [tokio](https://github.com/tokio-rs/tokio) or
//! [smol](https://github.com/smol-rs/smol) runtimes.
//!
//! # Features
//! - `tokio`: Enables asynchronous evaluation of CEL expressions using
//!   [tokio](https://github.com/tokio-rs/tokio).
//! - `smol`: Enables asynchronous evaluation of CEL expressions using
//!   [smol](https://github.com/smol-rs/smol).
//!

use async_scoped::spawner::{Blocker, FuncSpawner, Spawner};
use futures::Future;

pub(crate) mod abort;

/// Runtime trait for CEL asynchronous runtime.
pub trait Runtime: 'static {
    /// Scoped spawner for CEL asynchronous runtime.
    type ScopedSpawner: Spawner<()>
        + FuncSpawner<(), SpawnHandle = <Self::ScopedSpawner as Spawner<()>>::SpawnHandle>
        + Blocker
        + Default
        + Send
        + Sync
        + 'static;

    /// Blocking runner for CEL asynchronous runtime.
    type BlockingRunner: BlockingRunner;
}

/// Blocking runner trait for CEL asynchronous runtime.
pub trait BlockingRunner: 'static {
    /// Block on a future.
    fn block_on<F: Future>(fut: F) -> F::Output;
}

/// Tokio runtime for CEL asynchronous runtime.
#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
#[allow(missing_debug_implementations)]
pub enum Tokio {}

/// Tokio runtime implementation.
#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
pub mod tokio {
    #![allow(missing_debug_implementations)]
    use super::*;

    impl Runtime for Tokio {
        type ScopedSpawner = async_scoped::spawner::use_tokio::Tokio;
        type BlockingRunner = TokioBlockingRunner;
    }

    /// Tokio blocking runner for CEL asynchronous runtime.
    pub struct TokioBlockingRunner;
    impl BlockingRunner for TokioBlockingRunner {
        fn block_on<F: Future>(fut: F) -> F::Output {
            ::tokio::runtime::Handle::current().block_on(fut)
        }
    }
}

/// Smol runtime for CEL asynchronous runtime.
#[cfg(feature = "smol")]
#[cfg_attr(docsrs, doc(cfg(feature = "smol")))]
#[allow(missing_debug_implementations)]
pub enum Smol {}

/// Smol runtime for CEL asynchronous runtime.
#[cfg(feature = "smol")]
#[cfg_attr(docsrs, doc(cfg(feature = "smol")))]
pub mod smol {
    #![allow(missing_debug_implementations)]
    use super::*;

    impl Runtime for Smol {
        type ScopedSpawner = SmolScopedSpawner;
        type BlockingRunner = SmolBlockingRunner;
    }

    /// Smol scoped spawner for CEL asynchronous runtime.
    #[derive(Default)]
    #[allow(missing_debug_implementations)]
    pub struct SmolScopedSpawner;
    unsafe impl<T: Send + 'static> Spawner<T> for SmolScopedSpawner {
        type FutureOutput = T;
        type SpawnHandle = ::smol::Task<T>;

        fn spawn<F: Future<Output = T> + Send + 'static>(&self, f: F) -> Self::SpawnHandle {
            ::smol::spawn(f)
        }
    }
    unsafe impl<T: Send + 'static> FuncSpawner<T> for SmolScopedSpawner {
        type FutureOutput = T;
        type SpawnHandle = ::smol::Task<T>;

        fn spawn_func<F: FnOnce() -> T + Send + 'static>(&self, f: F) -> Self::SpawnHandle {
            ::smol::unblock(f)
        }
    }
    unsafe impl Blocker for SmolScopedSpawner {
        fn block_on<T, F: Future<Output = T>>(&self, f: F) -> T {
            ::smol::block_on(f)
        }
    }

    /// Smol blocking runner for CEL asynchronous runtime.
    pub struct SmolBlockingRunner;
    impl BlockingRunner for SmolBlockingRunner {
        fn block_on<F: Future>(fut: F) -> F::Output {
            ::smol::block_on(fut)
        }
    }
}
