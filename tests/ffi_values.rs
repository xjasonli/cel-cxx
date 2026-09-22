use cel_cxx::*;
use std::collections::HashMap;

#[derive(Default)]
struct ValueActivation {
    variables: VariableBindings<'static>,
    functions: FunctionBindings<'static>,
}

impl ValueActivation {
    fn new(value: Value) -> Result<Self, Error> {
        let mut activation = Self::default();
        activation
            .variables
            .bind_with_value("input", ValueType::Dyn, value)?;
        Ok(activation)
    }
}

impl ActivationInterface<'static> for ValueActivation {
    fn variables(&self) -> &VariableBindings<'static> {
        &self.variables
    }

    fn functions(&self) -> &FunctionBindings<'static> {
        &self.functions
    }
}

impl ActivationInterface<'static> for &ValueActivation {
    fn variables(&self) -> &VariableBindings<'static> {
        &self.variables
    }

    fn functions(&self) -> &FunctionBindings<'static> {
        &self.functions
    }
}

#[test]
fn scalar_integer_boundaries_round_trip() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable_with_type("input", ValueType::Dyn)?
        .build()?;
    let program = env.compile("input")?;
    for expected in [
        Value::Null,
        Value::Bool(false),
        Value::Bool(true),
        Value::Int(i64::MIN),
        Value::Int(i64::MIN + 1),
        Value::Int(-1),
        Value::Int(0),
        Value::Int(i64::MAX),
        Value::Uint(0),
        Value::Uint(i64::MAX as u64),
        Value::Uint(i64::MAX as u64 + 1),
        Value::Uint(u64::MAX),
    ] {
        let actual = program.evaluate(ValueActivation::new(expected.clone())?)?;
        assert_eq!(actual, expected);
    }
    Ok(())
}

#[test]
fn double_special_values_round_trip() -> Result<(), Error> {
    let env = Env::builder().declare_variable::<f64>("input")?.build()?;
    let program = env.compile("input")?;
    for expected in [
        0.0,
        -0.0,
        f64::from_bits(1),
        -f64::from_bits(1),
        f64::MIN_POSITIVE,
        f64::MIN,
        f64::MAX,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NAN,
    ] {
        let activation = Activation::new().bind_variable("input", expected)?;
        let Value::Double(actual) = program.evaluate(&activation)? else {
            panic!("expected double for {expected:?}");
        };
        if expected.is_nan() {
            assert!(actual.is_nan());
        } else {
            assert_eq!(actual.to_bits(), expected.to_bits(), "{expected:?}");
        }
    }
    Ok(())
}

#[test]
fn strings_preserve_unicode_nul_and_length() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<String>("input")?
        .build()?;
    let identity = env.compile("input")?;
    let size = env.compile("input.size()")?;
    for expected in [
        String::new(),
        "\0".to_owned(),
        "before\0after".to_owned(),
        "中文🙂e\u{301}\n\r\t".to_owned(),
        "🙂\0中".repeat(4096),
    ] {
        let activation = Activation::new().bind_variable("input", expected.clone())?;
        assert_eq!(
            identity.evaluate(&activation)?,
            Value::String(expected.clone().into())
        );
        assert_eq!(
            size.evaluate(&activation)?,
            Value::Int(expected.chars().count() as i64)
        );
    }
    Ok(())
}

#[test]
fn bytes_preserve_empty_nul_and_every_byte() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Vec<u8>>("input")?
        .build()?;
    let identity = env.compile("input")?;
    let size = env.compile("input.size()")?;
    for expected in [
        vec![],
        vec![0],
        vec![0, 255, 0, 128],
        (0..=255).collect(),
        vec![255; 16384],
    ] {
        let activation = Activation::new().bind_variable("input", expected.clone())?;
        assert_eq!(
            identity.evaluate(&activation)?,
            Value::Bytes(expected.clone().into())
        );
        assert_eq!(
            size.evaluate(&activation)?,
            Value::Int(expected.len() as i64)
        );
    }
    Ok(())
}

