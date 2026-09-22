#![cfg(any(feature = "tokio", feature = "smol"))]

use async_scoped::spawner::Spawner;
use cel_cxx::r#async::Runtime;
use cel_cxx::*;
use futures::channel::oneshot;
use futures::future::{self, BoxFuture, Either};
use std::collections::HashMap;
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Wraps a future with a 10s deadline so a deadlocked evaluation fails the
/// test instead of hanging the suite. `smol::Timer` relies on the async-io
/// global reactor, so it works on any executor; the tokio fallback requires
/// the ambient runtime provided by `#[tokio::test]`.
async fn deadline<T>(future: impl Future<Output = T>) -> T {
    #[cfg(feature = "smol")]
    {
        let timer = smol::Timer::after(Duration::from_secs(10));
        match future::select(Box::pin(timer), Box::pin(future)).await {
            Either::Left(_) => panic!("async evaluation did not finish"),
            Either::Right((result, _)) => result,
        }
    }
    #[cfg(all(feature = "tokio", not(feature = "smol")))]
    {
        tokio::time::timeout(Duration::from_secs(10), future)
            .await
            .expect("async evaluation did not finish")
    }
}

struct DropSignal(Option<oneshot::Sender<()>>);

impl Drop for DropSignal {
    fn drop(&mut self) {
        if let Some(sender) = self.0.take() {
            let _ = sender.send(());
        }
    }
}

struct CallControl {
    started: oneshot::Receiver<()>,
    dropped: oneshot::Receiver<()>,
    release: oneshot::Sender<i64>,
    captured_dropped: oneshot::Receiver<()>,
}

fn controlled_call() -> (
    impl Fn() -> BoxFuture<'static, Result<i64, Error>> + Send + Sync,
    CallControl,
) {
    let (started_tx, started) = oneshot::channel();
    let (dropped_tx, dropped) = oneshot::channel();
    let (release, release_rx) = oneshot::channel();
    let (captured_tx, captured_dropped) = oneshot::channel();
    let captured = Arc::new(DropSignal(Some(captured_tx)));
    let channels = Mutex::new(Some((started_tx, dropped_tx, release_rx)));
    let callback = move || -> BoxFuture<'static, Result<i64, Error>> {
        let captured = captured.clone();
        let channels = channels
            .lock()
            .unwrap()
            .take()
            .ok_or_else(|| Error::internal("callback must run exactly once"));
        Box::pin(async move {
            let _captured = captured;
            let (started, dropped, release) = channels?;
            let _drop_signal = DropSignal(Some(dropped));
            let _ = started.send(());
            release
                .await
                .map_err(|_| Error::cancelled("release dropped"))
        })
    };
    (
        callback,
        CallControl {
            started,
            dropped,
            release,
            captured_dropped,
        },
    )
}

async fn await_started(
    evaluation: &mut BoxFuture<'static, Result<Value, Error>>,
    started: oneshot::Receiver<()>,
) {
    deadline(async {
        match future::select(started, evaluation.as_mut()).await {
            Either::Left((result, _)) => result.expect("callback did not start"),
            Either::Right((result, _)) => panic!("evaluation finished before release: {result:?}"),
        }
    })
    .await;
}

async fn mixed_functions_and_providers<R: Runtime>() -> Result<(), Error>
where
    <R::ScopedSpawner as Spawner<()>>::FutureOutput: Send,
{
    let env = Env::builder()
        .declare_variable::<i64>("sync_value")?
        .declare_variable::<i64>("async_value")?
        .register_global_function("twice", |value: i64| value * 2)?
        .register_global_function("plus_one", |value: i64| async move { value + 1 })?
        .declare_global_function::<fn(i64) -> i64>("bound")?
        .use_runtime::<R>()
        .build()?;
    let activation = Activation::new()
        .bind_variable_provider("sync_value", || 20i64)?
        .bind_variable_provider("async_value", || async { 3i64 })?
        .bind_global_function("bound", |value: i64| async move { value - 2 })?;
    let program = env.compile("bound(plus_one(twice(sync_value)) + async_value)")?;
    for _ in 0..3 {
        assert_eq!(
            deadline(program.evaluate(&activation)).await?,
            Value::Int(42)
        );
    }
    Ok(())
}

