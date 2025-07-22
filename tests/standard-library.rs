use std::collections::HashMap;
use cel_cxx::*;

#[test]
fn test_presence_macros() -> Result<(), Error> {
    println!("test presence macros");

    let env = Env::builder()
        .declare_variable::<HashMap<String, i64>>("entries")?
        .declare_variable::<Vec<i64>>("values")?
        .build()?;

    // Test has function
    let program = env.compile("has(entries.key)")?;
    let mut map = HashMap::new();
    map.insert("key".to_string(), 42);
    let activation = Activation::new()
        .bind_variable("entries", map)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test all function
    let program = env.compile("values.all(x, x > 0)")?;
    let activation = Activation::new()
        .bind_variable("values", vec![1i64, 2i64, 3i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test exists function
    let program = env.compile("values.exists(x, x == 2)")?;
    let activation = Activation::new()
        .bind_variable("values", vec![1i64, 2i64, 3i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test filter function
    let program = env.compile("values.filter(x, x > 1)")?;
    let activation = Activation::new()
        .bind_variable("values", vec![1i64, 2i64, 3i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![Value::Int(2), Value::Int(3)]));

    // Test map function
    let program = env.compile("values.map(x, x * 2)")?;
    let activation = Activation::new()
        .bind_variable("values", vec![1i64, 2i64, 3i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![Value::Int(2), Value::Int(4), Value::Int(6)]));

    Ok(())
}

#[test]
fn test_logical_operators() -> Result<(), Error> {
    println!("test logical operators");

    let env = Env::builder()
        .declare_variable::<bool>("a")?
        .declare_variable::<bool>("b")?
        .build()?;

    // Test logical NOT
    let program = env.compile("!a")?;
    let activation = Activation::new()
        .bind_variable("a", true)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    // Test logical AND
    let program = env.compile("a && b")?;
    let activation = Activation::new()
        .bind_variable("a", true)?
        .bind_variable("b", false)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    // Test logical OR
    let program = env.compile("a || b")?;
    let activation = Activation::new()
        .bind_variable("a", true)?
        .bind_variable("b", false)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test conditional operator
    let program = env.compile("a ? 'yes' : 'no'")?;
    let activation = Activation::new()
        .bind_variable("a", true)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("yes".into()));

    Ok(())
}

#[test]
fn test_arithmetic_operators() -> Result<(), Error> {
    println!("test arithmetic operators");

    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<f64>("x")?
        .declare_variable::<f64>("y")?
        .build()?;

    // Test negation
    let program = env.compile("-a")?;
    let activation = Activation::new()
        .bind_variable("a", 5i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(-5));

    // Test addition
    let program = env.compile("a + b")?;
    let activation = Activation::new()
        .bind_variable("a", 3i64)?
        .bind_variable("b", 4i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(7));

    // Test subtraction
    let program = env.compile("a - b")?;
    let activation = Activation::new()
        .bind_variable("a", 10i64)?
        .bind_variable("b", 3i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(7));

    // Test multiplication
    let program = env.compile("a * b")?;
    let activation = Activation::new()
        .bind_variable("a", 6i64)?
        .bind_variable("b", 7i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(42));

    // Test division
    let program = env.compile("x / y")?;
    let activation = Activation::new()
        .bind_variable("x", 10.0)?
        .bind_variable("y", 2.0)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(5.0));

    // Test modulo
    let program = env.compile("a % b")?;
    let activation = Activation::new()
        .bind_variable("a", 10i64)?
        .bind_variable("b", 3i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(1));

    Ok(())
}

#[test]
fn test_comparison_operators() -> Result<(), Error> {
    println!("test comparison operators");

    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;

    // Test equality
    let program = env.compile("a == b")?;
    let activation = Activation::new()
        .bind_variable("a", 5i64)?
        .bind_variable("b", 5i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test inequality
    let program = env.compile("a != b")?;
    let activation = Activation::new()
        .bind_variable("a", 5i64)?
        .bind_variable("b", 3i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test less than
    let program = env.compile("a < b")?;
    let activation = Activation::new()
        .bind_variable("a", 3i64)?
        .bind_variable("b", 5i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test less than or equal
    let program = env.compile("a <= b")?;
    let activation = Activation::new()
        .bind_variable("a", 5i64)?
        .bind_variable("b", 5i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test greater than
    let program = env.compile("a > b")?;
    let activation = Activation::new()
        .bind_variable("a", 7i64)?
        .bind_variable("b", 3i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test greater than or equal
    let program = env.compile("a >= b")?;
    let activation = Activation::new()
        .bind_variable("a", 5i64)?
        .bind_variable("b", 5i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    Ok(())
}

#[test]
fn test_list_operators() -> Result<(), Error> {
    println!("test list operators");

    let env = Env::builder()
        .declare_variable::<Vec<i64>>("values")?
        .declare_variable::<i64>("index")?
        .declare_variable::<i64>("value")?
        .build()?;

    // Test list indexing
    let program = env.compile("values[index]")?;
    let activation = Activation::new()
        .bind_variable("values", vec![10i64, 20i64, 30i64])?
        .bind_variable("index", 1i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(20));

    // Test list membership
    let program = env.compile("value in values")?;
    let activation = Activation::new()
        .bind_variable("values", vec![10i64, 20i64, 30i64])?
        .bind_variable("value", 20i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test list size
    let program = env.compile("size(values)")?;
    let activation = Activation::new()
        .bind_variable("values", vec![10i64, 20i64, 30i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(3));

    Ok(())
}

#[test]
fn test_map_operators() -> Result<(), Error> {
    println!("test map operators");

    let env = Env::builder()
        .declare_variable::<HashMap<String, i64>>("entries")?
        .declare_variable::<String>("key")?
        .build()?;

    // Test map indexing
    let program = env.compile("entries[key]")?;
    let mut map = HashMap::new();
    map.insert("test".to_string(), 42);
    let activation = Activation::new()
        .bind_variable("entries", map)?
        .bind_variable("key", "test".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(42));

    // Test map key membership
    let program = env.compile("key in entries")?;
    let mut map = HashMap::new();
    map.insert("test".to_string(), 42);
    let activation = Activation::new()
        .bind_variable("entries", map)?
        .bind_variable("key", "test".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test map size
    let program = env.compile("size(entries)")?;
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    let activation = Activation::new()
        .bind_variable("entries", map)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(2));

    Ok(())
}

#[test]
fn test_bytes_functions() -> Result<(), Error> {
    println!("test bytes functions");

    let env = Env::builder()
        .declare_variable::<Vec<u8>>("data")?
        .build()?;

    // Test bytes size
    let program = env.compile("size(data)")?;
    let activation = Activation::new()
        .bind_variable("data", vec![1u8, 2u8, 3u8, 4u8])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(4));

    Ok(())
}

#[test]
fn test_string_functions() -> Result<(), Error> {
    println!("test string functions");

    let env = Env::builder()
        .declare_variable::<String>("str")?
        .declare_variable::<String>("substr")?
        .declare_variable::<String>("pattern")?
        .build()?;

    // Test contains
    let program = env.compile("str.contains(substr)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("substr", "world".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test startsWith
    let program = env.compile("str.startsWith(substr)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("substr", "hello".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test endsWith
    let program = env.compile("str.endsWith(substr)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("substr", "world".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test matches
    let program = env.compile("str.matches(pattern)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello123".to_string())?
        .bind_variable("pattern", r"hello\d+".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test string size
    let program = env.compile("size(str)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(5));

    Ok(())
}

#[test]
fn test_type_conversions() -> Result<(), Error> {
    println!("test type conversions");

    let env = Env::builder()
        .declare_variable::<i64>("int_val")?
        .declare_variable::<f64>("double_val")?
        .declare_variable::<String>("str_val")?
        .declare_variable::<bool>("bool_val")?
        .build()?;

    // Test string conversion
    let program = env.compile("string(int_val)")?;
    let activation = Activation::new()
        .bind_variable("int_val", 42i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("42".into()));

    // Test int conversion
    let program = env.compile("int(str_val)")?;
    let activation = Activation::new()
        .bind_variable("str_val", "123".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(123));

    // Test double conversion
    let program = env.compile("double(str_val)")?;
    let activation = Activation::new()
        .bind_variable("str_val", "3.15".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Double(3.15));

    // Test bool conversion
    let program = env.compile("bool(str_val)")?;
    let activation = Activation::new()
        .bind_variable("str_val", "true".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test type function
    let program = env.compile("type(int_val)")?;
    let activation = Activation::new()
        .bind_variable("int_val", 42i64)?;
    let res = program.evaluate(&activation)?;
    // Note: exact type representation may vary
    assert!(matches!(res, Value::Type(_)));

    Ok(())
}

#[test]
fn test_duration_timestamp() -> Result<(), Error> {
    println!("test duration and timestamp");

    let env = Env::builder()
        .declare_variable::<String>("duration_str")?
        .declare_variable::<String>("timestamp_str")?
        .build()?;

    // Test duration conversion
    let program = env.compile("duration(duration_str)")?;
    let activation = Activation::new()
        .bind_variable("duration_str", "1h30m".to_string())?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Duration(_)));

    // Test timestamp conversion
    let program = env.compile("timestamp(timestamp_str)")?;
    let activation = Activation::new()
        .bind_variable("timestamp_str", "2023-01-01T12:00:00Z".to_string())?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Timestamp(_)));

    Ok(())
}

#[test]
fn test_comprehensive_expressions() -> Result<(), Error> {
    println!("test comprehensive expressions");

    let env = Env::builder()
        .declare_variable::<Vec<i64>>("numbers")?
        .declare_variable::<HashMap<String, String>>("data")?
        .build()?;

    // Complex expression combining multiple operators
    let program = env.compile(
        "numbers.filter(x, x > 0).map(x, x * 2).exists(x, x > 10)"
    )?;
    let activation = Activation::new()
        .bind_variable("numbers", vec![-1i64, 2i64, 5i64, 8i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Complex map and string operations
    let program = env.compile(
        "has(data.name) && data.name.startsWith('user_') && size(data.name) > 5"
    )?;
    let mut data = HashMap::new();
    data.insert("name".to_string(), "user_john".to_string());
    let activation = Activation::new()
        .bind_variable("data", data)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    Ok(())
}
