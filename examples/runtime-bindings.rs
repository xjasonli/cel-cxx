//! Resolve typed overloads and supply implementations for each evaluation.
//!
//! Run with: `cargo run --example runtime-bindings`

use cel_cxx::*;

fn main() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Vec<i64>>("numbers")?
        .declare_variable::<Vec<String>>("labels")?
        .register_global_function("score", |values: Vec<i64>| values.len() as i64)?
        .register_global_function("score", |values: Vec<String>| values.len() as i64 * 10)?
        .declare_global_function::<fn() -> i64>("limit")?
        .build()?;

    // The declared list element types select the overload even for empty lists.
    let program = env.compile("[score(numbers), score(labels), limit()]")?;
    let baseline = Activation::new()
        .bind_variable("numbers", vec![1i64, 2, 3])?
        .bind_variable(
            "labels",
            vec![String::from("review"), String::from("publish")],
        )?
        .bind_global_function("limit", || 100i64)?;
    let result = program.evaluate(&baseline)?;
    assert_eq!(result, vec![3i64, 20, 100].into_value());
    println!("Default implementations: {result}");

    // Only score(list<int>) is replaced. The string overload retains its
    // registered implementation, and limit() receives an evaluation-specific value.
    let local_limit = 50i64;
    let customized = Activation::new()
        .bind_variable("numbers", Vec::<i64>::new())?
        .bind_variable("labels", vec![String::from("review")])?
        .bind_global_function("score", |values: Vec<i64>| 5 + values.len() as i64)?
        .bind_global_function("limit", move || local_limit)?;
    let result = program.evaluate(customized)?;
    assert_eq!(result, vec![5i64, 10, 50].into_value());
    println!("Customized implementations, including an empty list: {result}");

    let result = program.evaluate(&baseline)?;
    assert_eq!(result, vec![3i64, 20, 100].into_value());
    println!("The original activation still produces: {result}");

    // Bindings need a matching declared name, argument types, and call style.
    let invalid = Activation::new().bind_global_function("score", |_: bool| 0i64)?;
    let error = program
        .evaluate(invalid)
        .expect_err("score(bool) has no declaration");
    assert_eq!(error.code(), Code::InvalidArgument);
    println!("Rejected incompatible binding: {error}");

    let unbound = Activation::new()
        .bind_variable("numbers", Vec::<i64>::new())?
        .bind_variable("labels", Vec::<String>::new())?;
    let error = program
        .evaluate(unbound)
        .expect_err("limit() needs an implementation when called");
    println!("Declared function without an implementation: {error}");
    Ok(())
}