async fn concurrent_evaluations_are_isolated<R: Runtime>() -> Result<(), Error>
where
    <R::ScopedSpawner as Spawner<()>>::FutureOutput: Send,
{
    let env = Env::builder()
        .declare_variable::<i64>("input")?
        .declare_global_function::<fn(i64) -> i64>("offset")?
        .use_runtime::<R>()
        .build()?;
    let program = env.compile("offset(input)")?;
    let mut awaiting_start = Vec::new();
    let mut releases = Vec::new();
    for input in 0..32i64 {
        let (callback, control) = controlled_call();
        let activation = Activation::new()
            .bind_variable_provider("input", move || async move { input })?
            .bind_global_function("offset", move |value: i64| {
                let offset = callback();
                async move { Ok::<_, Error>(value + offset.await?) }
            })?;
        let mut evaluation = program.evaluate(&activation);
        releases.push(control.release);
        awaiting_start.push(async move {
            await_started(&mut evaluation, control.started).await;
            (evaluation, control.dropped)
        });
    }
    let pending = deadline(future::join_all(awaiting_start)).await;
    for (input, release) in releases.into_iter().enumerate() {
        release.send(input as i64).unwrap();
    }
    let results = deadline(future::join_all(pending.into_iter().map(
        |(evaluation, dropped)| async move {
            let result = evaluation.await;
            dropped.await.unwrap();
            result
        },
    )))
    .await;
    for (input, result) in results.into_iter().enumerate() {
        assert_eq!(result?, Value::Int(input as i64 * 2));
    }
    Ok(())
}

async fn evaluation_owns_its_inputs<R: Runtime>() -> Result<(), Error>
where
    <R::ScopedSpawner as Spawner<()>>::FutureOutput: Send,
{
    let (callback, control) = controlled_call();
    let env = Env::builder()
        .declare_variable::<String>("text")?
        .declare_global_function::<fn() -> i64>("wait")?
        .use_runtime::<R>()
        .build()?;
    let program = env.compile("wait() + size(text)")?;
    let activation = Activation::new()
        .bind_variable("text", String::from("边界\0🙂"))?
        .bind_global_function("wait", callback)?;
    let mut evaluation = program.evaluate(&activation);
    drop(activation);
    drop(program);
    drop(env);
    await_started(&mut evaluation, control.started).await;
    control.release.send(38).unwrap();
    assert_eq!(deadline(evaluation).await?, Value::Int(42));
    deadline(control.dropped).await.unwrap();
    Ok(())
}

async fn cancelling_function_drops_its_future<R: Runtime>() -> Result<(), Error>
where
    <R::ScopedSpawner as Spawner<()>>::FutureOutput: Send,
{
    let (callback, control) = controlled_call();
    let env = Env::builder()
        .register_global_function("wait", callback)?
        .use_runtime::<R>()
        .build()?;
    let program = env.compile("wait()")?;
    let mut evaluation = program.evaluate(());
    drop(program);
    drop(env);
    await_started(&mut evaluation, control.started).await;
    drop(evaluation);
    deadline(control.dropped).await.unwrap();
    deadline(control.captured_dropped).await.unwrap();
    assert!(control.release.send(42).is_err());
    Ok(())
}

async fn cancelling_provider_drops_its_future<R: Runtime>() -> Result<(), Error>
where
    <R::ScopedSpawner as Spawner<()>>::FutureOutput: Send,
{
    let (callback, control) = controlled_call();
    let env = Env::builder()
        .declare_variable::<i64>("value")?
        .use_runtime::<R>()
        .build()?;
    let program = env.compile("value + 1")?;
    let activation = Activation::new().bind_variable_provider("value", callback)?;
    let mut evaluation = program.evaluate(activation);
    drop(program);
    drop(env);
    await_started(&mut evaluation, control.started).await;
    drop(evaluation);
    deadline(control.dropped).await.unwrap();
    deadline(control.captured_dropped).await.unwrap();
    assert!(control.release.send(42).is_err());
    Ok(())
}

