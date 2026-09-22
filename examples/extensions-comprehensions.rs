//! Two-variable comprehensions expose list indexes or map keys alongside values.
//! Run with `cargo run --example extensions-comprehensions`.

use cel_cxx::*;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    println!("CEL comprehensions extension");
    let env = Env::builder().with_ext_comprehensions(true).build()?;

    for (expression, expected) in [
        ("[1, 2, 3].all(i, v, i < v)", true),
        ("[1, 2, 3].all(i, v, v % 2 != 0)", false),
        ("{'a': 1, 'b': -2}.all(k, v, v > 0)", false),
        ("[1, 10, 20].exists(i, v, i > 0 && v > 5)", true),
        ("{'a': 1, 'b': 10}.exists(k, v, k == 'b' && v > 5)", true),
        ("[1, 5, 2, 5].existsOne(i, v, v == 5)", false),
        ("[1, 2, 5, 3].existsOne(i, v, v == 5)", true),
        // Empty collections satisfy all, but neither exists nor existsOne.
        ("[].all(i, v, false)", true),
        ("[].exists(i, v, true)", false),
        ("{}.existsOne(k, v, true)", false),
    ] {
        show(&env, expression, expected)?;
    }

    show(
        &env,
        "[1, 2, 3, 4].transformList(i, v, v * v)",
        vec![1i64, 4, 9, 16],
    )?;
    show(
        &env,
        "[1, 2, 3, 4].transformList(i, v, v % 2 == 0, v * v)",
        vec![4i64, 16],
    )?;
    show(&env, "[].transformList(i, v, i)", Vec::<i64>::new())?;

    // Map iteration order is unspecified, so sort a map-derived list before comparing.
    let names = env
        .compile("{'a': 1, 'b': 2}.transformList(k, v, k + string(v))")?
        .evaluate(())?;
    let mut names = names
        .unwrap_list()
        .into_iter()
        .map(|value| value.unwrap_string().to_string())
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, ["a1", "b2"]);
    println!("Map entries as a sorted list: {names:?}");

    // Map equality compares entries without depending on their iteration order.
    show(
        &env,
        "[1, 2, 3].transformMap(i, v, v * 10)",
        HashMap::from([(0i64, 10i64), (1, 20), (2, 30)]),
    )?;
    show(
        &env,
        "{'a': 1, 'b': 2}.transformMap(k, v, v + 1)",
        HashMap::from([("a", 2i64), ("b", 3)]),
    )?;
    show(
        &env,
        "[1, 2, 3].transformMapEntry(i, v, {string(v): i})",
        HashMap::from([("1", 0i64), ("2", 1), ("3", 2)]),
    )?;
    show(
        &env,
        "{'a': 1, 'b': 2}.transformMapEntry(k, v, {k+k: v*v})",
        HashMap::from([("aa", 1i64), ("bb", 4)]),
    )?;

    // Build an index for active records while preserving their original positions.
    show(
        &env,
        r#"[{"id": "alpha", "active": true}, {"id": "beta", "active": false}]
             .transformMapEntry(i, row, row.active, {row.id: i})"#,
        HashMap::from([("alpha", 0i64)]),
    )?;
    Ok(())
}

fn show(env: &Env, expression: &str, expected: impl Into<Value>) -> Result<(), Error> {
    let result = env.compile(expression)?.evaluate(Activation::new())?;
    assert_eq!(result, expected.into(), "{expression}");
    println!("{expression} => {result}");
    Ok(())
}
