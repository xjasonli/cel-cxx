use cel_cxx::*;
use std::collections::HashMap;

#[cfg(feature = "derive")]
#[test]
fn test_opaque_function_overload() -> Result<(), Error> {
    const MY_NAMESPACE: &str = "testing";

    println!("test opaque function overload");

    #[derive(Opaque, Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, "TestOpaqueI64"))]
    #[cel_cxx(display)]
    struct TestOpaqueI64(i64);

    #[derive(Opaque, Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, "TestOpaqueU64"))]
    #[cel_cxx(display)]
    struct TestOpaqueU64(u64);

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
        println!("u64 overload result: {res:?}");

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
        println!("i64 overload result: {res:?}");

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
        println!("li64 num: {res:?}");
        assert_eq!(res, Value::Uint(6)); // len(5) + 1
    }

    // Test string list
    {
        let expr = "ls.num()";
        let program = env.compile(expr)?;
        let ls: Vec<String> = vec!["a".to_string(), "b".to_string()];

        let activation = Activation::new().bind_variable("ls", ls)?;

        let res = program.evaluate(&activation)?;
        println!("ls num: {res:?}");
        assert_eq!(res, Value::Uint(5)); // len(2) + 3
    }

    // Test optional list
    {
        let expr = "loi32.num()";
        let program = env.compile(expr)?;
        let loi32: Vec<Option<i32>> = vec![Some(1), None, Some(3)];

        let activation = Activation::new().bind_variable("loi32", loi32)?;

        let res = program.evaluate(&activation)?;
        println!("loi32 num: {res:?}");
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
            println!("msi64 name: {result}");
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
            println!("msoi64 name: {result}");
            assert!(result.contains("map of str->optioni64"));
        } else {
            return Err(Error::invalid_argument(
                "unexpected result type".to_string(),
            ));
        }
    }

    Ok(())
}

// Global list overloads with different element types coexist and resolve by
// the checked overload ID.
#[test]
fn test_global_same_kind_overload() -> Result<(), Error> {
    fn len_i64(a: Vec<i64>) -> Result<i64, Error> {
        Ok(a.len() as i64)
    }
    fn len_string(a: Vec<String>) -> Result<i64, Error> {
        Ok((a.len() as i64) * 10)
    }

    let env = Env::builder()
        .register_global_function("len", len_i64)?
        .register_global_function("len", len_string)?
        .declare_variable::<Vec<i64>>("li")?
        .declare_variable::<Vec<String>>("ls")?
        .build()?;

    {
        let program = env.compile("len(li)")?;
        let activation = Activation::new().bind_variable("li", vec![1i64, 2, 3, 4])?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(4));
    }
    {
        let program = env.compile("len(ls)")?;
        let activation =
            Activation::new().bind_variable("ls", vec!["a".to_string(), "b".to_string()])?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(20));
    }
    Ok(())
}

// A runtime binding whose (name, id) matches a compile-time decl supplies/overrides
// the impl for that overload.
#[test]
fn test_binding_overrides_registry_impl() -> Result<(), Error> {
    fn bump_a(x: i64) -> Result<i64, Error> {
        Ok(x * 2)
    }
    fn bump_b(x: i64) -> Result<i64, Error> {
        Ok(x * 3)
    }

    let env = Env::builder()
        .register_global_function("bump", bump_a)?
        .declare_variable::<i64>("x")?
        .build()?;

    let program = env.compile("bump(x)")?;
    let activation = Activation::new()
        .bind_variable("x", 5i64)?
        .bind_function("bump", false, bump_b)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(15)); // 5 * 3 — binding B overrides registry A
    Ok(())
}

