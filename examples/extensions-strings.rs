//! String extensions for indexing, transformation, formatting, and input normalization.
//!
//! Includes Unicode code-point indexing, embedded NULs, empty inputs, and errors.
//! Run with: `cargo run --example extensions-strings`

use cel_cxx::*;

fn check<T: IntoValue>(
    env: &Env,
    activation: &Activation,
    expression: &str,
    expected: T,
) -> Result<(), Error> {
    let result = env.compile(expression)?.evaluate(activation)?;
    assert_eq!(result, expected.into_value(), "{expression}");
    println!("  {expression} => {result}");
    Ok(())
}

fn main() -> Result<(), Error> {
    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("text")?
        .declare_variable::<String>("search")?
        .declare_variable::<String>("replacement")?
        .declare_variable::<i64>("start")?
        .declare_variable::<i64>("end")?
        .declare_variable::<Vec<String>>("words")?
        .build()?;

    positions(&env)?;
    transformations(&env)?;
    split_and_join(&env)?;
    replacement(&env)?;
    quote_and_format(&env)?;
    normalize_inputs(&env)?;
    expected_errors(&env)?;
    Ok(())
}

fn positions(env: &Env) -> Result<(), Error> {
    println!("Character positions and substrings");
    let activation = Activation::new()
        .bind_variable("text", "Hello, World! Hello, CEL!")?
        .bind_variable("start", 7i64)?
        .bind_variable("end", 12i64)?;
    for (expression, expected) in [
        ("text.charAt(0)", "H"),
        ("text.charAt(text.size() - 1)", "!"),
        // An index equal to the length is valid and returns an empty string.
        ("text.charAt(text.size())", ""),
        ("text.substring(start, end)", "World"),
        ("text.substring(14)", "Hello, CEL!"),
        ("text.substring(7, 7)", ""),
    ] {
        check(env, &activation, expression, expected)?;
    }
    for (expression, expected) in [
        ("text.indexOf('Hello')", 0i64),
        ("text.indexOf('Hello', 5)", 14),
        ("text.indexOf('missing')", -1),
        ("text.lastIndexOf('Hello')", 14),
        // The second argument is an inclusive upper bound on the match start.
        ("text.lastIndexOf('l', 10)", 10),
        ("text.lastIndexOf('missing')", -1),
        ("text.indexOf('')", 0),
        ("text.lastIndexOf('')", 25),
    ] {
        check(env, &activation, expression, expected)?;
    }

    // CEL string offsets count Unicode code points, unlike Rust byte offsets.
    // The embedded NUL remains part of the string through the FFI boundary.
    let text = "Aé中\0Z";
    assert_eq!(text.len(), 8);
    let unicode = Activation::new().bind_variable("text", text)?;
    check(env, &unicode, "text.size()", 5i64)?;
    check(env, &unicode, "text.charAt(1)", "é")?;
    check(env, &unicode, "text.charAt(3)", "\0")?;
    check(env, &unicode, "text.indexOf('中')", 2i64)?;
    check(env, &unicode, "text.substring(1, 4)", "é中\0")?;
    Ok(())
}

fn transformations(env: &Env) -> Result<(), Error> {
    println!("Whitespace, ASCII case conversion, and reversal");
    let activation = Activation::new();
    for (expression, expected) in [
        ("'  Hello World  '.trim()", "Hello World"),
        ("'\\t\\nHello\\r\\n'.trim()", "Hello"),
        ("'   '.trim()", ""),
        ("'MiXeD CaSe'.lowerAscii()", "mixed case"),
        ("'MiXeD CaSe'.upperAscii()", "MIXED CASE"),
        // Only ASCII letters change case; these functions are not Unicode case folding.
        ("'ÉCOLE Straße'.lowerAscii()", "École straße"),
        ("'ÉCOLE Straße'.upperAscii()", "ÉCOLE STRAßE"),
        ("'Hello, World!'.reverse()", "!dlroW ,olleH"),
        ("'é中'.reverse()", "中é"),
        ("''.reverse()", ""),
    ] {
        check(env, &activation, expression, expected)?;
    }

    // Reversal operates on code points, so a combining mark moves independently
    // of the base letter. It does not preserve visual grapheme clusters.
    let combining = Activation::new().bind_variable("text", "e\u{301}")?;
    check(env, &combining, "text.reverse()", "\u{301}e")?;
    Ok(())
}

fn split_and_join(env: &Env) -> Result<(), Error> {
    println!("Splitting and joining, including empty elements");
    let activation = Activation::new().bind_variable("words", vec!["Hello", "World", "CEL"])?;
    for (expression, expected) in [
        ("['a', 'b', 'c'].join()", "abc"),
        ("words.join(' ')", "Hello World CEL"),
        ("['single'].join(',')", "single"),
        ("[].join(',')", ""),
        ("['', 'middle', ''].join(',')", ",middle,"),
    ] {
        check(env, &activation, expression, expected)?;
    }
    for (expression, expected) in [
        ("'a,b,c'.split(',')", vec!["a", "b", "c"]),
        // A positive limit caps the number of resulting elements, keeping the tail intact.
        (
            "'one-two-three-four'.split('-', 2)",
            vec!["one", "two-three-four"],
        ),
        ("'a::b::c'.split('::')", vec!["a", "b", "c"]),
        ("'no-separators'.split(',')", vec!["no-separators"]),
        ("''.split(',')", vec![""]),
        ("',middle,'.split(',')", vec!["", "middle", ""]),
        ("'é中'.split('')", vec!["é", "中"]),
        ("'a,b,c'.split(',', 0)", vec![]),
        ("'a,b,c'.split(',', -1)", vec!["a", "b", "c"]),
    ] {
        check(env, &activation, expression, expected)?;
    }
    Ok(())
}

