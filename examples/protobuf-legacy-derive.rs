//! Example: Using ProtobufLegacyValue derive macro with protobuf-codegen generated types.
//!
//! This shows the end-to-end workflow:
//! 1. Generate Rust types from .proto with `#[derive(ProtobufLegacyValue)]` via protobuf-codegen
//! 2. Declare typed variables in the CEL environment
//! 3. Bind protobuf messages directly (no manual serialization)
//! 4. Evaluate CEL expressions and recover typed results
//! 5. Pass typed messages through Rust callbacks and inspect message presence
//!
//! Run with: cargo run --example protobuf-legacy-derive --features protobuf-legacy

#![cfg(feature = "protobuf-legacy")]

use cel_cxx::*;

// Import protobuf-codegen generated types (ProtobufLegacyValue derived via build.rs customize_callback)
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]
mod proto {
    include!(concat!(env!("OUT_DIR"), "/protobuf_legacy_gen/test.rs"));
}
use proto::*;

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

    // 3. Create protobuf messages (normal protobuf v3 usage)
    let mut msg = SimpleMessage::new();
    msg.name = "Hello from protobuf-legacy!".into();
    msg.id = 42;

    let mut address = Address::new();
    address.street = "123 Main St".into();
    address.city = "Wonderland".into();
    address.zip = "00000".into();

    let mut person = Person::new();
    person.name = "Alice".into();
    person.age = 30;
    person.address = ::protobuf::MessageField::some(address);
    person.status = proto::Status::STATUS_ACTIVE.into();
    person.tags = vec!["admin".into(), "user".into()];
    person.active = true;

    // 4. Bind directly — ProtobufLegacyValue handles serialization
    let activation = Activation::new()
        .bind_variable("msg", msg)?
        .bind_variable("person", person)?;

    // 5. Evaluate field access
    let program = env.compile("msg.name")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::String("Hello from protobuf-legacy!".into()));
    println!("msg.name = {result}");

    let program = env.compile("person.address.city")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::String("Wonderland".into()));
    println!("person.address.city = {result}");

    let program = env.compile("person.tags.size()")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Int(2));
    println!("person.tags.size() = {result}");

    // 6. Construct a message in CEL and recover it as a typed protobuf struct
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
    // Mutating this argument does not mutate the activation's serialized input.
    let renamed = env.compile("rename(msg, 'updated by Rust')")?;
    let renamed = SimpleMessage::from_value(&renamed.evaluate(&activation)?)?;
    assert_eq!(renamed.name, "updated by Rust");
    assert_eq!(renamed.id, 42);
    assert_eq!(
        env.compile("msg.name")?.evaluate(&activation)?,
        Value::String("Hello from protobuf-legacy!".into())
    );
    println!("Typed callback result: {renamed:?}");

    // 8. MessageField preserves the distinction between an absent message and
    // an explicitly present empty message through a CEL round trip.
    let city = env.compile("has(person.address) ? person.address.city : 'unknown'")?;
    for (address, expected) in [
        (::protobuf::MessageField::none(), "unknown"),
        (::protobuf::MessageField::some(Address::new()), ""),
    ] {
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
