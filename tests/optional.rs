use std::collections::HashMap;
use cel_cxx::*;

#[test]
fn test_optional_field_selection() -> Result<(), Error> {
    println!("test optional field selection");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<HashMap<String, String>>("msg")?
        .declare_variable::<HashMap<String, HashMap<String, HashMap<String, String>>>>("data")?
        .build()?;

    // Test optional field selection - field exists
    let program = env.compile("msg.?field")?;
    let mut msg = HashMap::new();
    msg.insert("field".to_string(), "value".to_string());
    let activation = Activation::new()
        .bind_variable("msg", msg)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optional field selection - field doesn't exist
    let program = env.compile("msg.?missing")?;
    let msg: HashMap<String, String> = HashMap::new();
    let activation = Activation::new()
        .bind_variable("msg", msg)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test chained optional field selection
    let program = env.compile("data.?user.?profile.?name")?;
    let mut profile = HashMap::new();
    profile.insert("name".to_string(), "John".to_string());
    let mut user = HashMap::new();
    user.insert("profile".to_string(), profile);
    let mut data = HashMap::new();
    data.insert("user".to_string(), user);
    let activation = Activation::new()
        .bind_variable("data", data)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    Ok(())
}

#[test]
fn test_optional_indexing() -> Result<(), Error> {
    println!("test optional indexing");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<HashMap<String, i64>>("entries")?
        .declare_variable::<Vec<i64>>("values")?
        .build()?;

    // Test optional map indexing - key exists
    let program = env.compile("entries[?'key']")?;
    let mut map = HashMap::new();
    map.insert("key".to_string(), 42);
    let activation = Activation::new()
        .bind_variable("entries", map)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optional map indexing - key doesn't exist
    let program = env.compile("entries[?'missing']")?;
    let map: HashMap<String, i64> = HashMap::new();
    let activation = Activation::new()
        .bind_variable("entries", map)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optional list indexing - valid index
    let program = env.compile("values[?1]")?;
    let activation = Activation::new()
        .bind_variable("values", vec![10i64, 20i64, 30i64])?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optional list indexing - invalid index
    let program = env.compile("values[?10]")?;
    let activation = Activation::new()
        .bind_variable("values", vec![10i64, 20i64, 30i64])?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    Ok(())
}

