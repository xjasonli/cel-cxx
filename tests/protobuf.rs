use cel_cxx::*;
use std::sync::LazyLock;

static TEST_DESCRIPTORS: LazyLock<Vec<u8>> = LazyLock::new(|| {
    use prost::Message;
    protox::compile(["tests/fixtures/test.proto"], ["tests/fixtures/"])
        .unwrap()
        .encode_to_vec()
});

#[test]
fn test_env_with_custom_descriptor_pool() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .build()?;

    // Basic expression should still work
    let program = env.compile("1 + 2")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Int(3));

    Ok(())
}

#[test]
fn test_well_known_types_with_custom_pool() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .build()?;

    // duration() should still work — verifies generated_pool underlay
    let program = env.compile("duration('3600s') == duration('1h')")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Bool(true));

    // timestamp() should still work
    let program =
        env.compile("timestamp('2023-01-01T00:00:00Z') < timestamp('2024-01-01T00:00:00Z')")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_default_env_still_works() -> Result<(), Error> {
    // Verify no regression — default env (no custom pool) still works
    let env = Env::builder().build()?;

    let program = env.compile("'hello' + ' ' + 'world'")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::String("hello world".into()));

    Ok(())
}

/// Build serialized protobuf bytes for test.SimpleMessage using prost encoding.
/// prost encoding matches standard protobuf wire format.
fn encode_simple_message(name: &str, id: i32) -> Vec<u8> {
    use prost::encoding::*;

    let mut buf = Vec::new();
    // field 1: string name (tag = 0x0a)
    if !name.is_empty() {
        encode_key(1, WireType::LengthDelimited, &mut buf);
        encode_varint(name.len() as u64, &mut buf);
        buf.extend_from_slice(name.as_bytes());
    }
    // field 2: int32 id (tag = 0x10)
    if id != 0 {
        encode_key(2, WireType::Varint, &mut buf);
        encode_varint(id as i64 as u64, &mut buf);
    }
    buf
}

// --- Low-level encoding helpers ---

fn encode_string_field(buf: &mut Vec<u8>, field_number: u32, value: &str) {
    use prost::encoding::*;
    if !value.is_empty() {
        encode_key(field_number, WireType::LengthDelimited, buf);
        encode_varint(value.len() as u64, buf);
        buf.extend_from_slice(value.as_bytes());
    }
}

fn encode_varint_field(buf: &mut Vec<u8>, field_number: u32, value: i32) {
    use prost::encoding::*;
    if value != 0 {
        encode_key(field_number, WireType::Varint, buf);
        encode_varint(value as i64 as u64, buf);
    }
}

fn encode_bool_field(buf: &mut Vec<u8>, field_number: u32, value: bool) {
    use prost::encoding::*;
    if value {
        encode_key(field_number, WireType::Varint, buf);
        encode_varint(1, buf);
    }
}

fn encode_nested_field(buf: &mut Vec<u8>, field_number: u32, nested: &[u8]) {
    use prost::encoding::*;
    if !nested.is_empty() {
        encode_key(field_number, WireType::LengthDelimited, buf);
        encode_varint(nested.len() as u64, buf);
        buf.extend_from_slice(nested);
    }
}

fn encode_repeated_strings(buf: &mut Vec<u8>, field_number: u32, values: &[&str]) {
    use prost::encoding::*;
    for v in values {
        encode_key(field_number, WireType::LengthDelimited, buf);
        encode_varint(v.len() as u64, buf);
        buf.extend_from_slice(v.as_bytes());
    }
}

fn encode_map_string_string(buf: &mut Vec<u8>, field_number: u32, entries: &[(&str, &str)]) {
    use prost::encoding::*;
    for (key, value) in entries {
        // Each map entry is a nested message with key=field1, value=field2
        let mut entry = Vec::new();
        encode_string_field(&mut entry, 1, key);
        encode_string_field(&mut entry, 2, value);

        encode_key(field_number, WireType::LengthDelimited, buf);
        encode_varint(entry.len() as u64, buf);
        buf.extend_from_slice(&entry);
    }
}

// --- Message-level encoding helpers ---

fn encode_address(street: &str, city: &str, zip: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_string_field(&mut buf, 1, street);
    encode_string_field(&mut buf, 2, city);
    encode_string_field(&mut buf, 3, zip);
    buf
}

fn encode_person(
    name: &str,
    age: i32,
    address: Option<&[u8]>,
    status: i32,
    tags: &[&str],
    active: bool,
) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_string_field(&mut buf, 1, name);
    encode_varint_field(&mut buf, 2, age);
    if let Some(addr) = address {
        encode_nested_field(&mut buf, 3, addr);
    }
    encode_varint_field(&mut buf, 4, status);
    encode_repeated_strings(&mut buf, 5, tags);
    encode_bool_field(&mut buf, 6, active);
    buf
}

fn encode_item(name: &str, price: i32) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_string_field(&mut buf, 1, name);
    encode_varint_field(&mut buf, 2, price);
    buf
}

fn encode_order(buyer: Option<&[u8]>, items: &[&[u8]], labels: &[(&str, &str)]) -> Vec<u8> {
    let mut buf = Vec::new();
    if let Some(b) = buyer {
        encode_nested_field(&mut buf, 1, b);
    }
    for item in items {
        encode_nested_field(&mut buf, 2, item);
    }
    encode_map_string_string(&mut buf, 3, labels);
    buf
}

fn encode_bytes_field(buf: &mut Vec<u8>, field_number: u32, value: &[u8]) {
    use prost::encoding::*;
    if !value.is_empty() {
        encode_key(field_number, WireType::LengthDelimited, buf);
        encode_varint(value.len() as u64, buf);
        buf.extend_from_slice(value);
    }
}

