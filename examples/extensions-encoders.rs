//! Base64 encoding, binary round trips, and decoding failures.
//! Run with `cargo run --example extensions-encoders`.

use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("CEL encoders extension");
    let env = Env::builder()
        .with_ext_encoders(true)
        .declare_variable::<Vec<u8>>("data")?
        .declare_variable::<String>("encoded")?
        .build()?;

    for (expression, expected) in [
        ("base64.encode(b'hello')", "aGVsbG8="),
        ("base64.encode(b'')", ""),
        ("base64.encode(b'Hello, World!')", "SGVsbG8sIFdvcmxkIQ=="),
        ("base64.encode(b'a')", "YQ=="),
        ("base64.encode(b'ab')", "YWI="),
        ("base64.encode(b'abc')", "YWJj"),
    ] {
        let result = env.compile(expression)?.evaluate(())?;
        assert_eq!(result, Value::from(expected));
        println!("{expression} => {result}");
    }

    // Decoding returns bytes. Padding may be omitted from otherwise valid input.
    let decode = env.compile("base64.decode(encoded)")?;
    for (encoded, expected) in [
        ("aGVsbG8=", b"hello".as_slice()),
        ("aGVsbG8", b"hello"),
        ("", b""),
    ] {
        let activation = Activation::new().bind_variable("encoded", encoded)?;
        let result = decode.evaluate(&activation)?;
        assert_eq!(result, Value::Bytes(expected.to_vec().into()));
        println!("Decoded {encoded:?}: {result}");
    }

    // Use bytes directly: arbitrary binary data need not be valid UTF-8.
    let roundtrip = env.compile("base64.decode(base64.encode(data))")?;
    for data in [
        vec![],
        vec![0u8],
        vec![0, 1, 2, 128, 255],
        "你好, CEL".as_bytes().to_vec(),
    ] {
        let activation = Activation::new().bind_variable("data", data.clone())?;
        assert_eq!(roundtrip.evaluate(&activation)?, Value::Bytes(data.into()));
    }

    // Persist a configuration payload and recover its exact bytes.
    let config = br#"{"host":"localhost","port":8080}"#.to_vec();
    let activation = Activation::new().bind_variable("data", config.clone())?;
    let encoded = env.compile("base64.encode(data)")?.evaluate(&activation)?;
    let encoded = encoded.unwrap_string();
    let activation = Activation::new().bind_variable("encoded", encoded.to_string())?;
    assert_eq!(decode.evaluate(&activation)?, Value::Bytes(config.into()));
    println!("Encoded configuration: {encoded}");

    // Convert decoded bytes back to a string when the payload is known to be UTF-8.
    let text = env
        .compile("string(base64.decode(base64.encode(bytes('你好, CEL'))))")?
        .evaluate(())?;
    assert_eq!(text, Value::from("你好, CEL"));

    // The decoder validates the alphabet and padding/length. A character-class
    // regex alone cannot establish validity: "a" uses the alphabet but is incomplete.
    for invalid in ["a", "%%%", "aGVsbG8===", "=aGVsbG8"] {
        let activation = Activation::new().bind_variable("encoded", invalid)?;
        let error = decode
            .evaluate(&activation)
            .expect_err("invalid Base64 must fail");
        assert_eq!(error.code(), Code::InvalidArgument);
        println!("Rejected {invalid:?}: {error}");
    }
    Ok(())
}
