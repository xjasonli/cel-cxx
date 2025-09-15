use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("🔧 CEL Comprehensions Extension Example");
    println!("======================================");

    let env = Env::builder()
        .with_ext_comprehensions(true)
        .build()?;

    println!("\n📌 Demo 1: `all` macro");
    demo_all_macro(&env)?;

    println!("\n📌 Demo 2: `exists` and `existsOne` macros");
    demo_exists_macros(&env)?;

    println!("\n📌 Demo 3: `transformList` macro");
    demo_transform_list(&env)?;

    println!("\n📌 Demo 4: `transformMap` and `transformMapEntry` macros");
    demo_transform_map(&env)?;

    println!("\n✅ All CEL Comprehensions Extension demos completed!");
    Ok(())
}

fn demo_all_macro(env: &Env) -> Result<(), Error> {
    let expressions = vec![
        ("[1, 2, 3].all(i, v, i < v)", "All elements `v` are greater than their index `i`"),
        ("[1, 2, 3].all(i, v, v % 2 != 0)", "All elements are odd"),
        ("{'a': 1, 'b': 2}.all(k, v, v > 0)", "All map values are positive"),
        ("{'a': 1, 'b': -2}.all(k, v, v > 0)", "All map values are positive (fail case)"),
    ];

    for (expr, desc) in expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("  - {}: `{}` -> {}", desc, expr, result);
    }
    Ok(())
}

fn demo_exists_macros(env: &Env) -> Result<(), Error> {
    let expressions = vec![
        ("[1, 10, 20].exists(i, v, v > 5)", "Exists an element greater than 5"),
        ("[1, 2, 3].exists(i, v, v > 5)", "Exists an element greater than 5 (fail case)"),
        ("{'a': 1, 'b': 10}.exists(k, v, v > 5)", "Exists a map value greater than 5"),
        ("[1, 5, 2, 5, 3].existsOne(i, v, v == 5)", "Exactly one element is 5 (fail case)"),
        ("[1, 2, 5, 3, 4].existsOne(i, v, v == 5)", "Exactly one element is 5"),
    ];

    for (expr, desc) in expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("  - {}: `{}` -> {}", desc, expr, result);
    }
    Ok(())
}

fn demo_transform_list(env: &Env) -> Result<(), Error> {
    let expressions = vec![
        ("[1, 2, 3, 4].transformList(i, v, v * v)", "Square all elements"),
        ("[1, 2, 3, 4].transformList(i, v, v % 2 == 0, v * v)", "Square even elements"),
        ("{'a': 1, 'b': 2}.transformList(k, v, k + string(v))", "Transform map to list of strings"),
    ];

    for (expr, desc) in expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("  - {}: `{}` -> {}", desc, expr, result);
    }
    Ok(())
}

fn demo_transform_map(env: &Env) -> Result<(), Error> {
    let expressions = vec![
        (
            "[1, 2, 3].transformMap(i, v, v * 10)", 
            "Transform list to map with values multiplied by 10"
        ),
        (
            "{'a': 1, 'b': 2}.transformMap(k, v, v + 1)", 
            "Increment map values"
        ),
        (
            "[1, 2, 3].transformMapEntry(i, v, {string(v): i})", 
            "Create a reverse lookup map from a list"
        ),
        (
            "{'a': 1, 'b': 2}.transformMapEntry(k, v, {k+k: v*v})", 
            "Transform map keys and values"
        ),
    ];

    for (expr, desc) in expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("  - {}: `{}` -> {}", desc, expr, result);
    }
    Ok(())
}
