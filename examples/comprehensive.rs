//! Rust function registration, opaque types, collections, and error handling.
//!
//! Run with `cargo run --example comprehensive --features derive`.

use cel_cxx::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
enum ValidationError {
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Age must be between 0 and 120: {0}")]
    Age(i64),
}

#[derive(Opaque, Debug, Clone, PartialEq)]
#[cel_cxx(type = "example.Student")]
#[cel_cxx(display = write!(fmt, "{} (age {}, grade {})", self.name, self.age, self.grade))]
struct Student {
    name: String,
    age: i64,
    grade: f64,
    subjects: Vec<String>,
}

impl Student {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn grade(&self) -> f64 {
        self.grade
    }

    fn is_passing(&self) -> bool {
        self.grade >= 60.0
    }

    fn has_subject(&self, subject: &str) -> bool {
        self.subjects.iter().any(|candidate| candidate == subject)
    }
}

fn letter_grade(score: f64) -> Result<&'static str, Error> {
    if !score.is_finite() || !(0.0..=100.0).contains(&score) {
        return Err(Error::invalid_argument("score must be between 0 and 100"));
    }
    Ok(match score {
        score if score >= 90.0 => "A",
        score if score >= 80.0 => "B",
        score if score >= 70.0 => "C",
        score if score >= 60.0 => "D",
        _ => "F",
    })
}

fn main() -> Result<(), Error> {
    println!("CEL-CXX Comprehensive Example\n");
    basic_operations()?;
    variable_operations()?;
    opaque_member_functions()?;
    type_conversions()?;
    generic_functions()?;
    error_handling()?;
    program_introspection()?;
    container_operations()?;
    Ok(())
}

fn basic_operations() -> Result<(), Error> {
    println!("1. Function signatures inferred from Rust types");
    let env = Env::builder()
        .declare_variable::<String>("name")?
        .declare_variable::<i64>("age")?
        .register_global_function("greet", |name: &str| format!("Hello, {name}!"))?
        .register_global_function("is_adult", |age: i64| age >= 18)?
        .register_global_function("grade", letter_grade)?
        .build()?;
    let activation = Activation::new()
        .bind_variable("name", "Alice")?
        .bind_variable("age", 25i64)?;
    for (expr, expected) in [
        ("1 + 1", 2i64.into_value()),
        ("greet(name)", "Hello, Alice!".into_value()),
        ("is_adult(age)", true.into_value()),
        ("grade(78.5)", "C".into_value()),
        // Fractional values just below a boundary belong to the lower band.
        ("grade(89.99)", "B".into_value()),
        ("grade(90.0)", "A".into_value()),
        ("grade(100.0)", "A".into_value()),
    ] {
        let result = env.compile(expr)?.evaluate(&activation)?;
        assert_eq!(result, expected, "{expr}");
        println!("  {expr} = {result}");
    }
    let error = env
        .compile("grade(100.01)")?
        .evaluate(())
        .expect_err("out-of-range grade must fail");
    assert_eq!(error.code(), Code::InvalidArgument);
    Ok(())
}

