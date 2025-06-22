//! Basic example demonstrating core cel-cxx functionality

use cel_cxx::*;

type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<(), Error> {
    println!("🚀 CEL-CXX Basic Example\n");

    // Create an environment with variables and functions
    let env = Env::builder()
        .declare_variable::<String>("name")?
        .declare_variable::<i64>("age")?
        .register_global_function("greet", |name: &str| {
            format!("Hello, {}!", name)
        })?
        .register_global_function("is_adult", |age: i64| age >= 18)?
        .build()?;

    // Compile and evaluate expressions
    let expressions = vec![
        "greet(name)",
        "is_adult(age)",
        "'Name: ' + name + ', Age: ' + string(age)",
        "age >= 18 ? 'adult' : 'minor'",
    ];

    let activation = Activation::new()
        .bind_variable("name", "Alice")?
        .bind_variable("age", 25i64)?;

    for expr in expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("{} = {}", expr, result);
    }

    println!("\n✅ Basic example completed!");
    Ok(())
}