// A runtime binding whose (name, id) has no compile-time declaration is rejected
// at eval time (the activation must not inject non-declared functions).
#[test]
fn test_binding_without_matching_decl_errors() -> Result<(), Error> {
    fn bump_int(x: i64) -> Result<i64, Error> {
        Ok(x * 2)
    }
    fn bump_string(x: String) -> Result<i64, Error> {
        Ok(x.len() as i64)
    }

    let env = Env::builder()
        .register_global_function("bump", bump_int)? // decl: bump(int)
        .declare_variable::<i64>("x")?
        .build()?;

    let program = env.compile("bump(x)")?;
    // bump(string) — no matching compile-time decl (decl is bump(int)).
    let activation =
        Activation::new()
            .bind_variable("x", 5i64)?
            .bind_function("bump", false, bump_string)?;
    let res = program.evaluate(&activation);
    assert!(
        res.is_err(),
        "expected eval error: binding 'bump(string)' has no compile-time declaration"
    );
    Ok(())
}

#[test]
fn test_binding_overload_rejects_member_without_receiver() -> Result<(), Error> {
    let mut bindings = FunctionBindings::new();
    bindings.bind_global("constant", || 1i64)?;
    let overloads = bindings.find_mut("constant").expect("bound function");
    let error = overloads
        .add(true, (|| 2i64).into_function())
        .expect_err("a member binding needs a receiver");
    assert_eq!(error.code(), Code::InvalidArgument);
    Ok(())
}

#[test]
fn test_binding_preserves_other_same_kind_overloads() -> Result<(), Error> {
    let env = Env::builder()
        .register_global_function("select", |_: Vec<i64>| 1i64)?
        .register_global_function("select", |_: Vec<String>| 2i64)?
        .build()?;
    let activation = Activation::new().bind_global_function("select", |_: Vec<i64>| 10i64)?;
    let program = env.compile("[select([1]), select(['a'])]")?;
    assert_eq!(program.evaluate(&activation)?, vec![10i64, 2].into());
    Ok(())
}

#[test]
fn test_decl_only_member_binding() -> Result<(), Error> {
    let env = Env::builder()
        .declare_member_function::<fn(Vec<i64>) -> i64>("score")?
        .declare_member_function::<fn(Vec<String>) -> i64>("score")?
        .build()?;
    let activation = Activation::new()
        .bind_member_function("score", |values: Vec<i64>| values.len() as i64)?
        .bind_member_function("score", |values: Vec<String>| values.len() as i64 * 10)?;
    let program = env.compile("[[1, 2].score(), ['a', 'b'].score()]")?;
    assert_eq!(program.evaluate(&activation)?, vec![2i64, 20].into());
    assert!(program.evaluate(()).is_err());
    Ok(())
}

#[test]
fn test_binding_without_declared_name_errors() -> Result<(), Error> {
    let env = Env::builder().build()?;
    let program = env.compile("1")?;
    let activation = Activation::new().bind_global_function("undeclared", || 2i64)?;
    let error = program
        .evaluate(&activation)
        .expect_err("bindings need a declaration even when unused");
    assert_eq!(error.code(), Code::InvalidArgument);
    assert!(error.message().contains("no compile-time declaration"));
    Ok(())
}

#[cfg(feature = "derive")]
#[test]
fn test_binding_rejects_different_types_with_same_display_name() -> Result<(), Error> {
    #[derive(Debug, Clone, PartialEq, Opaque)]
    #[cel_cxx(type = "int")]
    struct IntNamedOpaque;

    impl std::fmt::Display for IntNamedOpaque {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("opaque int")
        }
    }

    let env = Env::builder()
        .declare_global_function::<fn(i64) -> i64>("score")?
        .build()?;
    let activation = Activation::new().bind_global_function("score", |_: IntNamedOpaque| 42i64)?;
    let error = env
        .compile("1")?
        .evaluate(activation)
        .expect_err("a display-name collision must not bypass declaration validation");
    assert_eq!(error.code(), Code::InvalidArgument);
    assert!(error.message().contains("no compile-time declaration"));
    Ok(())
}

