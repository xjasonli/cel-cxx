#![cfg(feature = "prost")]

use cel_cxx::*;
use std::sync::LazyLock;

// Import prost-generated types (with ProstValue derived via build.rs)
include!(concat!(env!("OUT_DIR"), "/test.rs"));

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
    let msg = SimpleMessage {
        name: "hello".into(),
        id: 42,
    };
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
    let msg = SimpleMessage {
        name: "test".into(),
        id: 1,
    };
    let value: Value = msg.into();
    assert!(matches!(value, Value::Struct(_)));
}

// --- FromValue ---

#[test]
fn test_from_value_roundtrip() {
    let original = SimpleMessage {
        name: "roundtrip".into(),
        id: 99,
    };
    let value = original.clone().into_value();
    let recovered = SimpleMessage::from_value(&value).unwrap();
    assert_eq!(recovered, original);
}

#[test]
fn test_try_from_value_owned() {
    let msg = SimpleMessage {
        name: "try".into(),
        id: 7,
    };
    let value = msg.clone().into_value();
    let recovered = SimpleMessage::try_from(value).unwrap();
    assert_eq!(recovered, msg);
}

#[test]
fn test_try_from_value_ref() {
    let msg = SimpleMessage {
        name: "ref".into(),
        id: 8,
    };
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
fn test_declare_and_bind_prost_message() -> Result<(), Error> {
    let env = Env::builder()
        .with_file_descriptor_set(&TEST_DESCRIPTORS)
        .declare_variable::<SimpleMessage>("msg")?
        .build()?;

    let program = env.compile("msg.name")?;

    let msg = SimpleMessage {
        name: "Alice".into(),
        id: 1,
    };
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

    let msg = SimpleMessage {
        name: "Bob".into(),
        id: 42,
    };
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

    let person = Person {
        name: "Charlie".into(),
        age: 30,
        address: Some(Address {
            street: "123 Main St".into(),
            city: "Springfield".into(),
            zip: "62701".into(),
        }),
        status: Status::Active as i32,
        tags: vec!["admin".into()],
        active: true,
    };
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

    // CEL can construct messages using Type{field: value} syntax
    let program = env.compile("test.SimpleMessage{name: 'constructed', id: 100}")?;
    let result = program.evaluate(&Activation::new())?;

    let recovered = SimpleMessage::from_value(&result).unwrap();
    assert_eq!(recovered.name, "constructed");
    assert_eq!(recovered.id, 100);
    Ok(())
}

// --- type_name override ---

#[derive(Clone, PartialEq, prost::Message, cel_cxx::ProstValue)]
#[cel_cxx(type_name = "custom.OverriddenType")]
pub struct CustomTypeNameMsg {
    #[prost(string, tag = "1")]
    pub data: String,
}

#[test]
fn test_type_name_override() {
    let vt = <CustomTypeNameMsg as TypedValue>::value_type();
    assert_eq!(
        vt,
        ValueType::Struct(StructType::new("custom.OverriddenType"))
    );

    let msg = CustomTypeNameMsg {
        data: "test".into(),
    };
    let value = msg.into_value();
    match &value {
        Value::Struct(sv) => assert_eq!(sv.type_name(), "custom.OverriddenType"),
        other => panic!("expected Value::Struct, got {other:?}"),
    }
}
