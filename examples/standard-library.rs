use std::collections::HashMap;
use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("🔧 CEL Standard Library Example");
    println!("===============================");
    
    // Build environment with standard library
    let env = Env::builder()
        .declare_variable::<HashMap<String, String>>("data")?
        .declare_variable::<Vec<i64>>("numbers")?
        .declare_variable::<Vec<String>>("words")?
        .declare_variable::<String>("text")?
        .declare_variable::<Vec<u8>>("bytes_data")?
        .build()?;

    // Demo 1: Presence and Comprehension Macros
    println!("\n📌 Demo 1: Presence and Comprehension Macros");
    demo_presence_macros(&env)?;
    
    // Demo 2: Logical Operators
    println!("\n📌 Demo 2: Logical Operators");
    demo_logical_operators(&env)?;
    
    // Demo 3: Arithmetic Operators
    println!("\n📌 Demo 3: Arithmetic Operators");
    demo_arithmetic_operators(&env)?;
    
    // Demo 4: Comparison Operators
    println!("\n📌 Demo 4: Comparison Operators");
    demo_comparison_operators(&env)?;
    
    // Demo 5: List Operations
    println!("\n📌 Demo 5: List Operations");
    demo_list_operations(&env)?;
    
    // Demo 6: Map Operations
    println!("\n📌 Demo 6: Map Operations");
    demo_map_operations(&env)?;
    
    // Demo 7: String Functions
    println!("\n📌 Demo 7: String Functions");
    demo_string_functions(&env)?;
    
    // Demo 8: Type Conversions
    println!("\n📌 Demo 8: Type Conversions");
    demo_type_conversions(&env)?;
    
    println!("\n✅ All CEL Standard Library demos completed!");
    Ok(())
}

fn demo_presence_macros(env: &Env) -> Result<(), Error> {
    println!("  Testing presence and comprehension macros");
    
    // Create test data
    let mut map = HashMap::new();
    map.insert("name".to_string(), "Alice".to_string());
    map.insert("age".to_string(), "30".to_string());
    map.insert("active".to_string(), "true".to_string());
    
    let activation = Activation::new()
        .bind_variable("data", map)?
        .bind_variable("numbers", vec![1i64, 2i64, 3i64, 4i64, 5i64])?;
    
    let test_cases = vec![
        ("has(data.name)", "Check if 'name' field exists"),
        ("has(data.missing)", "Check if 'missing' field exists"),
        ("numbers.all(x, x > 0)", "All numbers are positive"),
        ("numbers.all(x, x > 3)", "All numbers are greater than 3"),
        ("numbers.exists(x, x == 3)", "At least one number equals 3"),
        ("numbers.exists(x, x > 10)", "At least one number greater than 10"),
        ("numbers.exists_one(x, x == 3)", "Exactly one number equals 3"),
        ("numbers.exists_one(x, x > 3)", "Exactly one number greater than 3"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    // Filter and map examples
    let filter_program = env.compile("numbers.filter(x, x > 2)")?;
    let filter_result = filter_program.evaluate(&activation)?;
    println!("    Filter numbers > 2: `numbers.filter(x, x > 2)` -> {filter_result}");
    
    let map_program = env.compile("numbers.map(x, x * 2)")?;
    let map_result = map_program.evaluate(&activation)?;
    println!("    Map numbers * 2: `numbers.map(x, x * 2)` -> {map_result}");
    
    Ok(())
}

fn demo_logical_operators(env: &Env) -> Result<(), Error> {
    println!("  Testing logical operators");
    
    let test_cases = vec![
        ("true && false", "Logical AND"),
        ("true || false", "Logical OR"),
        ("!true", "Logical NOT"),
        ("true ? 'yes' : 'no'", "Conditional operator (true)"),
        ("false ? 'yes' : 'no'", "Conditional operator (false)"),
        ("true && (false || true)", "Complex logical expression"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_arithmetic_operators(env: &Env) -> Result<(), Error> {
    println!("  Testing arithmetic operators");
    
    let test_cases = vec![
        ("10 + 5", "Addition"),
        ("10 - 3", "Subtraction"),
        ("6 * 7", "Multiplication"),
        ("15 / 3", "Division"),
        ("17 % 5", "Modulo"),
        ("-42", "Negation"),
        ("2 + 3 * 4", "Operator precedence"),
        ("(2 + 3) * 4", "Parentheses override precedence"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_comparison_operators(env: &Env) -> Result<(), Error> {
    println!("  Testing comparison operators");
    
    let test_cases = vec![
        ("5 == 5", "Equality (true)"),
        ("5 == 3", "Equality (false)"),
        ("5 != 3", "Inequality (true)"),
        ("5 != 5", "Inequality (false)"),
        ("5 < 10", "Less than (true)"),
        ("10 < 5", "Less than (false)"),
        ("5 <= 5", "Less than or equal (true)"),
        ("5 <= 3", "Less than or equal (false)"),
        ("10 > 5", "Greater than (true)"),
        ("5 > 10", "Greater than (false)"),
        ("5 >= 5", "Greater than or equal (true)"),
        ("3 >= 5", "Greater than or equal (false)"),
        ("'apple' < 'banana'", "String comparison"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_list_operations(env: &Env) -> Result<(), Error> {
    println!("  Testing list operations");
    
    let activation = Activation::new()
        .bind_variable("numbers", vec![10i64, 20i64, 30i64, 40i64, 50i64])?;
    
    let test_cases = vec![
        ("numbers[0]", "List indexing (first element)"),
        ("numbers[2]", "List indexing (middle element)"),
        ("numbers[numbers.size() - 1]", "List indexing (last element)"),
        ("30 in numbers", "List membership (true)"),
        ("60 in numbers", "List membership (false)"),
        ("numbers.size()", "List size"),
        ("size(numbers)", "List size (function form)"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_map_operations(env: &Env) -> Result<(), Error> {
    println!("  Testing map operations");
    
    let mut map = HashMap::new();
    map.insert("name".to_string(), "Bob".to_string());
    map.insert("age".to_string(), "25".to_string());
    map.insert("city".to_string(), "New York".to_string());
    
    let activation = Activation::new()
        .bind_variable("data", map)?;
    
    let test_cases = vec![
        ("data['name']", "Map indexing with string key"),
        ("data.name", "Map field access"),
        ("data.age", "Map field access (integer value)"),
        ("'name' in data", "Map key membership (true)"),
        ("'country' in data", "Map key membership (false)"),
        ("data.size()", "Map size"),
        ("size(data)", "Map size (function form)"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_string_functions(env: &Env) -> Result<(), Error> {
    println!("  Testing string functions");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello, World!".to_string())?;
    
    let test_cases = vec![
        ("text.contains('World')", "String contains (true)"),
        ("text.contains('xyz')", "String contains (false)"),
        ("text.startsWith('Hello')", "String starts with (true)"),
        ("text.startsWith('Hi')", "String starts with (false)"),
        ("text.endsWith('World!')", "String ends with (true)"),
        ("text.endsWith('Universe')", "String ends with (false)"),
        ("text.matches('Hello.*')", "Regular expression match"),
        ("text.size()", "String size"),
        ("size(text)", "String size (function form)"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_type_conversions(env: &Env) -> Result<(), Error> {
    println!("  Testing type conversions");
    
    let test_cases = vec![
        ("int(42.7)", "Convert double to int"),
        ("uint(42)", "Convert int to uint"),
        ("double(42)", "Convert int to double"),
        ("string(42)", "Convert int to string"),
        ("bytes('hello')", "Convert string to bytes"),
        ("string(bytes('world'))", "Convert bytes to string"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
} 