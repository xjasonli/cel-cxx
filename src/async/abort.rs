//! Abortable utilities for CEL asynchronous runtime.
//!
use futures::channel::oneshot;
use futures::Future;
use std::pin::Pin;

/// Create a new abortable pair.
pub(crate) fn abortable() -> (AbortHandle, Abortable) {
    AbortHandle::new()
}

/// Abort handle for CEL asynchronous runtime.
#[derive(Debug)]
pub(crate) struct AbortHandle(Option<oneshot::Sender<()>>);

impl AbortHandle {
    fn new() -> (Self, Abortable) {
        let (sender, receiver) = oneshot::channel();
        (Self(Some(sender)), Abortable { receiver })
    }

    fn abort(&mut self) {
        if let Some(tx) = self.0.take() {
            let _ = tx.send(());
        }
    }
}

impl Drop for AbortHandle {
    fn drop(&mut self) {
        self.abort();
    }
}

pin_project_lite::pin_project! {
    /// Abortable future for CEL asynchronous runtime.
    #[derive(Debug)]
    pub(crate) struct Abortable {
        #[pin]
        receiver: oneshot::Receiver<()>,
    }
}

impl Future for Abortable {
    type Output = ();
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        let receiver = this.receiver;
        receiver.poll(cx).map(|_| ())
    }
}
