//! Example: Using ProtobufLegacyValue derive macro with protobuf-codegen generated types.
//!
//! This shows the end-to-end workflow:
//! 1. Generate Rust types from .proto with `#[derive(ProtobufLegacyValue)]` via protobuf-codegen
//! 2. Declare typed variables in the CEL environment
//! 3. Bind protobuf messages directly (no manual serialization)
//! 4. Evaluate CEL expressions and recover typed results
//!
//! Run with: cargo run --example protobuf-legacy-derive --features protobuf-legacy

#![cfg(feature = "protobuf-legacy")]

use cel_cxx::*;

// Import protobuf-codegen generated types (ProtobufLegacyValue derived via build.rs customize_callback)
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_mut)]
mod proto {
    include!(concat!(
        env!("OUT_DIR"),
        "/protobuf_legacy_gen/test.rs"
    ));
}
use proto::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Compile the FileDescriptorSet for cel-cpp's type system
    let fds = {
        use prost::Message;
        protox::compile(["tests/fixtures/test.proto"], ["tests/fixtures/"])?.encode_to_vec()
    };

    // 2. Build the environment with typed variable declarations
    let env = Env::builder()
        .with_file_descriptor_set(&fds)
        .declare_variable::<SimpleMessage>("msg")?
        .declare_variable::<Person>("person")?
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
    println!("msg.name = {result}");

    let program = env.compile("person.address.city")?;
    let result = program.evaluate(&activation)?;
    println!("person.address.city = {result}");

    let program = env.compile("person.tags.size()")?;
    let result = program.evaluate(&activation)?;
    println!("person.tags.size() = {result}");

    // 6. Construct a message in CEL and recover it as a typed protobuf struct
    let program = env.compile("test.SimpleMessage{name: 'built in CEL', id: 999}")?;
    let result = program.evaluate(&Activation::new())?;
    let recovered = SimpleMessage::from_value(&result)?;
    println!(
        "Constructed in CEL: name={}, id={}",
        recovered.name, recovered.id
    );

    Ok(())
}
