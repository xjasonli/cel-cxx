use cel_cxx::*;
use std::sync::atomic::{AtomicUsize, Ordering};

const ERROR_CODES: [Code; 16] = [
    Code::Cancelled,
    Code::Unknown,
    Code::InvalidArgument,
    Code::DeadlineExceeded,
    Code::NotFound,
    Code::AlreadyExists,
    Code::PermissionDenied,
    Code::ResourceExhausted,
    Code::FailedPrecondition,
    Code::Aborted,
    Code::OutOfRange,
    Code::Unimplemented,
    Code::Internal,
    Code::Unavailable,
    Code::DataLoss,
    Code::Unauthenticated,
];

#[test]
fn test_function_preserves_error_code() -> Result<(), Error> {
    for code in ERROR_CODES {
        let env = Env::builder()
            .register_global_function("failure", move || {
                Err::<i64, _>(Error::new(code, "function failure: \0边界"))
            })?
            .build()?;
        let error = env
            .compile("failure()")?
            .evaluate(())
            .expect_err("function failure");
        assert_eq!(error.code(), code);
        assert_eq!(error.message(), "function failure: \0边界", "{code:?}");
    }
    Ok(())
}

#[test]
fn test_variable_provider_preserves_error_code() -> Result<(), Error> {
    let env = Env::builder().declare_variable::<i64>("value")?.build()?;
    let program = env.compile("value")?;
    for code in ERROR_CODES {
        let activation = Activation::new().bind_variable_provider("value", move || {
            Err::<i64, _>(Error::new(code, "provider failure: \0边界"))
        })?;

        let error = program.evaluate(activation).expect_err("provider failure");
        assert_eq!(error.code(), code);
        assert_eq!(error.message(), "provider failure: \0边界", "{code:?}");
    }
    Ok(())
}

#[cfg(feature = "tokio")]
#[tokio::test(flavor = "multi_thread")]
async fn test_async_variable_provider_preserves_error_code() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<i64>("value")?
        .use_tokio()
        .build()?;
    let program = env.compile("value")?;
    for code in ERROR_CODES {
        let activation = Activation::new().bind_variable_provider("value", move || async move {
            Err::<i64, _>(Error::new(code, "provider failure: \0边界"))
        })?;

        let error = program
            .evaluate(activation)
            .await
            .expect_err("provider failure");
        assert_eq!(error.code(), code);
        assert_eq!(error.message(), "provider failure: \0边界", "{code:?}");
    }
    Ok(())
}

#[cfg(feature = "async")]
#[test]
fn test_sync_evaluation_rejects_async_custom_bindings() -> Result<(), Error> {
    #[derive(Default)]
    struct CustomActivation {
        functions: FunctionBindings<'static>,
        variables: VariableBindings<'static>,
    }

    impl ActivationInterface<'static> for CustomActivation {
        fn functions(&self) -> &FunctionBindings<'static> {
            &self.functions
        }

        fn variables(&self) -> &VariableBindings<'static> {
            &self.variables
        }
    }

    let env = Env::builder()
        .declare_global_function::<fn() -> i64>("score")?
        .declare_variable::<i64>("value")?
        .build()?;

    let mut activation = CustomActivation::default();
    activation
        .functions
        .bind_global("score", || async { 42i64 })?;
    let error = env
        .compile("score()")?
        .evaluate(activation)
        .expect_err("a synchronous evaluator cannot run an async binding");
    assert_eq!(error.code(), Code::FailedPrecondition);

    let mut activation = CustomActivation::default();
    activation
        .variables
        .bind_provider("value", || async { 42i64 })?;
    let error = env
        .compile("value")?
        .evaluate(activation)
        .expect_err("a synchronous evaluator cannot run an async provider");
    assert_eq!(error.code(), Code::FailedPrecondition);
    Ok(())
}

#[test]
fn test_variable_provider_is_not_called_in_unselected_branches() -> Result<(), Error> {
    let calls = AtomicUsize::new(0);
    let env = Env::builder()
        .declare_variable::<bool>("probe")?
        .declare_variable::<bool>("condition")?
        .build()?;
    let mut activation = Activation::new().bind_variable_provider("probe", || {
        calls.fetch_add(1, Ordering::SeqCst);
        Err::<bool, _>(Error::unavailable("provider was called"))
    })?;

    for (expression, condition, expected) in [
        ("condition && probe", false, false),
        ("condition || probe", true, true),
        ("condition ? true : probe", true, true),
        ("condition ? probe : false", false, false),
    ] {
        activation = activation.bind_variable("condition", condition)?;
        assert_eq!(
            env.compile(expression)?.evaluate(&activation)?,
            Value::Bool(expected),
            "{expression}"
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0, "{expression}");
    }

    let error = env.compile("probe")?.evaluate(&activation).unwrap_err();
    assert_eq!(error.code(), Code::Unavailable);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}

