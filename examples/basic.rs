//! Declare variables and Rust functions, then reuse compiled CEL programs.
//!
//! Run with `cargo run --example basic`.

use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("CEL-CXX Basic Example\n");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("name")?
        .declare_variable::<i64>("age")?
        .declare_variable::<f64>("weight")?
        .declare_variable::<f64>("height")?
        .declare_variable::<Vec<String>>("hobbies")?
        .register_global_function("greet", |name: &str| format!("Hello, {name}!"))?
        .register_global_function("is_adult", |age: i64| age >= 18)?
        .register_global_function("calculate_bmi", |weight: f64, height: f64| {
            if !weight.is_finite() || !height.is_finite() || weight <= 0.0 || height <= 0.0 {
                return Err(Error::invalid_argument(
                    "weight and height must be finite and positive",
                ));
            }
            Ok(weight / (height * height))
        })?
        .register_global_function("format_hobbies", |hobbies: Vec<&str>| hobbies.join(", "))?
        .build()?;

    let activation = Activation::new()
        .bind_variable("name", "Alice")?
        .bind_variable("age", 25i64)?
        .bind_variable("weight", 70.0f64)?
        .bind_variable("height", 1.75f64)?
        .bind_variable("hobbies", vec!["reading", "swimming", "coding"])?;

    for (expr, expected) in [
        ("greet(name)", "Hello, Alice!".into_value()),
        ("is_adult(age)", true.into_value()),
        (
            "'Name: %s, Age: %d'.format([name, age])",
            "Name: Alice, Age: 25".into_value(),
        ),
        ("age >= 18 ? 'adult' : 'minor'", "adult".into_value()),
        (
            "calculate_bmi(weight, height)",
            (70.0 / (1.75 * 1.75)).into_value(),
        ),
        (
            "format_hobbies(hobbies)",
            "reading, swimming, coding".into_value(),
        ),
        (
            "'%d hobbies found'.format([size(hobbies)])",
            "3 hobbies found".into_value(),
        ),
    ] {
        let result = env.compile(expr)?.evaluate(&activation)?;
        assert_eq!(result, expected, "{expr}");
        println!("{expr} = {result}");
    }

    // Compile once and bind different request data for each evaluation.
    let eligibility = env.compile("is_adult(age)")?;
    for (age, expected) in [(17i64, false), (18, true), (65, true)] {
        let request = Activation::new().bind_variable("age", age)?;
        assert_eq!(eligibility.evaluate(&request)?, expected.into_value());
        println!("Age {age}: eligible = {expected}");
    }

    // Compile errors and evaluation errors occur at different stages.
    let type_error = env
        .compile("age + name")
        .expect_err("int + string must fail checking");
    println!("Type checking rejected age + name: {type_error}");
    let invalid_input = Activation::new()
        .bind_variable("weight", 70.0f64)?
        .bind_variable("height", 0.0f64)?;
    let error = env
        .compile("calculate_bmi(weight, height)")?
        .evaluate(&invalid_input)
        .expect_err("zero height must be rejected");
    assert_eq!(error.code(), Code::InvalidArgument);
    println!("Invalid input: {error}");

    Ok(())
}
