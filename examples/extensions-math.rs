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

    // Demo 1: Greatest and Least Operations
    println!("\n📌 Demo 1: Greatest and Least Operations");
    demo_greatest_least(&env)?;
    
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
    
    // Demo 6: Floating Point Helpers
    println!("\n📌 Demo 6: Floating Point Helpers");
    demo_floating_point(&env)?;
    
    // Demo 7: Square Root
    println!("\n📌 Demo 7: Square Root");
    demo_sqrt(&env)?;
    
    // Demo 8: Complex Mathematical Scenarios
    println!("\n📌 Demo 8: Complex Mathematical Scenarios");
    demo_complex_scenarios(&env)?;
    
    println!("\n✅ All CEL Math Extensions demos completed!");
    Ok(())
}

fn demo_greatest_least(env: &Env) -> Result<(), Error> {
    println!("  Testing math.greatest and math.least functions");
    
    let test_cases = vec![
        ("math.least(5, 3, 8, 1)", "Find minimum of multiple numbers"),
        ("math.greatest(5, 3, 8, 1)", "Find maximum of multiple numbers"),
        ("math.least([5, 3, 8, 1])", "Find minimum of a list"),
        ("math.greatest([5, 3, 8, 1])", "Find maximum of a list"),
        ("math.least(-5, -3, -8, -1)", "Find minimum of negative numbers"),
        ("math.greatest(-5, -3, -8, -1)", "Find maximum of negative numbers"),
        ("math.least(3.14, 2.71, 1.41)", "Find minimum of doubles"),
        ("math.greatest(3.14, 2.71, 1.41)", "Find maximum of doubles"),
        ("math.least(1u, 2u, 3u)", "Find minimum of unsigned integers"),
        ("math.greatest(1u, 2u, 3u)", "Find maximum of unsigned integers"),
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
        ("math.abs(42u)", "Absolute value of unsigned integer"),
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
        ("math.sign(42u)", "Sign of unsigned integer"),
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
        ("math.round(1.5)", "Round 1.5 (ties away from zero)"),
        ("math.round(-1.5)", "Round -1.5 (ties away from zero)"),
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
        ("math.bitAnd(12, 10)", "Bitwise AND: 12 & 10 (1100 & 1010 = 1000)"),
        ("math.bitOr(12, 10)", "Bitwise OR: 12 | 10 (1100 | 1010 = 1110)"),
        ("math.bitXor(12, 10)", "Bitwise XOR: 12 ^ 10 (1100 ^ 1010 = 0110)"),
        ("math.bitNot(5)", "Bitwise NOT: ~5"),
        ("math.bitNot(1)", "Bitwise NOT: ~1"),
        ("math.bitShiftLeft(5, 2)", "Left shift: 5 << 2 (101 << 2 = 10100)"),
        ("math.bitShiftLeft(1, 3)", "Left shift: 1 << 3"),
        ("math.bitShiftRight(20, 2)", "Right shift: 20 >> 2 (10100 >> 2 = 101)"),
        ("math.bitShiftRight(1024, 2)", "Right shift: 1024 >> 2"),
        ("math.bitAnd(255, 15)", "Masking operation: 255 & 15"),
        ("math.bitOr(1u, 2u)", "Unsigned bitwise OR: 1u | 2u"),
        ("math.bitAnd(3u, 2u)", "Unsigned bitwise AND: 3u & 2u"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_floating_point(env: &Env) -> Result<(), Error> {
    println!("  Testing floating point helper functions");
    
    let test_cases = vec![
        ("math.isFinite(1.2)", "Check if 1.2 is finite"),
        ("math.isFinite(1.0/0.0)", "Check if positive infinity is finite"),
        ("math.isFinite(-1.0/0.0)", "Check if negative infinity is finite"),
        ("math.isFinite(0.0/0.0)", "Check if NaN is finite"),
        ("math.isInf(1.0/0.0)", "Check if positive infinity is infinite"),
        ("math.isInf(-1.0/0.0)", "Check if negative infinity is infinite"),
        ("math.isInf(1.2)", "Check if 1.2 is infinite"),
        ("math.isInf(0.0/0.0)", "Check if NaN is infinite"),
        ("math.isNaN(0.0/0.0)", "Check if NaN is NaN"),
        ("math.isNaN(1.0/0.0)", "Check if infinity is NaN"),
        ("math.isNaN(1.2)", "Check if 1.2 is NaN"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_sqrt(env: &Env) -> Result<(), Error> {
    println!("  Testing math.sqrt function");
    
    let test_cases = vec![
        ("math.sqrt(81)", "Square root of 81"),
        ("math.sqrt(0)", "Square root of 0"),
        ("math.sqrt(2)", "Square root of 2"),
        ("math.sqrt(4u)", "Square root of 4u (unsigned)"),
        ("math.sqrt(985.25)", "Square root of 985.25"),
        ("math.sqrt(-15)", "Square root of -15 (should return NaN)"),
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
    
    // Statistical operations using greatest/least
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
    
    // Rounding and formatting with square root
    println!("\n  Rounding and formatting example:");
    let rounding_expr = r#"
        {
          "original": price,
          "rounded": math.round(price),
          "ceiling": math.ceil(price),
          "floor": math.floor(price),
          "truncated": math.trunc(price),
          "sqrt": math.sqrt(price)
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
          "all_permissions": math.bitAnd(permissions, 7) == 7,
          "shifted_left": math.bitShiftLeft(permissions, 1),
          "shifted_right": math.bitShiftRight(permissions, 1)
        }
    "#;
    let program = env.compile(flags_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("permissions", 6i64)?; // 110 in binary (read + write)
    let result = program.evaluate(&activation)?;
    println!("    Permission flags for value 6 (110 binary): {result}");
    
    // Sign-based logic with floating point validation
    println!("\n  Sign-based logic with floating point validation example:");
    let sign_expr = r#"
        {
          "balance": balance,
          "status": math.sign(balance) == 1.0 ? "positive" : 
                   math.sign(balance) == -1.0 ? "negative" : "zero",
          "abs_balance": math.abs(balance),
          "needs_attention": math.abs(balance) > 1000.0,
          "is_finite": math.isFinite(balance),
          "sqrt_abs": balance >= 0.0 ? math.sqrt(balance) : math.sqrt(math.abs(balance))
        }
    "#;
    let program = env.compile(sign_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("balance", -1500.0)?;
    let result = program.evaluate(&activation)?;
    println!("    Account analysis for balance -1500.0: {result}");
    
    // Complex mathematical expression using multiple functions
    println!("\n  Complex mathematical expression example:");
    let complex_expr = r#"
        {
          "input_values": values,
          "normalized": values.map(v, math.round((double(v) - double(math.least(values))) / (double(math.greatest(values)) - double(math.least(values))) * 100.0)),
          "outliers": values.filter(v, double(math.abs(v)) > math.sqrt(double(math.greatest(math.abs(math.least(values)), math.abs(math.greatest(values)))))),
          "signs": values.map(v, math.sign(v))
        }
    "#;
    let program = env.compile(complex_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("values", vec![-10i64, 5i64, 15i64, -20i64, 30i64])?;
    let result = program.evaluate(&activation)?;
    println!("    Complex analysis for [-10, 5, 15, -20, 30]: {result}");
    
    Ok(())
} 