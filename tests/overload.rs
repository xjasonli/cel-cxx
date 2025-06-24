use cel_cxx::*;
use std::collections::HashMap;

const MY_NAMESPACE: &str = "testing";

#[test]
fn test_opaque_function_overload() -> Result<(), Error> {
    println!("test opaque function overload");

    #[derive(Opaque, Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, "TestOpaqueI64"))]
    struct TestOpaqueI64(i64);

    #[derive(Opaque, Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, "TestOpaqueU64"))]
    struct TestOpaqueU64(u64);

    impl std::fmt::Display for TestOpaqueI64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestOpaqueI64({})", self.0)
        }
    }

    impl std::fmt::Display for TestOpaqueU64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestOpaqueU64({})", self.0)
        }
    }

    fn sum_i64(a: TestOpaqueI64, b: i64) -> Result<i64, Error> {
        Ok(a.0 + b)
    }

    fn sum_u64(a: TestOpaqueU64, b: i64) -> Result<u64, Error> {
        Ok(a.0 + b as u64 * 2)
    }

    let env = Env::builder()
        .register_member_function("sum", sum_i64)?
        .register_member_function("sum", sum_u64)?
        .declare_variable::<TestOpaqueU64>("mu")?
        .declare_variable::<TestOpaqueI64>("mi")?
        .declare_variable::<Vec<i64>>("il")?
        .build()?;

    // Test u64 overload
    {
        let expr = "il.map(m, mu.sum(m))";
        let program = env.compile(expr)?;

        let mu = TestOpaqueU64(16);
        let il: Vec<i64> = vec![1, 2, 3];

        let activation = Activation::new()
            .bind_variable("mu", mu)?
            .bind_variable("il", il)?;

        let res = program.evaluate(&activation)?;
        println!("u64 overload result: {:?}", res);

        let result_vec: Vec<u64> = res
            .try_into()
            .map_err(|_| Error::invalid_argument("conversion failed".to_string()))?;
        assert_eq!(result_vec, vec![18, 20, 22]);
    }

    // Test i64 overload
    {
        let expr = "il.map(m, mi.sum(m))";
        let program = env.compile(expr)?;

        let mi = TestOpaqueI64(10);
        let il: Vec<i64> = vec![1, 2, 3];

        let activation = Activation::new()
            .bind_variable("mi", mi)?
            .bind_variable("il", il)?;

        let res = program.evaluate(&activation)?;
        println!("i64 overload result: {:?}", res);

        let result_vec: Vec<i64> = res
            .try_into()
            .map_err(|_| Error::invalid_argument("conversion failed".to_string()))?;
        assert_eq!(result_vec, vec![11, 12, 13]);
    }

    Ok(())
}

#[test]
fn test_list_function_overload() -> Result<(), Error> {
    println!("test list function overload");

    fn num_i64(a: Vec<i64>) -> Result<usize, Error> {
        Ok(a.len() + 1)
    }
    fn num_string(a: Vec<String>) -> Result<usize, Error> {
        Ok(a.len() + 3)
    }
    fn num_optional(a: Vec<Option<i32>>) -> Result<usize, Error> {
        Ok(a.len() + 4)
    }

    let env = Env::builder()
        .register_member_function("num", num_i64)?
        .register_member_function("num", num_string)?
        .register_member_function("num", num_optional)?
        .declare_variable::<Vec<i64>>("li64")?
        .declare_variable::<Vec<String>>("ls")?
        .declare_variable::<Vec<Option<i32>>>("loi32")?
        .build()?;

    // Test i64 list
    {
        let expr = "li64.num()";
        let program = env.compile(expr)?;
        let li64: Vec<i64> = vec![1, 2, 3, 4, 5];

        let activation = Activation::new().bind_variable("li64", li64)?;

        let res = program.evaluate(&activation)?;
        println!("li64 num: {:?}", res);
        assert_eq!(res, Value::Uint(6)); // len(5) + 1
    }

    // Test string list
    {
        let expr = "ls.num()";
        let program = env.compile(expr)?;
        let ls: Vec<String> = vec!["a".to_string(), "b".to_string()];

        let activation = Activation::new().bind_variable("ls", ls)?;

        let res = program.evaluate(&activation)?;
        println!("ls num: {:?}", res);
        assert_eq!(res, Value::Uint(5)); // len(2) + 3
    }

    // Test optional list
    {
        let expr = "loi32.num()";
        let program = env.compile(expr)?;
        let loi32: Vec<Option<i32>> = vec![Some(1), None, Some(3)];

        let activation = Activation::new().bind_variable("loi32", loi32)?;

        let res = program.evaluate(&activation)?;
        println!("loi32 num: {:?}", res);
        assert_eq!(res, Value::Uint(7)); // len(3) + 4
    }

    Ok(())
}

#[test]
fn test_map_function_overload() -> Result<(), Error> {
    println!("test map function overload");

    fn name_str_i64(_: HashMap<String, i64>) -> Result<String, Error> {
        Ok("map of str->i64".to_string())
    }

    fn name_str_optional(_: HashMap<String, Option<i64>>) -> Result<String, Error> {
        Ok("map of str->optioni64".to_string())
    }

    let env = Env::builder()
        .register_member_function("name", name_str_i64)?
        .register_member_function("name", name_str_optional)?
        .declare_variable::<HashMap<String, i64>>("msi64")?
        .declare_variable::<HashMap<String, Option<i64>>>("msoi64")?
        .build()?;

    // Test string->i64 map
    {
        let expr = "msi64.name()";
        let program = env.compile(expr)?;
        let msi64: HashMap<String, i64> =
            HashMap::from([("a".to_string(), 1), ("b".to_string(), 2)]);

        let activation = Activation::new().bind_variable("msi64", msi64)?;

        let res = program.evaluate(&activation)?;
        if let Value::String(result) = res {
            println!("msi64 name: {}", result);
            assert!(result.contains("map of str->i64"));
        } else {
            return Err(Error::invalid_argument(
                "unexpected result type".to_string(),
            ));
        }
    }

    // Test string->optional map
    {
        let expr = "msoi64.name()";
        let program = env.compile(expr)?;
        let msoi64: HashMap<String, Option<i64>> =
            HashMap::from([("a".to_string(), Some(1)), ("b".to_string(), None)]);

        let activation = Activation::new().bind_variable("msoi64", msoi64)?;

        let res = program.evaluate(&activation)?;
        if let Value::String(result) = res {
            println!("msoi64 name: {}", result);
            assert!(result.contains("map of str->optioni64"));
        } else {
            return Err(Error::invalid_argument(
                "unexpected result type".to_string(),
            ));
        }
    }

    Ok(())
}
