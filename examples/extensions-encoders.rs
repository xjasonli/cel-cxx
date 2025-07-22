use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("🔧 CEL Encoders Extensions Example");
    println!("==================================");
    
    // Build environment with encoders extensions enabled
    let env = Env::builder()
        .with_ext_encoders(true)
        .declare_variable::<String>("text")?
        .declare_variable::<Vec<u8>>("data")?
        .declare_variable::<String>("encoded")?
        .declare_variable::<String>("config")?
        .declare_variable::<String>("token")?
        .declare_variable::<String>("input_data")?
        .declare_variable::<Vec<u8>>("binary_data")?
        .build()?;

    // Demo 1: Base64 Encoding
    println!("\n📌 Demo 1: Base64 Encoding");
    demo_base64_encode(&env)?;
    
    // Demo 2: Base64 Decoding
    println!("\n📌 Demo 2: Base64 Decoding");
    demo_base64_decode(&env)?;
    
    // Demo 3: Round-trip Encoding/Decoding
    println!("\n📌 Demo 3: Round-trip Encoding/Decoding");
    demo_roundtrip(&env)?;
    
    // Demo 4: Real-world Usage Scenarios
    println!("\n📌 Demo 4: Real-world Usage Scenarios");
    demo_realworld_scenarios(&env)?;
    
    println!("\n✅ All CEL Encoders Extensions demos completed!");
    Ok(())
}

fn demo_base64_encode(env: &Env) -> Result<(), Error> {
    println!("  Testing base64.encode function");
    
    let test_cases = vec![
        ("base64.encode(b'hello')", "Encode simple text"),
        ("base64.encode(b'')", "Encode empty bytes"),
        ("base64.encode(b'Hello, World!')", "Encode greeting message"),
        ("base64.encode(b'CEL is awesome!')", "Encode CEL message"),
        ("base64.encode(bytes('Binary data: \\x00\\x01\\x02\\x03'))", "Encode binary data"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_base64_decode(env: &Env) -> Result<(), Error> {
    println!("  Testing base64.decode function");
    
    let test_cases = vec![
        ("base64.decode('aGVsbG8=')", "Decode 'hello' with padding"),
        ("base64.decode('aGVsbG8')", "Decode 'hello' without padding"),
        ("base64.decode('')", "Decode empty string"),
        ("base64.decode('SGVsbG8sIFdvcmxkIQ==')", "Decode 'Hello, World!'"),
        ("base64.decode('Q0VMIGlzIGF3ZXNvbWUh')", "Decode 'CEL is awesome!'"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_roundtrip(env: &Env) -> Result<(), Error> {
    println!("  Testing round-trip encoding/decoding");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello, CEL World!".to_string())?;
    
    let test_cases = vec![
        ("string(base64.decode(base64.encode(bytes(text))))", "Round-trip text encoding"),
        ("base64.encode(base64.decode('SGVsbG8gV29ybGQ='))", "Round-trip known encoding"),
        ("string(base64.decode('SGVsbG8gV29ybGQ=')) == 'Hello World'", "Verify decoded content"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_realworld_scenarios(env: &Env) -> Result<(), Error> {
    println!("  Testing real-world usage scenarios");
    
    // Configuration storage
    println!("  Configuration storage example:");
    let config_expr = r#"
        {
          "original_config": config,
          "encoded_config": base64.encode(bytes(config)),
          "decoded_config": string(base64.decode(base64.encode(bytes(config)))),
          "config_matches": string(base64.decode(base64.encode(bytes(config)))) == config
        }
    "#;
    let program = env.compile(config_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("config", r#"{"host": "localhost", "port": 8080}"#.to_string())?;
    let result = program.evaluate(&activation)?;
    println!("    Config storage and retrieval: {result}");
    
    // API token handling
    println!("\n  API token handling example:");
    let token_expr = r#"
        {
          "token": token,
          "encoded_token": base64.encode(bytes(token)),
          "token_length": base64.encode(bytes(token)).size(),
          "is_valid_encoding": base64.encode(bytes(token)).size() > 0
        }
    "#;
    let program = env.compile(token_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("token", "user:secret123".to_string())?;
    let result = program.evaluate(&activation)?;
    println!("    API token encoding: {result}");
    
    // Data validation
    println!("\n  Data validation example:");
    let validation_expr = r#"
        {
          "input": input_data,
          "is_valid_base64": input_data.matches('^[A-Za-z0-9+/]*={0,2}$'),
          "decoded_size": input_data.matches('^[A-Za-z0-9+/]*={0,2}$') ? 
                         base64.decode(input_data).size() : -1,
          "validation_status": input_data.matches('^[A-Za-z0-9+/]*={0,2}$') ? 
                              "valid" : "invalid"
        }
    "#;
    let program = env.compile(validation_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("input_data", "SGVsbG8gV29ybGQ=".to_string())?;
    let result = program.evaluate(&activation)?;
    println!("    Base64 validation: {result}");
    
    // Binary data processing
    println!("\n  Binary data processing example:");
    let binary_expr = r#"
        {
          "original_bytes": binary_data,
          "encoded": base64.encode(binary_data),
          "decoded": base64.decode(base64.encode(binary_data)),
          "size_original": binary_data.size(),
          "size_encoded": base64.encode(binary_data).size(),
          "size_decoded": base64.decode(base64.encode(binary_data)).size(),
          "integrity_check": binary_data == base64.decode(base64.encode(binary_data))
        }
    "#;
    let program = env.compile(binary_expr.trim())?;
    let activation = Activation::new()
        .bind_variable("binary_data", vec![0u8, 1u8, 2u8, 255u8, 128u8])?;
    let result = program.evaluate(&activation)?;
    println!("    Binary data processing: {result}");
    
    Ok(())
} 