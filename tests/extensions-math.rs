use cel_cxx::*;

#[test]
fn test_math_least() -> Result<(), Error> {
    println!("test math.least function");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<Vec<i64>>("numbers")?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<i64>("c")?
        .build()?;

    // Test math.least with list literal
    let program = env.compile("math.least([1, 2, 3])")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(1));

    // Test math.least with multiple arguments
    let program = env.compile("math.least(a, b, c)")?;
    let activation = Activation::new()
        .bind_variable("a", 5i64)?
        .bind_variable("b", 2i64)?
        .bind_variable("c", 8i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(2));

    // Test math.least with negative numbers
    let program = env.compile("math.least([-5, -2, -8])")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(-8));

    // Test math.least with single element
    let program = env.compile("math.least([42])")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(42));

    // Test math.least with single argument
    let program = env.compile("math.least(1)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(1));

    Ok(())
}

#[test]
fn test_math_greatest() -> Result<(), Error> {
    println!("test math.greatest function");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<Vec<i64>>("numbers")?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<i64>("c")?
        .build()?;

    // Test math.greatest with list literal
    let program = env.compile("math.greatest([1, 2, 3])")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(3));

    // Test math.greatest with multiple arguments
    let program = env.compile("math.greatest(a, b, c)")?;
    let activation = Activation::new()
        .bind_variable("a", 5i64)?
        .bind_variable("b", 2i64)?
        .bind_variable("c", 8i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(8));

    // Test math.greatest with negative numbers
    let program = env.compile("math.greatest([-5, -2, -8])")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(-2));

    // Test math.greatest with single element
    let program = env.compile("math.greatest([42])")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(42));

    // Test math.greatest with single argument
    let program = env.compile("math.greatest(1)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(1));

    Ok(())
}

#[test]
fn test_math_abs() -> Result<(), Error> {
    println!("test math.abs function");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<i64>("int_val")?
        .declare_variable::<f64>("double_val")?
        .declare_variable::<u64>("uint_val")?
        .build()?;

    // Test math.abs with positive integer
    let program = env.compile("math.abs(int_val)")?;
    let activation = Activation::new()
        .bind_variable("int_val", 42i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(42));

    // Test math.abs with negative integer
    let program = env.compile("math.abs(int_val)")?;
    let activation = Activation::new()
        .bind_variable("int_val", -42i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(42));

    // Test math.abs with zero
    let program = env.compile("math.abs(int_val)")?;
    let activation = Activation::new()
        .bind_variable("int_val", 0i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(0));

    // Test math.abs with positive double
    let program = env.compile("math.abs(double_val)")?;
    let activation = Activation::new()
        .bind_variable("double_val", 3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(3.5));

    // Test math.abs with negative double
    let program = env.compile("math.abs(double_val)")?;
    let activation = Activation::new()
        .bind_variable("double_val", -3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(3.5));

    // Test math.abs with unsigned integer
    let program = env.compile("math.abs(uint_val)")?;
    let activation = Activation::new()
        .bind_variable("uint_val", 42u64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Uint(42));

    Ok(())
}

#[test]
fn test_math_sign() -> Result<(), Error> {
    println!("test math.sign function");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<i64>("int_val")?
        .declare_variable::<f64>("double_val")?
        .declare_variable::<u64>("uint_val")?
        .build()?;

    // Test math.sign with positive integer
    let program = env.compile("math.sign(int_val)")?;
    let activation = Activation::new()
        .bind_variable("int_val", 42i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(1));

    // Test math.sign with negative integer
    let program = env.compile("math.sign(int_val)")?;
    let activation = Activation::new()
        .bind_variable("int_val", -42i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(-1));

    // Test math.sign with zero
    let program = env.compile("math.sign(int_val)")?;
    let activation = Activation::new()
        .bind_variable("int_val", 0i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(0));

    // Test math.sign with positive double
    let program = env.compile("math.sign(double_val)")?;
    let activation = Activation::new()
        .bind_variable("double_val", 3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(1.0));

    // Test math.sign with negative double
    let program = env.compile("math.sign(double_val)")?;
    let activation = Activation::new()
        .bind_variable("double_val", -3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(-1.0));

    // Test math.sign with zero double
    let program = env.compile("math.sign(double_val)")?;
    let activation = Activation::new()
        .bind_variable("double_val", 0.0)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(0.0));

    // Test math.sign with unsigned integer
    let program = env.compile("math.sign(uint_val)")?;
    let activation = Activation::new()
        .bind_variable("uint_val", 42u64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Uint(1));

    // Test math.sign with zero unsigned integer
    let program = env.compile("math.sign(uint_val)")?;
    let activation = Activation::new()
        .bind_variable("uint_val", 0u64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Uint(0));

    Ok(())
}