#[test]
fn test_variable_provider_is_refreshed_between_evaluations() -> Result<(), Error> {
    let calls = AtomicUsize::new(0);
    let env = Env::builder().declare_variable::<i64>("value")?.build()?;
    let program = env.compile("value")?;
    let activation = Activation::new()
        .bind_variable_provider("value", || calls.fetch_add(1, Ordering::SeqCst) as i64)?;

    for expected in 0..3 {
        assert_eq!(program.evaluate(&activation)?, Value::Int(expected));
    }
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    Ok(())
}

#[test]
fn test_value_and_provider_bindings_replace_each_other() -> Result<(), Error> {
    let calls = AtomicUsize::new(0);
    let env = Env::builder().declare_variable::<i64>("value")?.build()?;
    let program = env.compile("value")?;

    let activation = Activation::new()
        .bind_variable_provider("value", || {
            calls.fetch_add(1, Ordering::SeqCst);
            1i64
        })?
        .bind_variable("value", 2i64)?;
    assert_eq!(program.evaluate(&activation)?, Value::Int(2));
    assert_eq!(calls.load(Ordering::SeqCst), 0);

    let activation = activation.bind_variable_provider("value", || {
        calls.fetch_add(1, Ordering::SeqCst);
        3i64
    })?;
    assert_eq!(program.evaluate(&activation)?, Value::Int(3));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}

#[test]
fn test_unbound_variable_fails_only_when_evaluated() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<bool>("needed")?
        .declare_variable::<i64>("missing")?
        .build()?;
    let program = env.compile("needed ? missing : 7")?;
    let activation = Activation::new().bind_variable("needed", false)?;
    assert_eq!(program.evaluate(&activation)?, Value::Int(7));

    let activation = activation.bind_variable("needed", true)?;
    let error = program.evaluate(&activation).unwrap_err();
    assert_ne!(error.code(), Code::Ok);
    assert!(error.message().contains("missing"), "{error}");

    let activation = activation.bind_variable("missing", 42i64)?;
    assert_eq!(program.evaluate(&activation)?, Value::Int(42));
    Ok(())
}

#[test]
fn test_registered_function_is_not_called_in_unselected_branches() -> Result<(), Error> {
    let calls = AtomicUsize::new(0);
    let env = Env::builder()
        .declare_variable::<bool>("condition")?
        .register_global_function("probe", || {
            calls.fetch_add(1, Ordering::SeqCst);
            true
        })?
        .build()?;
    for (expression, condition, expected) in [
        ("condition && probe()", false, false),
        ("condition || probe()", true, true),
        ("condition ? true : probe()", true, true),
        ("condition ? probe() : false", false, false),
    ] {
        let activation = Activation::new().bind_variable("condition", condition)?;
        assert_eq!(
            env.compile(expression)?.evaluate(activation)?,
            Value::Bool(expected),
            "{expression}"
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0, "{expression}");
    }
    assert_eq!(
        env.compile("probe()")?.evaluate(Activation::new())?,
        Value::Bool(true)
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}

#[cfg(feature = "tokio")]
#[tokio::test(flavor = "multi_thread")]
async fn test_async_function_preserves_error_code() -> Result<(), Error> {
    let env = Env::builder()
        .declare_global_function::<fn() -> i64>("failure")?
        .use_tokio()
        .build()?;
    let program = env.compile("failure()")?;
    for code in ERROR_CODES {
        let activation = AsyncActivation::new_async()
            .bind_global_function("failure", move || async move {
                Err::<i64, _>(Error::new(code, "function failure: \0边界"))
            })?;
        let error = program.evaluate(activation).await.unwrap_err();
        assert_eq!(error.code(), code);
        assert_eq!(error.message(), "function failure: \0边界", "{code:?}");
    }
    Ok(())
}

#[cfg(feature = "tokio")]
#[tokio::test(flavor = "multi_thread")]
async fn test_async_provider_is_lazy_and_refreshed_between_evaluations() -> Result<(), Error> {
    let calls = std::sync::Arc::new(AtomicUsize::new(0));
    let provider_calls = calls.clone();
    let env = Env::builder()
        .declare_variable::<i64>("value")?
        .declare_variable::<bool>("needed")?
        .use_tokio()
        .build()?;
    let activation = AsyncActivation::new_async()
        .bind_variable("needed", false)?
        .bind_variable_provider("value", move || {
            let calls = provider_calls.clone();
            async move { calls.fetch_add(1, Ordering::SeqCst) as i64 }
        })?;
    assert_eq!(
        env.compile("needed ? value : 7")?
            .evaluate(&activation)
            .await?,
        Value::Int(7)
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let program = env.compile("value")?;
    for expected in 0..3 {
        assert_eq!(program.evaluate(&activation).await?, Value::Int(expected));
    }
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    Ok(())
}
