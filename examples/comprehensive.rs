//! Comprehensive example showcasing all major features of cel-cxx
//! 
//! This example demonstrates:
//! - Zero-annotation function registration
//! - Custom types with derive macros
//! - Function overloads and generic functions
//! - Error handling with different error types
//! - Container operations and reference handling
//! - Type conversions and program introspection
//! - Member functions for opaque types

use cel_cxx::*;
use std::collections::HashMap;
use thiserror::Error;

/// Custom error type that is compatible with CEL-CXX error handling
#[derive(Error, Debug)]
enum ValidationError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Email validation failed: {0}")]
    EmailError(String),
    #[error("Age validation failed: {0}")]
    AgeError(String),
    #[error("Value {value} is outside range [{min}, {max}]")]
    RangeError { value: i64, min: i64, max: i64 },
}

/// Custom user type demonstrating derive macro usage
#[derive(Opaque, Debug, Clone, PartialEq)]
#[cel_cxx(type = "example.User")]
struct User {
    id: i64,
    name: String,
    email: String,
    age: i32,
    roles: Vec<String>,
    metadata: HashMap<String, String>,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User(id={}, name={})", self.id, self.name)
    }
}

/// Custom product type for ecommerce scenarios
#[derive(Opaque, Debug, Clone, PartialEq)]
#[cel_cxx(type = "example.Product")]
struct Product {
    id: i64,
    name: String,
    price: f64,
    category: String,
    tags: Vec<String>,
    in_stock: bool,
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(${:.2})", self.name, self.price)
    }
}

/// Student type for opaque member function demonstrations
#[derive(Opaque, Debug, Clone, PartialEq)]
#[cel_cxx(type = "example.Student")]
struct Student {
    name: String,
    age: i32,
    grade: f64,
    subjects: Vec<String>,
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Student {{ name: {}, age: {}, grade: {:.1} }}", self.name, self.age, self.grade)
    }
}

impl Student {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    
    fn get_age(&self) -> i32 {
        self.age
    }
    
    fn get_grade(&self) -> f64 {
        self.grade
    }
    
    fn is_passing(&self) -> bool {
        self.grade >= 60.0
    }
    
    fn has_subject(&self, subject: &str) -> bool {
        self.subjects.contains(&subject.to_string())
    }
    
    #[allow(unused)]
    fn add_subject(&mut self, subject: &str) -> &Self {
        if !self.subjects.contains(&subject.to_string()) {
            self.subjects.push(subject.to_string());
        }
        self
    }
    
