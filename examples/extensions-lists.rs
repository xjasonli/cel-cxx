//! List transformations, bounded pagination, and invalid indexes.
//! Run with `cargo run --example extensions-lists`.

use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("CEL lists extension");
    let env = Env::builder()
        .with_ext_lists(true)
        .with_ext_bindings(true)
        .declare_variable::<Vec<i64>>("items")?
        .declare_variable::<i64>("offset")?
        .declare_variable::<i64>("page_size")?
        .build()?;

    // slice uses a half-open interval: the end index is excluded.
    show(&env, "[1, 2, 3, 4].slice(1, 3)", vec![2i64, 3])?;
    show(&env, "[1, 2, 3].slice(3, 3)", Vec::<i64>::new())?;
    show(&env, "[].slice(0, 0)", Vec::<i64>::new())?;

    // flatten() removes one nesting level; use an explicit depth for deeper lists.
    show(
        &env,
        "[1, [2, [3, 4]]].flatten()",
        Value::List(vec![
            Value::Int(1),
            Value::Int(2),
            Value::from(vec![3i64, 4]),
        ]),
    )?;
    show(&env, "[1, [2, [3, 4]]].flatten(2)", vec![1i64, 2, 3, 4])?;
    show(
        &env,
        "[[1, 2], [3]].flatten(0)",
        vec![vec![1i64, 2], vec![3]],
    )?;
    show(
        &env,
        "[1, 2, [], [], [3, 4]].flatten()",
        vec![1i64, 2, 3, 4],
    )?;

    // distinct preserves the first occurrence; sort changes the order explicitly.
    show(&env, "[3, 1, 3, 2, 1].distinct()", vec![3i64, 1, 2])?;
    show(&env, "['b', 'b', 'a'].distinct()", vec!["b", "a"])?;
    show(
        &env,
        "[1, 'b', 2, 'b'].distinct()",
        Value::List(vec![Value::Int(1), Value::from("b"), Value::Int(2)]),
    )?;
    show(&env, "[].distinct()", Vec::<i64>::new())?;
    show(&env, "[5, 3, 1, 2].reverse()", vec![2i64, 1, 3, 5])?;
    show(&env, "[].reverse()", Vec::<i64>::new())?;
    show(&env, "[3, 1, 2].sort()", vec![1i64, 2, 3])?;
    show(&env, "[true, false, true].sort()", vec![false, true, true])?;
    show(
        &env,
        "['apple', 'pie', 'banana'].sortBy(s, s.size())",
        vec!["pie", "apple", "banana"],
    )?;
    show(
        &env,
        "[{'name':'foo','score':0}, {'name':'bar','score':-10}, {'name':'baz','score':1000}] \
         .sortBy(row, row.score).map(row, row.name)",
        vec!["bar", "foo", "baz"],
    )?;
    show(&env, "lists.range(5)", vec![0i64, 1, 2, 3, 4])?;
    show(&env, "lists.range(0)", Vec::<i64>::new())?;
    show(
        &env,
        "lists.range(6).map(n, n * n).filter(n, n > 10)",
        vec![16i64, 25],
    )?;

    // Clamp both bounds so a partial last page and an offset past the end are safe.
    // Clamp the size before adding it to the start to avoid integer overflow.
    let page = env.compile(
        "cel.bind(start, offset < 0 ? 0 : offset > items.size() ? items.size() : offset, \
         cel.bind(remaining, items.size() - start, \
         cel.bind(count, page_size < 0 ? 0 : page_size > remaining ? remaining : page_size, \
         items.slice(start, start + count))))",
    )?;
    for (items, offset, page_size, expected) in [
        (vec![0i64, 1, 2, 3, 4], 2i64, 2i64, vec![2i64, 3]),
        (vec![0, 1, 2, 3, 4], 4, 10, vec![4]),
        (vec![0, 1, 2], 10, 10, vec![]),
        (vec![], 0, 10, vec![]),
        (vec![0, 1], -1, 1, vec![0]),
        (vec![0, 1], 0, -1, vec![]),
        (vec![0, 1], 1, i64::MAX, vec![1]),
    ] {
        let activation = Activation::new()
            .bind_variable("items", items)?
            .bind_variable("offset", offset)?
            .bind_variable("page_size", page_size)?;
        let result = page.evaluate(&activation)?;
        assert_eq!(result, Value::from(expected));
        println!("Page at {offset}, size {page_size}: {result}");
    }

    for expression in [
        "[1, 2].slice(-1, 1)",
        "[1, 2].slice(0, 3)",
        "[1, 2].slice(2, 1)",
        "[].flatten(-1)",
        "lists.range(-1)",
    ] {
        let error = env
            .compile(expression)?
            .evaluate(())
            .expect_err("invalid list operation must fail");
        assert_eq!(error.code(), Code::InvalidArgument);
        println!("Rejected {expression}: {error}");
    }
    Ok(())
}

fn show(env: &Env, expression: &str, expected: impl Into<Value>) -> Result<(), Error> {
    let result = env.compile(expression)?.evaluate(Activation::new())?;
    assert_eq!(result, expected.into(), "{expression}");
    println!("{expression} => {result}");
    Ok(())
}
