//! Numeric helpers, bit patterns, finite-value checks, and safe normalization.
//! Run with `cargo run --example extensions-math`.

use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("CEL math extension");
    let env = Env::builder()
        .with_ext_math(true)
        .with_ext_bindings(true)
        .declare_variable::<Vec<i64>>("values")?
        .declare_variable::<f64>("value")?
        .build()?;

    for (expression, expected) in [
        ("math.least(5, 3, 8, 1)", 1i64),
        ("math.least([5, 3, 8, 1])", 1),
        ("math.greatest(5, 3, 8, 1)", 8),
        ("math.greatest([5, 3, 8, 1])", 8),
        ("math.least(-5, -3, -8, -1)", -8),
        ("math.abs(-5)", 5),
        ("math.abs(-2147483648)", 2147483648),
        ("math.sign(-5)", -1),
        ("math.sign(0)", 0),
        ("math.sign(5)", 1),
        ("math.bitAnd(12, 10)", 8),
        ("math.bitOr(12, 10)", 14),
        ("math.bitXor(12, 10)", 6),
        ("math.bitNot(5)", -6),
        ("math.bitShiftLeft(5, 2)", 20),
        ("math.bitShiftRight(20, 2)", 5),
        // Right shift fills with zero bits, including for a negative signed input.
        ("math.bitShiftRight(-1, 1)", i64::MAX),
        ("math.bitShiftLeft(1, 63)", i64::MIN),
        ("math.bitShiftLeft(1, 64)", 0),
    ] {
        show(&env, expression, expected)?;
    }
    show(&env, "math.greatest(1u, 2u, 3u)", 3u64)?;
    show(&env, "math.abs(42u)", 42u64)?;
    show(&env, "math.bitOr(1u, 2u)", 3u64)?;

    for (expression, expected) in [
        ("math.least(3.14, 2.71, 1.41)", 1.41),
        ("math.abs(-3.5)", 3.5),
        ("math.sign(-3.5)", -1.0),
        ("math.ceil(-3.2)", -3.0),
        ("math.floor(-3.8)", -4.0),
        // Ties round away from zero, unlike banker's rounding.
        ("math.round(1.5)", 2.0),
        ("math.round(-1.5)", -2.0),
        ("math.trunc(-3.8)", -3.0),
        ("math.sqrt(81)", 9.0),
        ("math.sqrt(4u)", 2.0),
        ("math.sqrt(2.25)", 1.5),
    ] {
        show(&env, expression, expected)?;
    }
    for (expression, expected) in [
        ("math.isFinite(1.2)", true),
        ("math.isFinite(1.0 / 0.0)", false),
        ("math.isInf(-1.0 / 0.0)", true),
        ("math.isNaN(0.0 / 0.0)", true),
        ("math.isNaN(math.sqrt(-15))", true),
        (
            "math.bitAnd(6, 4) != 0 && math.bitAnd(6, 2) != 0 && math.bitAnd(6, 1) == 0",
            true,
        ),
    ] {
        show(&env, expression, expected)?;
    }

    // NaN and infinities should fail a finite range policy explicitly.
    let in_range = env.compile("math.isFinite(value) && math.abs(value - 10.0) <= 0.5")?;
    for (value, expected) in [
        (9.5, true),
        (10.5, true),
        (10.6, false),
        (f64::NAN, false),
        (f64::INFINITY, false),
    ] {
        let activation = Activation::new().bind_variable("value", value)?;
        assert_eq!(in_range.evaluate(&activation)?, Value::Bool(expected));
    }

    // Empty input has no min/max. A constant input maps to zero instead of dividing
    // by zero. Convert to double before subtraction to avoid signed integer overflow.
    let normalized = env.compile(
        "values.size() == 0 ? [] : \
         cel.bind(low, double(math.least(values)), \
         cel.bind(high, double(math.greatest(values)), \
         values.map(v, high == low ? 0.0 : math.round((double(v) - low) / (high - low) * 100.0))))",
    )?;
    for (values, expected) in [
        (
            vec![-10i64, 5, 15, -20, 30],
            vec![20.0, 50.0, 70.0, 0.0, 100.0],
        ),
        (vec![7, 7], vec![0.0, 0.0]),
        (vec![], vec![]),
        (vec![i64::MIN, i64::MAX], vec![0.0, 100.0]),
    ] {
        let activation = Activation::new().bind_variable("values", values)?;
        let result = normalized.evaluate(&activation)?;
        assert_eq!(result, Value::from(expected));
        println!("Normalized percentages: {result}");
    }

    for expression in [
        "math.abs(-9223372036854775808)",
        "math.bitShiftLeft(1, -1)",
        "math.bitShiftRight(1, -1)",
    ] {
        let error = env
            .compile(expression)?
            .evaluate(())
            .expect_err("invalid math operation must fail");
        assert_eq!(error.code(), Code::InvalidArgument);
        println!("Rejected {expression}: {error}");
    }
    // Use a declared list so the checker knows the element type even when it is empty.
    let empty = Activation::new().bind_variable("values", Vec::<i64>::new())?;
    let error = env
        .compile("math.least(values)")?
        .evaluate(&empty)
        .expect_err("an empty list has no minimum");
    assert_eq!(error.code(), Code::InvalidArgument);
    Ok(())
}

fn show(env: &Env, expression: &str, expected: impl Into<Value>) -> Result<(), Error> {
    let result = env.compile(expression)?.evaluate(Activation::new())?;
    assert_eq!(result, expected.into(), "{expression}");
    println!("{expression} => {result}");
    Ok(())
}
