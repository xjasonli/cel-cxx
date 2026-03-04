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
fn test_bind_protobuf_variable_field_access() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    let bytes = encode_simple_message("Alice", 42);

    let activation = Activation::new().bind_protobuf_variable(
        "msg",
        "test.SimpleMessage",
        &bytes,
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
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    let bytes = encode_simple_message("Bob", 99);

    let activation = Activation::new().bind_protobuf_variable(
        "msg",
        "test.SimpleMessage",
        &bytes,
    )?;

    // Evaluate `msg` directly — should get a Value::Struct back
    let program = env.compile("msg")?;
    let result = program.evaluate(&activation)?;

    match &result {
        Value::Struct(s) => {
            assert_eq!(s.type_name, "test.SimpleMessage");
            // The bytes should round-trip (serialize the same message back)
            assert!(!s.bytes.is_empty());
        }
        other => panic!("Expected Value::Struct, got: {other:?}"),
    }

    Ok(())
}

#[test]
fn test_protobuf_field_access_string_result() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    let bytes = encode_simple_message("Charlie", 7);

    let activation = Activation::new().bind_protobuf_variable(
        "msg",
        "test.SimpleMessage",
        &bytes,
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
// Phase 2A: Full Protobuf Field Access Tests
// =============================================================================

// --- 2A.1: Scalar fields ---

#[test]
fn test_scalar_bool_field() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    let addr = encode_address("123 Main St", "Springfield", "62704");
    let bytes = encode_person("Alice", 30, Some(&addr), 1, &["rust"], true);

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    let program = env.compile("person.active == true")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_scalar_default_values() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    // Empty person — all fields at defaults
    let bytes = encode_person("", 0, None, 0, &[], false);

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

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
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    let addr = encode_address("456 Market St", "San Francisco", "94105");
    let bytes = encode_person("Bob", 25, Some(&addr), 0, &[], false);

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    let program = env.compile("person.address.city == 'San Francisco'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_deeply_nested_field_access() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("order", "test.Order")?
        .build()?;

    let addr = encode_address("789 Broadway", "NYC", "10001");
    let buyer = encode_person("Carol", 40, Some(&addr), 0, &[], false);
    let bytes = encode_order(Some(&buyer), &[], &[]);

    let activation =
        Activation::new().bind_protobuf_variable("order", "test.Order", &bytes)?;

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
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    let bytes = encode_person("Dave", 28, None, 0, &["rust", "cel", "proto"], false);

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    let program = env.compile("person.tags.size() == 3")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_repeated_string_index() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    let bytes = encode_person("Dave", 28, None, 0, &["rust", "cel", "proto"], false);

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    let program = env.compile("person.tags[0] == 'rust'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_repeated_string_exists() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    let bytes = encode_person("Dave", 28, None, 0, &["rust", "cel", "proto"], false);

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    let program = env.compile("person.tags.exists(t, t == 'cel')")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_repeated_message_field() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("order", "test.Order")?
        .build()?;

    let item1 = encode_item("Widget", 100);
    let item2 = encode_item("Gadget", 250);
    let bytes = encode_order(None, &[&item1, &item2], &[]);

    let activation =
        Activation::new().bind_protobuf_variable("order", "test.Order", &bytes)?;

    let program = env.compile("order.items[0].name == 'Widget'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_repeated_message_exists() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("order", "test.Order")?
        .build()?;

    let item1 = encode_item("Widget", 100);
    let item2 = encode_item("Gadget", 250);
    let bytes = encode_order(None, &[&item1, &item2], &[]);

    let activation =
        Activation::new().bind_protobuf_variable("order", "test.Order", &bytes)?;

    let program = env.compile("order.items.exists(i, i.price > 200)")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_repeated_empty() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    let bytes = encode_person("Eve", 22, None, 0, &[], false);

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

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
        .declare_protobuf_variable("order", "test.Order")?
        .build()?;

    let bytes = encode_order(None, &[], &[("env", "prod"), ("region", "us-east")]);

    let activation =
        Activation::new().bind_protobuf_variable("order", "test.Order", &bytes)?;

    let program = env.compile(r#"order.labels["env"] == "prod""#)?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_map_contains_key() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("order", "test.Order")?
        .build()?;

    let bytes = encode_order(None, &[], &[("env", "prod"), ("region", "us-east")]);

    let activation =
        Activation::new().bind_protobuf_variable("order", "test.Order", &bytes)?;

    let program = env.compile(r#""env" in order.labels"#)?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_map_size() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("order", "test.Order")?
        .build()?;

    let bytes = encode_order(None, &[], &[("env", "prod"), ("region", "us-east")]);

    let activation =
        Activation::new().bind_protobuf_variable("order", "test.Order", &bytes)?;

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
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    // status = STATUS_ACTIVE = 1
    let bytes = encode_person("Frank", 35, None, 1, &[], false);

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    let program = env.compile("person.status == 1")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_enum_field_named_constant() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    // status = STATUS_ACTIVE = 1
    let bytes = encode_person("Grace", 29, None, 1, &[], false);

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    let program = env.compile("person.status == test.Status.STATUS_ACTIVE")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_enum_field_default() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    // status = STATUS_UNSPECIFIED = 0 (default)
    let bytes = encode_person("Hank", 50, None, 0, &[], false);

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    let program = env.compile("person.status == 0")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// =============================================================================
// Phase 2B: Well-Known Type Handling Tests
// =============================================================================

// --- 2B.1: Duration fields ---

#[test]
fn test_wkt_duration_field() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("event", "test.Event")?
        .build()?;

    let ttl = encode_duration(3600, 0);
    let bytes = encode_event("test", None, Some(&ttl), None, None, None, None, None);

    let activation =
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

    let program = env.compile("event.ttl == duration('3600s')")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_wkt_duration_comparison() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("event", "test.Event")?
        .build()?;

    let ttl = encode_duration(3600, 0); // 1 hour
    let bytes = encode_event("test", None, Some(&ttl), None, None, None, None, None);

    let activation =
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

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
        .declare_protobuf_variable("event", "test.Event")?
        .build()?;

    // 2024-01-01T00:00:00Z = 1704067200 seconds since epoch
    let created_at = encode_timestamp(1704067200, 0);
    let bytes = encode_event("test", Some(&created_at), None, None, None, None, None, None);

    let activation =
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

    let program = env.compile("event.created_at == timestamp('2024-01-01T00:00:00Z')")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_wkt_timestamp_comparison() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("event", "test.Event")?
        .build()?;

    // 2024-01-01T00:00:00Z
    let created_at = encode_timestamp(1704067200, 0);
    let bytes = encode_event("test", Some(&created_at), None, None, None, None, None, None);

    let activation =
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

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
        .declare_protobuf_variable("event", "test.Event")?
        .build()?;

    let count = encode_int64_value(42);
    let bytes = encode_event("test", None, None, Some(&count), None, None, None, None);

    let activation =
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

    let program = env.compile("event.optional_count == 42")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_wkt_string_wrapper() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("event", "test.Event")?
        .build()?;

    let label = encode_string_value("important");
    let bytes = encode_event("test", None, None, None, Some(&label), None, None, None);

    let activation =
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

    let program = env.compile("event.optional_label == 'important'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_wkt_bool_wrapper() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("event", "test.Event")?
        .build()?;

    let flag = encode_bool_value(true);
    let bytes = encode_event("test", None, None, None, None, Some(&flag), None, None);

    let activation =
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

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
        .declare_protobuf_variable("event", "test.Event")?
        .build()?;

    let metadata = encode_struct_with_string_values(&[("env", "prod"), ("version", "1.0")]);
    let bytes = encode_event("test", None, None, None, None, None, Some(&metadata), None);

    let activation =
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

    let program = env.compile("event.metadata.env == 'prod'")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

#[test]
fn test_wkt_struct_has_key() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("event", "test.Event")?
        .build()?;

    let metadata = encode_struct_with_string_values(&[("env", "prod"), ("version", "1.0")]);
    let bytes = encode_event("test", None, None, None, None, None, Some(&metadata), None);

    let activation =
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

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
        .declare_protobuf_variable("event", "test.Event")?
        .build()?;

    // Pack a SimpleMessage into Any
    let inner = encode_simple_message("packed", 1);
    let payload = encode_any("type.googleapis.com/test.SimpleMessage", &inner);
    let bytes = encode_event("test", None, None, None, None, None, None, Some(&payload));

    let activation =
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

    // Try to check if the Any field is present via has()
    let program = env.compile("has(event.payload)")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// =============================================================================
// Phase 3B: Extract Protobuf Bytes from Evaluation Results
// =============================================================================

#[test]
fn test_as_protobuf_bytes() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    let bytes = encode_simple_message("Alice", 42);

    let activation = Activation::new().bind_protobuf_variable(
        "msg",
        "test.SimpleMessage",
        &bytes,
    )?;

    let program = env.compile("msg")?;
    let result = program.evaluate(&activation)?;

    let (type_name, result_bytes) = result.as_protobuf_bytes()?;
    assert_eq!(type_name, "test.SimpleMessage");
    assert!(!result_bytes.is_empty());

    Ok(())
}

#[test]
fn test_as_protobuf_bytes_error_on_non_struct() -> Result<(), Error> {
    let env = Env::builder().build()?;

    let program = env.compile("42")?;
    let result = program.evaluate(&Activation::new())?;

    assert!(result.as_protobuf_bytes().is_err());

    Ok(())
}

#[test]
fn test_struct_value_from_value() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    let bytes = encode_simple_message("Bob", 99);

    let activation = Activation::new().bind_protobuf_variable(
        "msg",
        "test.SimpleMessage",
        &bytes,
    )?;

    let program = env.compile("msg")?;
    let result = program.evaluate(&activation)?;

    // Extract via FromValue
    let sv = StructValue::from_value(&result)?;
    assert_eq!(sv.type_name, "test.SimpleMessage");
    assert!(!sv.bytes.is_empty());

    // Extract via borrowed reference
    let sv_ref = <&StructValue>::from_value(&result)?;
    assert_eq!(sv_ref.type_name, "test.SimpleMessage");

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
// Phase 3C: Rust-side Protobuf Field Access
// =============================================================================

#[test]
fn test_get_protobuf_field_string() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    let bytes = encode_simple_message("Alice", 42);
    let activation =
        Activation::new().bind_protobuf_variable("msg", "test.SimpleMessage", &bytes)?;

    let result = env.compile("msg")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    let name = env.get_protobuf_field(&sv, "name")?;
    assert_eq!(name, Value::String("Alice".into()));

    Ok(())
}

#[test]
fn test_get_protobuf_field_int() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    let bytes = encode_simple_message("Bob", 99);
    let activation =
        Activation::new().bind_protobuf_variable("msg", "test.SimpleMessage", &bytes)?;

    let result = env.compile("msg")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    let id = env.get_protobuf_field(&sv, "id")?;
    assert_eq!(id, Value::Int(99));

    Ok(())
}

#[test]
fn test_get_protobuf_field_nested() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    let addr = encode_address("123 Main St", "Springfield", "62704");
    let bytes = encode_person("Alice", 30, Some(&addr), 1, &["rust"], true);
    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    let result = env.compile("person")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    // Get the nested address field
    let address_value = env.get_protobuf_field(&sv, "address")?;
    let address_sv = StructValue::from_value(&address_value)?;

    // Now get city from the nested address
    let city = env.get_protobuf_field(&address_sv, "city")?;
    assert_eq!(city, Value::String("Springfield".into()));

    Ok(())
}

#[test]
fn test_get_protobuf_field_not_found() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    let bytes = encode_simple_message("Alice", 42);
    let activation =
        Activation::new().bind_protobuf_variable("msg", "test.SimpleMessage", &bytes)?;

    let result = env.compile("msg")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    // cel-cpp returns an error Value for non-existent fields
    let field = env.get_protobuf_field(&sv, "nonexistent");
    match field {
        Err(_) => {} // status error
        Ok(Value::Error(_)) => {} // error value
        Ok(other) => panic!("Expected error for nonexistent field, got: {other:?}"),
    }

    Ok(())
}

#[test]
fn test_has_protobuf_field() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    let addr = encode_address("123 Main St", "Springfield", "62704");
    let bytes = encode_person("Alice", 30, Some(&addr), 1, &["rust"], true);
    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    let result = env.compile("person")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    assert!(env.has_protobuf_field(&sv, "name")?);
    assert!(env.has_protobuf_field(&sv, "address")?);
    // "active" is true, so has() returns true
    assert!(env.has_protobuf_field(&sv, "active")?);

    // Empty person — fields at defaults
    let empty_bytes = encode_person("", 0, None, 0, &[], false);
    let activation2 =
        Activation::new().bind_protobuf_variable("person", "test.Person", &empty_bytes)?;
    let result2 = env.compile("person")?.evaluate(&activation2)?;
    let sv2 = StructValue::from_value(&result2)?;

    // Proto3 default values: has() returns false for default-valued fields
    assert!(!env.has_protobuf_field(&sv2, "name")?);
    assert!(!env.has_protobuf_field(&sv2, "address")?);
    assert!(!env.has_protobuf_field(&sv2, "active")?);

    Ok(())
}

#[test]
fn test_get_protobuf_field_wkt() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("event", "test.Event")?
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
        Activation::new().bind_protobuf_variable("event", "test.Event", &bytes)?;

    let result = env.compile("event")?.evaluate(&activation)?;
    let sv = StructValue::from_value(&result)?;

    // Duration field should return a CEL duration value
    let ttl_value = env.get_protobuf_field(&sv, "ttl")?;
    match &ttl_value {
        Value::Duration(d) => {
            assert_eq!(d.num_seconds(), 3600);
        }
        other => panic!("Expected Value::Duration, got: {other:?}"),
    }

    // Timestamp field should return a CEL timestamp value
    let ts_value = env.get_protobuf_field(&sv, "created_at")?;
    match &ts_value {
        Value::Timestamp(_) => {} // ok, it's a timestamp
        other => panic!("Expected Value::Timestamp, got: {other:?}"),
    }

    Ok(())
}

