use std::convert::Infallible;

use cel_cxx::*;

const MY_NAMESPACE: &str = "testing";

#[test]
fn test_math() -> Result<(), Error>{
    println!("test math");

    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;

    let program = env.compile("a+b")?;

    let activation = Activation::new()
        .bind_variable("a", 2i64)?
        .bind_variable("b", 3i64)?
        ;


    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(5));

    Ok(())
}

#[test]
fn test_simplefunc() -> Result<(), Error>{
    println!("test simple function");

    fn sum(a: i64, b: i64) -> Result<i64, Infallible> {Ok(a+b)}
    fn square(a: i64) -> Result<i64, Infallible> {Ok(a*a)}

    let env = Env::builder()
        .register_global_function("sum", sum)?
        .register_global_function("square", square)?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;

    let program = env.compile("square(sum(a,b))")?;

    let activation = Activation::new()
        .bind_variable("a", 2i64)?
        .bind_variable("b", 3i64)?
        ;

    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(25));

    Ok(())
}

#[test]
fn test_opaque_function() -> Result<(), Error> {
    println!("test opaque function");

    #[derive(Debug, Clone, PartialEq)]
    #[derive(Opaque)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, std::any::type_name::<Self>()))]
    struct MyOpaqueGeneric<T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static>(T);

    impl<T> std::fmt::Display for MyOpaqueGeneric<T>
    where T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    fn get_type(_: MyOpaqueGeneric<i64>) -> Result<String, Error> { Ok("i64".to_string()) }
    fn get_value(a: MyOpaqueGeneric<i64>) -> Result<i64, Error> { Ok(a.0) }

    fn sum(a: MyOpaqueGeneric<i64>, b: i64) -> Result<i64, Error> { Ok(a.0 + b) }

    let env = Env::builder()
        .register_member_function("get_type", get_type)?
        .register_member_function("get_value", get_value)?
        .register_member_function("sum", sum)?
        .declare_variable::<MyOpaqueGeneric<i64>>("mo")?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<MyOpaqueGeneric<i64>>("mo2")?
        .declare_variable::<Vec<i64>>("ml")?
        .build()?;

    {
        let expr = "a>1 ? mo.val() : mo.val()";
        let program = env.compile(expr)?;

        let mo: MyOpaqueGeneric<i64> = MyOpaqueGeneric(16);
        let activation = Activation::new()
            .bind_variable("mo", mo)?
            .bind_variable("a", 0i32)?
            ;

        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(16));
    }

    {
        let expr = "ml.map(m, mo2.sum(m))";
        let program = env.compile(expr)?;

        let mo: MyOpaqueGeneric<i64> = MyOpaqueGeneric(16);
        let ml: Vec<i64> = vec![84,85,86,87,88];
        let activation = Activation::new()
            .bind_variable("mo2", mo)?
            .bind_variable("ml",ml)?
            ;

        let res = program.evaluate(&activation)?;

        let list = Vec::<i64>::from_value(res)
            .map_err(|_| Error::invalid_argument("invalid vec i64".to_string()))?;

        assert_eq!(list, vec![92,93,94,95,96]);
    }

    Ok(())
}
