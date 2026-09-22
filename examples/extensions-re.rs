//! The C++ `re` extension captures groups, rewrites matched text, and reports errors.
//! Run with `cargo run --example extensions-re`.

use cel_cxx::*;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    println!("CEL re extension");
    let env = Env::builder()
        .with_ext_re(true)
        .declare_variable::<String>("text")?
        .declare_variable::<String>("pattern")?
        .build()?;

    // Rust and CEL raw strings keep the regular expression and rewrite readable.
    show(
        &env,
        r#"re.extract("Hello World", r"(\w+) (\w+)", r"\2, \1")"#,
        "World, Hello",
    )?;
    show(
        &env,
        r#"re.extract("2023-12-25", r"(\d{4})-(\d{2})-(\d{2})", r"\2/\3/\1")"#,
        "12/25/2023",
    )?;
    show(
        &env,
        r#"re.extract("(555) 123-4567", r"\((\d{3})\) (\d{3})-(\d{4})", r"\1-\2-\3")"#,
        "555-123-4567",
    )?;

    // capture returns the first capturing group and requires a full-string match.
    show(
        &env,
        r#"re.capture("user@example.com", r"(\w+)@[\w.]+")"#,
        "user",
    )?;
    show(
        &env,
        r#"re.capture("Price: $29.99", r"Price: \$(\d+\.\d+)")"#,
        "29.99",
    )?;
    show(
        &env,
        r#"re.capture("[ERROR] Something went wrong", r"\[(\w+)\].*")"#,
        "ERROR",
    )?;

    // captureN uses a group's name, or its one-based index as a string if unnamed.
    show(
        &env,
        r#"re.captureN("user@example.com", r"(\w+)@([\w.]+)")"#,
        HashMap::from([("1", "user"), ("2", "example.com")]),
    )?;
    show(
        &env,
        r#"re.captureN("John Doe", r"(?P<first>\w+) (?P<last>\w+)")"#,
        HashMap::from([("first", "John"), ("last", "Doe")]),
    )?;
    show(
        &env,
        r#"re.captureN("Version 1.2.3", r"Version (?P<major>\d+)\.(\d+)\.(\d+)")"#,
        HashMap::from([("major", "1"), ("2", "2"), ("3", "3")]),
    )?;

    // Reuse one compiled parser for log records supplied through the activation.
    let log = env.compile(r#"re.captureN(text, r"\[(?P<level>\w+)\] (?P<message>.*)")"#)?;
    for (text, level, message) in [
        ("[INFO] Ready", "INFO", "Ready"),
        (
            "[ERROR] Database connection failed",
            "ERROR",
            "Database connection failed",
        ),
        ("[INFO] ", "INFO", ""),
    ] {
        let activation = Activation::new().bind_variable("text", text)?;
        let result = log.evaluate(&activation)?;
        assert_eq!(
            result,
            Value::from(HashMap::from([("level", level), ("message", message)]))
        );
        println!("Parsed log: {result}");
    }

    // Dynamic patterns are validated during evaluation. Compilation must still succeed.
    let capture = env.compile("re.capture(text, pattern)")?;
    for (text, pattern) in [
        ("test", "["),
        ("no numbers here", r"(\d+)"),
        ("prefix 123 suffix", r"(\d+)"), // A substring match is insufficient.
        ("test", "test"),                // No capturing group.
    ] {
        let activation = Activation::new()
            .bind_variable("text", text)?
            .bind_variable("pattern", pattern)?;
        let error = capture
            .evaluate(&activation)
            .expect_err("invalid capture must fail");
        assert_eq!(error.code(), Code::InvalidArgument);
        println!("Rejected {text:?} with {pattern:?}: {error}");
    }
    for expression in [
        r#"re.captureN("test", "test")"#,
        r#"re.extract("test", "(test)", r"\2")"#,
    ] {
        let error = env
            .compile(expression)?
            .evaluate(())
            .expect_err("invalid capture/rewrite must fail");
        assert_eq!(error.code(), Code::InvalidArgument);
    }
    Ok(())
}

fn show(env: &Env, expression: &str, expected: impl Into<Value>) -> Result<(), Error> {
    let result = env.compile(expression)?.evaluate(Activation::new())?;
    assert_eq!(result, expected.into(), "{expression}");
    println!("{expression} => {result}");
    Ok(())
}