fn encode_varint_i64_field(buf: &mut Vec<u8>, field_number: u32, value: i64) {
    use prost::encoding::*;
    if value != 0 {
        encode_key(field_number, WireType::Varint, buf);
        #[allow(clippy::cast_sign_loss)]
        encode_varint(value as u64, buf);
    }
}

// --- WKT encoding helpers ---

fn encode_duration(seconds: i64, nanos: i32) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_varint_i64_field(&mut buf, 1, seconds);
    encode_varint_field(&mut buf, 2, nanos);
    buf
}

fn encode_timestamp(seconds: i64, nanos: i32) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_varint_i64_field(&mut buf, 1, seconds);
    encode_varint_field(&mut buf, 2, nanos);
    buf
}

fn encode_int64_value(value: i64) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_varint_i64_field(&mut buf, 1, value);
    buf
}

fn encode_string_value(value: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_string_field(&mut buf, 1, value);
    buf
}

fn encode_bool_value(value: bool) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_bool_field(&mut buf, 1, value);
    buf
}

/// Encode a google.protobuf.Struct with string-typed values.
/// Struct layout: field 1 = repeated MapEntry (map<string, Value>)
/// MapEntry: {1: string key, 2: Value message}
/// Value with string: {3: string_value}
fn encode_struct_with_string_values(entries: &[(&str, &str)]) -> Vec<u8> {
    use prost::encoding::*;
    let mut buf = Vec::new();
    for (key, val) in entries {
        // Value message: field 3 = string_value
        let mut value_msg = Vec::new();
        encode_string_field(&mut value_msg, 3, val);

        // MapEntry: {1: key, 2: Value}
        let mut entry = Vec::new();
        encode_string_field(&mut entry, 1, key);
        encode_nested_field(&mut entry, 2, &value_msg);

        // Struct field 1 = map<string, Value> entries
        encode_key(1, WireType::LengthDelimited, &mut buf);
        encode_varint(entry.len() as u64, &mut buf);
        buf.extend_from_slice(&entry);
    }
    buf
}

fn encode_any(type_url: &str, value: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_string_field(&mut buf, 1, type_url);
    encode_bytes_field(&mut buf, 2, value);
    buf
}

fn encode_event(
    name: &str,
    created_at: Option<&[u8]>,
    ttl: Option<&[u8]>,
    optional_count: Option<&[u8]>,
    optional_label: Option<&[u8]>,
    optional_flag: Option<&[u8]>,
    metadata: Option<&[u8]>,
    payload: Option<&[u8]>,
) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_string_field(&mut buf, 1, name);
    if let Some(v) = created_at {
        encode_nested_field(&mut buf, 2, v);
    }
    if let Some(v) = ttl {
        encode_nested_field(&mut buf, 3, v);
    }
    if let Some(v) = optional_count {
        encode_nested_field(&mut buf, 4, v);
    }
    if let Some(v) = optional_label {
        encode_nested_field(&mut buf, 5, v);
    }
    if let Some(v) = optional_flag {
        encode_nested_field(&mut buf, 6, v);
    }
    if let Some(v) = metadata {
        encode_nested_field(&mut buf, 7, v);
    }
    if let Some(v) = payload {
        encode_nested_field(&mut buf, 8, v);
    }
    buf
}

#[test]
fn test_bind_variable_dynamic_field_access() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.SimpleMessage")))?
        .build()?;

    let bytes = encode_simple_message("Alice", 42);

    let activation = Activation::new().bind_variable_dynamic(
        "msg",
        StructValue::from_bytes("test.SimpleMessage", bytes.as_slice()),
    )?;

    // Test string field access
    let program = env.compile("msg.name == 'Alice'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    // Test int field access
    let program = env.compile("msg.id > 0")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    // Test combined field access
    let program = env.compile("msg.name == 'Alice' && msg.id == 42")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_protobuf_value_roundtrip() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.SimpleMessage")))?
        .build()?;

    let bytes = encode_simple_message("Bob", 99);

    let activation = Activation::new().bind_variable_dynamic(
        "msg",
        StructValue::from_bytes("test.SimpleMessage", bytes.as_slice()),
    )?;

    // Evaluate `msg` directly — should get a Value::Struct back
    let program = env.compile("msg")?;
    let result = program.evaluate(&activation)?;

    match &result {
        Value::Struct(s) => {
            assert_eq!(s.type_name(), "test.SimpleMessage");
            // The bytes should round-trip (serialize the same message back)
            assert!(!s.to_bytes().is_empty());
        }
        other => panic!("Expected Value::Struct, got: {other:?}"),
    }

    Ok(())
}

#[test]
fn test_protobuf_field_access_string_result() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.SimpleMessage")))?
        .build()?;

    let bytes = encode_simple_message("Charlie", 7);

    let activation = Activation::new().bind_variable_dynamic(
        "msg",
        StructValue::from_bytes("test.SimpleMessage", bytes.as_slice()),
    )?;

    // Access a string field directly
    let program = env.compile("msg.name")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::String("Charlie".into()));

    // Access an int field directly
    let program = env.compile("msg.id")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Int(7));

    Ok(())
}

// =============================================================================
// Full Protobuf Field Access Tests
// =============================================================================

// --- 2A.1: Scalar fields ---

#[test]
fn test_scalar_bool_field() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    let addr = encode_address("123 Main St", "Springfield", "62704");
    let bytes = encode_person("Alice", 30, Some(&addr), 1, &["rust"], true);

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let program = env.compile("person.active == true")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_scalar_default_values() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    // Empty person — all fields at defaults
    let bytes = encode_person("", 0, None, 0, &[], false);

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let program = env.compile("person.age == 0 && person.active == false")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// --- 2A.2: Nested message fields ---

