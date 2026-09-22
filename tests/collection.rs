use cel_cxx::*;

#[test]
fn test_bound_collection_errors_are_propagated() -> Result<(), Error> {
    use std::collections::HashMap;

    let env = Env::builder()
        .declare_variable_with_type("input", ValueType::Dyn)?
        .build()?;
    let program = env.compile("input")?;
    let expected = Error::invalid_argument("invalid collection element");

    let activations = [
        (
            "list",
            Activation::new().bind_variable("input", vec![expected.clone()])?,
        ),
        (
            "map",
            Activation::new().bind_variable("input", HashMap::from([(1i64, expected.clone())]))?,
        ),
        (
            "optional",
            Activation::new().bind_variable("input", Optional::new(expected.clone()))?,
        ),
        (
            "nested map in list",
            Activation::new()
                .bind_variable("input", vec![HashMap::from([(1i64, expected.clone())])])?,
        ),
        (
            "nested list in optional",
            Activation::new().bind_variable("input", Optional::new(vec![expected.clone()]))?,
        ),
    ];

    for (case, activation) in activations {
        let error = program.evaluate(&activation).expect_err(case);
        assert_eq!(error.code(), expected.code(), "{case}");
        assert_eq!(error.message(), expected.message(), "{case}");
    }
    Ok(())
}

#[test]
fn test_bound_collection_propagates_message_parse_failure() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Vec<StructValue>>("input")?
        .build()?;
    let activation = Activation::new().bind_variable(
        "input",
        vec![StructValue::from_bytes("google.protobuf.Duration", [0x80])],
    )?;

    let error = env
        .compile("input.size()")?
        .evaluate(&activation)
        .expect_err("invalid message must not be dropped from the list");
    assert_eq!(error.code(), Code::InvalidArgument);
    assert!(error.message().contains("Failed to parse bytes as"));

    assert_eq!(
        env.compile("input.size() == 0 || true")?
            .evaluate(&activation)?,
        Value::Bool(true),
    );
    Ok(())
}

#[test]
fn test_map_function() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;

    let expr = "{1:2, 2:4, 3:6, 4:8, 5:10}.map(m, m)";
    let program = env.compile(expr)?;

    let value = program.evaluate(())?;

    let Value::List(list) = value else {
        return Err(Error::invalid_argument("expected list".to_string()));
    };

    let mapres: Result<Vec<i64>, _> = list
        .into_iter()
        .map(|v| {
            i64::try_from(v).map_err(|_| Error::invalid_argument("invalid vec i64".to_string()))
        })
        .collect();
    let mut keys = mapres?;
    keys.sort_unstable();
    assert_eq!(keys, vec![1, 2, 3, 4, 5]);

    Ok(())
}

#[cfg(feature = "derive")]
#[test]
fn test_hand_map_function() -> Result<(), Error> {
    use std::collections::HashMap;
    const MY_NAMESPACE: &str = "testing";

    #[derive(Opaque, Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, "TestOpaque"))]
    #[cel_cxx(display)]
    struct TestOpaque(i64);

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

    let Value::List(list) = res else {
        return Err(Error::invalid_argument("expected list".to_string()));
    };

    let mapres: Result<Vec<i64>, _> = list
        .into_iter()
        .map(|v| {
            i64::try_from(v).map_err(|_| Error::invalid_argument("invalid vec i64".to_string()))
        })
        .collect();
    let mut values = mapres?;
    values.sort_unstable();
    assert_eq!(values, vec![1, 2, 3]);

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
    let mut lists = mapres?;
    assert_eq!(lists.len(), 1);
    lists[0].sort_unstable();
    assert_eq!(lists, vec![vec![4, 5]]);

    Ok(())
}

#[test]
fn test_empty_collection_comprehensions() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Vec<i64>>("items")?
        .declare_variable::<std::collections::HashMap<String, i64>>("entries")?
        .build()?;
    let activation = Activation::new()
        .bind_variable("items", Vec::<i64>::new())?
        .bind_variable("entries", std::collections::HashMap::<String, i64>::new())?;

    for (expression, expected) in [
        ("items.all(x, x > 0)", Value::Bool(true)),
        ("items.exists(x, x > 0)", Value::Bool(false)),
        ("items.exists_one(x, x > 0)", Value::Bool(false)),
        ("items.map(x, x + 1)", Value::List(vec![])),
        ("items.filter(x, x > 0)", Value::List(vec![])),
        ("entries.map(k, entries[k])", Value::List(vec![])),
        ("entries.all(k, entries[k] > 0)", Value::Bool(true)),
        ("'absent' in entries", Value::Bool(false)),
    ] {
        assert_eq!(
            env.compile(expression)?.evaluate(&activation)?,
            expected,
            "{expression}"
        );
    }
    Ok(())
}

#[test]
fn test_list_index_boundaries() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Vec<i64>>("items")?
        .declare_variable::<i64>("index")?
        .build()?;
    let program = env.compile("items[index]")?;
    for items in [vec![], vec![i64::MIN, 0, i64::MAX]] {
        for index in [i64::MIN, -1, 0, 1, 2, 3, i64::MAX] {
            let activation = Activation::new()
                .bind_variable("items", items.clone())?
                .bind_variable("index", index)?;
            let result = program.evaluate(&activation);
            if let Some(expected) = usize::try_from(index)
                .ok()
                .and_then(|index| items.get(index))
            {
                assert_eq!(result?, Value::Int(*expected));
            } else {
                assert!(result.is_err(), "items={items:?}, index={index}");
            }
        }
    }
    Ok(())
}

#[test]
fn test_missing_map_key_and_present_zero() -> Result<(), Error> {
    use std::collections::HashMap;

    let env = Env::builder()
        .declare_variable::<HashMap<String, i64>>("entries")?
        .build()?;
    let activation =
        Activation::new().bind_variable("entries", HashMap::from([("present", 0i64)]))?;
    for (expression, expected) in [
        ("'present' in entries", Value::Bool(true)),
        ("'missing' in entries", Value::Bool(false)),
        ("has(entries.present)", Value::Bool(true)),
        ("has(entries.missing)", Value::Bool(false)),
        ("entries['present']", Value::Int(0)),
    ] {
        assert_eq!(
            env.compile(expression)?.evaluate(&activation)?,
            expected,
            "{expression}"
        );
    }
    assert!(env
        .compile("entries['missing']")?
        .evaluate(&activation)
        .is_err());
    Ok(())
}

#[test]
fn test_nested_collection_comprehensions_preserve_order_and_duplicates() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Vec<Vec<i64>>>("items")?
        .build()?;
    let activation =
        Activation::new().bind_variable("items", vec![vec![], vec![3i64, 1, 3], vec![0]])?;
    assert_eq!(
        env.compile("items.map(xs, xs.filter(x, x > 0))")?
            .evaluate(&activation)?,
        Value::List(vec![
            Value::List(vec![]),
            Value::List(vec![Value::Int(3), Value::Int(1), Value::Int(3)]),
            Value::List(vec![]),
        ]),
    );
    assert_eq!(
        env.compile("items.map(xs, xs.size())")?
            .evaluate(&activation)?,
        vec![0i64, 3, 1].into_value()
    );
    Ok(())
}
