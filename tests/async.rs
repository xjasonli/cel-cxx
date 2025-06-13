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
        sleep(Duration::from_secs(10)).await;
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
    }

    Ok(())
}

#[test]
fn test_simple_asyncfunc_timeout() -> Result<(), Error> {
    println!("test simple async timeout function");

    async fn asleep() -> Result<(), Error>{
        sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    async fn sum(a: i64, b: i64) -> Result<i64, Error> {
        sleep(Duration::from_secs(5)).await;
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

    let activation = Activation::new()
        .bind_variable("a", 5i64)?
        .bind_variable("b", 12i64)?;

    let wf = program.evaluate(&activation);

    let res = rt.block_on(async {
        tokio::select! {
            res = wf => {
                println!("Future done");
                return res;
            },
            _ = sleep(Duration::from_secs(3)) => {
                println!("Future was cancelled");
                return Err(Error::invalid_argument("Timeout".to_string()));
            },
        }
    });
    println!("res: {:?}", res);

    Ok(())
}

#[test]
fn test_opaque_asyncfunc() -> Result<(), Error> {
    println!("test opaque async function");

    #[derive(Opaque)]
    #[derive(Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, std::any::type_name::<Self>()))]
    struct MyOpaqueGeneric<T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static>(T);
    impl<T> std::fmt::Display for MyOpaqueGeneric<T>
    where T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    async fn val(a: MyOpaqueGeneric<i64>) -> Result<i64, Error> {
        sleep(Duration::from_secs(1)).await;
        Ok(a.0)
    }

    async fn sum(a: MyOpaqueGeneric<i64>, b: i64) -> Result<i64, Error> {
        sleep(Duration::from_secs(1)).await;
        Ok(a.0 + b)
    }

    let env = Env::builder()
        .register_global_function("sum", sum)?
        .register_member_function("val", val)?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<MyOpaqueGeneric<i64>>("mo")?
        .declare_variable::<Vec<MyOpaqueGeneric<i64>>>("ml")?
        .use_tokio()
        .build()?;

    {
        let program = env.compile("mo.val()")?;
        let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
        let _guard = rt.enter();

        let mo: MyOpaqueGeneric<i64> = MyOpaqueGeneric(16);
        let activation = Activation::new()
            .bind_variable("mo", mo)?
            .bind_variable("a", 12i64)?;

        let res = rt.block_on(async { program.evaluate(&activation).await });
        println!("res: {:?}", res?);
    }

    {
        let expr = "ml.map(m, m.val()%2!=0, sum(m,10))";
        let program = env.compile(expr)?;
        let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
        let _guard = rt.enter();

        let ml: Vec<MyOpaqueGeneric<i64>> = vec![MyOpaqueGeneric(1),MyOpaqueGeneric(3),MyOpaqueGeneric(4),MyOpaqueGeneric(5)];

        let activation = Activation::new()
            .bind_variable("ml", ml)?
            .bind_variable("a", 12i64)?;

        let res = rt.block_on(async { program.evaluate(&activation).await });
        println!("res: {:?}", Vec::<i64>::try_from(res?).map_err(|ev| Error::invalid_argument(ev.to_string())));
    }

    Ok(())
}

#[test]
fn call_async_func () -> Result<(), Error> {
    let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
    //f0 return future
    println!("f0:{}",rt.block_on(f0()));
    // println!("f0:{}",f0());
    Ok(())
}

async fn f0() -> i32 {
    sleep(Duration::from_secs(1)).await;
    16
}

#[test]
fn call_async_func_with_rt () -> Result<(), Error> {
    let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
    println!("f1:{}",rt.block_on(f1()));
    Ok(())
}

async fn f1() -> i32 {
    sleep(Duration::from_secs(1)).await;
    17
}

#[test]
fn call_async_block_with_rt() -> Result<(), Error> {
    let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
    println!("f2:{}",rt.block_on(f2()));
    Ok(())
}

fn f2() -> impl futures::Future<Output = i32> {
    async {
        sleep(Duration::from_secs(1)).await;
        18
    }
}

#[test]
fn call_async_spawn() -> Result<(), Error> {
    let rt = Runtime::new().map_err(|e| Error::invalid_argument(e.to_string()))?;
    let _guard = rt.enter();
    let h = f3("test1".to_string());
    let res = rt.block_on(h);
    println!("f3:{:?}", res);
    Ok(())
}

fn f3(msg: String) -> JoinHandle<i32> {
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        println!("spawn:{}", msg);
        19
    })
}

