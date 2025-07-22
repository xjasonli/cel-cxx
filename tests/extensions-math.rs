use cel_cxx::*;

#[test]
fn test_math_min() -> Result<(), Error> {
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

    Ok(())
}

#[test]
fn test_math_max() -> Result<(), Error> {
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

    Ok(())
}

#[test]
fn test_math_abs() -> Result<(), Error> {
    println!("test math.abs function");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<i64>("int_val")?
        .declare_variable::<f64>("double_val")?
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

    Ok(())
}

#[test]
fn test_math_sign() -> Result<(), Error> {
    println!("test math.sign function");

    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<i64>("int_val")?
        .declare_variable::<f64>("double_val")?
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

    // Test math.round
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
        .build()?;

    // Test math.bitAnd
    let program = env.compile("math.bitAnd(a, b)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?  // 1100 in binary
        .bind_variable("b", 10i64)?; // 1010 in binary
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(8)); // 1000 in binary

    // Test math.bitOr
    let program = env.compile("math.bitOr(a, b)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?  // 1100 in binary
        .bind_variable("b", 10i64)?; // 1010 in binary
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(14)); // 1110 in binary

    // Test math.bitXor
    let program = env.compile("math.bitXor(a, b)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?  // 1100 in binary
        .bind_variable("b", 10i64)?; // 1010 in binary
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(6)); // 0110 in binary

    // Test math.bitNot
    let program = env.compile("math.bitNot(a)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?; // 1100 in binary
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(!12i64)); // bitwise NOT

    // Test math.bitShiftLeft
    let program = env.compile("math.bitShiftLeft(a, shift)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?    // 1100 in binary
        .bind_variable("shift", 2i64)?; // shift left by 2
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(48)); // 110000 in binary

    // Test math.bitShiftRight
    let program = env.compile("math.bitShiftRight(a, shift)")?;
    let activation = Activation::new()
        .bind_variable("a", 12i64)?    // 1100 in binary
        .bind_variable("shift", 2i64)?; // shift right by 2
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(3)); // 11 in binary

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

    Ok(())
} 