#[cfg(feature = "tokio")]
#[tokio::test(flavor = "multi_thread")]
async fn test_async_overload_bindings() -> Result<(), Error> {
    let env = Env::builder()
        .register_global_function("score", |_: Vec<i64>| async { 1i64 })?
        .declare_global_function::<fn(Vec<String>) -> i64>("score")?
        .use_tokio()
        .build()?;
    let program = env.compile("[score([1]), score(['a'])]")?;
    let activation = AsyncActivation::new_async()
        .bind_global_function("score", |_: Vec<i64>| async { 10i64 })?
        .bind_global_function("score", |_: Vec<String>| async { 20i64 })?;
    assert_eq!(program.evaluate(&activation).await?, vec![10i64, 20].into());

    let activation = AsyncActivation::new_async()
        .bind_global_function("score", |_: Vec<String>| async { 20i64 })?;
    assert_eq!(program.evaluate(&activation).await?, vec![1i64, 20].into());

    let activation =
        AsyncActivation::new_async().bind_global_function("score", |_: bool| async { 30i64 })?;
    let error = program
        .evaluate(&activation)
        .await
        .expect_err("an async binding also needs a matching declaration");
    assert_eq!(error.code(), Code::InvalidArgument);
    assert!(error.message().contains("no compile-time declaration"));
    Ok(())
}

#[test]
fn test_zero_argument_overload_binding_preserves_other_arities() -> Result<(), Error> {
    let env = Env::builder()
        .register_global_function("score", || 1i64)?
        .register_global_function("score", |value: i64| value + 2)?
        .build()?;
    let program = env.compile("[score(), score(3)]")?;
    assert_eq!(program.evaluate(())?, vec![1i64, 5].into());
    let activation = Activation::new().bind_global_function("score", || 10i64)?;
    assert_eq!(program.evaluate(activation)?, vec![10i64, 5].into());
    Ok(())
}

#[test]
fn test_global_and_member_overloads_with_identical_argument_types() -> Result<(), Error> {
    let env = Env::builder()
        .register_global_function("score", |value: i64| value + 1)?
        .register_member_function("score", |value: i64| value + 2)?
        .build()?;
    let program = env.compile("[score(3), 3.score()]")?;
    assert_eq!(program.evaluate(())?, vec![4i64, 5].into());
    let activation = Activation::new().bind_member_function("score", |value: i64| value + 10)?;
    assert_eq!(program.evaluate(activation)?, vec![4i64, 13].into());
    Ok(())
}

#[test]
fn test_duplicate_signature_rejects_different_return_and_rust_integer_types() -> Result<(), Error> {
    let mut registry = FunctionRegistry::new();
    registry.declare_global::<fn(i64) -> i64>("score")?;
    let error = registry
        .declare_global::<fn(i32) -> String>("score")
        .expect_err("Rust integer widths map to the same CEL argument type");
    assert_eq!(error.code(), Code::InvalidArgument);
    let error = registry
        .register_global("score", |value: i64| value)
        .expect_err("an implementation cannot duplicate an existing declaration");
    assert_eq!(error.code(), Code::InvalidArgument);
    assert_eq!(registry.find("score").unwrap().entries().count(), 1);

    let mut bindings = FunctionBindings::new();
    bindings.bind_global("score", |value: i64| value)?;
    let error = bindings
        .bind_global("score", |value: i32| value.to_string())
        .expect_err("return types cannot distinguish duplicate signatures");
    assert_eq!(error.code(), Code::InvalidArgument);
    assert_eq!(bindings.find("score").unwrap().entries().count(), 1);
    Ok(())
}

#[test]
fn test_registration_rejects_member_without_receiver() -> Result<(), Error> {
    let mut registry = FunctionRegistry::new();
    let error = registry
        .register("score", true, || 1i64)
        .expect_err("a registered member implementation needs a receiver");
    assert_eq!(error.code(), Code::InvalidArgument);
    let error = registry
        .declare::<fn() -> i64>("score", true)
        .expect_err("a member declaration needs a receiver");
    assert_eq!(error.code(), Code::InvalidArgument);
    registry.register_global("score", || 1i64)?;
    assert_eq!(registry.find("score").unwrap().entries().count(), 1);
    Ok(())
}