    fn get_letter_grade(&self) -> String {
        match self.grade {
            90.0..=100.0 => "A".to_string(),
            80.0..=89.9 => "B".to_string(),
            70.0..=79.9 => "C".to_string(),
            60.0..=69.9 => "D".to_string(),
            _ => "F".to_string(),
        }
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<(), Error> {
    println!("🚀 CEL-CXX Comprehensive Example\n");

    demo1_basic_operations()?;
    demo2_variable_operations()?;
    demo3_opaque_member_functions()?;
    demo4_type_conversions()?;
    demo5_generic_functions()?;
    demo6_error_handling()?;
    demo7_program_introspection()?;
    demo8_container_operations()?;

    println!("\n✅ All demos completed successfully!");
    Ok(())
}

/// Demo 1: Basic operations with zero-annotation functions
fn demo1_basic_operations() -> Result<(), Error> {
    println!("📌 Demo 1: Basic Expressions & Zero-Annotation Functions");

    let env = Env::builder()
        .declare_variable::<String>("name")?
        .declare_variable::<i64>("age")?
        .declare_variable::<f64>("score")?
        
        // ✨ Zero-annotation functions - types automatically inferred!
        .register_global_function("greet", |name: &str| {
            format!("Hello, {}!", name)
        })?
        
        .register_global_function("is_adult", |age: i64| age >= 18)?
        
        .register_global_function("grade", |score: f64| -> String {
            match score {
                90.0..=100.0 => "A".to_string(),
                80.0..=89.9 => "B".to_string(),
                70.0..=79.9 => "C".to_string(),
                60.0..=69.9 => "D".to_string(),
                _ => "F".to_string(),
            }
        })?
        
        .register_global_function("calculate_discount", |age: i64, score: f64| -> f64 {
            let base_discount = if age >= 65 { 0.2 } else { 0.0 };
            let score_bonus = if score >= 90.0 { 0.1 } else { 0.0 };
            base_discount + score_bonus
        })?
        
        .build()?;

    // Test basic expressions including simple arithmetic
    let test_cases = vec![
        ("1 + 1", "Alice", 25i64, 95.0, "Simple arithmetic"),
        ("greet(name)", "Alice", 25i64, 95.0, "Function with string parameter"),
        ("is_adult(age)", "Bob", 16i64, 85.0, "Boolean function"),
        ("grade(score)", "Charlie", 30i64, 78.5, "String return function"),
        ("calculate_discount(age, score)", "Diana", 67i64, 92.0, "Multi-parameter function"),
    ];

    for (expr, name, age, score, description) in test_cases {
        let program = env.compile(expr)?;
        let activation = Activation::new()
            .bind_variable("name", name)?
            .bind_variable("age", age)?
            .bind_variable("score", score)?;

        let result = program.evaluate(&activation)?;
        println!("  {} = {} ({})", expr, result, description);
    }

    println!();
    Ok(())
}

/// Demo 2: Variable binding and providers
fn demo2_variable_operations() -> Result<(), Error> {
    println!("📌 Demo 2: Variable Binding & Providers");

    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_global_function::<fn() -> i64>("get_const")?
        .register_global_function("multiply", |x: i64, y: i64| x * y)?
        .build()?;

    // Test direct variable binding
    {
        println!("  Variable binding:");
        let program = env.compile("a + b")?;
        let activation = Activation::new()
            .bind_variable("a", 10)?
            .bind_variable("b", 20)?;
        let result = program.evaluate(&activation)?;
        println!("    a + b = {} (direct binding)", result);
    }

    // Test variable provider binding
    {
        println!("  Variable provider binding:");
        let program = env.compile("a * b")?;
        let activation = Activation::new()
            .bind_variable("a", 5)?
            .bind_variable_provider("b", || -> Result<i64, Error> {
                println!("    Provider called for variable 'b'");
                Ok(7)
            })?;
        let result = program.evaluate(&activation)?;
        println!("    a * b = {} (provider binding)", result);
    }

    // Test function declaration and binding
    {
        println!("  Function declaration & binding:");
        let program = env.compile("get_const() + multiply(a, 3)")?;
        let activation = Activation::new()
            .bind_variable("a", 4)?
            .bind_global_function("get_const", || -> Result<i64, Error> {
                Ok(100)
            })?;
        let result = program.evaluate(&activation)?;
        println!("    get_const() + multiply(a, 3) = {} (function binding)", result);
    }

    println!();
    Ok(())
}

/// Demo 3: Opaque types with member functions
fn demo3_opaque_member_functions() -> Result<(), Error> {
    println!("📌 Demo 3: Opaque Types & Member Functions");

    let env = Env::builder()
        .declare_variable::<Student>("student")?
        
        // ✨ Register struct methods directly using RustType::method_name syntax
        .register_member_function("get_name", Student::get_name)?
        .register_member_function("get_age", Student::get_age)?
        .register_member_function("get_grade", Student::get_grade)?
        .register_member_function("is_passing", Student::is_passing)?
        .register_member_function("has_subject", Student::has_subject)?
        .register_member_function("get_letter_grade", Student::get_letter_grade)?
        
        .build()?;

    let student = Student {
        name: "John Doe".to_string(),
        age: 18,
        grade: 87.5,
        subjects: vec!["Math".to_string(), "Physics".to_string()],
    };

    let activation = Activation::new()
        .bind_variable("student", student)?;

    let test_expressions = vec![
        ("student.get_name()", "Get student name"),
        ("student.get_age()", "Get student age"),
        ("student.get_grade()", "Get numerical grade"),
        ("student.get_letter_grade()", "Get letter grade"),
        ("student.is_passing()", "Check if passing"),
        ("student.has_subject('Math')", "Check if has Math subject"),
        ("student.has_subject('Chemistry')", "Check if has Chemistry subject"),
    ];

    for (expr, description) in test_expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("  {} = {} ({})", expr, result, description);
    }

    println!();
    Ok(())
}

/// Demo 4: Type conversions and standard Rust types
fn demo4_type_conversions() -> Result<(), Error> {
    println!("📌 Demo 4: Type Conversions & Standard Rust Types");

    // Functions returning different types (both direct values and Results)
    fn return_string_direct() -> String { "hello world".to_string() }
    fn return_int_result() -> Result<i64, std::io::Error> { Ok(42) }
    fn return_list() -> Vec<i64> { vec![1, 2, 3, 4, 5] }
    fn return_map() -> HashMap<String, i64> {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 100);
        map.insert("key2".to_string(), 200);
        map
    }
    fn return_optional_some() -> Option<String> { Some("optional value".to_string()) }
    fn return_optional_none() -> Option<String> { None }

