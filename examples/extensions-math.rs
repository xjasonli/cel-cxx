use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("🔧 CEL Math Extensions Example");
    println!("==============================");
    
    // Build environment with math extensions enabled
    let env = Env::builder()
        .with_ext_math(true)
        .declare_variable::<f64>("value")?
        .declare_variable::<f64>("target")?
        .declare_variable::<f64>("tolerance")?
        .declare_variable::<Vec<i64>>("values")?
        .declare_variable::<f64>("price")?
        .declare_variable::<i64>("permissions")?
        .declare_variable::<f64>("balance")?
        .build()?;

    // Demo 1: Min and Max Operations
    println!("\n📌 Demo 1: Min and Max Operations");
    demo_min_max(&env)?;
    
    // Demo 2: Absolute Value
    println!("\n📌 Demo 2: Absolute Value");
    demo_abs(&env)?;
    
    // Demo 3: Sign Function
    println!("\n📌 Demo 3: Sign Function");
    demo_sign(&env)?;
    
    // Demo 4: Rounding Functions
    println!("\n📌 Demo 4: Rounding Functions");
    demo_rounding(&env)?;
    
    // Demo 5: Bitwise Operations
    println!("\n📌 Demo 5: Bitwise Operations");
    demo_bitwise(&env)?;
    
    // Demo 6: Complex Mathematical Scenarios
    println!("\n📌 Demo 6: Complex Mathematical Scenarios");
    demo_complex_scenarios(&env)?;
    
    println!("\n✅ All CEL Math Extensions demos completed!");
    Ok(())
}

