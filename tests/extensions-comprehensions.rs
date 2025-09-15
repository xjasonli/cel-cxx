use cel_cxx::*;
use std::collections::HashMap;

fn create_env() -> Result<Env<'static>, Error> {
    Env::builder().with_ext_comprehensions(true).build()
}

#[test]
fn test_all_list() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("[1, 2, 3].all(i, j, i < j)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Bool(true));
    Ok(())
}

#[test]
fn test_all_map() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("{'hello': 'world', 'taco': 'taco'}.all(k, v, k != v)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Bool(false));
    Ok(())
}

#[test]
fn test_all_nested() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile(
        "{'h': ['hello', 'hi'], 'j': ['joke', 'jog']}.all(k, vals, vals.all(v, v.startsWith(k)))",
    )?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Bool(true));
    Ok(())
}

#[test]
fn test_exists_map() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile(
        "{'greeting': 'hello', 'farewell': 'goodbye'}.exists(k, v, k.startsWith('good') || v.endsWith('bye'))",
    )?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Bool(true));
    Ok(())
}

#[test]
fn test_exists_list() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("[1, 2, 4, 8, 16].exists(i, v, v == 1024 && i == 10)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Bool(false));
    Ok(())
}

#[test]
fn test_exists_one_list_false() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("[1, 2, 1, 3, 1, 4].existsOne(i, v, i == 1 || v == 1)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Bool(false));
    Ok(())
}

#[test]
fn test_exists_one_list_true() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("[1, 1, 2, 2, 3, 3].existsOne(i, v, i == 2 && v == 2)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Bool(true));
    Ok(())
}

#[test]
fn test_exists_one_map() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("{'i': 0, 'j': 1, 'k': 2}.existsOne(i, v, i == 'l' || v == 1)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Bool(true));
    Ok(())
}

#[test]
fn test_transform_list_from_list() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("[1, 2, 3].transformList(indexVar, valueVar, (indexVar * valueVar) + valueVar)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(
        result,
        Value::List(vec![Value::Int(1), Value::Int(4), Value::Int(9)])
    );
    Ok(())
}

#[test]
fn test_transform_list_from_list_with_filter() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile(
        "[1, 2, 3].transformList(indexVar, valueVar, indexVar % 2 != 0, (indexVar * valueVar) + valueVar)",
    )?;
    let result = program.evaluate(&Activation::new())?;
    // The original example had indexVar % 2 == 0 which results in [1,9]. 
    // But index 0 is the first element, so value is 1, (0*1)+1 = 1
    // Index 1 is the second element, value 2, filtered out.
    // Index 2 is the third element, value 3, (2*3)+3 = 9
    // So [1, 9] is the correct result for `indexVar % 2 == 0`.
    // Let's use `indexVar % 2 != 0` to get a different result.
    // Index 0: filtered out
    // Index 1: (1*2)+2 = 4
    // Index 2: filtered out
    assert_eq!(result, Value::List(vec![Value::Int(4)]));
    Ok(())
}

#[test]
fn test_transform_list_from_map_keys() -> Result<(), Error> {
    let env = create_env()?;
    let program =
        env.compile("{'greeting': 'hello', 'farewell': 'goodbye'}.transformList(k, _, k)")?;
    let result = program.evaluate(&Activation::new())?;
    let mut list = result.into_list().unwrap().to_vec();
    
    // Sort for consistent comparison as map key order is not guaranteed.
    list.sort_by(|a, b| a.as_string().unwrap().cmp(b.as_string().unwrap()));

    assert_eq!(
        list,
        vec![
            Value::String("farewell".into()),
            Value::String("greeting".into())
        ]
    );
    Ok(())
}

#[test]
fn test_transform_list_from_map_values() -> Result<(), Error> {
    let env = create_env()?;
    let program =
        env.compile("{'greeting': 'hello', 'farewell': 'goodbye'}.transformList(_, v, v)")?;
    let result = program.evaluate(&Activation::new())?;
    let mut list = result.into_list().unwrap().to_vec();
    
    // Sort for consistent comparison as map key order is not guaranteed.
    list.sort_by(|a, b| a.as_string().unwrap().cmp(b.as_string().unwrap()));

    assert_eq!(
        list,
        vec![
            Value::String("goodbye".into()),
            Value::String("hello".into()),
        ]
    );
    Ok(())
}

#[test]
fn test_transform_map_from_list() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("[1, 2, 3].transformMap(indexVar, valueVar, (indexVar * valueVar) + valueVar)")?;
    let result = program.evaluate(&Activation::new())?;
    let expected: HashMap<MapKey, Value> = [
        (MapKey::Int(0), Value::Int(1)),
        (MapKey::Int(1), Value::Int(4)),
        (MapKey::Int(2), Value::Int(9)),
    ]
    .iter()
    .cloned()
    .collect();
    assert_eq!(result, Value::Map(expected));
    Ok(())
}

#[test]
fn test_transform_map_from_list_with_filter() -> Result<(), Error> {
    let env = create_env()?;
    let program = env
        .compile("[1, 2, 3].transformMap(indexVar, valueVar, indexVar % 2 == 0, (indexVar * valueVar) + valueVar)")?;
    let result = program.evaluate(&Activation::new())?;
    let expected: HashMap<MapKey, Value> =
        [(MapKey::Int(0), Value::Int(1)), (MapKey::Int(2), Value::Int(9))]
            .iter()
            .cloned()
            .collect();
    assert_eq!(result, Value::Map(expected));
    Ok(())
}

#[test]
fn test_transform_map_from_map() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("{'greeting': 'hello'}.transformMap(k, v, v + '!')")?;
    let result = program.evaluate(&Activation::new())?;
    let expected: HashMap<MapKey, Value> =
        [(MapKey::String("greeting".into()), Value::String("hello!".into()))]
            .iter()
            .cloned()
            .collect();
    assert_eq!(result, Value::Map(expected));
    Ok(())
}

#[test]
fn test_transform_map_entry_from_map() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("{'greeting': 'hello'}.transformMapEntry(keyVar, valueVar, {valueVar: keyVar})")?;
    let result = program.evaluate(&Activation::new())?;
    let expected: HashMap<MapKey, Value> =
        [(MapKey::String("hello".into()), Value::String("greeting".into()))]
            .iter()
            .cloned()
            .collect();
    assert_eq!(result, Value::Map(expected));
    Ok(())
}

#[test]
fn test_transform_map_entry_from_list() -> Result<(), Error> {
    let env = create_env()?;
    let program = env.compile("[1, 2, 3].transformMapEntry(indexVar, valueVar, {valueVar: indexVar})")?;
    let result = program.evaluate(&Activation::new())?;
    let expected: HashMap<MapKey, Value> = [
        (MapKey::Int(1), Value::Int(0)),
        (MapKey::Int(2), Value::Int(1)),
        (MapKey::Int(3), Value::Int(2)),
    ]
    .iter()
    .cloned()
    .collect();
    assert_eq!(result, Value::Map(expected));
    Ok(())
}

#[test]
fn test_transform_map_entry_duplicate_key_error() -> Result<(), Error> {
    let env = create_env()?;
    let program = env
        .compile("{'greeting': 'aloha', 'farewell': 'aloha'}.transformMapEntry(keyVar, valueVar, {valueVar: keyVar})")?;
    let result = program.evaluate(&Activation::new());
    assert!(result.is_err());
    Ok(())
}