    let env = Env::builder()
        .register_global_function("return_string_direct", return_string_direct)?
        .register_global_function("return_int_result", return_int_result)?
        .register_global_function("return_list", return_list)?
        .register_global_function("return_map", return_map)?
        .register_global_function("return_optional_some", return_optional_some)?
        .register_global_function("return_optional_none", return_optional_none)?
        .build()?;

    let test_cases = vec![
        ("return_string_direct()", "String conversion"),
        ("return_int_result()", "Result<i64> conversion"),
        ("return_list()", "Vec<i64> conversion"),
        ("return_map()", "HashMap conversion"),
        ("return_optional_some()", "Option<String> Some conversion"),
        ("return_optional_none()", "Option<String> None conversion"),
    ];

    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(())?;
        println!("  {} = {} ({})", expr, result, description);
        
        // Demonstrate type conversion back to Rust types
        match expr {
            "return_string_direct()" => {
                let rust_string: String = result.try_into()
                    .map_err(|_| Error::invalid_argument("string conversion failed".to_string()))?;
                println!("    Converted back to Rust String: '{}'", rust_string);
            },
            "return_int_result()" => {
                let rust_int: i64 = result.try_into()
                    .map_err(|_| Error::invalid_argument("int conversion failed".to_string()))?;
                println!("    Converted back to Rust i64: {}", rust_int);
            },
            "return_list()" => {
                let rust_list: Vec<i64> = result.try_into()
                    .map_err(|_| Error::invalid_argument("list conversion failed".to_string()))?;
                println!("    Converted back to Rust Vec<i64>: {:?}", rust_list);
            },
            _ => {}
        }
    }

    println!();
    Ok(())
}

/// Demo 5: Generic functions and type annotations
fn demo5_generic_functions() -> Result<(), Error> {
    println!("📌 Demo 5: Generic Functions & Function Overloads");

    // Generic function that counts items in any Vec<T>
    fn count_items<T>(items: Vec<T>) -> i64 {
        items.len() as i64
    }

    // Generic function that processes maps
    fn get_map_size<K, V>(map: HashMap<K, V>) -> i64 {
        map.len() as i64
    }

    // Function working with references in containers
    fn join_strings(strings: Vec<&str>, separator: &str) -> String {
        strings.join(separator)
    }

    let env = Env::builder()
        .declare_variable::<Vec<String>>("string_list")?
        .declare_variable::<Vec<i64>>("int_list")?
        .declare_variable::<HashMap<String, i64>>("score_map")?
        .declare_variable::<Vec<f64>>("floats")?
        
        // Register generic functions with specific type annotations
        .register_global_function("count_strings", count_items::<String>)?
        .register_global_function("count_ints", count_items::<i64>)?
        .register_global_function("get_string_map_size", get_map_size::<String, i64>)?
        .register_global_function("join_strings", join_strings)?
        
        // Multiple functions with same name, different signatures (overloads)
        .register_global_function("process", |x: i64| x * 2)?
        .register_global_function("process", |x: f64| (x * 2.0).round())?
        .register_global_function("process", |x: String| x.to_uppercase())?
        
        // Overloaded member functions for different container types
        .register_member_function("sum", |numbers: Vec<i64>| {
            numbers.iter().sum::<i64>()
        })?
        
        .register_member_function("sum", |floats: Vec<f64>| {
            floats.iter().sum::<f64>()
        })?
        
        .build()?;

    let test_cases = vec![
        // Generic function tests
        ("count_strings(string_list)", "Count strings in list"),
        ("count_ints(int_list)", "Count integers in list"),
        ("get_string_map_size(score_map)", "Get map size"),
        ("join_strings(string_list, ' ')", "Join strings with space"),
        
        // Function overload tests
        ("process(42)", "Process integer (multiply by 2)"),
        ("process(3.14)", "Process float (multiply by 2, round)"),
        ("process('hello')", "Process string (uppercase)"),
        ("int_list.sum()", "Sum integers"),
        ("floats.sum()", "Sum floats"),
    ];

    let activation = Activation::new()
        .bind_variable("string_list", vec!["hello".to_string(), "world".to_string(), "rust".to_string()])?
        .bind_variable("int_list", vec![1, 2, 3, 4, 5, 6])?
        .bind_variable("floats", vec![1.5, 2.7, 3.14, 4.0])?
        .bind_variable("score_map", {
            let mut map = HashMap::new();
            map.insert("alice".to_string(), 95);
            map.insert("bob".to_string(), 87);
            map.insert("charlie".to_string(), 92);
            map
        })?;

    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("  {} = {} ({})", expr, result, description);
    }

    println!();
    Ok(())
}

