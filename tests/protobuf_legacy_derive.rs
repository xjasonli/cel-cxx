#![cfg(feature = "protobuf-legacy")]

use cel_cxx::*;
use std::sync::LazyLock;

// Import protobuf-legacy generated types (with ProtobufLegacyValue derived via build.rs post-processing)
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_mut)]
mod proto {
    include!(concat!(
        env!("OUT_DIR"),
        "/protobuf_legacy_gen/test.rs"
    ));
}

use proto::*;

static TEST_DESCRIPTORS: LazyLock<Vec<u8>> = LazyLock::new(|| {
    use prost::Message;
    protox::compile(["tests/fixtures/test.proto"], ["tests/fixtures/"])
        .unwrap()
        .encode_to_vec()
});

// --- TypedValue ---

#[test]
fn test_typed_value_returns_struct_type() {
    let vt = <SimpleMessage as TypedValue>::value_type();
    assert_eq!(vt, ValueType::Struct(StructType::new("test.SimpleMessage")));
}

#[test]
fn test_typed_value_nested_type() {
    let vt = <Person as TypedValue>::value_type();
    assert_eq!(vt, ValueType::Struct(StructType::new("test.Person")));
}

// --- IntoValue ---

#[test]
fn test_into_value_produces_struct() {
    let mut msg = SimpleMessage::new();
    msg.name = "hello".into();
    msg.id = 42;

    let value = msg.into_value();
    match &value {
        Value::Struct(sv) => {
            assert_eq!(sv.type_name(), "test.SimpleMessage");
            assert!(!sv.to_bytes().is_empty());
        }
        other => panic!("expected Value::Struct, got {other:?}"),
    }
}

#[test]
fn test_from_trait_conversion() {
    let mut msg = SimpleMessage::new();
    msg.name = "test".into();
    msg.id = 1;

    let value: Value = msg.into();
    assert!(matches!(value, Value::Struct(_)));
}

// --- FromValue ---

#[test]
fn test_from_value_roundtrip() {
    let mut original = SimpleMessage::new();
    original.name = "roundtrip".into();
    original.id = 99;

    let value = original.clone().into_value();
    let recovered = SimpleMessage::from_value(&value).unwrap();
    assert_eq!(recovered, original);
}

#[test]
fn test_try_from_value_owned() {
    let mut msg = SimpleMessage::new();
    msg.name = "try".into();
    msg.id = 7;

    let value = msg.clone().into_value();
    let recovered = SimpleMessage::try_from(value).unwrap();
    assert_eq!(recovered, msg);
}

#[test]
fn test_try_from_value_ref() {
    let mut msg = SimpleMessage::new();
    msg.name = "ref".into();
    msg.id = 8;

    let value = msg.clone().into_value();
    let recovered = SimpleMessage::try_from(&value).unwrap();
    assert_eq!(recovered, msg);
}

#[test]
fn test_from_value_wrong_type_returns_error() {
    let value = Value::Int(42);
    let result = SimpleMessage::from_value(&value);
    assert!(result.is_err());
}

// --- Full CEL integration ---

#[test]
fn test_declare_and_bind_protobuf_legacy_message() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable::<SimpleMessage>("msg")?
        .build()?;

    let program = env.compile("msg.name")?;

    let mut msg = SimpleMessage::new();
    msg.name = "Alice".into();
    msg.id = 1;

    let activation = Activation::new().bind_variable("msg", msg)?;

    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::String("Alice".into()));
    Ok(())
}

#[test]
fn test_cel_field_access_int() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable::<SimpleMessage>("msg")?
        .build()?;

    let program = env.compile("msg.id")?;

    let mut msg = SimpleMessage::new();
    msg.name = "Bob".into();
    msg.id = 42;

    let activation = Activation::new().bind_variable("msg", msg)?;

    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Int(42));
    Ok(())
}

#[test]
fn test_nested_message_roundtrip() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable::<Person>("p")?
        .build()?;

    let program = env.compile("p.address.city")?;

    let mut address = Address::new();
    address.street = "123 Main St".into();
    address.city = "Springfield".into();
    address.zip = "62701".into();

    let mut person = Person::new();
    person.name = "Charlie".into();
    person.age = 30;
    person.address = ::protobuf::MessageField::some(address);
    person.status = proto::Status::STATUS_ACTIVE.into();
    person.tags = vec!["admin".into()];
    person.active = true;

    let activation = Activation::new().bind_variable("p", person)?;

    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::String("Springfield".into()));
    Ok(())
}

#[test]
fn test_cel_eval_returns_message_and_from_value() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable::<SimpleMessage>("msg")?
        .build()?;

    let program = env.compile("test.SimpleMessage{name: 'constructed', id: 100}")?;
    let result = program.evaluate(&Activation::new())?;

    let recovered = SimpleMessage::from_value(&result).unwrap();
    assert_eq!(recovered.name, "constructed");
    assert_eq!(recovered.id, 100);
    Ok(())
}