#[test]
fn test_nested_message_field_access() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    let addr = encode_address("456 Market St", "San Francisco", "94105");
    let bytes = encode_person("Bob", 25, Some(&addr), 0, &[], false);

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let program = env.compile("person.address.city == 'San Francisco'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_deeply_nested_field_access() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("order", ValueType::Struct(StructType::new("test.Order")))?
        .build()?;

    let addr = encode_address("789 Broadway", "NYC", "10001");
    let buyer = encode_person("Carol", 40, Some(&addr), 0, &[], false);
    let bytes = encode_order(Some(&buyer), &[], &[]);

    let activation =
        Activation::new().bind_variable_dynamic("order", StructValue::from_bytes("test.Order", bytes.as_slice()))?;

    let program = env.compile("order.buyer.address.city == 'NYC'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// --- 2A.3: Repeated fields ---

#[test]
fn test_repeated_string_size() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    let bytes = encode_person("Dave", 28, None, 0, &["rust", "cel", "proto"], false);

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let program = env.compile("person.tags.size() == 3")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_repeated_string_index() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    let bytes = encode_person("Dave", 28, None, 0, &["rust", "cel", "proto"], false);

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let program = env.compile("person.tags[0] == 'rust'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_repeated_string_exists() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    let bytes = encode_person("Dave", 28, None, 0, &["rust", "cel", "proto"], false);

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let program = env.compile("person.tags.exists(t, t == 'cel')")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_repeated_message_field() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("order", ValueType::Struct(StructType::new("test.Order")))?
        .build()?;

    let item1 = encode_item("Widget", 100);
    let item2 = encode_item("Gadget", 250);
    let bytes = encode_order(None, &[&item1, &item2], &[]);

    let activation =
        Activation::new().bind_variable_dynamic("order", StructValue::from_bytes("test.Order", bytes.as_slice()))?;

    let program = env.compile("order.items[0].name == 'Widget'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_repeated_message_exists() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("order", ValueType::Struct(StructType::new("test.Order")))?
        .build()?;

    let item1 = encode_item("Widget", 100);
    let item2 = encode_item("Gadget", 250);
    let bytes = encode_order(None, &[&item1, &item2], &[]);

    let activation =
        Activation::new().bind_variable_dynamic("order", StructValue::from_bytes("test.Order", bytes.as_slice()))?;

    let program = env.compile("order.items.exists(i, i.price > 200)")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_repeated_empty() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    let bytes = encode_person("Eve", 22, None, 0, &[], false);

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let program = env.compile("person.tags.size() == 0")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// --- 2A.4: Map fields ---

#[test]
fn test_map_field_access() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("order", ValueType::Struct(StructType::new("test.Order")))?
        .build()?;

    let bytes = encode_order(None, &[], &[("env", "prod"), ("region", "us-east")]);

    let activation =
        Activation::new().bind_variable_dynamic("order", StructValue::from_bytes("test.Order", bytes.as_slice()))?;

    let program = env.compile(r#"order.labels["env"] == "prod""#)?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_map_contains_key() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("order", ValueType::Struct(StructType::new("test.Order")))?
        .build()?;

    let bytes = encode_order(None, &[], &[("env", "prod"), ("region", "us-east")]);

    let activation =
        Activation::new().bind_variable_dynamic("order", StructValue::from_bytes("test.Order", bytes.as_slice()))?;

    let program = env.compile(r#""env" in order.labels"#)?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_map_size() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("order", ValueType::Struct(StructType::new("test.Order")))?
        .build()?;

    let bytes = encode_order(None, &[], &[("env", "prod"), ("region", "us-east")]);

    let activation =
        Activation::new().bind_variable_dynamic("order", StructValue::from_bytes("test.Order", bytes.as_slice()))?;

    let program = env.compile("order.labels.size() == 2")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// --- 2A.5: Enum fields ---

#[test]
fn test_enum_field_as_int() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    // status = STATUS_ACTIVE = 1
    let bytes = encode_person("Frank", 35, None, 1, &[], false);

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let program = env.compile("person.status == 1")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_enum_field_named_constant() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    // status = STATUS_ACTIVE = 1
    let bytes = encode_person("Grace", 29, None, 1, &[], false);

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let program = env.compile("person.status == test.Status.STATUS_ACTIVE")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_enum_field_default() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    // status = STATUS_UNSPECIFIED = 0 (default)
    let bytes = encode_person("Hank", 50, None, 0, &[], false);

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let program = env.compile("person.status == 0")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// =============================================================================
// Well-Known Type Handling Tests
// =============================================================================

// --- 2B.1: Duration fields ---

#[test]
fn test_wkt_duration_field() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    let ttl = encode_duration(3600, 0);
    let bytes = encode_event("test", None, Some(&ttl), None, None, None, None, None);

    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    let program = env.compile("event.ttl == duration('3600s')")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_wkt_duration_comparison() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    let ttl = encode_duration(3600, 0); // 1 hour
    let bytes = encode_event("test", None, Some(&ttl), None, None, None, None, None);

    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    // 30m = 1800s, so 3600s > 1800s
    let program = env.compile("event.ttl > duration('1800s')")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// --- 2B.2: Timestamp fields ---

#[test]
fn test_wkt_timestamp_field() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    // 2024-01-01T00:00:00Z = 1704067200 seconds since epoch
    let created_at = encode_timestamp(1704067200, 0);
    let bytes = encode_event("test", Some(&created_at), None, None, None, None, None, None);

    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    let program = env.compile("event.created_at == timestamp('2024-01-01T00:00:00Z')")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_wkt_timestamp_comparison() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    // 2024-01-01T00:00:00Z
    let created_at = encode_timestamp(1704067200, 0);
    let bytes = encode_event("test", Some(&created_at), None, None, None, None, None, None);

    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    let program = env.compile("event.created_at > timestamp('2023-01-01T00:00:00Z')")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// --- 2B.3: Wrapper types ---

#[test]
fn test_wkt_int64_wrapper() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    let count = encode_int64_value(42);
    let bytes = encode_event("test", None, None, Some(&count), None, None, None, None);

    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    let program = env.compile("event.optional_count == 42")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_wkt_string_wrapper() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    let label = encode_string_value("important");
    let bytes = encode_event("test", None, None, None, Some(&label), None, None, None);

    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    let program = env.compile("event.optional_label == 'important'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_wkt_bool_wrapper() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    let flag = encode_bool_value(true);
    let bytes = encode_event("test", None, None, None, None, Some(&flag), None, None);

    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    let program = env.compile("event.optional_flag == true")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// --- 2B.4: Struct/Value fields ---

#[test]
fn test_wkt_struct_field_access() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    let metadata = encode_struct_with_string_values(&[("env", "prod"), ("version", "1.0")]);
    let bytes = encode_event("test", None, None, None, None, None, Some(&metadata), None);

    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    let program = env.compile("event.metadata.env == 'prod'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_wkt_struct_has_key() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    let metadata = encode_struct_with_string_values(&[("env", "prod"), ("version", "1.0")]);
    let bytes = encode_event("test", None, None, None, None, None, Some(&metadata), None);

    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    let program = env.compile("has(event.metadata.env)")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// --- 2B.5: Any fields ---

#[test]
fn test_wkt_any_type_url() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    // Pack a SimpleMessage into Any
    let inner = encode_simple_message("packed", 1);
    let payload = encode_any("type.googleapis.com/test.SimpleMessage", &inner);
    let bytes = encode_event("test", None, None, None, None, None, None, Some(&payload));

    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    // Try to check if the Any field is present via has()
    let program = env.compile("has(event.payload)")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// =============================================================================
// Extract Protobuf Bytes from Evaluation Results
// =============================================================================

#[test]
fn test_as_struct_accessors() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.SimpleMessage")))?
        .build()?;

    let bytes = encode_simple_message("Alice", 42);

    let activation = Activation::new().bind_variable_dynamic(
        "msg",
        StructValue::from_bytes("test.SimpleMessage", bytes.as_slice()),
    )?;

    let program = env.compile("msg")?;
    let result = program.evaluate(&activation)?;

    let sv = result.as_struct().unwrap();
    assert_eq!(sv.type_name(), "test.SimpleMessage");
    assert!(!sv.to_bytes().is_empty());

    Ok(())
}

#[test]
fn test_as_struct_none_on_non_struct() -> Result<(), Error> {
    let env = Env::builder().build()?;

    let program = env.compile("42")?;
    let result = program.evaluate(&Activation::new())?;

    assert!(result.as_struct().is_none());

    Ok(())
}

#[test]
fn test_struct_value_from_value() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.SimpleMessage")))?
        .build()?;

    let bytes = encode_simple_message("Bob", 99);

    let activation = Activation::new().bind_variable_dynamic(
        "msg",
        StructValue::from_bytes("test.SimpleMessage", bytes.as_slice()),
    )?;

    let program = env.compile("msg")?;
    let result = program.evaluate(&activation)?;

    // Extract via FromValue
    let sv = StructValue::from_value(&result)?;
    assert_eq!(sv.type_name(), "test.SimpleMessage");
    assert!(!sv.to_bytes().is_empty());

    // Extract via borrowed reference
    let sv_ref = <&StructValue>::from_value(&result)?;
    assert_eq!(sv_ref.type_name(), "test.SimpleMessage");

    Ok(())
}

#[test]
fn test_struct_value_from_value_error_on_non_struct() -> Result<(), Error> {
    let result = Value::Int(42);

    assert!(StructValue::from_value(&result).is_err());
    assert!(<&StructValue>::from_value(&result).is_err());

    Ok(())
}

// =============================================================================
// Rust-side Protobuf Field Access
// =============================================================================

#[test]
fn test_get_struct_field_string() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.SimpleMessage")))?
        .build()?;

    let bytes = encode_simple_message("Alice", 42);
    let activation =
        Activation::new().bind_variable_dynamic("msg", StructValue::from_bytes("test.SimpleMessage", bytes.as_slice()))?;

    let result = env.compile("msg")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    let name = env.get_struct_field(&sv, "name")?;
    assert_eq!(name, Value::String("Alice".into()));

    Ok(())
}

#[test]
fn test_get_struct_field_int() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.SimpleMessage")))?
        .build()?;

    let bytes = encode_simple_message("Bob", 99);
    let activation =
        Activation::new().bind_variable_dynamic("msg", StructValue::from_bytes("test.SimpleMessage", bytes.as_slice()))?;

    let result = env.compile("msg")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    let id = env.get_struct_field(&sv, "id")?;
    assert_eq!(id, Value::Int(99));

    Ok(())
}

#[test]
fn test_get_struct_field_nested() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    let addr = encode_address("123 Main St", "Springfield", "62704");
    let bytes = encode_person("Alice", 30, Some(&addr), 1, &["rust"], true);
    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let result = env.compile("person")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    // Get the nested address field
    let address_value = env.get_struct_field(&sv, "address")?;
    let address_sv = StructValue::from_value(&address_value)?;

    // Now get city from the nested address
    let city = env.get_struct_field(&address_sv, "city")?;
    assert_eq!(city, Value::String("Springfield".into()));

    Ok(())
}

#[test]
fn test_get_struct_field_not_found() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.SimpleMessage")))?
        .build()?;

    let bytes = encode_simple_message("Alice", 42);
    let activation =
        Activation::new().bind_variable_dynamic("msg", StructValue::from_bytes("test.SimpleMessage", bytes.as_slice()))?;

    let result = env.compile("msg")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    // cel-cpp returns an error Value for non-existent fields
    let field = env.get_struct_field(&sv, "nonexistent");
    match field {
        Err(_) => {} // status error
        Ok(Value::Error(_)) => {} // error value
        Ok(other) => panic!("Expected error for nonexistent field, got: {other:?}"),
    }

    Ok(())
}

#[test]
fn test_has_struct_field() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    let addr = encode_address("123 Main St", "Springfield", "62704");
    let bytes = encode_person("Alice", 30, Some(&addr), 1, &["rust"], true);
    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    let result = env.compile("person")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    assert!(env.has_struct_field(&sv, "name")?);
    assert!(env.has_struct_field(&sv, "address")?);
    // "active" is true, so has() returns true
    assert!(env.has_struct_field(&sv, "active")?);

    // Empty person — fields at defaults
    let empty_bytes = encode_person("", 0, None, 0, &[], false);
    let activation2 =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", empty_bytes.as_slice()))?;
    let result2 = env.compile("person")?.evaluate(&activation2)?;
    let sv2 = StructValue::from_value(&result2)?;

    // Proto3 default values: has() returns false for default-valued fields
    assert!(!env.has_struct_field(&sv2, "name")?);
    assert!(!env.has_struct_field(&sv2, "address")?);
    assert!(!env.has_struct_field(&sv2, "active")?);

    Ok(())
}

#[test]
fn test_get_struct_field_wkt() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    let ttl = encode_duration(3600, 0);
    let created_at = encode_timestamp(1704067200, 0);
    let bytes = encode_event(
        "test_event",
        Some(&created_at),
        Some(&ttl),
        None,
        None,
        None,
        None,
        None,
    );
    let activation =
        Activation::new().bind_variable_dynamic("event", StructValue::from_bytes("test.Event", bytes.as_slice()))?;

    let result = env.compile("event")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    // Duration field should return a CEL duration value
    let ttl_value = env.get_struct_field(&sv, "ttl")?;
    match &ttl_value {
        Value::Duration(d) => {
            assert_eq!(d.num_seconds(), 3600);
        }
        other => panic!("Expected Value::Duration, got: {other:?}"),
    }

    // Timestamp field should return a CEL timestamp value
    let ts_value = env.get_struct_field(&sv, "created_at")?;
    match &ts_value {
        Value::Timestamp(_) => {} // ok, it's a timestamp
        other => panic!("Expected Value::Timestamp, got: {other:?}"),
    }

    Ok(())
}