// =============================================================================
// Phase 4: Message Construction in CEL Expressions
// =============================================================================

#[test]
fn test_message_construction_simple() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .build()?;

    let program = env.compile("test.SimpleMessage{name: 'Bob', id: 25}")?;
    let result = program.evaluate(&Activation::new())?;

    let (type_name, bytes) = result.as_protobuf_bytes()?;
    assert_eq!(type_name, "test.SimpleMessage");
    assert!(!bytes.is_empty());

    // Verify by binding the result back and reading fields
    let env2 = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    let activation =
        Activation::new().bind_protobuf_variable("msg", "test.SimpleMessage", bytes)?;

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

    let (type_name, bytes) = result.as_protobuf_bytes()?;
    assert_eq!(type_name, "test.Person");

    // Verify nested field access
    let env2 = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", bytes)?;

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

    let (type_name, bytes) = result.as_protobuf_bytes()?;
    assert_eq!(type_name, "test.Order");

    // Verify round-trip
    let env2 = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("order", "test.Order")?
        .build()?;

    let activation =
        Activation::new().bind_protobuf_variable("order", "test.Order", bytes)?;

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
        .declare_protobuf_variable("msg", "test.SimpleMessage")?
        .build()?;

    let bytes = encode_simple_message("Dan", 42);
    let activation =
        Activation::new().bind_protobuf_variable("msg", "test.SimpleMessage", &bytes)?;

    // Constructed message should equal the bound variable with same content
    let program = env.compile("msg == test.SimpleMessage{name: 'Dan', id: 42}")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// =============================================================================
