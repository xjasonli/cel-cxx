//! Local bindings, lexical scope, and reusable expressions.
//! Run with `cargo run --example extensions-bindings`.

use cel_cxx::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() -> Result<(), Error> {
    println!("CEL bindings extension");
    let calls = Arc::new(AtomicUsize::new(0));
    let function_calls = calls.clone();
    let env = Env::builder()
        .with_ext_bindings(true)
        .with_ext_strings(true)
        .declare_variable::<String>("name")?
        .declare_variable::<i64>("age")?
        .declare_variable::<Vec<i64>>("numbers")?
        .register_global_function("load_score", move || {
            function_calls.fetch_add(1, Ordering::SeqCst);
            21i64
        })?
        .build()?;

    let greeting = env.compile("cel.bind(greeting, 'Hello, ' + name, greeting + '!')")?;
    let activation = Activation::new().bind_variable("name", "Alice")?;
    let result = greeting.evaluate(&activation)?;
    assert_eq!(result, Value::from("Hello, Alice!"));
    println!("Greeting: {result}");

    // A binding is evaluated once even when its value is referenced repeatedly.
    let cached = env.compile("cel.bind(score, load_score(), score + score)")?;
    assert_eq!(cached.evaluate(())?, Value::Int(42));
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    // Names are scoped to the binding body. An inner binding can shadow an outer one.
    let scoped = env.compile("cel.bind(x, 2, cel.bind(x, x + 3, x * 2) + x)")?;
    assert_eq!(scoped.evaluate(())?, Value::Int(12));
    assert!(env.compile("cel.bind(local, 1, local) + local").is_err());

    // Reuse a filtered collection without assuming how many items survive.
    let processing = env.compile(
        r#"cel.bind(selected, numbers.filter(n, n > 2),
             cel.bind(squares, selected.map(n, n * n),
               {"count": squares.size(), "squares": squares}
             ))"#,
    )?;
    for numbers in [vec![1i64, 2, 3, 4, 5], vec![1, 2], vec![]] {
        let expected_squares = numbers
            .iter()
            .copied()
            .filter(|n| *n > 2)
            .map(|n| Value::Int(n * n))
            .collect::<Vec<_>>();
        let expected = Value::Map(std::collections::HashMap::from([
            (
                MapKey::from("count"),
                Value::Int(expected_squares.len() as i64),
            ),
            (MapKey::from("squares"), Value::List(expected_squares)),
        ]));
        let activation = Activation::new().bind_variable("numbers", numbers)?;
        let result = processing.evaluate(&activation)?;
        assert_eq!(result, expected);
        println!("Filtered squares: {result}");
    }

    // Compile once and evaluate the age boundaries with independent activations.
    let category = env.compile(
        "cel.bind(adult, age >= 18, cel.bind(senior, age >= 65, \
         senior ? 'senior' : adult ? 'adult' : 'minor'))",
    )?;
    for (age, expected) in [
        (17i64, "minor"),
        (18, "adult"),
        (64, "adult"),
        (65, "senior"),
    ] {
        let activation = Activation::new().bind_variable("age", age)?;
        let result = category.evaluate(&activation)?;
        assert_eq!(result, Value::from(expected));
        println!("Age {age}: {result}");
    }

    // Normalize the input once before using it in several checks.
    let validate = env.compile(
        "cel.bind(normalized, name.trim(), \
         normalized.size() > 0 && normalized.size() <= 50)",
    )?;
    for (name, expected) in [(" Alice ", true), ("   ", false), ("", false)] {
        let activation = Activation::new().bind_variable("name", name)?;
        assert_eq!(validate.evaluate(&activation)?, Value::Bool(expected));
    }
    Ok(())
}
