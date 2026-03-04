//! Example: Protobuf message integration with CEL expressions
//!
//! This example demonstrates the complete protobuf workflow:
//!
//! ## Workflow overview
//!
//! 1. **Compile** `.proto` files to a `FileDescriptorSet` at runtime using
//!    [`protox`](https://crates.io/crates/protox) (pure Rust, no external binary needed),
//!    or offline via `protoc --descriptor_set_out`.
//!
//! 2. **Build** an environment with the descriptors and declare protobuf variables.
//! 3. **Serialize** protobuf messages and bind them as CEL variables.
//! 4. **Evaluate** CEL expressions that access message fields.
//! 5. **Extract** results — scalars directly, or protobuf messages via `as_protobuf_bytes()`.
//! 6. **Read fields** from Rust with `env.get_protobuf_field()` / `env.has_protobuf_field()`.
//!
//! Run with: `cargo run --example protobuf`

use cel_cxx::*;

fn compile_descriptors() -> Vec<u8> {
    use prost::Message;
    protox::compile(["tests/fixtures/test.proto"], ["tests/fixtures/"])
        .expect("failed to compile test.proto")
        .encode_to_vec()
}

/// Encode a test.SimpleMessage { name: string, id: int32 } using raw protobuf encoding.
fn encode_simple_message(name: &str, id: i32) -> Vec<u8> {
    use prost::encoding::*;

    let mut buf = Vec::new();
    if !name.is_empty() {
        encode_key(1, WireType::LengthDelimited, &mut buf);
        encode_varint(name.len() as u64, &mut buf);
        buf.extend_from_slice(name.as_bytes());
    }
    if id != 0 {
        encode_key(2, WireType::Varint, &mut buf);
        encode_varint(id as i64 as u64, &mut buf);
    }
    buf
}

fn main() -> Result<(), Error> {
    // =========================================================================
    // 1. Build an environment with protobuf descriptors
    // =========================================================================
    let env = Env::builder()
        .with_file_descriptor_set(&compile_descriptors())
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    // =========================================================================
    // 2. Serialize a protobuf message and bind it as a CEL variable
    // =========================================================================
    let bytes = encode_simple_message("Alice", 42);
    println!("Serialized message: {} bytes", bytes.len());

    let activation =
        Activation::new().bind_protobuf_variable("msg", "test.SimpleMessage", &bytes)?;

    // =========================================================================
    // 3. Evaluate CEL expressions with field access
    // =========================================================================
    println!("\n--- Field access expressions ---");
    let expressions = [
        "msg.name",
        "msg.id",
        "msg.name == 'Alice'",
        "msg.id > 0",
        "msg.name == 'Alice' && msg.id == 42",
    ];

    for expr in &expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("{expr} => {result}");
    }

    // =========================================================================
    // 4. Extract protobuf results with as_protobuf_bytes() and StructValue
    // =========================================================================
    println!("\n--- Extracting protobuf results ---");
    let program = env.compile("msg")?;
    let result = program.evaluate(&activation)?;

    // Option A: borrow type name and bytes directly
    let (type_name, result_bytes) = result.as_protobuf_bytes()?;
    println!("Type: {type_name}, bytes length: {}", result_bytes.len());

    // Option B: extract an owned StructValue via FromValue
    let sv = StructValue::from_value(&result)?;
    println!(
        "StructValue {{ type_name: {:?}, bytes: {} bytes }}",
        sv.type_name,
        sv.bytes.len()
    );

    // =========================================================================
    // 5. Rust-side field access with get_protobuf_field / has_protobuf_field
    // =========================================================================
    println!("\n--- Rust-side field access ---");
    let name_value = env.get_protobuf_field(&sv, "name")?;
    let id_value = env.get_protobuf_field(&sv, "id")?;
    println!("get_protobuf_field(name) => {name_value}");
    println!("get_protobuf_field(id)   => {id_value}");

    let has_name = env.has_protobuf_field(&sv, "name")?;
    let has_id = env.has_protobuf_field(&sv, "id")?;
    println!("has_protobuf_field(name) => {has_name}");
    println!("has_protobuf_field(id)   => {has_id}");

    // =========================================================================
    // 6. Message construction in CEL expressions (Type{field: value} syntax)
    // =========================================================================
    println!("\n--- Message construction ---");
    let env2 = Env::builder()
        .with_file_descriptor_set(&compile_descriptors())
        .build()?;

    let program = env2.compile("test.SimpleMessage{name: 'Bob', id: 99}")?;
    let constructed = program.evaluate(&Activation::new())?;

    let (type_name, bytes) = constructed.as_protobuf_bytes()?;
    println!("Constructed {type_name}: {} bytes", bytes.len());

    // Inline field access on constructed message
    let program = env2.compile("test.SimpleMessage{name: 'Bob', id: 99}.name")?;
    let name = program.evaluate(&Activation::new())?;
    println!("Inline field access: test.SimpleMessage{{...}}.name => {name}");

    Ok(())
}
