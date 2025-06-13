#![allow(
    unreachable_pub,
    missing_docs,
    unused,
)]

pub(crate) use imp::*;

#[cfg(not(feature = "async"))]
mod imp {
    pub type MaybeFuture<'a, T, E = crate::Error> = Result<T, E>;
}


#[cfg(feature = "async")]
mod imp {
    use futures::future::{Future, BoxFuture};
    use std::pin::Pin;

    pub enum MaybeFuture<'a, T, E> {
        Result(Result<T, E>),
        Future(BoxFuture<'a, Result<T, E>>),
    }

    impl<'a, T, E> MaybeFuture<'a, T, E> {
        pub fn is_result(&self) -> bool {
            matches!(self, MaybeFuture::Result(_))
        }

        pub fn is_future(&self) -> bool {
            matches!(self, MaybeFuture::Future(_))
        }

        pub fn result_ref(&self) -> Option<&Result<T, E>> {
            match self {
                MaybeFuture::Result(t) => Some(t),
                MaybeFuture::Future(_) => None,
            }
        }

        pub fn future_ref(&self) -> Option<&BoxFuture<'a, Result<T, E>>> {
            match self {
                MaybeFuture::Result(_) => None,
                MaybeFuture::Future(f) => Some(f),
            }
        }

        pub fn result_mut(&mut self) -> Option<&mut Result<T, E>> {
            match self {
                MaybeFuture::Result(t) => Some(t),
                MaybeFuture::Future(_) => None,
            }
        }
        
        pub fn future_mut(&mut self) -> Option<&mut BoxFuture<'a, Result<T, E>>> {
            match self {
                MaybeFuture::Result(_) => None,
                MaybeFuture::Future(f) => Some(f),
            }
        }

        pub fn into_result(self) -> Option<Result<T, E>> {
            match self {
                MaybeFuture::Result(t) => Some(t),
                MaybeFuture::Future(_) => None,
            }
        }

        pub fn into_future(self) -> Option<BoxFuture<'a, Result<T, E>>> {
            match self {
                MaybeFuture::Result(_) => None,
                MaybeFuture::Future(f) => Some(f),
            }
        }

        pub fn expect_result(self, msg: &str) -> Result<T, E> {
            self.into_result().expect(msg)
        }

        pub fn expect_future(self, msg: &str) -> BoxFuture<'a, Result<T, E>> {
            self.into_future().expect(msg)
        }

        pub fn unwrap_result(self) -> Result<T, E> {
            self.into_result().unwrap()
        }

        pub fn unwrap_future(self) -> BoxFuture<'a, Result<T, E>> {
            self.into_future().unwrap()
        }
    }

    impl<'a, T: std::fmt::Debug, E: std::fmt::Debug> std::fmt::Debug for MaybeFuture<'a, T, E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MaybeFuture::Result(t) => t.fmt(f),
                MaybeFuture::Future(_) => f.write_str("<future>"),
            }
        }
    }

    impl<'a, T, E> From<Result<T, E>> for MaybeFuture<'a, T, E> {
        fn from(t: Result<T, E>) -> Self {
            MaybeFuture::Result(t)
        }
    }

    impl<'a, Fut, T, E> From<Pin<Box<Fut>>> for MaybeFuture<'a, T, E>
    where
        Fut: Future<Output = Result<T, E>> + Send + 'a,
    {
        fn from(f: Pin<Box<Fut>>) -> Self {
            MaybeFuture::Future(f)
        }
    }
}