// =============================================================================
// Message Construction in CEL Expressions
// =============================================================================

#[test]
fn test_message_construction_simple() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .build()?;

    let program = env.compile("test.SimpleMessage{name: 'Bob', id: 25}")?;
    let result = program.evaluate(&Activation::new())?;

    let sv = result.as_struct().unwrap();
    assert_eq!(sv.type_name(), "test.SimpleMessage");
    assert!(!sv.to_bytes().is_empty());

    // Verify by binding the result back and reading fields
    let env2 = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.SimpleMessage")))?
        .build()?;

    let activation =
        Activation::new().bind_variable_dynamic("msg", StructValue::from_bytes("test.SimpleMessage", sv.to_bytes()))?;

    let program = env2.compile("msg.name == 'Bob' && msg.id == 25")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_message_construction_with_nested() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .build()?;

    let program = env.compile(
        "test.Person{name: 'Alice', age: 30, address: test.Address{street: '123 Main', city: 'NYC', zip: '10001'}, active: true}",
    )?;
    let result = program.evaluate(&Activation::new())?;

    let sv = result.as_struct().unwrap();
    assert_eq!(sv.type_name(), "test.Person");

    // Verify nested field access
    let env2 = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", sv.to_bytes()))?;

    let program = env2.compile("person.address.city == 'NYC' && person.active == true")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_message_construction_with_repeated() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .build()?;

    let program = env.compile(
        "test.Order{buyer: test.Person{name: 'Carol'}, items: [test.Item{name: 'Widget', price: 100}, test.Item{name: 'Gadget', price: 250}]}",
    )?;
    let result = program.evaluate(&Activation::new())?;

    let sv = result.as_struct().unwrap();
    assert_eq!(sv.type_name(), "test.Order");

    // Verify round-trip
    let env2 = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("order", ValueType::Struct(StructType::new("test.Order")))?
        .build()?;

    let activation =
        Activation::new().bind_variable_dynamic("order", StructValue::from_bytes("test.Order", sv.to_bytes()))?;

    let program = env2.compile("order.buyer.name == 'Carol' && order.items.size() == 2 && order.items[1].price == 250")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_message_construction_field_access_inline() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .build()?;

    // Access a field directly on a constructed message — no round-trip needed
    let program = env.compile("test.SimpleMessage{name: 'Eve', id: 7}.name")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::String("Eve".into()));

    Ok(())
}

