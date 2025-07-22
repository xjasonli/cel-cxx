use cel_cxx::*;

#[test]
fn test_base64_encode() -> Result<(), Error> {
    println!("test base64.encode function");

    let env = Env::builder()
        .with_ext_encoders(true)
        .declare_variable::<Vec<u8>>("input")?
        .build()?;

    // Test base64 encoding - takes bytes, returns string
    let program = env.compile("base64.encode(input)")?;
    let activation = Activation::new()
        .bind_variable("input", b"Hello, World!".to_vec())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("SGVsbG8sIFdvcmxkIQ==".into()));

    // Test base64 encoding with empty bytes
    let program = env.compile("base64.encode(input)")?;
    let activation = Activation::new()
        .bind_variable("input", Vec::<u8>::new())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("".into()));

    Ok(())
}

#[test]
fn test_base64_decode() -> Result<(), Error> {
    println!("test base64.decode function");

    let env = Env::builder()
        .with_ext_encoders(true)
        .declare_variable::<String>("encoded")?
        .build()?;

    // Test base64 decoding - takes string, returns bytes
    let program = env.compile("base64.decode(encoded)")?;
    let activation = Activation::new()
        .bind_variable("encoded", "SGVsbG8sIFdvcmxkIQ==".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bytes(b"Hello, World!".into()));

    // Test base64 decoding with empty string
    let program = env.compile("base64.decode(encoded)")?;
    let activation = Activation::new()
        .bind_variable("encoded", "".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bytes((&[] as &[u8]).into()));

    Ok(())
}

#[test]
fn test_base64_round_trip() -> Result<(), Error> {
    println!("test base64 round trip encoding/decoding");

    let env = Env::builder()
        .with_ext_encoders(true)
        .declare_variable::<Vec<u8>>("original")?
        .build()?;

    // Test round trip encoding and decoding
    let program = env.compile("base64.decode(base64.encode(original))")?;
    let activation = Activation::new()
        .bind_variable("original", b"Test message with special chars: @#$%^&*()".to_vec())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bytes(b"Test message with special chars: @#$%^&*()".into()));

    Ok(())
}

#[test]
fn test_comprehensive_encoding_operations() -> Result<(), Error> {
    println!("test comprehensive encoding operations");

    let env = Env::builder()
        .with_ext_encoders(true)
        .declare_variable::<Vec<u8>>("data")?
        .declare_variable::<Vec<u8>>("token")?
        .build()?;

    // Test encoding for API tokens - concatenate bytes then encode
    let program = env.compile("base64.encode(data + b':' + token)")?;
    let activation = Activation::new()
        .bind_variable("data", b"user123".to_vec())?
        .bind_variable("token", b"secret456".to_vec())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("dXNlcjEyMzpzZWNyZXQ0NTY=".into()));

    // Test conditional encoding
    let program = env.compile("size(data) > 0 ? base64.encode(data) : ''")?;
    let activation = Activation::new()
        .bind_variable("data", b"test".to_vec())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("dGVzdA==".into()));

    // Test with empty data
    let program = env.compile("size(data) > 0 ? base64.encode(data) : ''")?;
    let activation = Activation::new()
        .bind_variable("data", Vec::<u8>::new())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("".into()));

    Ok(())
}

#[test]
fn test_string_to_bytes_encoding() -> Result<(), Error> {
    println!("test encoding string content as bytes");

    let env = Env::builder()
        .with_ext_encoders(true)
        .declare_variable::<String>("text")?
        .build()?;

    // Convert string to bytes then encode
    let program = env.compile("base64.encode(bytes(text))")?;
    let activation = Activation::new()
        .bind_variable("text", "Hello, 世界!".to_string())?;
    let res = program.evaluate(&activation)?;
    // "Hello, 世界!" in UTF-8 bytes encoded as base64
    assert!(matches!(res, Value::String(_)));

    Ok(())
}

#[test]
fn test_decode_to_string_conversion() -> Result<(), Error> {
    println!("test decoding to bytes then converting to string");

    let env = Env::builder()
        .with_ext_encoders(true)
        .declare_variable::<String>("encoded")?
        .build()?;

    // Decode to bytes then convert to string
    let program = env.compile("string(base64.decode(encoded))")?;
    let activation = Activation::new()
        .bind_variable("encoded", "SGVsbG8sIFdvcmxkIQ==".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("Hello, World!".into()));

    Ok(())
} 