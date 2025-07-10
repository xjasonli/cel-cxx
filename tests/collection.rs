use cel_cxx::*;

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

    let Value::List(list) = value else {
        return Err(Error::invalid_argument("expected list".to_string()));
    };

    let mapres: Result<Vec<i64>, _> = list
        .into_iter()
        .map(|v| {
            i64::try_from(v).map_err(|_| Error::invalid_argument("invalid vec i64".to_string()))
        })
        .collect();
    println!("map res: {:?}", mapres?);

    Ok(())
}

#[cfg(feature = "derive")]
#[test]
fn test_hand_map_function() -> Result<(), Error> {
    use std::collections::HashMap;
    const MY_NAMESPACE: &str = "testing";

    println!("test map function");

    #[derive(Opaque, Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, "TestOpaque"))]
    struct TestOpaque(i64);

    impl std::fmt::Display for TestOpaque {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestOpaque({})", self.0)
        }
    }

    fn get_type(_a: TestOpaque) -> Result<String, Error> {
        Ok("i64".to_string())
    }

    fn get_value(a: TestOpaque) -> Result<i64, Error> {
        Ok(a.0)
    }

    fn add_value(a: TestOpaque, b: i64) -> Result<i64, Error> {
        Ok(a.0 + b)
    }

    let env = Env::builder()
        .register_member_function("get_type", get_type)?
        .register_member_function("get_value", get_value)?
        .register_member_function("add_value", add_value)?
        .register_global_function("add_value", add_value)?
        .register_global_function("get_value", get_value)?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<HashMap<i64, TestOpaque>>("mp")?
        .build()?;

    // let expr = "mp.map(m, m.filter(key, m[key].sum(key) > 5))";
    let expr = "mp.map(m, m>2, m-2)";
    let program = env.compile(expr)?;

    let a: i64 = 5;
    let mp: HashMap<i64, TestOpaque> = HashMap::from([
        (1, TestOpaque(2)),
        (2, TestOpaque(4)),
        (3, TestOpaque(6)),
        (4, TestOpaque(8)),
        (5, TestOpaque(10)),
    ]);

    let activation = Activation::new()
        .bind_variable("mp", mp)?
        .bind_variable("a", a)?;

    let res = program.evaluate(&activation)?;
    println!("res: {:?}", res);

    let Value::List(list) = res else {
        return Err(Error::invalid_argument("expected list".to_string()));
    };

    let mapres: Result<Vec<i64>, _> = list
        .into_iter()
        .map(|v| {
            i64::try_from(v).map_err(|_| Error::invalid_argument("invalid vec i64".to_string()))
        })
        .collect();
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

    let activation = Activation::new();

    let res = program.evaluate(&activation)?;
    println!("res: {:?}", res);

    let Value::List(list) = res else {
        return Err(Error::invalid_argument("expected list".to_string()));
    };

    let mapres: Result<Vec<Vec<i64>>, _> = list
        .into_iter()
        .map(|v| {
            Vec::<i64>::try_from(v)
                .map_err(|_| Error::invalid_argument("invalid vec i64".to_string()))
        })
        .collect();
    println!("list res: {:?}", mapres?);

    Ok(())
}
