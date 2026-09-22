//! Cancel an evaluation and observe the pending callback's resources being released.
//!
//! Run with: `cargo run --example async-cancellation --features tokio`

use cel_cxx::*;
use futures::channel::oneshot;
use std::sync::Mutex;
use std::time::Duration;

struct NotifyOnDrop(Option<oneshot::Sender<()>>);

impl Drop for NotifyOnDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.0.take() {
            let _ = sender.send(());
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (started_tx, started_rx) = oneshot::channel();
    let (dropped_tx, dropped_rx) = oneshot::channel();
    let notifications = Mutex::new(Some((started_tx, dropped_tx)));
    let env = Env::builder()
        .declare_variable::<bool>("wait")?
        // The closure owns its captures so the evaluation worker may outlive
        // the caller's activation or program handle.
        .register_global_function("fetch", move || {
            let channels = notifications
                .lock()
                .map_err(|_| Error::internal("notification mutex poisoned"))
                .and_then(|mut channels| {
                    channels
                        .take()
                        .ok_or_else(|| Error::failed_precondition("fetch is a one-shot demo"))
                });
            async move {
                let (started, dropped) = channels?;
                let _resource = NotifyOnDrop(Some(dropped));
                let _ = started.send(());
                // Model an operation that remains pending until it is cancelled.
                futures::future::pending::<Result<i64, Error>>().await
            }
        })?
        .use_tokio()
        .build()?;
    let program = env.compile("wait ? fetch() : 42")?;
    let activation = Activation::new().bind_variable("wait", true)?;
    let mut evaluation = program.evaluate(&activation);
    drop(activation);

    // Wait until the callback has actually started, with a deadline so a broken
    // setup fails visibly. No sleep is needed to guess when the callback runs.
    tokio::time::timeout(Duration::from_secs(5), async {
        tokio::select! {
            started = started_rx => {
                started.map_err(|_| Error::internal("callback did not start"))?;
                Ok::<(), Error>(())
            }
            result = &mut evaluation => {
                Err(Error::internal(format!("evaluation completed unexpectedly: {result:?}")))
            }
        }
    })
    .await??;

    // Dropping the returned future signals cancellation to the CEL worker.
    // Cancellation takes effect while an async callback is pending; it does not
    // interrupt a synchronous Rust callback or undo completed side effects.
    drop(evaluation);
    tokio::time::timeout(Duration::from_secs(5), dropped_rx).await??;
    println!("Evaluation cancelled; the pending callback's resource was dropped.");

    let activation = Activation::new().bind_variable("wait", false)?;
    let result = program.evaluate(activation).await?;
    assert_eq!(result, Value::Int(42));
    println!("A later evaluation of the same program succeeds: {result}");
    Ok(())
}
