//! Example: Using ProstValue derive macro with prost-generated protobuf types.
//!
//! This shows the end-to-end workflow:
//! 1. Generate Rust types from .proto with `#[derive(ProstValue)]` via prost-build
//! 2. Declare typed variables in the CEL environment
//! 3. Bind prost messages directly (no manual serialization)
//! 4. Evaluate CEL expressions and recover typed results
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
        protox::compile(["tests/fixtures/test.proto"], ["tests/fixtures/"])?.encode_to_vec()
    };

    // 2. Build the environment with typed variable declarations
    let env = Env::builder()
        .with_file_descriptor_set(&fds)
        .declare_variable::<SimpleMessage>("msg")?
        .declare_variable::<Person>("person")?
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
    println!("msg.name = {result}");

    let program = env.compile("person.address.city")?;
    let result = program.evaluate(&activation)?;
    println!("person.address.city = {result}");

    let program = env.compile("person.tags.size()")?;
    let result = program.evaluate(&activation)?;
    println!("person.tags.size() = {result}");

    // 6. Construct a message in CEL and recover it as a typed prost struct
    let program = env.compile("test.SimpleMessage{name: 'built in CEL', id: 999}")?;
    let result = program.evaluate(&Activation::new())?;
    let recovered = SimpleMessage::from_value(&result)?;
    println!(
        "Constructed in CEL: name={}, id={}",
        recovered.name, recovered.id
    );

    Ok(())
}
