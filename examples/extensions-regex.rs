//! Optional substring extraction, repeated matches, and regex replacement.
//! Run with `cargo run --example extensions-regex`.

use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("CEL regex extension");
    let env = Env::builder()
        .with_ext_regex(true)
        .with_ext_bindings(true)
        .with_optional(true)
        .declare_variable::<String>("text")?
        .declare_variable::<String>("pattern")?
        .build()?;

    // extract searches within the input. No match produces optional.none(); one
    // capturing group returns that group, while no group returns the whole match.
    for (expression, expected) in [
        (
            r"regex.extract('Hello World 123', r'\d+').orValue('missing')",
            "123",
        ),
        (
            r"regex.extract('Hello World 123', r'Hello (\w+)').orValue('missing')",
            "World",
        ),
        (
            "regex.extract('Hello', 'xyz').orValue('missing')",
            "missing",
        ),
    ] {
        show(&env, expression, expected)?;
    }
    show(&env, "regex.extract('', 'xyz').hasValue()", false)?;
    show(&env, "regex.extract('', '^$').hasValue()", true)?;
    show(&env, "regex.extract('', '^$').value()", "")?;
    show(
        &env,
        r"regex.extractAll('Contact john@example.com or admin@test.org', r'[\w.+-]+@[\w.-]+\.[a-zA-Z]{2,}')",
        vec!["john@example.com", "admin@test.org"],
    )?;
    show(
        &env,
        r"regex.extractAll('id:123, id:456, id:789', r'id:(\d+)')",
        vec!["123", "456", "789"],
    )?;
    show(
        &env,
        r"regex.extractAll('no matches', r'\d+')",
        Vec::<String>::new(),
    )?;
    show(&env, "regex.extractAll('abc', '')", Vec::<String>::new())?;

    for (expression, expected) in [
        (
            "regex.replace('hello world hello', 'hello', 'hi')",
            "hi world hi",
        ),
        ("regex.replace('banana', 'a', 'x', 0)", "banana"),
        ("regex.replace('banana', 'a', 'x', 1)", "bxnana"),
        ("regex.replace('banana', 'a', 'x', 2)", "bxnxna"),
        ("regex.replace('banana', 'a', 'x', -1)", "bxnxnx"),
        (
            r"regex.replace('John Doe', r'(\w+) (\w+)', r'\2, \1')",
            "Doe, John",
        ),
        (
            r"regex.replace('2023-12-25', r'(\d{4})-(\d{2})-(\d{2})', r'\3/\2/\1')",
            "25/12/2023",
        ),
    ] {
        show(&env, expression, expected)?;
    }

    // Format ten digits and preserve other inputs. Counting first protects indexing.
    let phone = env.compile(
        r#"cel.bind(digits, regex.extractAll(text, r'\d'),
             digits.size() == 10 ?
               '(' + digits[0] + digits[1] + digits[2] + ') ' +
               digits[3] + digits[4] + digits[5] + '-' +
               digits[6] + digits[7] + digits[8] + digits[9] : text)"#,
    )?;
    for (text, expected) in [
        ("555-123-4567", "(555) 123-4567"),
        ("15551234567", "15551234567"),
        ("", ""),
    ] {
        let activation = Activation::new().bind_variable("text", text)?;
        let result = phone.evaluate(&activation)?;
        assert_eq!(result, Value::from(expected));
        println!("Phone {text:?}: {result}");
    }

    // Extracting pieces is not address validation: anchor a complete policy pattern.
    let email = env.compile(r"text.matches(r'^[^\s@]+@[^\s@]+\.[^\s@]+$')")?;
    for (text, expected) in [
        ("test.user@company.com", true),
        ("a@b@c.com", false),
        ("missing-domain@", false),
    ] {
        let activation = Activation::new().bind_variable("text", text)?;
        assert_eq!(email.evaluate(&activation)?, Value::Bool(expected));
    }
    show(
        &env,
        r"regex.extract('2023-12-25 14:30:15 [ERROR] Connection failed', r'\[(\w+)\]').orValue('INFO')",
        "ERROR",
    )?;
    show(
        &env,
        r"regex.replace('Hello   World\tCEL', r'\s+', ' ')",
        "Hello World CEL",
    )?;

    // Malformed patterns are rejected; regex.extract accepts at most one capture group.
    let extract = env.compile("regex.extract(text, pattern)")?;
    for pattern in ["[", "(a)(b)"] {
        let activation = Activation::new()
            .bind_variable("text", "ab")?
            .bind_variable("pattern", pattern)?;
        let error = extract
            .evaluate(&activation)
            .expect_err("invalid pattern must fail");
        assert_eq!(error.code(), Code::InvalidArgument);
        println!("Rejected {pattern:?}: {error}");
    }
    Ok(())
}

fn show(env: &Env, expression: &str, expected: impl Into<Value>) -> Result<(), Error> {
    let result = env.compile(expression)?.evaluate(Activation::new())?;
    assert_eq!(result, expected.into(), "{expression}");
    println!("{expression} => {result}");
    Ok(())
}