/// Demo 6: Error handling with different error types including Box<dyn std::error::Error>
fn demo6_error_handling() -> Result<(), Error> {
    println!("📌 Demo 6: Error Handling & Validation");

    let env = Env::builder()
        .declare_variable::<String>("input")?
        .declare_variable::<i64>("divisor")?
        .declare_variable::<String>("email")?
        
        // Functions returning different error types using thiserror-derived types
        .register_global_function("safe_parse", |s: &str| -> Result<i64, ValidationError> {
            s.parse::<i64>().map_err(|e| ValidationError::ParseError(e.to_string()))
        })?
        
        .register_global_function("safe_divide", |a: i64, b: i64| -> Result<f64, ValidationError> {
            if b == 0 {
                Err(ValidationError::DivisionByZero)
            } else {
                Ok(a as f64 / b as f64)
            }
        })?
        
        // Additional validation functions with ValidationError
        .register_global_function("validate_email", |email: &str| -> Result<bool, ValidationError> {
            if email.is_empty() {
                return Err(ValidationError::EmailError("Email cannot be empty".to_string()));
            }
            if !email.contains('@') {
                return Err(ValidationError::EmailError("Email must contain @ symbol".to_string()));
            }
            if !email.contains('.') {
                return Err(ValidationError::EmailError("Email must contain . symbol".to_string()));
            }
            let parts: Vec<&str> = email.split('@').collect();
            if parts.len() != 2 {
                return Err(ValidationError::EmailError("Email must contain exactly one @ symbol".to_string()));
            }
            if parts[0].is_empty() || parts[1].is_empty() {
                return Err(ValidationError::EmailError("Email local and domain parts cannot be empty".to_string()));
            }
            Ok(true)
        })?
        
        .register_global_function("validate_age", |age: i64| -> Result<String, ValidationError> {
            match age {
                0..=17 => Ok("minor".to_string()),
                18..=64 => Ok("adult".to_string()),
                65..=120 => Ok("senior".to_string()),
                _ => Err(ValidationError::AgeError(format!("Invalid age: {}", age))),
            }
        })?
        
        .register_global_function("validate_range", |x: i64, min: i64, max: i64| -> Result<i64, ValidationError> {
            if x < min || x > max {
                Err(ValidationError::RangeError { value: x, min, max })
            } else {
                Ok(x)
            }
        })?
        
        .build()?;

    let test_cases = vec![
        // Success cases
        ("safe_parse('42')", "42", 2i64, "valid@email.com", true, "Parse valid number"),
        ("safe_divide(10, divisor)", "10", 2i64, "test@example.com", true, "Safe division"),
        ("validate_email(email)", "25", 1i64, "user@domain.com", true, "Valid email"),
        ("validate_age(safe_parse(input))", "25", 1i64, "user@test.com", true, "Valid age"),
        
        // Error cases
        ("safe_parse('invalid')", "invalid", 2i64, "test@test.com", false, "Parse invalid number"),
        ("safe_divide(10, divisor)", "10", 0i64, "test@test.com", false, "Division by zero"),
        ("validate_email(email)", "10", 1i64, "invalid-email", false, "Invalid email format"),
        ("validate_age(safe_parse(input))", "150", 1i64, "test@test.com", false, "Invalid age"),
        ("validate_range(safe_parse(input), 1, 100)", "150", 2i64, "test@test.com", false, "Value out of range"),
    ];

    for (expr, input, divisor, email, should_succeed, description) in test_cases {
        let program = env.compile(expr)?;
        let activation = Activation::new()
            .bind_variable("input", input)?
            .bind_variable("divisor", divisor)?
            .bind_variable("email", email)?;

        match program.evaluate(&activation) {
            Ok(result) => {
                println!("  {} = {} ✅ ({})", expr, result, description);
                if !should_succeed {
                    println!("    ⚠️  Expected this to fail!");
                }
            }
            Err(e) => {
                println!("  {} -> ERROR: {} ❌ ({})", expr, e, description);
                if should_succeed {
                    println!("    ⚠️  Expected this to succeed!");
                }
            }
        }
    }

    println!();
    Ok(())
}

