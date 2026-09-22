//! Example: Using ProstValue derive macro with prost-generated protobuf types.
//!
//! This shows the end-to-end workflow:
//! 1. Generate Rust types from .proto with `#[derive(ProstValue)]` via prost-build
//! 2. Declare typed variables in the CEL environment
//! 3. Bind prost messages directly (no manual serialization)
//! 4. Evaluate CEL expressions and recover typed results
//! 5. Pass typed messages through Rust callbacks and inspect message presence
//!
//! Run with: cargo run --example protobuf-prost-derive --features prost

#![cfg(feature = "prost")]

use cel_cxx::*;

// Import prost-generated types (ProstValue derived via build.rs type_attribute)
include!(concat!(env!("OUT_DIR"), "/test.rs"));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Compile the FileDescriptorSet for cel-cpp's type system
    let fds = {
        use prost::Message;
        let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
        protox::compile([fixtures.join("test.proto")], [fixtures])?.encode_to_vec()
    };

    // 2. Build the environment with typed variable declarations
    let env = Env::builder()
        .with_file_descriptor_set(&fds)
        .declare_variable::<SimpleMessage>("msg")?
        .declare_variable::<Person>("person")?
        .register_global_function("rename", |mut msg: SimpleMessage, name: String| {
            msg.name = name;
            msg
        })?
        .build()?;

    // 3. Create prost messages (normal prost usage)
    let msg = SimpleMessage {
        name: "Hello from prost!".into(),
        id: 42,
    };

    let person = Person {
        name: "Alice".into(),
        age: 30,
        address: Some(Address {
            street: "123 Main St".into(),
            city: "Wonderland".into(),
            zip: "00000".into(),
        }),
        status: Status::Active as i32,
        tags: vec!["admin".into(), "user".into()],
        active: true,
    };

    // 4. Bind directly — ProstValue handles serialization
    let activation = Activation::new()
        .bind_variable("msg", msg)?
        .bind_variable("person", person)?;

    // 5. Evaluate field access
    let program = env.compile("msg.name")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::String("Hello from prost!".into()));
    println!("msg.name = {result}");

    let program = env.compile("person.address.city")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::String("Wonderland".into()));
    println!("person.address.city = {result}");

    let program = env.compile("person.tags.size()")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Int(2));
    println!("person.tags.size() = {result}");

    // 6. Construct a message in CEL and recover it as a typed prost struct
    let program = env.compile("test.SimpleMessage{name: 'built in CEL', id: 999}")?;
    let result = program.evaluate(&Activation::new())?;
    let recovered = SimpleMessage::from_value(&result)?;
    assert_eq!(recovered.name, "built in CEL");
    assert_eq!(recovered.id, 999);
    println!(
        "Constructed in CEL: name={}, id={}",
        recovered.name, recovered.id
    );

    // 7. Typed callbacks deserialize their arguments and serialize their result.
    // The callback changes a copy; the message bound in the activation is intact.
    let renamed = env.compile("rename(msg, 'updated by Rust')")?;
    let renamed = SimpleMessage::from_value(&renamed.evaluate(&activation)?)?;
    assert_eq!(renamed.name, "updated by Rust");
    assert_eq!(renamed.id, 42);
    assert_eq!(
        env.compile("msg.name")?.evaluate(&activation)?,
        Value::String("Hello from prost!".into())
    );
    println!("Typed callback result: {renamed:?}");

    // 8. Reuse one program for absent and explicitly present empty messages.
    // Prost represents message presence with Option<T>; has() follows it even
    // when every scalar field in the nested message has its default value.
    let city = env.compile("has(person.address) ? person.address.city : 'unknown'")?;
    for (address, expected) in [(None, "unknown"), (Some(Address::default()), "")] {
        let person = Person {
            address,
            ..Default::default()
        };
        let activation = Activation::new().bind_variable("person", person.clone())?;
        let result = city.evaluate(&activation)?;
        assert_eq!(result, Value::String(expected.into()));
        let round_trip = Person::from_value(&env.compile("person")?.evaluate(&activation)?)?;
        assert_eq!(round_trip, person);
        println!(
            "Address present: {}, city: {result}",
            person.address.is_some()
        );
    }

    Ok(())
}
