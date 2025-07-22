use cel_cxx::*;

#[test]
fn test_cel_bind_basic() -> Result<(), Error> {
    println!("test cel.bind basic functionality");

    let env = Env::builder()
        .with_ext_bindings(true)
        .declare_variable::<i64>("x")?
        .declare_variable::<i64>("y")?
        .build()?;

    // Test basic binding
    let program = env.compile("cel.bind(temp, x + y, temp * 2)")?;
    let activation = Activation::new()
        .bind_variable("x", 3i64)?
        .bind_variable("y", 4i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(14)); // (3 + 4) * 2 = 14

    Ok(())
}

#[test]
fn test_cel_bind_complex_expression() -> Result<(), Error> {
    println!("test cel.bind with complex expressions");

    let env = Env::builder()
        .with_ext_bindings(true)
        .declare_variable::<Vec<i64>>("numbers")?
        .build()?;

    // Test binding with list operations (simplified without fold)
    let program = env.compile("cel.bind(filtered, numbers.filter(x, x > 0), cel.bind(sum, filtered.map(x, x * x), size(sum) > 0 ? sum[0] : 0))")?;
    let activation = Activation::new()
        .bind_variable("numbers", vec![-2i64, 1i64, 3i64, -1i64, 4i64, 2i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(1)); // First squared positive number: 1^2 = 1

    Ok(())
}

#[test]
fn test_cel_bind_nested() -> Result<(), Error> {
    println!("test cel.bind with nested bindings");

    let env = Env::builder()
        .with_ext_bindings(true)
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<i64>("c")?
        .build()?;

    // Test nested bindings
    let program = env.compile("cel.bind(sum, a + b, cel.bind(product, sum * c, product + sum))")?;
    let activation = Activation::new()
        .bind_variable("a", 2i64)?
        .bind_variable("b", 3i64)?
        .bind_variable("c", 4i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(25)); // sum = 2+3 = 5, product = 5*4 = 20, result = 20+5 = 25

    Ok(())
}

#[test]
fn test_cel_bind_string_operations() -> Result<(), Error> {
    println!("test cel.bind with string operations");

    let env = Env::builder()
        .with_ext_bindings(true)
        .declare_variable::<String>("first_name")?
        .declare_variable::<String>("last_name")?
        .build()?;

    // Test string binding
    let program = env.compile("cel.bind(full_name, first_name + ' ' + last_name, 'Hello, ' + full_name + '!')")?;
    let activation = Activation::new()
        .bind_variable("first_name", "John".to_string())?
        .bind_variable("last_name", "Doe".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("Hello, John Doe!".into()));

    Ok(())
}

#[test]
fn test_cel_bind_conditional() -> Result<(), Error> {
    println!("test cel.bind with conditional logic");

    let env = Env::builder()
        .with_ext_bindings(true)
        .declare_variable::<i64>("score")?
        .build()?;

    // Test conditional binding
    let program = env.compile("cel.bind(grade, score >= 90 ? 'A' : score >= 80 ? 'B' : score >= 70 ? 'C' : 'F', 'Your grade is: ' + grade)")?;
    let activation = Activation::new()
        .bind_variable("score", 85i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("Your grade is: B".into()));

    Ok(())
}

#[test]
fn test_cel_bind_comprehensive() -> Result<(), Error> {
    println!("test comprehensive cel.bind operations");

    let env = Env::builder()
        .with_ext_bindings(true)
        .declare_variable::<Vec<i64>>("values")?
        .declare_variable::<i64>("threshold")?
        .build()?;

    // Test complex data processing with bindings
    let program = env.compile(
        "cel.bind(valid_values, values.filter(x, x > 0), cel.bind(above_threshold, valid_values.filter(x, x > threshold), cel.bind(count, size(above_threshold), cel.bind(percentage, count * 100 / size(valid_values), 'Found ' + string(count) + ' values above threshold (' + string(percentage) + '%)'))))"
    )?;
    let activation = Activation::new()
        .bind_variable("values", vec![-1i64, 2i64, 5i64, 8i64, 3i64, 12i64, -2i64, 7i64])?
        .bind_variable("threshold", 5i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("Found 3 values above threshold (50%)".into()));

    Ok(())
} 