#[test]
fn test_message_construction_equality() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.SimpleMessage")))?
        .build()?;

    let bytes = encode_simple_message("Dan", 42);
    let activation =
        Activation::new().bind_variable_dynamic("msg", StructValue::from_bytes("test.SimpleMessage", bytes.as_slice()))?;

    // Constructed message should equal the bound variable with same content
    let program = env.compile("msg == test.SimpleMessage{name: 'Dan', id: 42}")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// =============================================================================
// Advanced Features & Polish
// =============================================================================

// --- 6.1: Dedicated has() tests for proto3 field presence ---

#[test]
fn test_has_proto3_field_presence() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    // Person with all fields set
    let addr = encode_address("123 Main St", "Springfield", "62704");
    let bytes = encode_person("Alice", 30, Some(&addr), 1, &["rust"], true);
    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    // has() returns true for non-default fields
    let cases = [
        ("has(person.name)", true),
        ("has(person.age)", true),
        ("has(person.address)", true),
        ("has(person.active)", true),
    ];
    for (expr, expected) in &cases {
        let result = env.compile(expr)?.evaluate(&activation)?;
        assert_eq!(result, Value::Bool(*expected), "failed for: {expr}");
    }

    // Empty person — all fields at proto3 defaults
    let empty_bytes = encode_person("", 0, None, 0, &[], false);
    let activation2 =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", empty_bytes.as_slice()))?;

    // has() returns false for default-valued proto3 fields
    let default_cases = [
        ("has(person.name)", false),
        ("has(person.age)", false),
        ("has(person.address)", false),
        ("has(person.active)", false),
    ];
    for (expr, expected) in &default_cases {
        let result = env.compile(expr)?.evaluate(&activation2)?;
        assert_eq!(result, Value::Bool(*expected), "failed for default: {expr}");
    }

    Ok(())
}

