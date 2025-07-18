use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("🔧 CEL String Extensions Example");
    println!("=================================");
    
    // Build environment with string extensions enabled
    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("text")?
        .declare_variable::<String>("search")?
        .declare_variable::<String>("replacement")?
        .declare_variable::<i64>("index")?
        .declare_variable::<i64>("start")?
        .declare_variable::<i64>("end")?
        .declare_variable::<Vec<String>>("words")?
        .declare_variable::<String>("email")?
        .declare_variable::<String>("data")?
        .build()?;

    // Demo 1: charAt function
    println!("\n📌 Demo 1: charAt Function");
    demo_char_at(&env)?;
    
    // Demo 2: indexOf functions
    println!("\n📌 Demo 2: indexOf Functions");
    demo_index_of(&env)?;
    
    // Demo 3: lastIndexOf functions
    println!("\n📌 Demo 3: lastIndexOf Functions");
    demo_last_index_of(&env)?;
    
    // Demo 4: strings.quote function
    println!("\n📌 Demo 4: strings.quote Function");
    demo_strings_quote(&env)?;
    
    // Demo 5: substring functions
    println!("\n📌 Demo 5: substring Functions");
    demo_substring(&env)?;
    
    // Demo 6: trim function
    println!("\n📌 Demo 6: trim Function");
    demo_trim(&env)?;
    
    // Demo 7: join function
    println!("\n📌 Demo 7: join Function");
    demo_join(&env)?;
    
    // Demo 8: split function
    println!("\n📌 Demo 8: split Function");
    demo_split(&env)?;
    
    // Demo 9: Case conversion functions
    println!("\n📌 Demo 9: Case Conversion Functions");
    demo_case_conversion(&env)?;
    
    // Demo 10: replace function
    println!("\n📌 Demo 10: replace Function");
    demo_replace(&env)?;
    
    // Demo 11: format function
    println!("\n📌 Demo 11: format Function");
    demo_format(&env)?;
    
    // Demo 12: reverse function
    println!("\n📌 Demo 12: reverse Function");
    demo_reverse(&env)?;
    
    // Demo 13: Complex string processing
    println!("\n📌 Demo 13: Complex String Processing");
    demo_complex_processing(&env)?;

    println!("\n✅ All string extension examples completed successfully!");
    Ok(())
}