#[test]
fn test_optional_creation_functions() -> Result<(), Error> {
    println!("test optional creation functions");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<i64>("value")?
        .declare_variable::<String>("str")?
        .build()?;

    // Test optional.of()
    let program = env.compile("optional.of(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 42i64)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optional.ofNonZeroValue() with non-zero value
    let program = env.compile("optional.ofNonZeroValue(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 42i64)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optional.ofNonZeroValue() with zero value
    let program = env.compile("optional.ofNonZeroValue(value)")?;
    let activation = Activation::new()
        .bind_variable("value", 0i64)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optional.ofNonZeroValue() with empty string
    let program = env.compile("optional.ofNonZeroValue(str)")?;
    let activation = Activation::new()
        .bind_variable("str", "".to_string())?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optional.none()
    let program = env.compile("optional.none()")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    Ok(())
}

#[test]
fn test_optional_access_functions() -> Result<(), Error> {
    println!("test optional access functions");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<i64>("value")?
        .build()?;

    // Test hasValue() on optional with value
    let program = env.compile("optional.of(value).hasValue()")?;
    let activation = Activation::new()
        .bind_variable("value", 42i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test hasValue() on optional.none()
    let program = env.compile("optional.none().hasValue()")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    // Test value() on optional with value
    let program = env.compile("optional.of(value).value()")?;
    let activation = Activation::new()
        .bind_variable("value", 42i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(42));

    // Note: Testing value() on optional.none() would cause a runtime error
    // This is by design - value() should only be called after checking hasValue()

    Ok(())
}

#[test]
fn test_optional_chaining_functions() -> Result<(), Error> {
    println!("test optional chaining functions");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<i64>("value1")?
        .declare_variable::<i64>("value2")?
        .declare_variable::<i64>("default")?
        .build()?;

    // Test or() with first optional having value
    let program = env.compile("optional.of(value1).or(optional.of(value2))")?;
    let activation = Activation::new()
        .bind_variable("value1", 42i64)?
        .bind_variable("value2", 100i64)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test or() with first optional being none
    let program = env.compile("optional.none().or(optional.of(value2))")?;
    let activation = Activation::new()
        .bind_variable("value2", 100i64)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test orValue() with optional having value
    let program = env.compile("optional.of(value1).orValue(default)")?;
    let activation = Activation::new()
        .bind_variable("value1", 42i64)?
        .bind_variable("default", 0i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(42));

    // Test orValue() with optional being none
    let program = env.compile("optional.none().orValue(default)")?;
    let activation = Activation::new()
        .bind_variable("default", 0i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(0));

    Ok(())
}

#[test]
fn test_optional_transformation_functions() -> Result<(), Error> {
    println!("test optional transformation functions");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<i64>("value")?
        .build()?;

    // Test optMap() with optional having value
    let program = env.compile("optional.of(value).optMap(x, x * 2)")?;
    let activation = Activation::new()
        .bind_variable("value", 21i64)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optMap() with optional being none
    let program = env.compile("optional.none().optMap(x, x * 2)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optFlatMap() with optional having value
    let program = env.compile("optional.of(value).optFlatMap(x, optional.of(x * 2))")?;
    let activation = Activation::new()
        .bind_variable("value", 21i64)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test optFlatMap() with optional being none
    let program = env.compile("optional.none().optFlatMap(x, optional.of(x * 2))")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    Ok(())
}

#[test]
fn test_optional_utility_functions() -> Result<(), Error> {
    println!("test optional utility functions");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<Vec<Optional<i64>>>("optionals")?
        .build()?;

    // Test optional.unwrap() with list of optionals
    let program = env.compile("optional.unwrap(optionals)")?;
    let optionals = vec![
        Optional::new(1),
        Optional::new(2),
        Optional::none(),
        Optional::new(3),
    ];
    let activation = Activation::new()
        .bind_variable("optionals", optionals)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::List(_)));

    Ok(())
}

#[test]
fn test_optional_construction() -> Result<(), Error> {
    println!("test optional construction");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<bool>("condition")?
        .declare_variable::<String>("key")?
        .declare_variable::<Optional<i64>>("value")?
        .build()?;

    // Test optional map construction
    let program = env.compile("{?key: value}")?;
    let activation = Activation::new()
        .bind_variable("key", "test".to_string())?
        .bind_variable("value", Optional::new(42i64))?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Map(_)));

    // Test optional list construction
    let program = env.compile("[1, ?value, 3]")?;
    let activation = Activation::new()
        .bind_variable("value", Optional::new(2i64))?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::List(_)));

    // Test conditional optional construction
    let program = env.compile("condition ? {?key: value} : {}")?;
    let activation = Activation::new()
        .bind_variable("condition", true)?
        .bind_variable("key", "test".to_string())?
        .bind_variable("value", Optional::new(42i64))?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Map(_)));

    Ok(())
}

#[test]
fn test_optional_chaining_behavior() -> Result<(), Error> {
    println!("test optional chaining behavior");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<HashMap<String, HashMap<String, HashMap<String, String>>>>("data")?
        .build()?;

    // Test viral chaining - all fields exist
    let program = env.compile("data.?user.?profile.?name")?;
    let mut profile = HashMap::new();
    profile.insert("name".to_string(), "John".to_string());
    let mut user = HashMap::new();
    user.insert("profile".to_string(), profile);
    let mut data = HashMap::new();
    data.insert("user".to_string(), user);
    let activation = Activation::new()
        .bind_variable("data", data)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test viral chaining - middle field missing
    let program = env.compile("data.?user.?missing.?name")?;
    let user: HashMap<String, HashMap<String, HashMap<String, String>>> = HashMap::new();
    let mut data = HashMap::new();
    data.insert("user".to_string(), user);
    let activation = Activation::new()
        .bind_variable("data", data)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::Optional(_)));

    // Test mixed chaining with orValue
    let program = env.compile("data.?user.?profile.?name.orValue('Unknown')")?;
    let data: HashMap<String, HashMap<String, HashMap<String, String>>> = HashMap::new();
    let activation = Activation::new()
        .bind_variable("data", data)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("Unknown".into()));

    Ok(())
}

