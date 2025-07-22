use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("🔧 CEL Regex Extensions Example");
    println!("===============================");
    
    // Build environment with regex extensions enabled
    let env = Env::builder()
        .with_ext_regex(true)
        .with_ext_bindings(true)
        .with_optional(true)
        .declare_variable::<String>("text")?
        .declare_variable::<String>("email")?
        .declare_variable::<String>("phone")?
        .declare_variable::<String>("log_line")?
        .build()?;

    // Demo 1: Basic Pattern Extraction
    println!("\n📌 Demo 1: Basic Pattern Extraction");
    demo_basic_extraction(&env)?;
    
    // Demo 2: Extract All Matches
    println!("\n📌 Demo 2: Extract All Matches");
    demo_extract_all(&env)?;
    
    // Demo 3: Pattern Replacement
    println!("\n📌 Demo 3: Pattern Replacement");
    demo_pattern_replacement(&env)?;
    
    // Demo 4: Data Processing
    println!("\n📌 Demo 4: Data Processing");
    demo_data_processing(&env)?;
    
    println!("\n✅ All CEL Regex Extensions demos completed!");
    Ok(())
}

fn demo_basic_extraction(env: &Env) -> Result<(), Error> {
    println!("  Testing basic regex pattern extraction");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello World 123".to_string())?;
    
    let test_cases = vec![
        ("regex.extract(text, '[0-9]+')", "Extract digits"),
        ("regex.extract(text, '^Hello')", "Extract from start of string"),
        ("regex.extract(text, '.*World.*')", "Extract containing 'World'"),
        ("regex.extract(text, 'Hello ([A-Za-z]+) [0-9]+')", "Extract word after 'Hello' (single capture group)"),
        ("regex.extract(text, 'xyz')", "Extract non-existent pattern"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_extract_all(env: &Env) -> Result<(), Error> {
    println!("  Testing regex extractAll function");
    
    let activation = Activation::new()
        .bind_variable("text", "Contact john@example.com or admin@test.org for help".to_string())?;
    
    let test_cases = vec![
        ("regex.extractAll(text, '[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}')", "Extract all email addresses"),
        ("regex.extractAll(text, '\\\\w+')", "Extract all words"),
        ("regex.extractAll('id:123, id:456, id:789', 'id:(\\\\d+)')", "Extract all IDs with capture group"),
        ("regex.extractAll('no matches here', 'xyz')", "Extract from text with no matches"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_pattern_replacement(env: &Env) -> Result<(), Error> {
    println!("  Testing regex replace function");
    
    let test_cases = vec![
        ("regex.replace('hello world hello', 'hello', 'hi')", "Replace all occurrences"),
        ("regex.replace('banana', 'a', 'x', 1)", "Replace first occurrence only"),
        ("regex.replace('banana', 'a', 'x', 2)", "Replace first two occurrences"),
        ("regex.replace('John Doe', '(\\\\w+) (\\\\w+)', r'\\2, \\1')", "Replace with capture groups"),
        ("regex.replace('2023-12-25', '(\\\\d{4})-(\\\\d{2})-(\\\\d{2})', r'\\3/\\2/\\1')", "Date format conversion"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_data_processing(env: &Env) -> Result<(), Error> {
    println!("  Testing real-world data processing scenarios");
    
    // Email processing - separate extractions for username and domain
    println!("  Email processing example:");
    let email_processing = r#"
        cel.bind(email_input, email,
          cel.bind(username, regex.extract(email_input, '^([^@]+)@'),
            cel.bind(domain, regex.extract(email_input, '@([^@]+)$'),
              {
                "email": email_input,
                "has_username": username.hasValue(),
                "username": username.orValue("unknown"),
                "has_domain": domain.hasValue(),
                "domain": domain.orValue("unknown"),
                "is_valid": username.hasValue() && domain.hasValue()
              }
            )
          )
        )
    "#;
    let program = env.compile(email_processing.trim())?;
    let activation = Activation::new()
        .bind_variable("email", "test.user@company.com".to_string())?;
    let result = program.evaluate(&activation)?;
    println!("    Email processing result: {result}");
    
    // Phone number formatting
    println!("\n  Phone number formatting example:");
    let phone_formatting = r#"
        cel.bind(phone_input, phone,
          cel.bind(digits, regex.extractAll(phone_input, '\\d'),
            cel.bind(digit_count, digits.size(),
              cel.bind(formatted, 
                digit_count == 10 ? 
                  '(' + digits[0] + digits[1] + digits[2] + ') ' + 
                  digits[3] + digits[4] + digits[5] + '-' + 
                  digits[6] + digits[7] + digits[8] + digits[9] : 
                  phone_input,
                {
                  "original": phone_input,
                  "digits": digits,
                  "digit_count": digit_count,
                  "formatted": formatted,
                  "is_valid_length": digit_count == 10
                }
              )
            )
          )
        )
    "#;
    let program = env.compile(phone_formatting.trim())?;
    let activation = Activation::new()
        .bind_variable("phone", "15551234567".to_string())?;
    let result = program.evaluate(&activation)?;
    println!("    Phone formatting result: {result}");
    
    // Log parsing - separate extractions for different parts
    println!("\n  Log parsing example:");
    let log_parsing = r#"
        cel.bind(log_entry, log_line,
          cel.bind(timestamp, regex.extract(log_entry, '^(\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}:\\d{2})'),
            cel.bind(level, regex.extract(log_entry, '\\[(\\w+)\\]'),
              cel.bind(message, regex.extract(log_entry, '\\] (.+)$'),
                {
                  "raw_log": log_entry,
                  "timestamp": timestamp.orValue("unknown"),
                  "level": level.orValue("INFO"),
                  "message": message.orValue("no message"),
                  "has_timestamp": timestamp.hasValue(),
                  "has_level": level.hasValue(),
                  "is_error": level.hasValue() && (level.value() == "ERROR" || level.value() == "FATAL")
                }
              )
            )
          )
        )
    "#;
    let program = env.compile(log_parsing.trim())?;
    let activation = Activation::new()
        .bind_variable("log_line", "2023-12-25 14:30:15 [ERROR] Database connection failed".to_string())?;
    let result = program.evaluate(&activation)?;
    println!("    Log parsing result: {result}");
    
    // Text cleaning
    println!("\n  Text cleaning example:");
    let text_cleaning = r#"
        cel.bind(messy_text, text,
          cel.bind(step1, regex.replace(messy_text, '\\s+', ' '),
            cel.bind(step2, regex.replace(step1, '[!@#$%^&*()]+', ''),
              {
                "original": messy_text,
                "normalized_spaces": step1,
                "removed_special": step2,
                "final": step2,
                "word_count": regex.extractAll(step2, '\\w+').size()
              }
            )
          )
        )
    "#;
    let program = env.compile(text_cleaning.trim())?;
    let activation = Activation::new()
        .bind_variable("text", "  Hello!!!   World@@@   CEL   is   awesome!!!  ".to_string())?;
    let result = program.evaluate(&activation)?;
    println!("    Text cleaning result: {result}");
    
    Ok(())
} 