// Phase 6: Advanced Features & Polish
// =============================================================================

// --- 6.1: Dedicated has() tests for proto3 field presence ---

#[test]
fn test_has_proto3_field_presence() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    // Person with all fields set
    let addr = encode_address("123 Main St", "Springfield", "62704");
    let bytes = encode_person("Alice", 30, Some(&addr), 1, &["rust"], true);
    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

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
        Activation::new().bind_protobuf_variable("person", "test.Person", &empty_bytes)?;

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
        .declare_protobuf_variable("person", "test.Person")?
        .build()?;

    let addr = encode_address("123 Main St", "Springfield", "62704");
    let bytes = encode_person("Alice", 30, Some(&addr), 1, &["rust"], true);
    let activation =
        Activation::new().bind_protobuf_variable("person", "test.Person", &bytes)?;

    // type() should return the message type name
    let program = env.compile("type(person) == test.Person")?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Bool(true));

    Ok(())
}

// --- 6.4: Error handling — descriptive error messages ---

#[test]
fn test_error_unknown_type_compile() {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_protobuf_variable("msg", "test.DoesNotExist")
        .unwrap()
        .build()
        .unwrap();

    // Compiling an expression that accesses a field on the unknown type should fail
    // with a descriptive error message
    let result = env.compile("msg.name");
    assert!(result.is_err(), "expected compile error for unknown type field access");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("undeclared reference"),
        "error should mention undeclared reference, got: {err_msg}"
    );
}

#[test]
fn test_error_get_protobuf_field_unknown_type() {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .build()
        .unwrap();

    // Construct a StructValue with a type name not in the descriptor pool
    let sv = StructValue::new("test.NonExistent", vec![]);

    let result = env.get_protobuf_field(&sv, "name");
    assert!(result.is_err(), "expected error for unknown type");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("NonExistent") || err_msg.contains("not found"),
        "error should mention the type name, got: {err_msg}"
    );
}