// --- 6.2: type() on message values ---

#[test]
fn test_type_on_message_value() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;

    let addr = encode_address("123 Main St", "Springfield", "62704");
    let bytes = encode_person("Alice", 30, Some(&addr), 1, &["rust"], true);
    let activation =
        Activation::new().bind_variable_dynamic("person", StructValue::from_bytes("test.Person", bytes.as_slice()))?;

    // type() should return the message type name
    let program = env.compile("type(person) == test.Person")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// --- 6.4: Error handling — descriptive error messages ---

#[test]
fn test_error_unknown_type_declaration() {
    let error = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Struct(StructType::new("test.DoesNotExist")))
        .unwrap()
        .build()
        .expect_err("an unknown message type must fail environment construction");

    assert_eq!(error.code(), Code::InvalidArgument);
    let err_msg = error.to_string();
    assert!(
        err_msg.contains("Message type not found: test.DoesNotExist"),
        "error should describe the invalid type declaration, got: {err_msg}"
    );
}

#[test]
fn test_named_type_values_resolve_descriptors_and_well_known_types() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable::<ValueType>("value")?
        .build()?;
    let program = env.compile("value")?;

    for (source, expected) in [
        (
            ValueType::Struct(StructType::new("test.SimpleMessage")),
            ValueType::Struct(StructType::new("test.SimpleMessage")),
        ),
        (
            ValueType::Enum(EnumType::new("test.Status")),
            ValueType::Enum(EnumType::new("test.Status")),
        ),
        (
            ValueType::Struct(StructType::new("google.protobuf.Duration")),
            ValueType::Duration,
        ),
        (
            ValueType::Struct(StructType::new("google.protobuf.Timestamp")),
            ValueType::Timestamp,
        ),
        (
            ValueType::Struct(StructType::new("google.protobuf.Int64Value")),
            ValueType::IntWrapper,
        ),
        (
            ValueType::Struct(StructType::new("google.protobuf.Value")),
            ValueType::Dyn,
        ),
        (
            ValueType::Struct(StructType::new("google.protobuf.ListValue")),
            ValueType::List(ListType::new(ValueType::Dyn)),
        ),
        (
            ValueType::Struct(StructType::new("google.protobuf.Struct")),
            ValueType::Map(MapType::new(MapKeyType::String, ValueType::Dyn)),
        ),
        (
            ValueType::Enum(EnumType::new("google.protobuf.NullValue")),
            ValueType::Int,
        ),
    ] {
        let activation = Activation::new().bind_variable("value", source)?;
        assert_eq!(program.evaluate(&activation)?, Value::Type(expected));
    }

    Ok(())
}

#[test]
fn test_unknown_named_type_values_return_errors() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<ValueType>("value")?
        .build()?;
    let program = env.compile("value")?;

    for value_type in [
        ValueType::Struct(StructType::new("test.DoesNotExist")),
        ValueType::Enum(EnumType::new("test.DoesNotExist")),
        ValueType::List(ListType::new(ValueType::Struct(StructType::new(
            "test.DoesNotExist",
        )))),
        ValueType::Optional(OptionalType::new(ValueType::Enum(EnumType::new(
            "test.DoesNotExist",
        )))),
    ] {
        let activation = Activation::new().bind_variable("value", value_type)?;
        let error = program
            .evaluate(&activation)
            .expect_err("an unresolved type value must return a CEL error");
        assert_eq!(error.code(), Code::InvalidArgument);
        assert!(error.to_string().contains("test.DoesNotExist"));
    }

    Ok(())
}

#[test]
fn test_error_unknown_enum_type_declaration() {
    let error = Env::builder()
        .declare_variable_with_type("value", ValueType::Enum(EnumType::new("test.DoesNotExist")))
        .unwrap()
        .build()
        .expect_err("an unknown enum type must fail environment construction");

    assert_eq!(error.code(), Code::InvalidArgument);
    assert!(error
        .to_string()
        .contains("Enum type not found: test.DoesNotExist"));
}

#[test]
fn test_error_get_struct_field_unknown_type() {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .build()
        .unwrap();

    // Construct a StructValue with a type name not in the descriptor pool
    let sv = StructValue::from_bytes("test.NonExistent", vec![]);

    let result = env.get_struct_field(&sv, "name");
    assert!(result.is_err(), "expected error for unknown type");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("NonExistent") || err_msg.contains("not found"),
        "error should mention the type name, got: {err_msg}"
    );
}

#[test]
fn malformed_message_bytes_fail_all_deserialization_entry_points() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Dyn)?
        .build()?;
    let program = env.compile("msg")?;

    for (name, bytes) in [
        ("test.SimpleMessage", vec![0x0a]),
        ("test.SimpleMessage", vec![0x0a, 0x03, b'a']),
        ("test.SimpleMessage", vec![0x10, 0x80]),
        ("test.SimpleMessage", vec![0x00]),
        ("test.Person", vec![0x1a, 0x02, 0x0a, 0x80]),
        ("test.Order", vec![0x12, 0x02, 0x10, 0x80]),
    ] {
        let value = StructValue::from_bytes(name, bytes);
        let activation = Activation::new().bind_variable_dynamic("msg", value.clone())?;
        for result in [
            program.evaluate(&activation).map(|_| ()),
            env.get_struct_field(&value, "name").map(|_| ()),
            env.has_struct_field(&value, "name").map(|_| ()),
        ] {
            let error = result.expect_err("malformed protobuf must not produce a partial message");
            assert_eq!(error.code(), Code::InvalidArgument, "{value:?}");
            assert!(error.to_string().contains(name), "{error}");
        }
    }

    let activation = Activation::new().bind_variable_dynamic(
        "msg",
        StructValue::from_bytes("test.SimpleMessage", encode_simple_message("valid", 7)),
    )?;
    assert_eq!(
        env.get_struct_field(program.evaluate(&activation)?.as_struct().unwrap(), "id")?,
        Value::Int(7),
    );
    Ok(())
}

