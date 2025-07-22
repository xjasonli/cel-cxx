use cel_cxx::*;

#[test]
fn test_list_slice() -> Result<(), Error> {
    println!("test list slice function");

    let env = Env::builder()
        .with_ext_lists(true)
        .declare_variable::<Vec<i64>>("items")?
        .declare_variable::<i64>("start")?
        .declare_variable::<i64>("end")?
        .build()?;

    // Test slice with start and end
    let program = env.compile("items.slice(start, end)")?;
    let activation = Activation::new()
        .bind_variable("items", vec![1i64, 2i64, 3i64, 4i64, 5i64])?
        .bind_variable("start", 1i64)?
        .bind_variable("end", 4i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![Value::Int(2), Value::Int(3), Value::Int(4)]));

    // Test slice with empty result (start == end)
    let program = env.compile("items.slice(start, end)")?;
    let activation = Activation::new()
        .bind_variable("items", vec![1i64, 2i64, 3i64, 4i64, 5i64])?
        .bind_variable("start", 2i64)?
        .bind_variable("end", 2i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![]));

    Ok(())
}

#[test]
fn test_list_flatten() -> Result<(), Error> {
    println!("test list flatten function");

    let env = Env::builder()
        .with_ext_lists(true)
        .declare_variable::<Vec<Vec<i64>>>("nested_items")?
        .build()?;

    // Test flatten with nested lists
    let program = env.compile("nested_items.flatten()")?;
    let activation = Activation::new()
        .bind_variable("nested_items", vec![vec![1i64, 2i64], vec![3i64, 4i64], vec![5i64]])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![
        Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4), Value::Int(5)
    ]));

    Ok(())
}

#[test]
fn test_list_unique() -> Result<(), Error> {
    println!("test list distinct function");

    let env = Env::builder()
        .with_ext_lists(true)
        .declare_variable::<Vec<i64>>("items")?
        .build()?;

    // Test distinct with duplicates
    let program = env.compile("items.distinct()")?;
    let activation = Activation::new()
        .bind_variable("items", vec![1i64, 2i64, 2i64, 3i64, 1i64, 4i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4)]));

    Ok(())
}

#[test]
fn test_list_reverse() -> Result<(), Error> {
    println!("test list reverse function");

    let env = Env::builder()
        .with_ext_lists(true)
        .declare_variable::<Vec<i64>>("items")?
        .build()?;

    // Test reverse
    let program = env.compile("items.reverse()")?;
    let activation = Activation::new()
        .bind_variable("items", vec![1i64, 2i64, 3i64, 4i64, 5i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![Value::Int(5), Value::Int(4), Value::Int(3), Value::Int(2), Value::Int(1)]));

    Ok(())
}

#[test]
fn test_list_sort() -> Result<(), Error> {
    println!("test list sort function");

    let env = Env::builder()
        .with_ext_lists(true)
        .declare_variable::<Vec<i64>>("items")?
        .build()?;

    // Test sort
    let program = env.compile("items.sort()")?;
    let activation = Activation::new()
        .bind_variable("items", vec![3i64, 1i64, 4i64, 1i64, 5i64, 9i64, 2i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![
        Value::Int(1), Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4), Value::Int(5), Value::Int(9)
    ]));

    Ok(())
}

#[test]
fn test_list_range() -> Result<(), Error> {
    println!("test list range function");

    let env = Env::builder()
        .with_ext_lists(true)
        .declare_variable::<i64>("n")?
        .build()?;

    // Test lists.range(n) - generates [0, 1, 2, ..., n-1]
    let program = env.compile("lists.range(n)")?;
    let activation = Activation::new()
        .bind_variable("n", 5i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![Value::Int(0), Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4)]));

    // Test lists.range(0) - empty list
    let program = env.compile("lists.range(n)")?;
    let activation = Activation::new()
        .bind_variable("n", 0i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![]));

    // Test lists.range(1) - single element [0]
    let program = env.compile("lists.range(n)")?;
    let activation = Activation::new()
        .bind_variable("n", 1i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![Value::Int(0)]));

    Ok(())
}

#[test]
fn test_list_comprehensive_operations() -> Result<(), Error> {
    println!("test comprehensive list operations");

    let env = Env::builder()
        .with_ext_lists(true)
        .declare_variable::<Vec<i64>>("numbers")?
        .build()?;

    // Test chained operations: distinct -> sort -> reverse -> slice
    let program = env.compile("numbers.distinct().sort().reverse().slice(0, 3)")?;
    let activation = Activation::new()
        .bind_variable("numbers", vec![3i64, 1i64, 4i64, 1i64, 5i64, 9i64, 2i64, 6i64])?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![Value::Int(9), Value::Int(6), Value::Int(5)]));

    // Test with range and processing: generate range, map to squares, filter > 10
    let program = env.compile("lists.range(6).map(x, x * x).filter(x, x > 10)")?;
    let activation = Activation::new();
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![Value::Int(16), Value::Int(25)]));

    Ok(())
} 