#[test]
fn test_math_rounding() -> Result<(), Error> {
    println!("test math rounding functions");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<f64>("value")?
        .build()?;

    // Test math.ceil
    let program = env.compile("math.ceil(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(4.0));

    let program = env.compile("math.ceil(value)")?;
    let activation = Activation::new()
        .bind_variable("value", -3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(-3.0));

    let program = env.compile("math.ceil(1.2)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(2.0));

    // Test math.floor
    let program = env.compile("math.floor(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(3.0));

    let program = env.compile("math.floor(value)")?;
    let activation = Activation::new()
        .bind_variable("value", -3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(-4.0));

    let program = env.compile("math.floor(1.2)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(1.0));

    // Test math.round (ties away from zero)
    let program = env.compile("math.round(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(4.0));

    let program = env.compile("math.round(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 3.64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(4.0));

    let program = env.compile("math.round(1.5)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(2.0));

    let program = env.compile("math.round(-1.5)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(-2.0));

    // Test math.trunc
    let program = env.compile("math.trunc(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(3.0));

    let program = env.compile("math.trunc(value)")?;
    let activation = Activation::new()
        .bind_variable("value", -3.5)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(-3.0));

    let program = env.compile("math.trunc(-1.3)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(-1.0));

    Ok(())
}

#[test]
fn test_math_bitwise() -> Result<(), Error> {
    println!("test math bitwise functions");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<i64>("shift")?
        .declare_variable::<u64>("ua")?
        .declare_variable::<u64>("ub")?
        .build()?;

    // Test math.bitAnd with integers
    let program = env.compile("math.bitAnd(a, b)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?  // 1100 in binary
        .bind_variable("b", 10i64)?; // 1010 in binary
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(8)); // 1000 in binary

    let program = env.compile("math.bitAnd(3, 5)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(1));

    // Test math.bitAnd with unsigned integers
    let program = env.compile("math.bitAnd(ua, ub)")?;
    let activation = Activation::new()
        .bind_variable("ua", 3u64)?
        .bind_variable("ub", 2u64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Uint(2));

    // Test math.bitOr with integers
    let program = env.compile("math.bitOr(a, b)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?  // 1100 in binary
        .bind_variable("b", 10i64)?; // 1010 in binary
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(14)); // 1110 in binary

    let program = env.compile("math.bitOr(1u, 2u)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Uint(3));

    // Test math.bitXor with integers
    let program = env.compile("math.bitXor(a, b)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?  // 1100 in binary
        .bind_variable("b", 10i64)?; // 1010 in binary
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(6)); // 0110 in binary

    let program = env.compile("math.bitXor(3u, 5u)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Uint(6));

    // Test math.bitNot
    let program = env.compile("math.bitNot(a)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?; // 1100 in binary
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(!12i64)); // bitwise NOT

    let program = env.compile("math.bitNot(1)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(-2));

    let program = env.compile("math.bitNot(0u)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Uint(!0u64));

    // Test math.bitShiftLeft
    let program = env.compile("math.bitShiftLeft(a, shift)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?    // 1100 in binary
        .bind_variable("shift", 2i64)?; // shift left by 2
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(48)); // 110000 in binary

    let program = env.compile("math.bitShiftLeft(1, 2)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(4));

    let program = env.compile("math.bitShiftLeft(1u, 2)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Uint(4));

    // Test math.bitShiftRight
    let program = env.compile("math.bitShiftRight(a, shift)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?    // 1100 in binary
        .bind_variable("shift", 2i64)?; // shift right by 2
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(3)); // 11 in binary

    let program = env.compile("math.bitShiftRight(1024, 2)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(256));

    let program = env.compile("math.bitShiftRight(1024u, 2)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Uint(256));

    Ok(())
}

#[test]
fn test_math_floating_point_helpers() -> Result<(), Error> {
    println!("test math floating point helper functions");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<f64>("value")?
        .build()?;

    // Test math.isFinite
    let program = env.compile("math.isFinite(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 1.2)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    let program = env.compile("math.isFinite(1.0/0.0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false)); // positive infinity

    let program = env.compile("math.isFinite(-1.0/0.0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false)); // negative infinity

    let program = env.compile("math.isFinite(0.0/0.0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false)); // NaN

    // Test math.isInf
    let program = env.compile("math.isInf(1.0/0.0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true)); // positive infinity

    let program = env.compile("math.isInf(-1.0/0.0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true)); // negative infinity

    let program = env.compile("math.isInf(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 1.2)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    let program = env.compile("math.isInf(0.0/0.0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false)); // NaN is not infinity

    // Test math.isNaN
    let program = env.compile("math.isNaN(0.0/0.0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true)); // NaN

    let program = env.compile("math.isNaN(1.0/0.0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false)); // infinity is not NaN

    let program = env.compile("math.isNaN(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 1.2)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    Ok(())
}

#[test]
fn test_math_sqrt() -> Result<(), Error> {
    println!("test math.sqrt function");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<f64>("double_val")?
        .declare_variable::<i64>("int_val")?
        .declare_variable::<u64>("uint_val")?
        .build()?;

    // Test math.sqrt with double
    let program = env.compile("math.sqrt(double_val)")?;
    let activation = Activation::new()
        .bind_variable("double_val", 81.0)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(9.0));

    let program = env.compile("math.sqrt(985.25)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    // Should be approximately 31.388692231439016
    if let Value::Double(val) = res {
        assert!((val - 31.388692231439016).abs() < 0.000000000000001);
    } else {
        panic!("Expected double value");
    }

    // Test math.sqrt with integer
    let program = env.compile("math.sqrt(int_val)")?;
    let activation = Activation::new()
        .bind_variable("int_val", 81i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(9.0));

    let program = env.compile("math.sqrt(0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(0.0));

    // Test math.sqrt with unsigned integer
    let program = env.compile("math.sqrt(uint_val)")?;
    let activation = Activation::new()
        .bind_variable("uint_val", 4u64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(2.0));

    // Test math.sqrt with negative number (should return NaN)
    let program = env.compile("math.sqrt(-15)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    if let Value::Double(val) = res {
        assert!(val.is_nan());
    } else {
        panic!("Expected double value");
    }

    Ok(())
}

#[test]
fn test_math_double_operations() -> Result<(), Error> {
    println!("test math operations with doubles");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<Vec<f64>>("double_values")?
        .declare_variable::<f64>("x")?
        .declare_variable::<f64>("y")?
        .build()?;

    // Test math.least with doubles
    let program = env.compile("math.least([3.5, 2.71, 1.41])")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(1.41));

    // Test math.greatest with doubles
    let program = env.compile("math.greatest([3.5, 2.71, 1.41])")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(3.5));

    // Test math.least with two doubles
    let program = env.compile("math.least(x, y)")?;
    let activation = Activation::new()
        .bind_variable("x", 3.5)?
        .bind_variable("y", 2.71)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(2.71));

    // Test math.greatest with two doubles
    let program = env.compile("math.greatest(x, y)")?;
    let activation = Activation::new()
        .bind_variable("x", 3.5)?
        .bind_variable("y", 2.71)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(3.5));

    // Test with mixed positive and negative doubles
    let program = env.compile("math.greatest(-42.0, -21.5, -100.0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(-21.5));

    let program = env.compile("math.least(-42.0, -21.5, -100.0)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(-100.0));

    Ok(())
}

#[test]
fn test_math_comprehensive_operations() -> Result<(), Error> {
    println!("test comprehensive math operations");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<Vec<i64>>("values")?
        .declare_variable::<f64>("score")?
        .declare_variable::<i64>("flags")?
        .declare_variable::<f64>("balance")?
        .build()?;

    // Range validation example
    let program = env.compile("math.least(values) >= 0 && math.greatest(values) <= 100")?;
    let activation = Activation::new()
        .bind_variable("values", vec![10i64, 50i64, 90i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Rounding and formatting example
    let program = env.compile("math.round(score * 100.0) / 100.0")?;
    let activation = Activation::new()
        .bind_variable("score", 85.6789)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(85.68));

    // Sign-based logic example
    let program = env.compile("math.sign(score) > 0.0 ? 'positive' : 'negative'")?;
    let activation = Activation::new()
        .bind_variable("score", 85.6789)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("positive".into()));

    // Bitwise flags example
    let program = env.compile("math.bitAnd(flags, 4) != 0")?; // Check if bit 2 is set
    let activation = Activation::new()
        .bind_variable("flags", 7i64)?; // 111 in binary
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Absolute value with comparison
    let program = env.compile("math.abs(score - 85.0) < 1.0")?;
    let activation = Activation::new()
        .bind_variable("score", 85.6789)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Complex expression with multiple math functions
    let program = env.compile("math.sqrt(math.abs(balance)) > 10.0 && math.isFinite(balance)")?;
    let activation = Activation::new()
        .bind_variable("balance", -150.0)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Statistical operations
    let program = env.compile("math.greatest(values) - math.least(values)")?; // range
    let activation = Activation::new()
        .bind_variable("values", vec![1i64, 5i64, 3i64, 9i64, 2i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(8)); // 9 - 1 = 8

    // Floating point validation
    let program = env.compile("math.isFinite(score) && !math.isNaN(score) && !math.isInf(score)")?;
    let activation = Activation::new()
        .bind_variable("score", 85.6789)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    Ok(())
} 