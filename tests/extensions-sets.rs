use cel_cxx::*;

#[test]
fn test_set_contains() -> Result<(), Error> {
    println!("test sets.contains function");

    let env = Env::builder()
        .with_ext_sets(true)
        .declare_variable::<Vec<i64>>("set1")?
        .declare_variable::<Vec<i64>>("set2")?
        .build()?;

    // Test contains - true case (set1 contains all elements of set2)
    let program = env.compile("sets.contains(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", vec![1i64, 2i64, 3i64, 4i64])?
        .bind_variable("set2", vec![2i64, 3i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test contains - false case
    let program = env.compile("sets.contains(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", vec![1i64, 2i64])?
        .bind_variable("set2", vec![2i64, 3i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    // Test contains - empty set case (empty set is subset of any set)
    let program = env.compile("sets.contains(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", vec![1i64, 2i64, 3i64])?
        .bind_variable("set2", Vec::<i64>::new())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test contains - empty container case
    let program = env.compile("sets.contains(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", Vec::<i64>::new())?
        .bind_variable("set2", vec![1i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    Ok(())
}

#[test]
fn test_set_equivalent() -> Result<(), Error> {
    println!("test sets.equivalent function");

    let env = Env::builder()
        .with_ext_sets(true)
        .declare_variable::<Vec<i64>>("set1")?
        .declare_variable::<Vec<i64>>("set2")?
        .build()?;

    // Test equivalent - true case (same elements, different order)
    let program = env.compile("sets.equivalent(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", vec![1i64, 2i64, 3i64])?
        .bind_variable("set2", vec![3i64, 1i64, 2i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test equivalent - false case
    let program = env.compile("sets.equivalent(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", vec![1i64, 2i64])?
        .bind_variable("set2", vec![2i64, 3i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    // Test equivalent - with duplicates (duplicates should be ignored)
    let program = env.compile("sets.equivalent(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", vec![1i64, 2i64, 2i64, 3i64])?
        .bind_variable("set2", vec![1i64, 2i64, 3i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test equivalent - empty sets
    let program = env.compile("sets.equivalent(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", Vec::<i64>::new())?
        .bind_variable("set2", Vec::<i64>::new())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    Ok(())
}

#[test]
fn test_set_intersects() -> Result<(), Error> {
    println!("test sets.intersects function");

    let env = Env::builder()
        .with_ext_sets(true)
        .declare_variable::<Vec<i64>>("set1")?
        .declare_variable::<Vec<i64>>("set2")?
        .build()?;

    // Test intersects - true case (has common element)
    let program = env.compile("sets.intersects(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", vec![1i64, 2i64, 3i64])?
        .bind_variable("set2", vec![3i64, 4i64, 5i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test intersects - false case (no common elements)
    let program = env.compile("sets.intersects(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", vec![1i64, 2i64])?
        .bind_variable("set2", vec![3i64, 4i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    // Test intersects - empty set case (empty set intersects with nothing)
    let program = env.compile("sets.intersects(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", Vec::<i64>::new())?
        .bind_variable("set2", vec![1i64, 2i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    // Test intersects - both empty
    let program = env.compile("sets.intersects(set1, set2)")?;
    let activation = Activation::new()
        .bind_variable("set1", Vec::<i64>::new())?
        .bind_variable("set2", Vec::<i64>::new())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    Ok(())
}

#[test]
fn test_set_comprehensive_operations() -> Result<(), Error> {
    println!("test comprehensive set operations");

    let env = Env::builder()
        .with_ext_sets(true)
        .declare_variable::<Vec<String>>("user_permissions")?
        .declare_variable::<Vec<String>>("required_permissions")?
        .declare_variable::<Vec<String>>("admin_permissions")?
        .declare_variable::<Vec<String>>("group_a_permissions")?
        .declare_variable::<Vec<String>>("group_b_permissions")?
        .build()?;

    // Test permission checking - user has required permissions
    let program = env.compile("sets.contains(user_permissions, required_permissions)")?;
    let activation = Activation::new()
        .bind_variable("user_permissions", vec!["read".to_string(), "write".to_string(), "execute".to_string()])?
        .bind_variable("required_permissions", vec!["read".to_string(), "write".to_string()])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test group equivalence - two groups have equivalent permissions
    let program = env.compile("sets.equivalent(group_a_permissions, group_b_permissions)")?;
    let activation = Activation::new()
        .bind_variable("group_a_permissions", vec!["read".to_string(), "write".to_string(), "delete".to_string()])?
        .bind_variable("group_b_permissions", vec!["delete".to_string(), "read".to_string(), "write".to_string()])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    // Test admin check - user has any admin permissions
    let program = env.compile("sets.intersects(user_permissions, admin_permissions)")?;
    let activation = Activation::new()
        .bind_variable("user_permissions", vec!["read".to_string(), "write".to_string()])?
        .bind_variable("admin_permissions", vec!["delete".to_string(), "admin".to_string(), "sudo".to_string()])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(false));

    // Test admin check - user has admin permissions
    let program = env.compile("sets.intersects(user_permissions, admin_permissions)")?;
    let activation = Activation::new()
        .bind_variable("user_permissions", vec!["read".to_string(), "write".to_string(), "admin".to_string()])?
        .bind_variable("admin_permissions", vec!["delete".to_string(), "admin".to_string(), "sudo".to_string()])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    Ok(())
} 