/// Demo 7: Program introspection and return types
fn demo7_program_introspection() -> Result<(), Error> {
    println!("📌 Demo 7: Program Introspection & Return Types");

    // Test with numeric expression
    {
        let env = Env::builder()
            .declare_variable::<i64>("a")?
            .declare_variable::<i64>("b")?
            .build()?;
        let program = env.compile("a + b")?;
        let return_type = program.return_type();
        println!("  Expression: 'a + b'");
        println!("    Return type: {}", return_type);
        
        let activation = Activation::new()
            .bind_variable("a", 10)?
            .bind_variable("b", 20)?;
        let result = program.evaluate(&activation)?;
        println!("    Result: {}", result);
    }

    // Test with string expression
    {
        let env = Env::builder()
            .declare_variable::<String>("a")?
            .declare_variable::<String>("b")?
            .build()?;
        let program = env.compile("a + b")?;
        let return_type = program.return_type();
        println!("  Expression: 'a + b' (strings)");
        println!("    Return type: {}", return_type);
        
        let activation = Activation::new()
            .bind_variable("a", "Hello ".to_string())?
            .bind_variable("b", "World!".to_string())?;
        let result = program.evaluate(&activation)?;
        println!("    Result: {}", result);
    }

    // Test with boolean expression
    {
        let env = Env::builder()
            .declare_variable::<i64>("age")?
            .build()?;
        let program = env.compile("age >= 18")?;
        let return_type = program.return_type();
        println!("  Expression: 'age >= 18'");
        println!("    Return type: {}", return_type);
        
        let activation = Activation::new()
            .bind_variable("age", 25i64)?;
        let result = program.evaluate(&activation)?;
        println!("    Result: {}", result);
    }

    println!();
    Ok(())
}

/// Demo 8: Advanced container operations and reference handling
fn demo8_container_operations() -> Result<(), Error> {
    println!("📌 Demo 8: Container Operations & Reference Handling");

    let env = Env::builder()
        .declare_variable::<Vec<&str>>("string_refs")?
        .declare_variable::<HashMap<i64, &str>>("lookup_table")?
        .declare_variable::<Option<&str>>("maybe_value")?
        .declare_variable::<Vec<User>>("users")?
        
        // Functions working with reference types
        .register_global_function("longest_string", |strings: Vec<&str>| -> Option<String> {
            strings.iter()
                .max_by_key(|s| s.len())
                .map(|s| s.to_string())
        })?
        
        .register_global_function("lookup", |table: HashMap<i64, &str>, key: i64| -> Option<String> {
            table.get(&key).map(|s| s.to_string())
        })?
        
        .register_global_function("string_length", |s: Option<&str>| -> i64 {
            s.map(|s| s.len() as i64).unwrap_or(0)
        })?
        
        // Advanced filtering and mapping
        .register_global_function("filter_adults", |users: Vec<User>| -> Vec<User> {
            users.into_iter().filter(|u| u.age >= 18).collect()
        })?
        
        .register_global_function("group_by_age_range", |users: Vec<User>| -> HashMap<String, Vec<String>> {
            let mut groups: HashMap<String, Vec<String>> = HashMap::new();
            for user in users {
                let range = match user.age {
                    0..=17 => "minor",
                    18..=64 => "adult", 
                    _ => "senior",
                };
                groups.entry(range.to_string()).or_default().push(user.name);
            }
            groups
        })?
        
        .build()?;

    // Prepare test data with proper lifetimes
    let source_strings = vec!["hello".to_string(), "world".to_string(), "rust".to_string(), "programming".to_string()];
    let string_refs: Vec<&str> = source_strings.iter().map(|s| s.as_str()).collect();
    
    let lookup_table: HashMap<i64, &str> = HashMap::from([
        (1, "one"),
        (2, "two"), 
        (3, "three"),
    ]);
    
    let maybe_value: Option<&str> = Some("test string");
    
    let users = vec![
        User {
            id: 1, name: "Alice".to_string(), email: "alice@test.com".to_string(),
            age: 25, roles: vec![], metadata: HashMap::new(),
        },
        User {
            id: 2, name: "Bob".to_string(), email: "bob@test.com".to_string(),
            age: 16, roles: vec![], metadata: HashMap::new(),
        },
        User {
            id: 3, name: "Carol".to_string(), email: "carol@test.com".to_string(),
            age: 67, roles: vec![], metadata: HashMap::new(),
        },
    ];

    let test_expressions = vec![
        ("longest_string(string_refs)", "Find longest string"),
        ("lookup(lookup_table, 2)", "Lookup value by key"),
        ("string_length(maybe_value)", "Get optional string length"),
        ("filter_adults(users).size()", "Count adult users"),
        ("group_by_age_range(users).size()", "Group users by age range"),
    ];

    let activation = Activation::new()
        .bind_variable("string_refs", string_refs)?
        .bind_variable("lookup_table", lookup_table)?
        .bind_variable("maybe_value", maybe_value)?
        .bind_variable("users", users)?;

    for (expr, description) in test_expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("  {} = {} ({})", expr, result, description);
    }

    println!();
    Ok(())
}