fn demo_min_max(env: &Env) -> Result<(), Error> {
    println!("  Testing math.least and math.greatest functions");
    
    let test_cases = vec![
        ("math.least(5, 3, 8, 1)", "Find minimum of multiple numbers"),
        ("math.greatest(5, 3, 8, 1)", "Find maximum of multiple numbers"),
        ("math.least([5, 3, 8, 1])", "Find minimum of a list"),
        ("math.greatest([5, 3, 8, 1])", "Find maximum of a list"),
        ("math.least(-5, -3, -8, -1)", "Find minimum of negative numbers"),
        ("math.greatest(-5, -3, -8, -1)", "Find maximum of negative numbers"),
        ("math.least(3.14, 2.71, 1.41)", "Find minimum of doubles"),
        ("math.greatest(3.14, 2.71, 1.41)", "Find maximum of doubles"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_abs(env: &Env) -> Result<(), Error> {
    println!("  Testing math.abs function");
    
    let test_cases = vec![
        ("math.abs(5)", "Absolute value of positive integer"),
        ("math.abs(-5)", "Absolute value of negative integer"),
        ("math.abs(0)", "Absolute value of zero"),
        ("math.abs(3.14)", "Absolute value of positive double"),
        ("math.abs(-3.14)", "Absolute value of negative double"),
        ("math.abs(-2147483648)", "Absolute value of large negative number"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_sign(env: &Env) -> Result<(), Error> {
    println!("  Testing math.sign function");
    
    let test_cases = vec![
        ("math.sign(5)", "Sign of positive integer"),
        ("math.sign(-5)", "Sign of negative integer"),
        ("math.sign(0)", "Sign of zero"),
        ("math.sign(3.14)", "Sign of positive double"),
        ("math.sign(-3.14)", "Sign of negative double"),
        ("math.sign(0.0)", "Sign of zero double"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_rounding(env: &Env) -> Result<(), Error> {
    println!("  Testing rounding functions");
    
    let test_cases = vec![
        ("math.ceil(3.2)", "Ceiling of 3.2"),
        ("math.ceil(-3.2)", "Ceiling of -3.2"),
        ("math.floor(3.8)", "Floor of 3.8"),
        ("math.floor(-3.8)", "Floor of -3.8"),
        ("math.round(3.2)", "Round 3.2"),
        ("math.round(3.7)", "Round 3.7"),
        ("math.round(-3.2)", "Round -3.2"),
        ("math.round(-3.7)", "Round -3.7"),
        ("math.trunc(3.8)", "Truncate 3.8"),
        ("math.trunc(-3.8)", "Truncate -3.8"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_bitwise(env: &Env) -> Result<(), Error> {
    println!("  Testing bitwise operations");
    
    let test_cases = vec![
        ("math.bitAnd(12, 10)", "Bitwise AND: 12 & 10"),
        ("math.bitOr(12, 10)", "Bitwise OR: 12 | 10"),
        ("math.bitXor(12, 10)", "Bitwise XOR: 12 ^ 10"),
        ("math.bitNot(5)", "Bitwise NOT: ~5"),
        ("math.bitShiftLeft(5, 2)", "Left shift: 5 << 2"),
        ("math.bitShiftRight(20, 2)", "Right shift: 20 >> 2"),
        ("math.bitAnd(255, 15)", "Masking operation: 255 & 15"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_complex_scenarios(env: &Env) -> Result<(), Error> {
    println!("  Testing complex mathematical scenarios");
    
    // Range validation
    println!("  Range validation example:");
    let range_expr = r#"
        math.abs(value - target) <= tolerance ? "within range" : "out of range"
    "#;
    let program = env.compile(range_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("value", 10.5)?
        .bind_variable("target", 10.0)?
        .bind_variable("tolerance", 1.0)?;
    let result = program.evaluate(&activation)?;
    println!("    Range validation (value=10.5, target=10.0, tolerance=1.0): {result}");
    
    // Statistical operations
    println!("\n  Statistical operations example:");
    let stats_expr = r#"
        {
          "min": math.least(values),
          "max": math.greatest(values),
          "range": math.greatest(values) - math.least(values),
          "abs_max": math.greatest(math.abs(math.least(values)), math.abs(math.greatest(values)))
        }
    "#;
    let program = env.compile(stats_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("values", vec![-5i64, 3i64, -8i64, 12i64, 0i64])?;
    let result = program.evaluate(&activation)?;
    println!("    Statistics for [-5, 3, -8, 12, 0]: {result}");
    
    // Rounding and formatting
    println!("\n  Rounding and formatting example:");
    let rounding_expr = r#"
        {
          "original": price,
          "rounded": math.round(price),
          "ceiling": math.ceil(price),
          "floor": math.floor(price),
          "truncated": math.trunc(price)
        }
    "#;
    let program = env.compile(rounding_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("price", 29.87)?;
    let result = program.evaluate(&activation)?;
    println!("    Rounding operations for price 29.87: {result}");
    
    // Bitwise flags
    println!("\n  Bitwise flags example:");
    let flags_expr = r#"
        {
          "read_permission": math.bitAnd(permissions, 4) != 0,
          "write_permission": math.bitAnd(permissions, 2) != 0,
          "execute_permission": math.bitAnd(permissions, 1) != 0,
          "all_permissions": math.bitAnd(permissions, 7) == 7
        }
    "#;
    let program = env.compile(flags_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("permissions", 6i64)?; // 110 in binary (read + write)
    let result = program.evaluate(&activation)?;
    println!("    Permission flags for value 6 (110 binary): {result}");
    
    // Sign-based logic
    println!("\n  Sign-based logic example:");
    let sign_expr = r#"
        {
          "balance": balance,
          "status": math.sign(balance) == 1.0 ? "positive" : 
                   math.sign(balance) == -1.0 ? "negative" : "zero",
          "abs_balance": math.abs(balance),
          "needs_attention": math.abs(balance) > 1000.0
        }
    "#;
    let program = env.compile(sign_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("balance", -1500.0)?;
    let result = program.evaluate(&activation)?;
    println!("    Account analysis for balance -1500.0: {result}");
    
    Ok(())
} 