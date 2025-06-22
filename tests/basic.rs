use std::convert::Infallible;
use std::collections::HashMap;

use cel_cxx::*;

const MY_NAMESPACE: &str = "testing";

#[test]
fn test_math() -> Result<(), Error>{
    println!("test math");

    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;

    let program = env.compile("a+b")?;

    let activation = Activation::new()
        .bind_variable("a", 2i64)?
        .bind_variable("b", 3i64)?
        ;

    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(5));

    Ok(())
}

#[test]
fn test_function_return_types() -> Result<(), Error> {
    println!("test function return types - value types vs Result<T, E>");

    // Functions returning value types directly
    fn add_direct(a: i64, b: i64) -> i64 { a + b }
    fn concat_direct(a: &str, b: &str) -> String { format!("{}{}", a, b) }
    fn get_length_direct(s: &str) -> i64 { s.len() as i64 }
    
    // Functions returning Result<T, E>
    fn add_result(a: i64, b: i64) -> Result<i64, Infallible> { Ok(a + b) }
    fn divide_result(a: i64, b: i64) -> Result<i64, std::io::Error> {
        if b == 0 {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "division by zero"))
        } else {
            Ok(a / b)
        }
    }
    fn parse_int_result(s: &str) -> Result<i64, std::num::ParseIntError> {
        s.parse::<i64>()
    }

    let env = Env::builder()
        // Register functions with direct value returns
        .register_global_function("add_direct", add_direct)?
        .register_global_function("concat_direct", concat_direct)?
        .register_global_function("get_length_direct", get_length_direct)?
        // Register functions with Result returns
        .register_global_function("add_result", add_result)?
        .register_global_function("divide_result", divide_result)?
        .register_global_function("parse_int_result", parse_int_result)?
        .declare_variable::<String>("text")?
        .build()?;

    // Test direct value returns
    {
        let program = env.compile("add_direct(10, 20)")?;
        let res = program.evaluate(())?;
        assert_eq!(res, Value::Int(30));
    }

    {
        let program = env.compile("concat_direct('Hello, ', 'World!')")?;
        let res = program.evaluate(())?;
        assert_eq!(res, Value::String("Hello, World!".into()));
    }

    {
        let activation = Activation::new()
            .bind_variable("text", "Hello".to_string())?;
        let program = env.compile("get_length_direct(text)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(5));
    }

    // Test Result returns
    {
        let program = env.compile("add_result(15, 25)")?;
        let res = program.evaluate(())?;
        assert_eq!(res, Value::Int(40));
    }

    {
        let program = env.compile("divide_result(20, 4)")?;
        let res = program.evaluate(())?;
        assert_eq!(res, Value::Int(5));
    }

    {
        let program = env.compile("parse_int_result('42')")?;
        let res = program.evaluate(())?;
        assert_eq!(res, Value::Int(42));
    }

    // Test error handling - divide by zero should return an error
    {
        let program = env.compile("divide_result(10, 0)")?;
        let res = program.evaluate(());
        assert!(res.is_err());
    }

    // Test error handling - invalid parse should return an error
    {
        let program = env.compile("parse_int_result('not_a_number')")?;
        let res = program.evaluate(());
        assert!(res.is_err());
    }

    Ok(())
}