fn variable_operations() -> Result<(), Error> {
    println!("\n2. Variables, lazy providers, and request-specific functions");
    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .declare_global_function::<fn() -> i64>("get_const")?
        .register_global_function("multiply", |x: i64, y: i64| x * y)?
        .build()?;
    let activation = Activation::new()
        .bind_variable("a", 10i64)?
        .bind_variable("b", 20i64)?;
    assert_eq!(
        env.compile("a + b")?.evaluate(&activation)?,
        30i64.into_value()
    );

    let calls = AtomicUsize::new(0);
    // A synchronous provider can borrow state for the lifetime of this activation.
    let activation = Activation::new()
        .bind_variable("a", 5i64)?
        .bind_variable_provider("b", || {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok::<i64, Error>(7)
        })?;
    assert_eq!(
        env.compile("true ? a : b")?.evaluate(&activation)?,
        5i64.into_value()
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        env.compile("a * b")?.evaluate(&activation)?,
        35i64.into_value()
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    println!("  Provider b is invoked only when its branch is evaluated");

    let program = env.compile("get_const() + multiply(a, 3)")?;
    for (constant, expected) in [(100i64, 112i64), (200, 212)] {
        let request = Activation::new()
            .bind_variable("a", 4i64)?
            .bind_global_function("get_const", move || constant)?;
        let result = program.evaluate(&request)?;
        assert_eq!(result, expected.into_value());
        println!("  get_const() = {constant}: {result}");
    }
    Ok(())
}

fn opaque_member_functions() -> Result<(), Error> {
    println!("\n3. Opaque types with explicitly registered member functions");
    let env = Env::builder()
        .declare_variable::<Student>("student")?
        .register_member_function("name", Student::name)?
        .register_member_function("grade", Student::grade)?
        .register_member_function("is_passing", Student::is_passing)?
        .register_member_function("has_subject", Student::has_subject)?
        .register_global_function("grade", letter_grade)?
        .build()?;
    let student = Student {
        name: "John Doe".to_owned(),
        age: 18,
        grade: 89.99,
        subjects: vec!["Math".to_owned(), "Physics".to_owned()],
    };
    let activation = Activation::new().bind_variable("student", student)?;
    for (expr, expected) in [
        ("student.name()", "John Doe".into_value()),
        ("student.grade()", 89.99f64.into_value()),
        ("grade(student.grade())", "B".into_value()),
        ("student.is_passing()", true.into_value()),
        ("student.has_subject('Math')", true.into_value()),
        ("student.has_subject('Chemistry')", false.into_value()),
    ] {
        let result = env.compile(expr)?.evaluate(&activation)?;
        assert_eq!(result, expected, "{expr}");
        println!("  {expr} = {result}");
    }
    // Opaque derives do not expose Rust fields as CEL message fields.
    assert!(env.compile("student.name").is_err());
    Ok(())
}

fn type_conversions() -> Result<(), Error> {
    println!("\n4. Rust return values and conversion back from CEL");
    let env = Env::builder()
        .with_optional(true)
        .register_global_function("text", || "hello world".to_owned())?
        .register_global_function("number", || Ok::<i64, std::io::Error>(42))?
        .register_global_function("numbers", || vec![1i64, 2, 3])?
        .register_global_function("scores", || HashMap::from([("alice", 95i64), ("bob", 87)]))?
        .register_global_function("present", || Some("value".to_owned()))?
        .register_global_function("absent", || None::<String>)?
        .build()?;
    let text = String::try_from(env.compile("text()")?.evaluate(())?)?;
    let number = i64::try_from(env.compile("number()")?.evaluate(())?)?;
    let numbers = Vec::<i64>::try_from(env.compile("numbers()")?.evaluate(())?)?;
    let scores = HashMap::<String, i64>::try_from(env.compile("scores()")?.evaluate(())?)?;
    let present = Option::<String>::try_from(env.compile("present()")?.evaluate(())?)?;
    let absent = Option::<String>::try_from(env.compile("absent()")?.evaluate(())?)?;
    assert_eq!(text, "hello world");
    assert_eq!(number, 42);
    assert_eq!(numbers, vec![1, 2, 3]);
    assert_eq!(
        scores,
        HashMap::from([("alice".to_owned(), 95), ("bob".to_owned(), 87)])
    );
    assert_eq!(present.as_deref(), Some("value"));
    assert_eq!(absent, None);
    println!("  String: {text}; int: {number}; list: {numbers:?}");
    println!("  Present optional: {present:?}; absent optional: {absent:?}");
    // Converting a CEL string to a Rust integer is not implicit parsing.
    assert!(i64::try_from(env.compile("text()")?.evaluate(())?).is_err());
    Ok(())
}

fn generic_functions() -> Result<(), Error> {
    println!("\n5. Generic Rust functions and statically checked overloads");
    fn count<T>(items: Vec<T>) -> i64 {
        items.len() as i64
    }
    let env = Env::builder()
        .declare_variable::<Vec<i64>>("integers")?
        .declare_variable::<Vec<f64>>("floats")?
        .register_global_function("count_strings", count::<String>)?
        .register_global_function("count_ints", count::<i64>)?
        .register_global_function("join", |items: Vec<&str>, separator: &str| {
            items.join(separator)
        })?
        .register_global_function("process", |x: i64| x * 2)?
        .register_global_function("process", |x: f64| (x * 2.0).round())?
        .register_global_function("process", |x: &str| x.to_uppercase())?
        .register_member_function("sum", |items: Vec<i64>| items.iter().sum::<i64>())?
        .register_member_function("sum", |items: Vec<f64>| items.iter().sum::<f64>())?
        .build()?;
    let activation = Activation::new()
        .bind_variable("integers", vec![1i64, 2, 3])?
        .bind_variable("floats", vec![1.5f64, 2.5])?;
    for (expr, expected) in [
        ("count_strings(['hello', 'world'])", 2i64.into_value()),
        ("count_ints(integers)", 3i64.into_value()),
        ("join(['hello', 'world'], ' ')", "hello world".into_value()),
        ("process(42)", 84i64.into_value()),
        ("process(3.14)", 6f64.into_value()),
        ("process('hello')", "HELLO".into_value()),
        ("integers.sum()", 6i64.into_value()),
        ("floats.sum()", 4f64.into_value()),
    ] {
        let result = env.compile(expr)?.evaluate(&activation)?;
        assert_eq!(result, expected, "{expr}");
        println!("  {expr} = {result}");
    }
    // The declared list element type selects the overload even when the list is empty.
    let empty = Activation::new()
        .bind_variable("integers", Vec::<i64>::new())?
        .bind_variable("floats", Vec::<f64>::new())?;
    assert_eq!(
        env.compile("integers.sum()")?.evaluate(&empty)?,
        0i64.into_value()
    );
    assert_eq!(
        env.compile("floats.sum()")?.evaluate(&empty)?,
        0f64.into_value()
    );
    assert!(env.compile("process(true)").is_err());
    Ok(())
}

fn error_handling() -> Result<(), Error> {
    println!("\n6. Custom errors and CEL status codes");
    let env = Env::builder()
        .register_global_function("safe_parse", |s: &str| -> Result<i64, ValidationError> {
            Ok(s.parse()?)
        })?
        .register_global_function(
            "safe_divide",
            |a: i64, b: i64| -> Result<f64, ValidationError> {
                if b == 0 {
                    Err(ValidationError::DivisionByZero)
                } else {
                    Ok(a as f64 / b as f64)
                }
            },
        )?
        .register_global_function(
            "age_group",
            |age: i64| -> Result<&'static str, ValidationError> {
                match age {
                    0..=17 => Ok("minor"),
                    18..=64 => Ok("adult"),
                    65..=120 => Ok("senior"),
                    _ => Err(ValidationError::Age(age)),
                }
            },
        )?
        .register_global_function("denied", || -> Result<bool, Error> {
            Err(Error::permission_denied("access denied"))
        })?
        .build()?;
    for (expr, expected) in [
        ("safe_parse('42')", 42i64.into_value()),
        ("safe_divide(10, 2)", 5f64.into_value()),
        ("age_group(safe_parse('25'))", "adult".into_value()),
        ("age_group(17)", "minor".into_value()),
        ("age_group(18)", "adult".into_value()),
        ("age_group(65)", "senior".into_value()),
        ("age_group(120)", "senior".into_value()),
    ] {
        let result = env.compile(expr)?.evaluate(())?;
        assert_eq!(result, expected, "{expr}");
        println!("  {expr} = {result}");
    }
    // Ordinary Rust errors use Unknown unless their source chain carries a CEL error.
    for (expr, message) in [
        ("safe_parse('invalid')", "Parse error"),
        ("safe_divide(10, 0)", "Division by zero"),
        ("age_group(-1)", "Age must be"),
        ("age_group(121)", "Age must be"),
    ] {
        let error = env
            .compile(expr)?
            .evaluate(())
            .expect_err("validation must fail");
        assert_eq!(error.code(), Code::Unknown);
        assert!(error.message().contains(message), "{error}");
        println!("  {expr}: {error}");
    }
    let error = env
        .compile("denied()")?
        .evaluate(())
        .expect_err("permission denied");
    assert_eq!(error.code(), Code::PermissionDenied);
    assert_eq!(error.message(), "access denied");
    println!("  Explicit CEL status is preserved: {error}");
    Ok(())
}

