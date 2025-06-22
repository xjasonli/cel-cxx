use tokio::task::JoinHandle;
use cel_cxx::*;
use tokio::time::{sleep, Duration};
use tokio::runtime::Runtime;

const MY_NAMESPACE: &str = "testing";

#[test]
fn test_simple_asyncfunc() -> Result<(), Error> {
    println!("test simple async function");

    async fn asleep() -> Result<(), Error>{
        sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    async fn sum(a: i64, b: i64) -> Result<i64, Error> {
        sleep(Duration::from_millis(100)).await;
        Ok(a + b)
    }

    let env = Env::builder()
        .register_global_function("asleep", asleep)?
        .register_global_function("sum", sum)?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .use_tokio()
        .build()?;

    let program = env.compile("sum(a,b)")?;

    let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
    let _guard = rt.enter();

    {
        let activation = Activation::new()
            .bind_variable("a", 5i64)?
            .bind_variable("b", 12i64)?;

        let res = rt.block_on(async { program.evaluate(&activation).await });
        println!("res: {:?}", res);
        assert_eq!(res?, Value::Int(17));
    }

    Ok(())
}

#[test]
fn test_simple_asyncfunc_timeout() -> Result<(), Error> {
    println!("test simple async timeout function");

    async fn slow_sum(a: i64, b: i64) -> Result<i64, Error> {
        sleep(Duration::from_secs(2)).await;
        Ok(a + b)
    }

    let env = Env::builder()
        .register_global_function("slow_sum", slow_sum)?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .use_tokio()
        .build()?;

    let program = env.compile("slow_sum(a,b)")?;

    let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
    let _guard = rt.enter();

    let activation = Activation::new()
        .bind_variable("a", 5i64)?
        .bind_variable("b", 12i64)?;

    let wf = program.evaluate(&activation);

    let res = rt.block_on(async {
        tokio::select! {
            res = wf => {
                println!("Future completed");
                return res;
            },
            _ = sleep(Duration::from_secs(1)) => {
                println!("Future was cancelled due to timeout");
                return Err(Error::invalid_argument("Timeout".to_string()));
            },
        }
    });
    
    assert!(res.is_err());
    println!("Expected timeout occurred: {:?}", res);

    Ok(())
}

#[test]
fn test_opaque_asyncfunc() -> Result<(), Error> {
    println!("test opaque async function");

    #[derive(Opaque, Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, "AsyncOpaque"))]
    struct AsyncOpaque(i64);

    impl std::fmt::Display for AsyncOpaque {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AsyncOpaque({})", self.0)
        }
    }

    async fn get_value(a: AsyncOpaque) -> Result<i64, Error> {
        sleep(Duration::from_millis(50)).await;
        Ok(a.0)
    }

    async fn add_async(a: AsyncOpaque, b: i64) -> Result<i64, Error> {
        sleep(Duration::from_millis(50)).await;
        Ok(a.0 + b)
    }

    let env = Env::builder()
        .register_global_function("add_async", add_async)?
        .register_member_function("get_value", get_value)?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<AsyncOpaque>("mo")?
        .declare_variable::<Vec<AsyncOpaque>>("ml")?
        .use_tokio()
        .build()?;

    {
        let program = env.compile("mo.get_value()")?;
        let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
        let _guard = rt.enter();

        let mo = AsyncOpaque(16);
        let activation = Activation::new()
            .bind_variable("mo", mo)?
            .bind_variable("a", 12i64)?;

        let res = rt.block_on(async { program.evaluate(&activation).await })?;
        assert_eq!(res, Value::Int(16));
        println!("Opaque member function result: {}", res);
    }

    {
        let expr = "ml.map(m, add_async(m, 10))";
        let program = env.compile(expr)?;
        let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
        let _guard = rt.enter();

        let ml: Vec<AsyncOpaque> = vec![
            AsyncOpaque(1),
            AsyncOpaque(3),
            AsyncOpaque(4),
            AsyncOpaque(5)
        ];

        let activation = Activation::new()
            .bind_variable("ml", ml)?
            .bind_variable("a", 12i64)?;

        let res = rt.block_on(async { program.evaluate(&activation).await })?;
        let result_list: Vec<i64> = res.try_into()
            .map_err(|_| Error::invalid_argument("conversion failed".to_string()))?;
        assert_eq!(result_list, vec![11, 13, 14, 15]);
        println!("Async collection operation result: {:?}", result_list);
    }

    Ok(())
}

#[test]
fn test_async_variable_provider() -> Result<(), Error> {
    println!("test async variable provider");

    async fn get_async_value() -> Result<i64, Error> {
        sleep(Duration::from_millis(100)).await;
        Ok(42)
    }

    let env = Env::builder()
        .declare_variable::<i64>("async_var")?
        .declare_variable::<i64>("sync_var")?
        .use_tokio()
        .build()?;

    let program = env.compile("async_var + sync_var")?;
    
    let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
    let _guard = rt.enter();

    let activation = Activation::new()
        .bind_variable("sync_var", 8i64)?
        .bind_variable_provider("async_var", || get_async_value())?;

    let res = rt.block_on(async { program.evaluate(&activation).await })?;
    assert_eq!(res, Value::Int(50));
    println!("Async variable provider result: {}", res);

    Ok(())
}

#[test]
fn call_async_func() -> Result<(), Error> {
    let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
    let result = rt.block_on(simple_async_fn());
    println!("Simple async function result: {}", result);
    assert_eq!(result, 42);
    Ok(())
}

async fn simple_async_fn() -> i32 {
    sleep(Duration::from_millis(10)).await;
    42
}

#[test]
fn call_async_spawn() -> Result<(), Error> {
    let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
    let _guard = rt.enter();

    let handle = spawn_async_task("test message".to_string());
    let result = rt.block_on(handle).map_err(|e| Error::invalid_argument(e.to_string()))?;
    println!("Spawned async task result: {}", result);
    assert_eq!(result, 100);
    Ok(())
}

fn spawn_async_task(msg: String) -> JoinHandle<i32> {
    tokio::spawn(async move {
        println!("Processing message: {}", msg);
        sleep(Duration::from_millis(50)).await;
        100
    })
}

