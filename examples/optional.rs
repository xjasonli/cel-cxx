//! Optional access, construction, fallback, and Rust Option conversion.
//!
//! Run with `cargo run --example optional`.

use cel_cxx::*;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    println!("CEL Optional Types Example\n");

    // Enable optional syntax and the optional standard library together.
    // Both Rust Option<T> and Optional<T> represent CEL optional_type(T).
    let env = Env::builder()
        .with_optional(true)
        .declare_variable::<HashMap<String, String>>("person")?
        .declare_variable::<Vec<String>>("items")?
        .declare_variable::<HashMap<String, i64>>("scores")?
        .declare_variable::<Option<String>>("opt_name")?
        .declare_variable::<Option<i64>>("opt_age")?
        .declare_variable::<HashMap<String, HashMap<String, String>>>("data")?
        .build()?;

    let activation = Activation::new()
        .bind_variable("person", HashMap::from([("name", "Alice")]))?
        .bind_variable("items", vec!["apple", "banana"])?
        .bind_variable("scores", HashMap::from([("alice", 100i64)]))?
        .bind_variable("opt_name", Some("Bob"))?
        .bind_variable("opt_age", None::<i64>)?
        .bind_variable(
            "data",
            HashMap::from([("profile", HashMap::from([("name", "Eve")]))]),
        )?;

    println!("Optional field, map, and list access:");
    for (expr, expected) in [
        ("person.?name", Some("Alice").into_value()),
        ("person.?age", None::<String>.into_value()),
        ("person.?name.orValue('default')", "Alice".into_value()),
        ("person.?age.orValue('default')", "default".into_value()),
        ("person.?name.hasValue()", true.into_value()),
        ("person.?age.hasValue()", false.into_value()),
        ("scores[?'alice']", Some(100i64).into_value()),
        ("scores[?'bob']", None::<i64>.into_value()),
        ("scores[?'bob'].orValue(50)", 50i64.into_value()),
        ("items[?0]", Some("apple").into_value()),
        ("items[?1]", Some("banana").into_value()),
        ("items[?2]", None::<String>.into_value()),
        ("items[?-1]", None::<String>.into_value()),
        ("items[?2].orValue('default')", "default".into_value()),
        ("data.?profile[?'name']", Some("Eve").into_value()),
        ("data.?missing[?'name']", None::<String>.into_value()),
    ] {
        let result = env.compile(expr)?.evaluate(&activation)?;
        assert_eq!(result, expected, "{expr}");
        println!("  {expr} = {result}");
    }

    // Optional entries omit absent values; they do not insert null.
    // Message construction uses the same ?field syntax when a protobuf type is declared.
    println!("\nOptional map and list construction:");
    for (expr, expected) in [
        (
            "{?'name': opt_name, ?'age': opt_age}",
            HashMap::from([("name", "Bob")]).into_value(),
        ),
        (
            "{?'name': optional.of('Chris')}",
            HashMap::from([("name", "Chris")]).into_value(),
        ),
        (
            "{?'age': optional.none()}",
            HashMap::<String, i64>::new().into_value(),
        ),
        (
            "[?opt_name, ?optional.none(), 'fixed']",
            vec!["Bob", "fixed"].into_value(),
        ),
        ("[?opt_age, 0]", vec![0i64].into_value()),
    ] {
        let result = env.compile(expr)?.evaluate(&activation)?;
        assert_eq!(result, expected, "{expr}");
        println!("  {expr} = {result}");
    }

    // Reuse the same program with present, empty, and absent values.
    let program = env.compile("{?'name': opt_name, 'status': 'active'}")?;
    for name in [Some("David"), Some(""), None] {
        let request = Activation::new().bind_variable("opt_name", name)?;
        let mut expected = HashMap::from([("status", "active")]);
        if let Some(name) = name {
            expected.insert("name", name);
        }
        let result = program.evaluate(&request)?;
        assert_eq!(result, expected.into_value());
        println!("  name = {name:?}: {result}");
    }

    // Absence and a present null value are different states.
    for (expr, expected) in [
        ("optional.none().hasValue()", false),
        ("optional.of(null).hasValue()", true),
        ("optional.of(null).value() == null", true),
        ("optional.of(7).orValue(1 / 0) == 7", true),
    ] {
        let result = env.compile(expr)?.evaluate(())?;
        assert_eq!(result, expected.into_value(), "{expr}");
        println!("  {expr} = {result}");
    }

    // ofNonZeroValue wraps non-default values; zero values become none.
    println!("\nOptional creation, chaining, and list access:");
    for (expr, expected) in [
        ("optional.ofNonZeroValue(42).value()", 42i64.into_value()),
        (
            "optional.ofNonZeroValue(0).hasValue()",
            false.into_value(),
        ),
        (
            "optional.ofNonZeroValue('').hasValue()",
            false.into_value(),
        ),
        // or() falls back to another optional and stays lazy.
        (
            "optional.of('Alice').or(optional.of('Bob')).value()",
            "Alice".into_value(),
        ),
        (
            "optional.none().or(optional.of('Bob')).value()",
            "Bob".into_value(),
        ),
        (
            "optional.none().or(optional.none()).hasValue()",
            false.into_value(),
        ),
        // optMap transforms the wrapped value; optFlatMap flattens nested optionals.
        (
            "optional.of(21).optMap(x, x * 2).value()",
            42i64.into_value(),
        ),
        (
            "optional.none().optMap(x, x * 2).hasValue()",
            false.into_value(),
        ),
        (
            "optional.of(optional.of(7)).optFlatMap(x, x).value()",
            7i64.into_value(),
        ),
        // first()/last() return optionals; empty lists produce none.
        ("[10, 20, 30].first().value()", 10i64.into_value()),
        ("[10, 20, 30].last().value()", 30i64.into_value()),
        ("[].first().hasValue()", false.into_value()),
        ("[].last().hasValue()", false.into_value()),
        ("[].first().orValue(-1)", (-1i64).into_value()),
    ] {
        let result = env.compile(expr)?.evaluate(())?;
        assert_eq!(result, expected, "{expr}");
        println!("  {expr} = {result}");
    }

    // optional.unwrap flattens a list of optionals, dropping absent entries.
    let unwrapped = env
        .compile("optional.unwrap([optional.of(1), optional.none(), optional.of(3)])")?
        .evaluate(())?;
    assert_eq!(unwrapped, vec![1i64, 3].into_value());
    println!("  optional.unwrap([of(1), none, of(3)]) = {unwrapped}");

    let extract = env.compile("person.?name")?;
    let name = Option::<String>::try_from(extract.evaluate(&activation)?)?;
    assert_eq!(name.as_deref(), Some("Alice"));
    println!("\nConverted to Rust Option<String>: {name:?}");

    let error = env
        .compile("optional.none().value()")?
        .evaluate(())
        .expect_err("extracting an absent optional must fail");
    assert_eq!(error.code(), Code::FailedPrecondition);
    println!("Extracting an absent value: {error}");
    Ok(())
}