fn program_introspection() -> Result<(), Error> {
    println!("\n7. Checked return types and reusing compiled expressions");
    let env = Env::builder().declare_variable::<i64>("age")?.build()?;
    for (expr, expected_type, expected_value) in [
        ("age + 1", ValueType::Int, 26i64.into_value()),
        ("string(age)", ValueType::String, "25".into_value()),
        ("age >= 18", ValueType::Bool, true.into_value()),
    ] {
        let program = env.compile(expr)?;
        assert_eq!(program.return_type(), &expected_type);
        let activation = Activation::new().bind_variable("age", 25i64)?;
        let result = program.evaluate(&activation)?;
        assert_eq!(result, expected_value);
        println!("  {expr}: {} = {result}", program.return_type());
    }
    let error = env
        .compile("age + 'years'")
        .expect_err("incompatible operands must fail checking");
    println!("  Compile-time type error: {error}");
    Ok(())
}

fn container_operations() -> Result<(), Error> {
    println!("\n8. Borrowed container values and optional lookup results");
    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<Vec<&str>>("words")?
        .declare_variable::<HashMap<i64, &str>>("lookup_table")?
        .declare_variable::<Vec<Student>>("students")?
        .register_member_function("name", Student::name)?
        .register_member_function("is_adult", |student: &Student| student.age >= 18)?
        .register_global_function("longest", |words: Vec<&str>| {
            words
                .into_iter()
                .max_by_key(|word| word.chars().count())
                .map(str::to_owned)
        })?
        .register_global_function("lookup", |table: HashMap<i64, &str>, key: i64| {
            table.get(&key).map(|value| (*value).to_owned())
        })?
        .build()?;
    let owned_words = [
        "hello".to_owned(),
        "programming".to_owned(),
        "你好".to_owned(),
    ];
    let activation = Activation::new()
        .bind_variable(
            "words",
            owned_words.iter().map(String::as_str).collect::<Vec<_>>(),
        )?
        .bind_variable("lookup_table", HashMap::from([(1i64, "one"), (2, "two")]))?;
    for (expr, expected) in [
        ("longest(words)", Some("programming").into_value()),
        ("lookup(lookup_table, 2)", Some("two").into_value()),
        ("lookup(lookup_table, 9)", None::<String>.into_value()),
        (
            "lookup(lookup_table, 9).orValue('missing')",
            "missing".into_value(),
        ),
    ] {
        let result = env.compile(expr)?.evaluate(&activation)?;
        assert_eq!(result, expected, "{expr}");
        println!("  {expr} = {result}");
    }
    // CEL comprehensions can call registered methods on opaque list elements.
    let students = [("Alice", 18i64), ("Bob", 17), ("Carol", 21)]
        .into_iter()
        .map(|(name, age)| Student {
            name: name.to_owned(),
            age,
            grade: 80.0,
            subjects: vec![],
        })
        .collect::<Vec<_>>();
    let classroom = Activation::new().bind_variable("students", students)?;
    let adults = env
        .compile("students.filter(s, s.is_adult()).map(s, s.name())")?
        .evaluate(&classroom)?;
    assert_eq!(adults, vec!["Alice", "Carol"].into_value());
    println!("  Adult students: {adults}");

    let empty = Activation::new().bind_variable("words", Vec::<String>::new())?;
    let result = env.compile("longest(words)")?.evaluate(&empty)?;
    assert_eq!(result, None::<String>.into_value());
    Ok(())
}
