use cel_cxx::*;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    println!("🔧 CEL Optional Types Example (Proposal-246 Syntax)");
    println!("===================================================");

    // The `enable_optional_syntax` flag has been enabled in the FFI layer.
    // Now we can directly use the optional syntax in CEL expressions.
    let env = Env::builder()
        .declare_variable::<HashMap<String, String>>("person")?
        .declare_variable::<Vec<String>>("items")?
        .declare_variable::<HashMap<String, i64>>("scores")?
        .declare_variable::<Option<String>>("opt_name")?
        .declare_variable::<Option<i64>>("opt_age")?
        .declare_variable::<HashMap<String, HashMap<String, String>>>("data")?
        .build()?;

    // Demo 1: Optional Field Selection - msg.?field
    println!("\n📌 Demo 1: Optional Field Selection (msg.?field)");
    demo_optional_field_selection(&env)?;

    // Demo 2: Optional Map Access - map[?key]
    println!("\n📌 Demo 2: Optional Map Access (map[?key])");
    demo_optional_map_access(&env)?;

    // Demo 3: Optional List Access - list[?index]
    println!("\n📌 Demo 3: Optional List Access (list[?index])");
    demo_optional_list_access(&env)?;

    // Demo 4: Optional Message Construction - Msg{?field: <expr>}
    println!("\n📌 Demo 4: Optional Message Construction (Msg{{?field: <expr>}})");
    demo_optional_message_construction(&env)?;

    // Demo 5: Optional Map Construction - {?key: <expr>}
    println!("\n📌 Demo 5: Optional Map Construction ({{?key: <expr>}})");
    demo_optional_map_construction(&env)?;

    // Demo 6: Chained Optional Operations
    println!("\n📌 Demo 6: Chained Optional Operations");
    demo_chained_optional_operations(&env)?;

    println!("\n✅ All CEL proposal-246 optional syntax examples completed successfully!");
    Ok(())
}

fn demo_optional_field_selection(env: &Env) -> Result<(), Error> {
    println!("  Testing optional field selection: person.?name");

    let mut person = HashMap::new();
    person.insert("name".to_string(), "Alice".to_string());

    let activation = Activation::new().bind_variable("person", person)?;

    let test_cases = vec![
        ("person.?name", "Existing field: 'name'"),
        ("person.?age", "Missing field: 'age'"),
        (
            "person.?name.orValue('default')",
            "Existing field with orValue",
        ),
        (
            "person.?age.orValue('default_age')",
            "Missing field with orValue",
        ),
        ("person.?name.hasValue()", "hasValue on existing field"),
        ("person.?age.hasValue()", "hasValue on missing field"),
    ];

    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }

    Ok(())
}

fn demo_optional_map_access(env: &Env) -> Result<(), Error> {
    println!("  Testing optional map access: scores[?key]");

    let mut scores = HashMap::new();
    scores.insert("alice".to_string(), 100);

    let activation = Activation::new().bind_variable("scores", scores)?;

    let test_cases = vec![
        ("scores[?'alice']", "Existing key: 'alice'"),
        ("scores[?'bob']", "Missing key: 'bob'"),
        ("scores[?'alice'].orValue(0)", "Existing key with orValue"),
        ("scores[?'bob'].orValue(50)", "Missing key with orValue"),
        ("scores[?'alice'].hasValue()", "hasValue on existing key"),
        ("scores[?'bob'].hasValue()", "hasValue on missing key"),
    ];

    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }

    Ok(())
}

fn demo_optional_list_access(env: &Env) -> Result<(), Error> {
    println!("  Testing optional list access: items[?index]");

    let items = vec!["apple".to_string(), "banana".to_string()];

    let activation = Activation::new().bind_variable("items", items)?;

    let test_cases = vec![
        ("items[?0]", "Valid index: 0"),
        ("items[?1]", "Valid index: 1"),
        ("items[?2]", "Out of bounds index: 2"),
        ("items[?-1]", "Negative index: -1"),
        ("items[?0].orValue('default')", "Valid index with orValue"),
        ("items[?2].orValue('default')", "Out of bounds with orValue"),
        ("items[?1].hasValue()", "hasValue on valid index"),
        ("items[?2].hasValue()", "hasValue on out of bounds index"),
    ];

    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }

    Ok(())
}

fn demo_optional_message_construction(env: &Env) -> Result<(), Error> {
    println!("  Testing optional message construction: Person{{?name: opt_name}} (simulated)");

    let activation = Activation::new()
        .bind_variable("opt_name", Some("Bob".to_string()))?
        .bind_variable("opt_age", None::<i64>)?;

    // NOTE: Direct message construction with optional fields is not easily demonstrable
    // without a concrete message type. We simulate the logic with maps.
    let test_cases = vec![
        (
            "{?'name': opt_name, ?'age': opt_age}",
            "Construct with one present, one absent optional",
        ),
        (
            "{?'name': optional.of('Chris')}",
            "Construct with a present optional",
        ),
        (
            "{?'age': optional.none()}",
            "Construct with an absent optional",
        ),
    ];

    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }

    Ok(())
}

fn demo_optional_map_construction(env: &Env) -> Result<(), Error> {
    println!("  Testing optional map construction: {{?key: <expr>}}");

    // Test case 1: optional value is present
    let activation1 = Activation::new().bind_variable("opt_name", Some("David".to_string()))?;

    let prog1 = env.compile("{?'name': opt_name, 'status': 'active'}")?;
    let result1 = prog1.evaluate(&activation1)?;
    println!("    With present optional: `{{?'name': opt_name, 'status': 'active'}}` -> {result1}");

    // Test case 2: optional value is absent
    let activation2 = Activation::new().bind_variable("opt_name", None::<String>)?;

    // We can reuse prog1
    let result2 = prog1.evaluate(&activation2)?;
    println!("    With absent optional: `{{?'name': opt_name, 'status': 'active'}}` -> {result2}");

    // Test case 3: Mixed optional values
    let activation3 = Activation::new(); // No variables needed
    let prog3 = env.compile("{?'a': optional.of(1), ?'b': optional.none()}")?;
    let result3 = prog3.evaluate(&activation3)?;
    println!(
        "    With mixed literals: `{{?'a': optional.of(1), ?'b': optional.none()}}` -> {result3}"
    );

    Ok(())
}

fn demo_chained_optional_operations(env: &Env) -> Result<(), Error> {
    println!("  Testing chained optional operations");

    let mut data = HashMap::new();
    let inner_map = HashMap::from([("name".to_string(), "Eve".to_string())]);
    data.insert("maybe_map".to_string(), inner_map);

    let activation = Activation::new().bind_variable("data", data)?;

    // This simulates something like `data.?maybe_map[?'name']`
    let test_cases = vec![
        (
            "data.?maybe_map.orValue({})[?'name']",
            "Chain of optional selection and map access",
        ),
        (
            "data.?non_existent_map.orValue({})[?'name']",
            "Chain with missing first part",
        ),
    ];

    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }

    Ok(())
}
