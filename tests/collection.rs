use std::collections::HashMap;

use cel_cxx::*;


const MY_NAMESPACE: &str = "testing";

#[test]
fn test_map_function() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;


    // let expr = "{1:2, 2:4, 3:6, 4:8, 5:10}.map(m, m.filter(key, key > 3))";
    let expr = "{1:2, 2:4, 3:6, 4:8, 5:10}.map(m, m)";
    let program = env.compile(expr)?;

    let value = program.evaluate(())?;
    println!("value: {:?}", value);
    let Value::List(list) = value else { todo!() };
    let mapres: Result<Vec<i64>, _> = list.into_iter()
        .map(|v| i64::try_from(v).map_err(|_| Error::invalid_argument("invalid vec i64".to_string()))).collect();
    println!("map res: {:?}", mapres?);

    Ok(())
}

#[test]
fn test_hand_map_function() -> Result<(), Error> {
    println!("test map function");

    #[derive(Opaque)]
    #[derive(Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, std::any::type_name::<Self>()))]
    struct MyOpaqueGeneric<T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static>(T);

    impl<T> std::fmt::Display for MyOpaqueGeneric<T>
    where
        T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    fn ty(_a: MyOpaqueGeneric<i64>) -> Result<String, Error> { Ok("i64".to_string()) }
    fn val(a: MyOpaqueGeneric<i64>) -> Result<i64,Error> { Ok(a.0) }
    fn sum(a: MyOpaqueGeneric<i64>, b: i64) -> Result<i64,Error> { Ok(a.0 + b) }

    let env = Env::builder()
        .register_member_function("ty", ty)?
        .register_member_function("val", val)?
        .register_member_function("sum", sum)?
        .register_global_function("sum", sum)?
        .register_global_function("val", val)?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<HashMap<i64, MyOpaqueGeneric<i64>>>("mp")?
        .build()?;


    // let expr = "mp.map(m, m.filter(key, m[key].sum(key) > 5))";
    let expr = "mp.map(m, m>2, m-2)";
    let program = env.compile(expr)?;

    let a: i64 = 5;
    let mp: HashMap<i64, MyOpaqueGeneric<i64>> = HashMap::from([
        (1, MyOpaqueGeneric(2)),
        (2, MyOpaqueGeneric(4)),
        (3, MyOpaqueGeneric(6)),
        (4, MyOpaqueGeneric(8)),
        (5, MyOpaqueGeneric(10)),
    ]);

    let activation = Activation::new()
        .bind_variable("mp", mp)?
        .bind_variable("a", a)?
        ;

    let res = program.evaluate(&activation)?;
    println!("res: {:?}", res);
    let Value::List(list) = res else {
        assert!(false, "invalid value type");
        return Ok(())
    };
    let mapres: Result<Vec<i64>, _> = list.into_iter()
        .map(|v| i64::try_from(v).map_err(|_| Error::invalid_argument("invalid vec i64".to_string()))).collect();
    println!("map res: {:?}", mapres?);

    // let Value::String(tystr) = res else { todo!() };
    // assert_eq!("i64", tystr);

    Ok(())
}

#[test]
fn test_list_expr() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;

    let expr = "[{1:2, 2:4, 3:6, 4:8, 5:10}].map(m, m.filter(key, key > 3))";
    let program = env.compile(expr)?;

    let activation = Activation::new()
        ;

    let res = program.evaluate(&activation)?;
    println!("res: {:?}", res);
    let Value::List(list) = res else { todo!() };
    let mapres: Result<Vec<Vec<i64>>, _> = list.into_iter()
        .map(|v| Vec::<i64>::try_from(v).map_err(|_| Error::invalid_argument("invalid vec i64".to_string()))).collect();
    println!("list res: {:?}", mapres?);

    Ok(())
}