#[test]
fn explicitly_empty_nested_messages_and_wrappers_retain_presence() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("msg", ValueType::Dyn)?
        .build()?;

    for (type_name, field, present_bytes, expected) in [
        ("test.Person", "address", vec![0x1a, 0x00], None),
        (
            "test.Event",
            "optional_count",
            vec![0x22, 0x00],
            Some(Value::Int(0)),
        ),
        (
            "test.Event",
            "optional_label",
            vec![0x2a, 0x00],
            Some(Value::String(String::new().into())),
        ),
        (
            "test.Event",
            "optional_flag",
            vec![0x32, 0x00],
            Some(Value::Bool(false)),
        ),
    ] {
        let absent = StructValue::from_bytes(type_name, vec![]);
        let present = StructValue::from_bytes(type_name, present_bytes);
        let presence = env.compile(format!("has(msg.{field})"))?;

        for (message, expected_presence) in [(&absent, false), (&present, true)] {
            assert_eq!(env.has_struct_field(message, field)?, expected_presence);
            let activation = Activation::new().bind_variable_dynamic("msg", message.clone())?;
            assert_eq!(
                presence.evaluate(&activation)?,
                Value::Bool(expected_presence)
            );
        }
        if let Some(expected) = expected {
            assert_eq!(env.get_struct_field(&absent, field)?, Value::Null);
            assert_eq!(env.get_struct_field(&present, field)?, expected);
        } else {
            let address = env.get_struct_field(&present, field)?;
            assert_eq!(
                env.get_struct_field(address.as_struct().unwrap(), "city")?,
                Value::String(String::new().into())
            );
        }
    }
    Ok(())
}

#[test]
fn duplicate_scalar_fields_use_the_last_value_and_ignore_unknown_fields() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type(
            "msg",
            ValueType::Struct(StructType::new("test.SimpleMessage")),
        )?
        .build()?;
    let mut bytes = encode_simple_message("first", -1);
    bytes.extend_from_slice(&encode_simple_message("last", i32::MAX));
    encode_varint_field(&mut bytes, 100, 42);
    encode_string_field(&mut bytes, 101, "unknown payload");
    let value = StructValue::from_bytes("test.SimpleMessage", bytes);
    let activation = Activation::new().bind_variable_dynamic("msg", value)?;

    assert_eq!(
        env.compile("msg.name")?.evaluate(&activation)?,
        Value::String("last".into())
    );
    assert_eq!(
        env.compile("msg.id")?.evaluate(&activation)?,
        Value::Int(i32::MAX as i64)
    );
    let roundtrip = env.compile("msg")?.evaluate(&activation)?;
    assert_eq!(
        env.get_struct_field(roundtrip.as_struct().unwrap(), "id")?,
        Value::Int(i32::MAX as i64)
    );
    Ok(())
}

#[test]
fn unknown_proto3_enum_numbers_roundtrip_as_integers() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .build()?;
    let program = env.compile("person.status")?;

    for number in [0, 1, 2, 3, -1, i32::MIN, i32::MAX] {
        let person = StructValue::from_bytes(
            "test.Person",
            encode_person("", 0, None, number, &[], false),
        );
        let activation = Activation::new().bind_variable_dynamic("person", person.clone())?;
        assert_eq!(program.evaluate(&activation)?, Value::Int(number as i64));
        assert_eq!(
            env.get_struct_field(&person, "status")?,
            Value::Int(number as i64)
        );
    }
    Ok(())
}

#[test]
fn repeated_empty_strings_and_default_map_entries_are_preserved() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type("person", ValueType::Struct(StructType::new("test.Person")))?
        .declare_variable_with_type("order", ValueType::Struct(StructType::new("test.Order")))?
        .build()?;
    let person = StructValue::from_bytes(
        "test.Person",
        encode_person("", 0, None, 0, &["", "中文", "", "last"], false),
    );
    let order = StructValue::from_bytes(
        "test.Order",
        encode_order(None, &[], &[("", ""), ("key", "first"), ("key", "last")]),
    );
    let activation = Activation::new()
        .bind_variable_dynamic("person", person)?
        .bind_variable_dynamic("order", order)?;

    for expression in [
        "person.tags == ['', '中文', '', 'last']",
        "person.tags.size() == 4",
        "order.labels.size() == 2",
        "'' in order.labels && order.labels[''] == ''",
        "order.labels['key'] == 'last'",
    ] {
        assert_eq!(
            env.compile(expression)?.evaluate(&activation)?,
            Value::Bool(true),
            "{expression}"
        );
    }
    Ok(())
}

