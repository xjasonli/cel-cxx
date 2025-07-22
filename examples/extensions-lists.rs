use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("🔧 CEL Lists Extensions Example");
    println!("==============================");
    
    // Build environment with lists extensions enabled
    let env = Env::builder()
        .with_ext_lists(true)
        .with_ext_bindings(true)
        .build()?;

    // Demo 1: List Slicing
    println!("\n📌 Demo 1: List Slicing");
    demo_slice(&env)?;
    
    // Demo 2: List Flattening
    println!("\n📌 Demo 2: List Flattening");
    demo_flatten(&env)?;
    
    // Demo 3: List Deduplication
    println!("\n📌 Demo 3: List Deduplication");
    demo_distinct(&env)?;
    
    // Demo 4: List Reversal
    println!("\n📌 Demo 4: List Reversal");
    demo_reverse(&env)?;
    
    // Demo 5: List Sorting
    println!("\n📌 Demo 5: List Sorting");
    demo_sort(&env)?;
    
    // Demo 6: Number Ranges
    println!("\n📌 Demo 6: Number Ranges");
    demo_range(&env)?;
    
    // Demo 7: Complex Data Transformation
    println!("\n📌 Demo 7: Complex Data Transformation");
    demo_complex_transformation(&env)?;
    
    println!("\n✅ All CEL Lists Extensions demos completed!");
    Ok(())
}

fn demo_slice(env: &Env) -> Result<(), Error> {
    println!("  Testing list.slice function");
    
    let test_cases = vec![
        ("[1, 2, 3, 4].slice(1, 3)", "Slice from index 1 to 3"),
        ("['a', 'b', 'c', 'd'].slice(0, 2)", "Slice from the beginning"),
        ("[1, 2, 3].slice(1, 1)", "Slice with start and end at the same index"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_flatten(env: &Env) -> Result<(), Error> {
    println!("  Testing list.flatten function");
    
    let test_cases = vec![
        ("[1, [2, 3], [4]].flatten()", "Simple flatten"),
        ("[1, [2, [3, 4]]].flatten()", "Flatten one level"),
        ("[1, 2, [], [], [3, 4]].flatten()", "Flatten with empty lists"),
        ("[1, [2, [3, [4]]]].flatten(2)", "Flatten with depth limit"),
        ("[1, [2, [3, [4]]]].flatten()", "Complete flatten"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_distinct(env: &Env) -> Result<(), Error> {
    println!("  Testing list.distinct function");
    
    let test_cases = vec![
        ("[1, 2, 2, 3, 3, 3].distinct()", "Deduplicate integers"),
        ("['b', 'b', 'c', 'a', 'c'].distinct()", "Deduplicate strings, preserving order"),
        ("[1, 'b', 2, 'b'].distinct()", "Deduplicate mixed types"),
        ("[true, false, true].distinct()", "Deduplicate booleans"),
        ("[].distinct()", "Deduplicate empty list"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_reverse(env: &Env) -> Result<(), Error> {
    println!("  Testing list.reverse function");
    
    let test_cases = vec![
        ("[5, 3, 1, 2].reverse()", "Reverse a list of integers"),
        ("['a', 'b', 'c'].reverse()", "Reverse a list of strings"),
        ("[].reverse()", "Reverse an empty list"),
        ("[[1, 2], [3, 4]].reverse()", "Reverse a list of lists"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_sort(env: &Env) -> Result<(), Error> {
    println!("  Testing list.sort and list.sortBy functions");
    
    let test_cases = vec![
        ("[3, 2, 1].sort()", "Sort a list of integers"),
        ("['b', 'c', 'a'].sort()", "Sort a list of strings"),
        ("[true, false, true].sort()", "Sort a list of booleans"),
        ("['apple', 'pie', 'banana'].sortBy(s, s.size())", "Sort by string length"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    // Sort by object property
    println!("  Sorting list of objects by property:");
    let sort_by_expr = r#"
        [
          {"name": "foo", "score": 0},
          {"name": "bar", "score": -10},
          {"name": "baz", "score": 1000}
        ].sortBy(e, e.score).map(e, e.name)
    "#;
    let program = env.compile(sort_by_expr)?;
    let result = program.evaluate(&Activation::new())?;
    println!("    Sort by score: `{}` -> {}", sort_by_expr.trim(), result);
    
    Ok(())
}

fn demo_range(env: &Env) -> Result<(), Error> {
    println!("  Testing lists.range function");
    
    let test_cases = vec![
        ("lists.range(5)", "Generate a range of 5 numbers"),
        ("lists.range(0)", "Generate an empty range"),
        ("lists.range(1)", "Generate a range with a single number"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_complex_transformation(env: &Env) -> Result<(), Error> {
    println!("  Testing complex data transformation");
    
    // Scenario 1: Pagination
    println!("  Pagination example:");
    let pagination_expr = r#"
        cel.bind(items, lists.range(101),
          cel.bind(page, 2,
            cel.bind(page_size, 10,
              cel.bind(start, (page - 1) * page_size,
                items.slice(start, start + page_size)
              )
            )
          )
        )
    "#;
    let program = env.compile(pagination_expr)?;
    let result = program.evaluate(&Activation::new())?;
    println!("    Pagination (page 2, size 10): {result}");
    
    // Scenario 2: Data cleaning and processing
    println!("\n  Data cleaning and processing example:");
    let cleaning_expr = r#"
        cel.bind(user_ids, [1, 3, 2, 1, 4, 2, 5],
          user_ids.distinct().sort()
        )
    "#;
    let program = env.compile(cleaning_expr)?;
    let result = program.evaluate(&Activation::new())?;
    println!("    Deduplicated and sorted user IDs: {result}");
    
    // Scenario 3: Player data analysis
    println!("\n  Player data analysis example:");
    let player_analysis_expr = r#"
        cel.bind(players, [
          {"name": "Alice", "scores": [85, 92, 78]},
          {"name": "Bob", "scores": [91, 87, 95]},
          {"name": "Charlie", "scores": [76, 88, 82]}
        ],
          players.map(player, {
            "name": player.name,
            "avg_score": double(player.scores[0] + player.scores[1] + player.scores[2]) / 3.0,
            "best_score": player.scores.sort().reverse()[0]
          }).sortBy(p, p.avg_score).reverse()
        )
    "#;
    let program = env.compile(player_analysis_expr)?;
    let result = program.evaluate(&Activation::new())?;
    println!("    Player data sorted by average score (desc): {result}");
    
    Ok(())
} 