async fn cancellation_does_not_affect_other_evaluations<R: Runtime>() -> Result<(), Error>
where
    <R::ScopedSpawner as Spawner<()>>::FutureOutput: Send,
{
    let env = Env::builder()
        .declare_global_function::<fn() -> i64>("wait")?
        .use_runtime::<R>()
        .build()?;
    let program = env.compile("wait()")?;
    let (first_callback, first) = controlled_call();
    let (second_callback, second) = controlled_call();
    let first_activation = Activation::new().bind_global_function("wait", first_callback)?;
    let second_activation = Activation::new().bind_global_function("wait", second_callback)?;
    let mut first_evaluation = program.evaluate(first_activation);
    let mut second_evaluation = program.evaluate(second_activation);
    deadline(future::join(
        await_started(&mut first_evaluation, first.started),
        await_started(&mut second_evaluation, second.started),
    ))
    .await;
    drop(first_evaluation);
    deadline(first.dropped).await.unwrap();
    assert!(first.release.send(0).is_err());
    second.release.send(42).unwrap();
    assert_eq!(deadline(second_evaluation).await?, Value::Int(42));
    deadline(second.dropped).await.unwrap();
    Ok(())
}

async fn evaluation_owns_nested_types_and_values<R: Runtime>() -> Result<(), Error>
where
    <R::ScopedSpawner as Spawner<()>>::FutureOutput: Send,
{
    type Rows = Vec<HashMap<String, Optional<i64>>>;
    let rows = vec![
        HashMap::new(),
        HashMap::from([
            (String::from("present\0🙂"), Optional::new(i64::MIN)),
            (String::from("absent"), Optional::none()),
        ]),
    ];
    let expected = rows.clone().into_value();
    let (callback, control) = controlled_call();
    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<Rows>("rows")?
        .declare_global_function::<fn() -> i64>("wait")?
        .declare_global_function::<fn(Rows) -> Rows>("echo")?
        .use_runtime::<R>()
        .build()?;
    let program = env.compile("wait() == 42 ? echo(rows) : []")?;
    let activation = Activation::new()
        .bind_variable("rows", rows)?
        .bind_global_function("wait", callback)?
        .bind_global_function("echo", |rows: Rows| async move { rows })?;
    let mut evaluation = program.evaluate(&activation);
    await_started(&mut evaluation, control.started).await;
    drop(activation);
    drop(program);
    drop(env);
    control.release.send(42).unwrap();
    assert_eq!(deadline(evaluation).await?, expected);
    deadline(control.captured_dropped).await.unwrap();
    Ok(())
}

async fn unused_providers_are_not_polled<R: Runtime>() -> Result<(), Error>
where
    <R::ScopedSpawner as Spawner<()>>::FutureOutput: Send,
{
    let calls = Arc::new(AtomicUsize::new(0));
    let provider_calls = calls.clone();
    let env = Env::builder()
        .declare_variable::<bool>("probe")?
        .declare_variable::<bool>("condition")?
        .use_runtime::<R>()
        .build()?;
    let mut activation = Activation::new().bind_variable_provider("probe", move || {
        provider_calls.fetch_add(1, Ordering::SeqCst);
        async { Err::<bool, _>(Error::data_loss("provider failure: \0边界")) }
    })?;
    for (expression, expected) in [
        ("condition && probe", false),
        ("condition || probe", true),
        ("condition ? true : probe", true),
        ("condition ? probe : false", false),
    ] {
        activation = activation.bind_variable("condition", expected)?;
        assert_eq!(
            deadline(env.compile(expression)?.evaluate(&activation)).await?,
            Value::Bool(expected),
            "{expression}"
        );
    }
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let error = deadline(env.compile("probe")?.evaluate(activation))
        .await
        .unwrap_err();
    assert_eq!(error.code(), Code::DataLoss);
    assert_eq!(error.message(), "provider failure: \0边界");
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}

macro_rules! runtime_tests {
    ($($name:ident),+ $(,)?) => {
        #[cfg(feature = "tokio")]
        mod tokio_runtime {
            use super::*;
            $(
                #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
                async fn $name() -> Result<(), Error> {
                    super::$name::<cel_cxx::r#async::Tokio>().await
                }
            )+
        }

        #[cfg(feature = "smol")]
        mod smol_runtime {
            use super::*;
            $(
                #[test]
                fn $name() -> Result<(), Error> {
                    smol::block_on(super::$name::<cel_cxx::r#async::Smol>())
                }
            )+
        }
    };
}

runtime_tests!(
    mixed_functions_and_providers,
    concurrent_evaluations_are_isolated,
    evaluation_owns_its_inputs,
    evaluation_owns_nested_types_and_values,
    cancelling_function_drops_its_future,
    cancelling_provider_drops_its_future,
    cancellation_does_not_affect_other_evaluations,
    unused_providers_are_not_polled,
);