fn replacement(env: &Env) -> Result<(), Error> {
    println!("Literal replacement and replacement limits");
    let activation = Activation::new()
        .bind_variable("text", "Hello World, Hello CEL!")?
        .bind_variable("search", "Hello")?
        .bind_variable("replacement", "Hi")?;
    for (expression, expected) in [
        ("text.replace(search, replacement)", "Hi World, Hi CEL!"),
        (
            "text.replace(search, replacement, 1)",
            "Hi World, Hello CEL!",
        ),
        (
            "text.replace(search, replacement, 0)",
            "Hello World, Hello CEL!",
        ),
        ("text.replace(search, replacement, -1)", "Hi World, Hi CEL!"),
        ("'no match'.replace('xyz', 'abc')", "no match"),
        ("''.replace('a', 'b')", ""),
        // Replacement treats punctuation literally; it does not interpret a regex.
        ("'a.b.c'.replace('.', '/')", "a/b/c"),
    ] {
        check(env, &activation, expression, expected)?;
    }
    Ok(())
}

fn quote_and_format(env: &Env) -> Result<(), Error> {
    println!("CEL string quoting and formatted output");
    let activation = Activation::new();
    for (expression, expected) in [
        ("strings.quote('Hello World')", "\"Hello World\""),
        ("strings.quote('Line 1\\nLine 2')", "\"Line 1\\nLine 2\""),
        (
            "strings.quote('Quote: \"Hello\"')",
            "\"Quote: \\\"Hello\\\"\"",
        ),
        ("strings.quote('')", "\"\""),
        ("'Hello, %s!'.format(['World'])", "Hello, World!"),
        ("'Number: %d'.format([42])", "Number: 42"),
        ("'Float: %.2f'.format([3.14159])", "Float: 3.14"),
        (
            "'%s has %d apples'.format(['Alice', 5])",
            "Alice has 5 apples",
        ),
        ("'Progress: %d%%'.format([50])", "Progress: 50%"),
        ("'No placeholders'.format([])", "No placeholders"),
    ] {
        check(env, &activation, expression, expected)?;
    }

    // strings.quote produces a CEL string literal, including escapes for control
    // characters. Evaluate that literal to verify it preserves the original input.
    let text = "line\n\"quoted\"\0\\é";
    let activation = Activation::new().bind_variable("text", text)?;
    let quoted = env.compile("strings.quote(text)")?.evaluate(&activation)?;
    let literal = String::from_value(&quoted)?;
    check(env, &Activation::new(), &literal, text)?;
    Ok(())
}

fn normalize_inputs(env: &Env) -> Result<(), Error> {
    println!("Reuse a normalization program across inputs");
    // This simple comma-delimited format has no quoting or escaping rules.
    // Empty fields are discarded deliberately; use a CSV parser for full CSV.
    let program = env.compile(
        "text.split(',').map(word, word.trim().lowerAscii()).filter(word, word != '').join(',')",
    )?;
    for (text, expected) in [
        ("  Alpha, BETA ,, Gamma  ", "alpha,beta,gamma"),
        ("", ""),
        (" , ", ""),
        ("ÉCOLE, CEL", "École,cel"),
    ] {
        let activation = Activation::new().bind_variable("text", text)?;
        let result = program.evaluate(&activation)?;
        assert_eq!(result, Value::String(expected.into()));
        println!("  {text:?} => {result}");
    }

    // Split only once when a value may itself contain the separator.
    check(
        env,
        &Activation::new(),
        "'url:https://example.com'.split(':', 2)",
        vec!["url", "https://example.com"],
    )?;
    Ok(())
}

fn expected_errors(env: &Env) -> Result<(), Error> {
    println!("Invalid arguments produce CEL errors");
    let activation = Activation::new();
    for expression in [
        "'abc'.charAt(-1)",
        "'abc'.charAt(4)",
        "'abc'.substring(2, 1)",
        "'abc'.substring(0, 4)",
        "'abc'.indexOf('a', -1)",
        "'abc'.lastIndexOf('a', -1)",
        "'%d'.format(['not a number'])",
        "'%'.format([])",
    ] {
        let error = env
            .compile(expression)?
            .evaluate(&activation)
            .expect_err("invalid string operation must return an error");
        assert_eq!(error.code(), Code::InvalidArgument, "{expression}: {error}");
        println!("  {expression} => {error}");
    }
    Ok(())
}