#[test]
fn test_reference_types() -> Result<(), Error> {
    println!("test reference types - valid and invalid patterns");

    // Valid: Vec<&str> - can borrow from Vec<Value>
    fn process_string_list(items: Vec<&str>) -> String {
        items.join(", ")
    }

    // Valid: HashMap<i64, &str> - can borrow from HashMap<Value, Value>
    fn process_string_map(map: HashMap<i64, &str>) -> String {
        map.values().cloned().collect::<Vec<_>>().join(", ")
    }

    // Valid: Option<&str> - can borrow from Option<Value>
    fn process_optional_string(opt: Option<&str>) -> String {
        opt.unwrap_or("default").to_string()
    }

    // Valid: Direct string references
    fn process_string_ref(s: &str) -> String {
        s.to_uppercase()
    }

    // Valid: Functions returning borrowed data (lifetime handled safely)
    fn get_first_string(items: Vec<&str>) -> String {
        items.first().unwrap_or(&"").to_string()
    }

    let env = Env::builder()
        .register_global_function("process_string_list", process_string_list)?
        .register_global_function("process_string_map", process_string_map)?
        .register_global_function("process_optional_string", process_optional_string)?
        .register_global_function("process_string_ref", process_string_ref)?
        .register_global_function("get_first_string", get_first_string)?
        .declare_variable::<Vec<String>>("string_list")?
        .declare_variable::<HashMap<i64, String>>("string_map")?
        .declare_variable::<Option<String>>("optional_string")?
        .declare_variable::<String>("single_string")?
        .build()?;

    // Test Vec<&str>
    {
        let activation = Activation::new()
            .bind_variable("string_list", vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()])?;
        let program = env.compile("process_string_list(string_list)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::String("apple, banana, cherry".into()));
    }

    // Test HashMap<i64, &str>
    {
        let mut map = HashMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());
        
        let activation = Activation::new()
            .bind_variable("string_map", map)?;
        let program = env.compile("process_string_map(string_map)")?;
        let res = program.evaluate(&activation)?;
        // Note: HashMap iteration order is not guaranteed, so we check if result contains expected values
        let result_str: String = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert result to string".to_string()))?;
        assert!(result_str.contains("one"));
        assert!(result_str.contains("two"));
        assert!(result_str.contains("three"));
    }

    // Test Option<&str>
    {
        let activation = Activation::new()
            .bind_variable("optional_string", Some("hello".to_string()))?;
        let program = env.compile("process_optional_string(optional_string)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::String("hello".into()));
    }

    {
        let activation = Activation::new()
            .bind_variable("optional_string", None::<String>)?;
        let program = env.compile("process_optional_string(optional_string)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::String("default".into()));
    }

    // Test &str
    {
        let activation = Activation::new()
            .bind_variable("single_string", "hello world".to_string())?;
        let program = env.compile("process_string_ref(single_string)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::String("HELLO WORLD".into()));
    }

    // Test function returning borrowed data
    {
        let activation = Activation::new()
            .bind_variable("string_list", vec!["first".to_string(), "second".to_string(), "third".to_string()])?;
        let program = env.compile("get_first_string(string_list)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::String("first".into()));
    }

    Ok(())
}

#[test]
fn test_container_special_cases() -> Result<(), Error> {
    println!("test container special cases - Vec, HashMap, Option");

    // Valid container patterns
    fn sum_numbers(numbers: Vec<i64>) -> i64 {
        numbers.iter().sum()
    }

    fn count_items<T>(items: Vec<T>) -> i64 {
        items.len() as i64
    }

    fn get_map_value(map: HashMap<String, i64>, key: &str) -> Option<i64> {
        map.get(key).copied()
    }

    fn unwrap_or_default(opt: Option<i64>) -> i64 {
        opt.unwrap_or(0)
    }

    // Functions working with nested containers
    fn process_nested_list(nested: Vec<Vec<i64>>) -> i64 {
        nested.iter().map(|inner| inner.iter().sum::<i64>()).sum()
    }

    fn process_map_of_lists(map: HashMap<String, Vec<i64>>) -> i64 {
        map.values().map(|list| list.iter().sum::<i64>()).sum()
    }

    let env = Env::builder()
        .register_global_function("sum_numbers", sum_numbers)?
        .register_global_function("count_strings", count_items::<String>)?
        .register_global_function("count_ints", count_items::<i64>)?
        .register_global_function("get_map_value", get_map_value)?
        .register_global_function("unwrap_or_default", unwrap_or_default)?
        .register_global_function("process_nested_list", process_nested_list)?
        .register_global_function("process_map_of_lists", process_map_of_lists)?
        .declare_variable::<Vec<i64>>("numbers")?
        .declare_variable::<Vec<String>>("strings")?
        .declare_variable::<HashMap<String, i64>>("score_map")?
        .declare_variable::<Option<i64>>("maybe_number")?
        .declare_variable::<Vec<Vec<i64>>>("nested_numbers")?
        .declare_variable::<HashMap<String, Vec<i64>>>("map_of_lists")?
        .build()?;

    // Test Vec<i64>
    {
        let activation = Activation::new()
            .bind_variable("numbers", vec![1, 2, 3, 4, 5])?;
        let program = env.compile("sum_numbers(numbers)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(15));
    }

    // Test Vec<String> with generic function
    {
        let activation = Activation::new()
            .bind_variable("strings", vec!["a".to_string(), "b".to_string(), "c".to_string()])?;
        let program = env.compile("count_strings(strings)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(3));
    }

    // Test Vec<i64> with generic function
    {
        let activation = Activation::new()
            .bind_variable("numbers", vec![1, 2, 3, 4, 5])?;
        let program = env.compile("count_ints(numbers)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(5));
    }

    // Test HashMap
    {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), 100);
        map.insert("bob".to_string(), 85);
        map.insert("charlie".to_string(), 92);
        
        let activation = Activation::new()
            .bind_variable("score_map", map)?;
        let program = env.compile("get_map_value(score_map, 'alice')")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Optional(Some(Value::Int(100)).into()));
    }

    // Test Option
    {
        let activation = Activation::new()
            .bind_variable("maybe_number", Some(42))?;
        let program = env.compile("unwrap_or_default(maybe_number)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(42));
    }

    {
        let activation = Activation::new()
            .bind_variable("maybe_number", None::<i64>)?;
        let program = env.compile("unwrap_or_default(maybe_number)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(0));
    }

    // Test nested containers
    {
        let nested = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![6, 7, 8, 9]
        ];
        let activation = Activation::new()
            .bind_variable("nested_numbers", nested)?;
        let program = env.compile("process_nested_list(nested_numbers)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(45)); // 6 + 9 + 30 = 45
    }

    // Test map of lists
    {
        let mut map = HashMap::new();
        map.insert("group1".to_string(), vec![1, 2, 3]);
        map.insert("group2".to_string(), vec![4, 5]);
        map.insert("group3".to_string(), vec![6, 7, 8]);
        
        let activation = Activation::new()
            .bind_variable("map_of_lists", map)?;
        let program = env.compile("process_map_of_lists(map_of_lists)")?;
        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(36)); // 6 + 9 + 21 = 36
    }

    Ok(())
}

#[test]
fn test_standard_conversions() -> Result<(), Error> {
    println!("test standard Rust conversions - TryFrom/Into/From/TryInto");

    // Test functions that return different value types
    fn return_bool() -> bool { true }
    fn return_int() -> i64 { 42 }
    fn return_uint() -> u64 { 100 }
    fn return_double() -> f64 { 3.14 }
    fn return_string() -> String { "hello".to_string() }
    fn return_bytes() -> Vec<u8> { vec![1, 2, 3, 4] }
    fn return_list() -> Vec<i64> { vec![1, 2, 3] }
    fn return_map() -> HashMap<String, i64> {
        let mut map = HashMap::new();
        map.insert("key".to_string(), 42);
        map
    }
    fn return_optional_some() -> Option<i64> { Some(123) }
    fn return_optional_none() -> Option<i64> { None }

    let env = Env::builder()
        .register_global_function("return_bool", return_bool)?
        .register_global_function("return_int", return_int)?
        .register_global_function("return_uint", return_uint)?
        .register_global_function("return_double", return_double)?
        .register_global_function("return_string", return_string)?
        .register_global_function("return_bytes", return_bytes)?
        .register_global_function("return_list", return_list)?
        .register_global_function("return_map", return_map)?
        .register_global_function("return_optional_some", return_optional_some)?
        .register_global_function("return_optional_none", return_optional_none)?
        .build()?;

    // Test bool conversion
    {
        let program = env.compile("return_bool()")?;
        let res = program.evaluate(())?;
        let rust_bool: bool = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert to bool".to_string()))?;
        assert_eq!(rust_bool, true);
    }

    // Test int conversion
    {
        let program = env.compile("return_int()")?;
        let res = program.evaluate(())?;
        let rust_int: i64 = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert to i64".to_string()))?;
        assert_eq!(rust_int, 42);
    }

    // Test uint conversion
    {
        let program = env.compile("return_uint()")?;
        let res = program.evaluate(())?;
        let rust_uint: u64 = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert to u64".to_string()))?;
        assert_eq!(rust_uint, 100);
    }

    // Test double conversion
    {
        let program = env.compile("return_double()")?;
        let res = program.evaluate(())?;
        let rust_double: f64 = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert to f64".to_string()))?;
        assert_eq!(rust_double, 3.14);
    }

    // Test string conversion
    {
        let program = env.compile("return_string()")?;
        let res = program.evaluate(())?;
        let rust_string: String = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert to String".to_string()))?;
        assert_eq!(rust_string, "hello");
    }

    // Test bytes conversion
    {
        let program = env.compile("return_bytes()")?;
        let res = program.evaluate(())?;
        let rust_bytes: Vec<u8> = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert to Vec<u8>".to_string()))?;
        assert_eq!(rust_bytes, vec![1, 2, 3, 4]);
    }

    // Test list conversion
    {
        let program = env.compile("return_list()")?;
        let res = program.evaluate(())?;
        let rust_list: Vec<i64> = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert to Vec<i64>".to_string()))?;
        assert_eq!(rust_list, vec![1, 2, 3]);
    }

    // Test map conversion
    {
        let program = env.compile("return_map()")?;
        let res = program.evaluate(())?;
        let rust_map: HashMap<String, i64> = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert to HashMap".to_string()))?;
        assert_eq!(rust_map.get("key"), Some(&42));
    }

    // Test optional conversion - Some
    {
        let program = env.compile("return_optional_some()")?;
        let res = program.evaluate(())?;
        let rust_optional: Option<i64> = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert to Option<i64>".to_string()))?;
        assert_eq!(rust_optional, Some(123));
    }

    // Test optional conversion - None
    {
        let program = env.compile("return_optional_none()")?;
        let res = program.evaluate(())?;
        let rust_optional: Option<i64> = res.try_into()
            .map_err(|_| Error::invalid_argument("failed to convert to Option<i64>".to_string()))?;
        assert_eq!(rust_optional, None);
    }

    Ok(())
}

#[test]
fn test_into_value_conversions() -> Result<(), Error> {
    println!("test Into<Value> conversions from Rust types");

    // Test creating values from Rust types using Into
    let bool_val: Value = true.into();
    assert_eq!(bool_val, Value::Bool(true));

    let int_val: Value = 42i64.into();
    assert_eq!(int_val, Value::Int(42));

    let uint_val: Value = 100u64.into();
    assert_eq!(uint_val, Value::Uint(100));

    let double_val: Value = 3.14f64.into();
    assert_eq!(double_val, Value::Double(3.14));

    let string_val: Value = "hello".to_string().into();
    assert_eq!(string_val, Value::String("hello".into()));

    let bytes_val: Value = vec![1u8, 2, 3].into();
    assert_eq!(bytes_val, Value::Bytes(vec![1u8, 2, 3].into()));

    let list_val: Value = vec![1i64, 2, 3].into();
    if let Value::List(list) = list_val {
        assert_eq!(list.len(), 3);
    } else {
        panic!("Expected List value");
    }

    let mut map = HashMap::new();
    map.insert("key".to_string(), 42i64);
    let map_val: Value = map.into();
    if let Value::Map(_) = map_val {
        // Success
    } else {
        panic!("Expected Map value");
    }

    let optional_some_val: Value = Some(42i64).into();
    if let Value::Optional(_) = optional_some_val {
        // Success
    } else {
        panic!("Expected Optional value");
    }

    let optional_none_val: Value = None::<i64>.into();
    if let Value::Optional(_) = optional_none_val {
        // Success
    } else {
        panic!("Expected Optional value");
    }

    Ok(())
}

#[test]
fn test_simplefunc() -> Result<(), Error>{
    println!("test simple function");

    fn sum(a: i64, b: i64) -> Result<i64, Infallible> {Ok(a+b)}
    fn square(a: i64) -> Result<i64, Infallible> {Ok(a*a)}

    let env = Env::builder()
        .register_global_function("sum", sum)?
        .register_global_function("square", square)?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;

    let program = env.compile("square(sum(a,b))")?;

    let activation = Activation::new()
        .bind_variable("a", 2i64)?
        .bind_variable("b", 3i64)?
        ;

    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(25));

    Ok(())
}

#[test]
fn test_opaque_function() -> Result<(), Error> {
    println!("test opaque function");

    #[derive(Debug, Clone, PartialEq)]
    #[derive(Opaque)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, "MyOpaque"))]
    struct MyOpaque(i64);

    impl std::fmt::Display for MyOpaque {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyOpaque({})", self.0)
        }
    }

    fn get_type(_: MyOpaque) -> Result<String, Error> { Ok("i64".to_string()) }
    fn get_value(a: MyOpaque) -> Result<i64, Error> { Ok(a.0) }
    fn sum(a: MyOpaque, b: i64) -> Result<i64, Error> { Ok(a.0 + b) }

    let env = Env::builder()
        .register_member_function("get_type", get_type)?
        .register_member_function("get_value", get_value)?
        .register_member_function("sum", sum)?
        .declare_variable::<MyOpaque>("mo")?
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_variable::<MyOpaque>("mo2")?
        .declare_variable::<Vec<i64>>("ml")?
        .build()?;

    {
        let expr = "a>1 ? mo.get_value() : mo.get_value()";
        let program = env.compile(expr)?;

        let mo = MyOpaque(16);
        let activation = Activation::new()
            .bind_variable("mo", mo)?
            .bind_variable("a", 0i32)?
            ;

        let res = program.evaluate(&activation)?;
        assert_eq!(res, Value::Int(16));
    }

    {
        let expr = "ml.map(m, mo2.sum(m))";
        let program = env.compile(expr)?;

        let mo = MyOpaque(16);
        let ml: Vec<i64> = vec![84,85,86,87,88];
        let activation = Activation::new()
            .bind_variable("mo2", mo)?
            .bind_variable("ml",ml)?
            ;

        let res = program.evaluate(&activation)?;

        let list: Vec<i64> = res.try_into()
            .map_err(|_| Error::invalid_argument("invalid vec i64".to_string()))?;

        assert_eq!(list, vec![100,101,102,103,104]);
    }

    Ok(())
}
