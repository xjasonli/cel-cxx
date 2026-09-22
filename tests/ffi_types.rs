use cel_cxx::*;

#[test]
fn opaque_type_parameters_roundtrip_across_inline_storage_boundary() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<ValueType>("value")?
        .build()?;
    let program = env.compile("value")?;
    let parameters = [
        ValueType::String,
        ValueType::Duration,
        ValueType::Optional(OptionalType::new(ValueType::Int)),
        ValueType::List(ListType::new(ValueType::Timestamp)),
        ValueType::Bool,
        ValueType::Uint,
        ValueType::Double,
        ValueType::Map(MapType::new(MapKeyType::String, ValueType::Bytes)),
    ];

    for count in 0..=parameters.len() {
        let value_type = ValueType::Opaque(OpaqueType::new(
            "ParameterBoundary",
            parameters[..count].to_vec(),
        ));
        let activation = Activation::new().bind_variable("value", value_type.clone())?;
        assert_eq!(
            program.evaluate(&activation)?,
            Value::Type(value_type),
            "parameter count {count}",
        );
    }
    Ok(())
}

#[test]
fn type_parameter_names_roundtrip_in_nested_types() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<ValueType>("value")?
        .build()?;
    let program = env.compile("value")?;

    for name in [
        "",
        "T",
        "类型参数",
        "before\0after",
        "LongTypeParameterNameThatExceedsSmallStringStorage",
    ] {
        let parameter = ValueType::TypeParam(TypeParamType::new(name));
        for value_type in [
            parameter.clone(),
            ValueType::List(ListType::new(parameter.clone())),
            ValueType::Map(MapType::new(
                MapKeyType::TypeParam(TypeParamType::new(name)),
                ValueType::Int,
            )),
            ValueType::Map(MapType::new(MapKeyType::String, parameter.clone())),
            ValueType::Optional(OptionalType::new(parameter.clone())),
            ValueType::Type(TypeType::new(Some(parameter.clone()))),
            ValueType::Opaque(OpaqueType::new("Generic", vec![parameter.clone()])),
            ValueType::Function(FunctionType::new(parameter.clone(), vec![parameter])),
        ] {
            let activation = Activation::new().bind_variable("value", value_type.clone())?;
            assert_eq!(
                program.evaluate(&activation)?,
                Value::Type(value_type),
                "type parameter name {name:?}",
            );
        }
    }
    Ok(())
}