fn demo_char_at(env: &Env) -> Result<(), Error> {
    println!("  Testing charAt function");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello, World!")?
        .bind_variable("index", 0)?;
    
    let test_cases = vec![
        ("text.charAt(0)", "Get first character"),
        ("text.charAt(7)", "Get character at index 7"),
        ("text.charAt(text.size() - 1)", "Get last character"),
        ("'CEL'.charAt(1)", "Get character from literal string"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_index_of(env: &Env) -> Result<(), Error> {
    println!("  Testing indexOf functions");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello, World! Hello, CEL!")?
        .bind_variable("search", "Hello")?;
    
    let test_cases = vec![
        ("text.indexOf('o')", "Find first 'o'"),
        ("text.indexOf('Hello')", "Find first 'Hello'"),
        ("text.indexOf('xyz')", "Find non-existent substring"),
        ("text.indexOf('Hello', 5)", "Find 'Hello' starting from index 5"),
        ("text.indexOf('o', 10)", "Find 'o' starting from index 10"),
        ("'programming'.indexOf('gram')", "Find in literal string"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_last_index_of(env: &Env) -> Result<(), Error> {
    println!("  Testing lastIndexOf functions");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello, World! Hello, CEL!")?;
    
    let test_cases = vec![
        ("text.lastIndexOf('o')", "Find last 'o'"),
        ("text.lastIndexOf('Hello')", "Find last 'Hello'"),
        ("text.lastIndexOf('xyz')", "Find non-existent substring"),
        ("text.lastIndexOf('l', 10)", "Find last 'l' before index 10"),
        ("'banana'.lastIndexOf('a')", "Find last 'a' in 'banana'"),
        ("'abcabc'.lastIndexOf('abc')", "Find last 'abc' occurrence"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_strings_quote(env: &Env) -> Result<(), Error> {
    println!("  Testing strings.quote function");
    
    let activation = Activation::new();
    
    let test_cases = vec![
        ("strings.quote('Hello World')", "Quote simple string"),
        ("strings.quote('Line 1\\nLine 2')", "Quote string with newline"),
        ("strings.quote('Tab\\there')", "Quote string with tab"),
        ("strings.quote('Quote: \"Hello\"')", "Quote string with quotes"),
        ("strings.quote('Backslash: \\\\')", "Quote string with backslash"),
        ("strings.quote('')", "Quote empty string"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_substring(env: &Env) -> Result<(), Error> {
    println!("  Testing substring functions");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello, World!")?
        .bind_variable("start", 7)?
        .bind_variable("end", 12)?;
    
    let test_cases = vec![
        ("text.substring(0, 5)", "Get first 5 characters"),
        ("text.substring(7)", "Get substring from index 7 to end"),
        ("text.substring(7, 12)", "Get substring from index 7 to 12"),
        ("text.substring(start, end)", "Get substring using variables"),
        ("'Programming'.substring(0, 7)", "Get 'Program' from 'Programming'"),
        ("'CEL Language'.substring(4)", "Get 'Language' from 'CEL Language'"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_trim(env: &Env) -> Result<(), Error> {
    println!("  Testing trim function");
    
    let activation = Activation::new();
    
    let test_cases = vec![
        ("'  Hello World  '.trim()", "Trim spaces from both ends"),
        ("'\\t\\nHello\\r\\n'.trim()", "Trim various whitespace characters"),
        ("'   '.trim()", "Trim string with only spaces"),
        ("'NoSpaces'.trim()", "Trim string with no spaces"),
        ("'  Leading'.trim()", "Trim leading spaces"),
        ("'Trailing  '.trim()", "Trim trailing spaces"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> '{result}'");
    }
    
    Ok(())
}

fn demo_join(env: &Env) -> Result<(), Error> {
    println!("  Testing join function");
    
    let activation = Activation::new()
        .bind_variable("words", vec!["Hello".to_string(), "World".to_string(), "CEL".to_string()])?;
    
    let test_cases = vec![
        ("['a', 'b', 'c'].join()", "Join with default separator"),
        ("['a', 'b', 'c'].join(', ')", "Join with comma separator"),
        ("words.join(' ')", "Join words with space"),
        ("words.join('-')", "Join words with dash"),
        ("['single'].join(',')", "Join single element"),
        ("[].join(',')", "Join empty list"),
        ("['one', 'two', 'three'].join(' | ')", "Join with pipe separator"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_split(env: &Env) -> Result<(), Error> {
    println!("  Testing split function");
    
    let activation = Activation::new()
        .bind_variable("text", "apple,banana,cherry,date")?;
    
    let test_cases = vec![
        ("'a,b,c'.split(',')", "Split by comma"),
        ("text.split(',')", "Split CSV data"),
        ("'hello world'.split(' ')", "Split by space"),
        ("'one-two-three-four'.split('-', 2)", "Split with limit"),
        ("'a::b::c'.split('::')", "Split by multi-character separator"),
        ("'no-separators'.split(',')", "Split with no separators"),
        ("''.split(',')", "Split empty string"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_case_conversion(env: &Env) -> Result<(), Error> {
    println!("  Testing case conversion functions");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello, World!")?;
    
    let test_cases = vec![
        ("'Hello World'.lowerAscii()", "Convert to lowercase"),
        ("'Hello World'.upperAscii()", "Convert to uppercase"),
        ("text.lowerAscii()", "Convert variable to lowercase"),
        ("text.upperAscii()", "Convert variable to uppercase"),
        ("'MiXeD CaSe'.lowerAscii()", "Convert mixed case to lowercase"),
        ("'already lowercase'.upperAscii()", "Convert lowercase to uppercase"),
        ("'123ABC'.lowerAscii()", "Convert with numbers"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_replace(env: &Env) -> Result<(), Error> {
    println!("  Testing replace function");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello World, Hello CEL!")?
        .bind_variable("search", "Hello")?
        .bind_variable("replacement", "Hi")?;
    
    let test_cases = vec![
        ("'hello world'.replace('world', 'CEL')", "Replace substring"),
        ("text.replace('Hello', 'Hi')", "Replace all occurrences"),
        ("text.replace('Hello', 'Hi', 1)", "Replace first occurrence only"),
        ("text.replace(search, replacement)", "Replace using variables"),
        ("'foo foo foo'.replace('foo', 'bar')", "Replace multiple occurrences"),
        ("'no match'.replace('xyz', 'abc')", "Replace non-existent substring"),
        ("''.replace('a', 'b')", "Replace in empty string"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_format(env: &Env) -> Result<(), Error> {
    println!("  Testing format function");
    
    let activation = Activation::new()
        .bind_variable("text", "World")?;
    
    let test_cases = vec![
        ("'Hello, %s!'.format(['World'])", "Format with string"),
        ("'Number: %d'.format([42])", "Format with integer"),
        ("'Float: %.2f'.format([3.14159])", "Format with float precision"),
        ("'%s has %d apples'.format(['Alice', 5])", "Format with multiple values"),
        ("'Hello, %s!'.format([text])", "Format with variable"),
        ("'No placeholders'.format([])", "Format without placeholders"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_reverse(env: &Env) -> Result<(), Error> {
    println!("  Testing reverse function");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello")?;
    
    let test_cases = vec![
        ("'hello'.reverse()", "Reverse simple string"),
        ("text.reverse()", "Reverse variable"),
        ("'racecar'.reverse()", "Reverse palindrome"),
        ("'12345'.reverse()", "Reverse numbers"),
        ("'a'.reverse()", "Reverse single character"),
        ("''.reverse()", "Reverse empty string"),
        ("'Hello, World!'.reverse()", "Reverse with punctuation"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_complex_processing(env: &Env) -> Result<(), Error> {
    println!("  Testing complex string processing combinations");
    
    let activation = Activation::new()
        .bind_variable("email", "  User@Example.Com  ")?
        .bind_variable("data", "name:John,age:30,city:NYC")?
        .bind_variable("words", vec!["hello".to_string(), "world".to_string()])?;
    
    let test_cases = vec![
        (
            "email.trim().lowerAscii().substring(0, email.trim().lowerAscii().indexOf('@'))",
            "Extract and normalize username from email"
        ),
        (
            "email.trim().lowerAscii().substring(email.trim().lowerAscii().indexOf('@') + 1)",
            "Extract and normalize domain from email"
        ),
        (
            "data.split(',').map(item, item.split(':')[1]).join(' | ')",
            "Extract all values from CSV-like data"
        ),
        (
            "words.join(' ').upperAscii().reverse()",
            "Join, uppercase, and reverse words"
        ),
        (
            "strings.quote(email.trim().replace('@', ' at '))",
            "Quote email with @ replacement"
        ),
        (
            "'Hello World'.split(' ').map(word, word.reverse()).join('-')",
            "Split, reverse each word, and rejoin"
        ),
        (
            "data.replace(':', '=').replace(',', ' AND ')",
            "Transform CSV format"
        ),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
} 