#[test]
fn test_optional_data_processing() -> Result<(), Error> {
    println!("test optional data processing");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<Vec<HashMap<String, String>>>("users")?
        .build()?;

    // Test filtering with optional field access
    let program = env.compile("users.filter(u, u.?active.orValue('false') == 'true')")?;
    let mut user1 = HashMap::new();
    user1.insert("name".to_string(), "Alice".to_string());
    user1.insert("active".to_string(), "true".to_string());
    let mut user2 = HashMap::new();
    user2.insert("name".to_string(), "Bob".to_string());
    // No active field for user2
    let users = vec![user1, user2];
    let activation = Activation::new()
        .bind_variable("users", users)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::List(_)));

    // Test mapping with optional transformation
    let program = env.compile("users.map(u, u.?name.orValue('Anonymous'))")?;
    let mut user1 = HashMap::new();
    user1.insert("name".to_string(), "Alice".to_string());
    let user2 = HashMap::new(); // No name field
    let users = vec![user1, user2];
    let activation = Activation::new()
        .bind_variable("users", users)?;
    let res = program.evaluate(&activation)?;
    assert!(matches!(res, Value::List(_)));

    Ok(())
}

#[test]
fn test_optional_error_handling() -> Result<(), Error> {
    println!("test optional error handling");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<Optional<i64>>("optional_int")?
        .declare_variable::<HashMap<String, i64>>("config")?
        .declare_variable::<HashMap<String, HashMap<String, String>>>("config2")?
        .declare_variable::<Vec<i64>>("values")?
        .build()?;

    // Test optional.orValue() with optional having value
    let program = env.compile("optional_int.orValue(0)")?;
    let activation = Activation::new()
        .bind_variable("optional_int", Optional::new(42i64))?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(42));

    // Test safe configuration access
    let program = env.compile("config.?timeout.orValue(30)")?;
    let mut config = HashMap::new();
    config.insert("host".to_string(), 8080); // No timeout field
    let activation = Activation::new()
        .bind_variable("config", config)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(30));

    // Test safe list access
    let program = env.compile("values[?0].orValue(-1)")?;
    let values: Vec<i64> = vec![]; // Empty list
    let activation = Activation::new()
        .bind_variable("values", values)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(-1));

    // Test chained safe access
    let program = env.compile("config2.?database.?host.orValue('localhost')")?;
    let config2: HashMap<String, HashMap<String, String>> = HashMap::new(); // No database field
    let activation = Activation::new()
        .bind_variable("config2", config2)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("localhost".into()));

    Ok(())
}

#[test]
fn test_optional_best_practices() -> Result<(), Error> {
    println!("test optional best practices");

    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<HashMap<String, i64>>("data")?
        .build()?;

    // Test: Always use orValue() for final values
    let program = env.compile("data.?value.orValue(0)")?;
    let data: HashMap<String, i64> = HashMap::new();
    let activation = Activation::new()
        .bind_variable("data", data)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(0));

    // Test: Use hasValue() for conditional logic
    let program = env.compile("data.?value.hasValue() ? data.?value.value() : 42")?;
    let mut data = HashMap::new();
    data.insert("value".to_string(), 100);
    let activation = Activation::new()
        .bind_variable("data", data)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(100));

    // Test: Chain operations safely
    let program = env.compile("data.?value.optMap(x, x * 2).orValue(0)")?;
    let mut data = HashMap::new();
    data.insert("value".to_string(), 21);
    let activation = Activation::new()
        .bind_variable("data", data)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(42));

    Ok(())
} 