#[test]
fn heterogeneous_nested_collections_survive_evaluator_drop() -> Result<(), Error> {
    let expected = Value::List(vec![
        Value::Null,
        Value::List(vec![]),
        Value::Map(HashMap::new()),
        Value::Map(HashMap::from([(
            MapKey::String("nested\0key".into()),
            Value::List(vec![
                Value::Uint(u64::MAX),
                Value::String("🙂".into()),
                Value::Bytes(vec![0, 255].into()),
            ]),
        )])),
    ]);
    let actual = {
        let env = Env::builder()
            .declare_variable_with_type("input", ValueType::Dyn)?
            .build()?;
        let program = env.compile("input")?;
        program.evaluate(ValueActivation::new(expected.clone())?)?
    };
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn all_map_key_kinds_and_extremes_round_trip() -> Result<(), Error> {
    let keys = [
        MapKey::Bool(false),
        MapKey::Bool(true),
        MapKey::Int(i64::MIN),
        MapKey::Int(i64::MAX),
        MapKey::Uint(0),
        MapKey::Uint(u64::MAX),
        MapKey::String("".into()),
        MapKey::String("a\0b".into()),
        MapKey::String("中文🙂".into()),
    ];
    let expected = Value::Map(
        keys.into_iter()
            .enumerate()
            .map(|(index, key)| (key, Value::Int(index as i64)))
            .collect(),
    );
    let env = Env::builder()
        .declare_variable_with_type("input", ValueType::Dyn)?
        .build()?;
    let activation = ValueActivation::new(expected.clone())?;
    assert_eq!(env.compile("input")?.evaluate(&activation)?, expected);
    assert_eq!(
        env.compile("input.size()")?.evaluate(&activation)?,
        Value::Int(9)
    );
    for (expression, index) in [
        ("input[false]", 0),
        ("input[true]", 1),
        ("input[18446744073709551615u]", 5),
        ("input['a\\x00b']", 7),
    ] {
        assert_eq!(
            env.compile(expression)?.evaluate(&activation)?,
            Value::Int(index),
            "{expression}"
        );
    }
    Ok(())
}

#[test]
fn optional_none_null_empty_and_nested_values_remain_distinct() -> Result<(), Error> {
    let env = Env::builder()
        .with_optional(true)
        .declare_variable_with_type("input", ValueType::Dyn)?
        .build()?;
    let identity = env.compile("input")?;
    let has_value = env.compile("input.hasValue()")?;
    for expected in [
        Value::Optional(Optional::none()),
        Value::Optional(Optional::new(Value::Null)),
        Value::Optional(Optional::new(Value::List(vec![]))),
        Value::Optional(Optional::new(Value::Map(HashMap::new()))),
        Value::Optional(Optional::new(Value::Optional(Optional::none()))),
        Value::Optional(Optional::new(Value::Optional(Optional::new(Value::Int(0))))),
    ] {
        let Value::Optional(optional) = &expected else {
            unreachable!()
        };
        let present = optional.as_option().is_some();
        let activation = ValueActivation::new(expected.clone())?;
        assert_eq!(identity.evaluate(&activation)?, expected);
        assert_eq!(has_value.evaluate(&activation)?, Value::Bool(present));
    }
    Ok(())
}

#[test]
fn collection_errors_propagate_at_each_position_and_nesting_depth() -> Result<(), Error> {
    let expected = Error::permission_denied("nested value denied\0detail");
    let env = Env::builder()
        .with_optional(true)
        .declare_variable_with_type("input", ValueType::Dyn)?
        .build()?;
    let program = env.compile("input")?;
    let mut cases = Vec::new();
    for index in 0..3 {
        let mut values = vec![Value::Int(1), Value::Null, Value::Int(3)];
        values[index] = Value::Error(expected.clone());
        cases.push(Value::List(values));
    }
    let mut nested = Value::Error(expected.clone());
    for depth in 0..12 {
        nested = match depth % 3 {
            0 => Value::List(vec![Value::Int(0), nested, Value::Null]),
            1 => Value::Map(HashMap::from([(MapKey::String("nested".into()), nested)])),
            _ => Value::Optional(Optional::new(nested)),
        };
        cases.push(nested.clone());
    }
    for (case, value) in cases.into_iter().enumerate() {
        let error = program
            .evaluate(ValueActivation::new(value)?)
            .expect_err("conversion must retain the nested error");
        assert_eq!(error.code(), expected.code(), "case {case}");
        assert_eq!(error.message(), expected.message(), "case {case}");
    }
    Ok(())
}

#[test]
fn conversion_errors_follow_cel_short_circuit_rules() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable_with_type("input", ValueType::Dyn)?
        .build()?;
    let expected = Error::resource_exhausted("nested resource limit");
    let activation = ValueActivation::new(Value::List(vec![Value::Error(expected.clone())]))?;
    for (expression, expected) in [
        ("true || input == 0", Value::Bool(true)),
        ("input == 0 || true", Value::Bool(true)),
        ("false && input == 0", Value::Bool(false)),
        ("input == 0 && false", Value::Bool(false)),
        ("true ? 7 : input", Value::Int(7)),
        ("false ? input : 9", Value::Int(9)),
    ] {
        assert_eq!(
            env.compile(expression)?.evaluate(&activation)?,
            expected,
            "{expression}"
        );
    }
    for expression in [
        "false || input == 0",
        "true && input == 0",
        "true ? input : 7",
    ] {
        let error = env
            .compile(expression)?
            .evaluate(&activation)
            .expect_err(expression);
        assert_eq!(error.code(), expected.code(), "{expression}");
        assert_eq!(error.message(), expected.message(), "{expression}");
    }
    Ok(())
}