#[test]
fn recursively_nested_named_type_errors_keep_the_missing_name() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable::<ValueType>("value")?
        .build()?;
    let program = env.compile("value")?;
    let missing = ValueType::Struct(StructType::new("test.MissingNested"));

    for value_type in [
        ValueType::Map(MapType::new(
            MapKeyType::String,
            ValueType::List(ListType::new(missing.clone())),
        )),
        ValueType::Type(TypeType::new(Some(missing.clone()))),
        ValueType::Opaque(OpaqueType::new(
            "Box",
            vec![ValueType::Int, missing.clone()],
        )),
        ValueType::Function(FunctionType::new(missing.clone(), vec![ValueType::Int])),
        ValueType::Function(FunctionType::new(
            ValueType::Int,
            vec![ValueType::String, missing],
        )),
        ValueType::Struct(StructType::new("test.Status")),
        ValueType::Enum(EnumType::new("test.Person")),
    ] {
        let activation = Activation::new().bind_variable("value", value_type.clone())?;
        let error = program
            .evaluate(&activation)
            .expect_err("invalid nested or wrong-kind named type must fail");
        assert_eq!(error.code(), Code::InvalidArgument, "{value_type:?}");
        assert!(error.to_string().contains("test."), "{error}");

        let error = Env::builder()
            .with_file_descriptor_set(&TEST_DESCRIPTORS)
            .declare_variable_with_type("bad", value_type)?
            .build()
            .expect_err("invalid named type must also fail declaration conversion");
        assert_eq!(error.code(), Code::InvalidArgument);
        assert!(error.to_string().contains("test."), "{error}");
    }
    Ok(())
}

#[test]
fn nested_well_known_types_normalize_without_erasing_regular_enum_types() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable::<ValueType>("value")?
        .build()?;
    let program = env.compile("value")?;
    let duration = ValueType::Struct(StructType::new("google.protobuf.Duration"));
    let timestamp = ValueType::Struct(StructType::new("google.protobuf.Timestamp"));
    let status = ValueType::Enum(EnumType::new("test.Status"));

    for (source, expected) in [
        (
            ValueType::List(ListType::new(duration.clone())),
            ValueType::List(ListType::new(ValueType::Duration)),
        ),
        (
            ValueType::Map(MapType::new(MapKeyType::String, timestamp.clone())),
            ValueType::Map(MapType::new(MapKeyType::String, ValueType::Timestamp)),
        ),
        (
            ValueType::Optional(OptionalType::new(ValueType::Struct(StructType::new(
                "google.protobuf.DoubleValue",
            )))),
            ValueType::Optional(OptionalType::new(ValueType::DoubleWrapper)),
        ),
        (
            ValueType::Type(TypeType::new(Some(ValueType::Struct(StructType::new(
                "google.protobuf.StringValue",
            ))))),
            ValueType::Type(TypeType::new(Some(ValueType::StringWrapper))),
        ),
        (
            ValueType::Opaque(OpaqueType::new(
                "TypedBox",
                vec![status.clone(), duration.clone()],
            )),
            ValueType::Opaque(OpaqueType::new(
                "TypedBox",
                vec![status.clone(), ValueType::Duration],
            )),
        ),
        (
            ValueType::Function(FunctionType::new(timestamp, vec![duration, status.clone()])),
            ValueType::Function(FunctionType::new(
                ValueType::Timestamp,
                vec![ValueType::Duration, status],
            )),
        ),
    ] {
        let activation = Activation::new().bind_variable("value", source.clone())?;
        assert_eq!(
            program.evaluate(&activation)?,
            Value::Type(expected),
            "{source:?}"
        );
    }
    Ok(())
}

#[test]
fn same_message_name_is_resolved_with_each_environments_own_pool() -> Result<(), Error> {
    use prost::Message;

    let mut descriptors =
        prost_types::FileDescriptorSet::decode(TEST_DESCRIPTORS.as_slice()).unwrap();
    let message = descriptors
        .file
        .iter_mut()
        .flat_map(|file| file.message_type.iter_mut())
        .find(|message| message.name.as_deref() == Some("SimpleMessage"))
        .unwrap();
    message.field[0].name = Some("label".into());
    message.field[0].json_name = Some("label".into());

    let original = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable_with_type(
            "msg",
            ValueType::Struct(StructType::new("test.SimpleMessage")),
        )?
        .build()?;
    let renamed = Env::builder()
        .with_file_descriptor_set(&descriptors.encode_to_vec())
        .declare_variable_with_type(
            "msg",
            ValueType::Struct(StructType::new("test.SimpleMessage")),
        )?
        .build()?;
    assert!(original.compile("msg.label").is_err());
    assert!(renamed.compile("msg.name").is_err());

    let value = StructValue::from_bytes(
        "test.SimpleMessage",
        encode_simple_message("same wire data", 1),
    );
    let activation = Activation::new().bind_variable_dynamic("msg", value.clone())?;
    for _ in 0..4 {
        assert_eq!(
            original.compile("msg.name")?.evaluate(&activation)?,
            Value::String("same wire data".into())
        );
        assert_eq!(
            renamed.compile("msg.label")?.evaluate(&activation)?,
            Value::String("same wire data".into())
        );
    }
    drop(original);
    assert_eq!(
        renamed.get_struct_field(&value, "label")?,
        Value::String("same wire data".into())
    );
    Ok(())
}

#[test]
fn invalid_descriptor_sets_fail_without_poisoning_later_environments() -> Result<(), Error> {
    use prost::Message;

    let mut missing_dependency =
        prost_types::FileDescriptorSet::decode(TEST_DESCRIPTORS.as_slice()).unwrap();
    missing_dependency
        .file
        .iter_mut()
        .find(|file| file.package.as_deref() == Some("test"))
        .unwrap()
        .dependency
        .push("missing.proto".into());

    for bytes in [
        vec![0x0a],
        vec![0x0a, 0x80],
        vec![0x00],
        missing_dependency.encode_to_vec(),
    ] {
        assert!(Env::builder()
            .with_file_descriptor_set(&bytes)
            .build()
            .is_err());
    }
    for bytes in [&[][..], TEST_DESCRIPTORS.as_slice()] {
        let env = Env::builder().with_file_descriptor_set(bytes).build()?;
        assert_eq!(
            env.compile("duration('1s') == duration('1000ms')")?
                .evaluate(())?,
            Value::Bool(true)
        );
    }
    Ok(())
}