#[test]
fn test_binding_validates_arity_argument_type_and_call_style() -> Result<(), Error> {
    let env = Env::builder()
        .declare_global_function::<fn(i64) -> i64>("score")?
        .build()?;
    let program = env.compile("1")?;
    let cases = [
        (
            "missing argument",
            Activation::new().bind_global_function("score", || 0i64)?,
        ),
        (
            "extra argument",
            Activation::new().bind_global_function("score", |a: i64, b: i64| a + b)?,
        ),
        (
            "different argument type",
            Activation::new().bind_global_function("score", |_: String| 0i64)?,
        ),
        (
            "member call style",
            Activation::new().bind_member_function("score", |value: i64| value)?,
        ),
    ];
    for (case, activation) in cases {
        let error = program.evaluate(activation).expect_err(case);
        assert_eq!(error.code(), Code::InvalidArgument, "{case}");
        assert!(
            error.message().contains("no compile-time declaration"),
            "{case}: {error}"
        );
    }
    Ok(())
}

#[test]
fn test_checker_rejects_calls_with_wrong_arity_type_or_call_style() -> Result<(), Error> {
    let env = Env::builder()
        .register_global_function("score", |value: i64| value)?
        .build()?;
    for expression in ["score()", "score(1, 2)", "score('1')", "1.score()"] {
        let error = env.compile(expression).expect_err(expression);
        assert_eq!(error.code(), Code::InvalidArgument, "{expression}: {error}");
    }
    assert_eq!(env.compile("score(1)")?.evaluate(())?, Value::Int(1));
    Ok(())
}

#[test]
fn test_nested_collection_overloads_resolve_empty_and_populated_values() -> Result<(), Error> {
    type IntMaps = Vec<HashMap<String, i64>>;
    type StringMaps = Vec<HashMap<String, String>>;
    let env = Env::builder()
        .register_global_function("score", |_: IntMaps| 1i64)?
        .register_global_function("score", |_: StringMaps| 2i64)?
        .declare_variable::<IntMaps>("ints")?
        .declare_variable::<StringMaps>("strings")?
        .build()?;
    let program = env.compile("[score(ints), score(strings)]")?;
    for (ints, strings) in [
        (IntMaps::new(), StringMaps::new()),
        (vec![HashMap::new()], vec![HashMap::new()]),
        (
            vec![HashMap::from([("key".into(), 1i64)])],
            vec![HashMap::from([("key".into(), "value".into())])],
        ),
    ] {
        let activation = Activation::new()
            .bind_variable("ints", ints)?
            .bind_variable("strings", strings)?
            .bind_global_function("score", |_: IntMaps| 10i64)?;
        assert_eq!(program.evaluate(activation)?, vec![10i64, 2].into());
    }
    Ok(())
}

#[test]
fn test_function_binding_does_not_leak_between_evaluations() -> Result<(), Error> {
    let env = Env::builder()
        .register_global_function("score", |value: i64| value + 1)?
        .build()?;
    let program = env.compile("score(1)")?;
    let first = Activation::new().bind_global_function("score", |value: i64| value + 10)?;
    let second = Activation::new().bind_global_function("score", |value: i64| value + 20)?;

    for _ in 0..3 {
        assert_eq!(program.evaluate(&first)?, Value::Int(11));
        assert_eq!(program.evaluate(())?, Value::Int(2));
        assert_eq!(program.evaluate(&second)?, Value::Int(21));
        assert_eq!(program.evaluate(&first)?, Value::Int(11));
    }
    Ok(())
}

#[test]
fn test_missing_implementation_fails_only_when_called() -> Result<(), Error> {
    let env = Env::builder()
        .declare_global_function::<fn() -> i64>("score")?
        .declare_variable::<bool>("needed")?
        .build()?;
    let program = env.compile("needed ? score() : 7")?;
    let activation = Activation::new().bind_variable("needed", false)?;
    assert_eq!(program.evaluate(&activation)?, Value::Int(7));

    let activation = activation.bind_variable("needed", true)?;
    let error = program.evaluate(&activation).unwrap_err();
    assert_eq!(error.code(), Code::Unknown);
    assert!(error.message().contains("score"), "{error}");

    let activation = activation.bind_global_function("score", || 42i64)?;
    assert_eq!(program.evaluate(&activation)?, Value::Int(42));
    let unbound = Activation::new().bind_variable("needed", true)?;
    assert_eq!(program.evaluate(unbound).unwrap_err().code(), Code::Unknown);
    Ok(())
}
