//! Basic example demonstrating core cel-cxx functionality

use cel_cxx::*;

type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<(), Error> {
    println!("🚀 CEL-CXX Basic Example\n");

    // Create an environment with variables and functions
    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("name")?
        .declare_variable::<i64>("age")?
        .declare_variable::<f64>("weight")?
        .declare_variable::<f64>("height")?
        .declare_variable::<Vec<String>>("hobbies")?
        .register_global_function("greet", |name: &str| format!("Hello, {name}!"))?
        .register_global_function("is_adult", |age: i64| age >= 18)?
        .register_global_function("calculate_bmi", |weight: f64, height: f64| weight / (height * height))?
        .register_global_function("format_hobbies", |hobbies: Vec<&str>| hobbies.join(", "))?
        .build()?;

    // Compile and evaluate expressions
    let expressions = vec![
        "greet(name)",
        "is_adult(age)",
        "'Name: %s, Age: %d'.format([name, age])",
        "age >= 18 ? 'adult' : 'minor'",
        "calculate_bmi(weight, height)",
        "format_hobbies(hobbies)",
        "'%d hobbies found'.format([size(hobbies)])",
    ];

    let activation = Activation::new()
        .bind_variable("name", "Alice")?
        .bind_variable("age", 25i64)?
        .bind_variable("weight", 70.0f64)?
        .bind_variable("height", 1.75f64)?
        .bind_variable("hobbies", vec!["reading".to_string(), "swimming".to_string(), "coding".to_string()])?;

    for expr in expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("{expr} = {result}");
    }

    println!("\n✅ Basic example completed!");
    Ok(())
}
