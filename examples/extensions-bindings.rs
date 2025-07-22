use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("🔧 CEL Bindings Extensions Example");
    println!("==================================");
    
    // Build environment with bindings extensions enabled
    let env = Env::builder()
        .with_ext_bindings(true)
        .with_ext_strings(true)
        .declare_variable::<String>("name")?
        .declare_variable::<i64>("age")?
        .declare_variable::<f64>("price")?
        .declare_variable::<Vec<i64>>("numbers")?
        .declare_variable::<String>("text")?
        .build()?;

    // Demo 1: Basic Variable Binding
    println!("\n📌 Demo 1: Basic Variable Binding");
    demo_basic_binding(&env)?;
    
    // Demo 2: Nested Bindings
    println!("\n📌 Demo 2: Nested Bindings");
    demo_nested_bindings(&env)?;
    
    // Demo 3: Performance Optimization
    println!("\n📌 Demo 3: Performance Optimization");
    demo_performance_optimization(&env)?;
    
    // Demo 4: Complex Data Processing
    println!("\n📌 Demo 4: Complex Data Processing");
    demo_complex_processing(&env)?;
    
    // Demo 5: Conditional Logic with Bindings
    println!("\n📌 Demo 5: Conditional Logic with Bindings");
    demo_conditional_logic(&env)?;
    
    println!("\n✅ All CEL Bindings Extensions demos completed!");
    Ok(())
}

fn demo_basic_binding(env: &Env) -> Result<(), Error> {
    println!("  Testing basic cel.bind functionality");
    
    let activation = Activation::new()
        .bind_variable("name", "Alice".to_string())?
        .bind_variable("age", 30i64)?;
    
    let test_cases = vec![
        ("cel.bind(greeting, 'Hello, ' + name, greeting + '!')", "Simple string binding"),
        ("cel.bind(doubled_age, age * 2, 'Age doubled: ' + string(doubled_age))", "Arithmetic binding"),
        ("cel.bind(is_adult, age >= 18, is_adult ? 'Adult' : 'Minor')", "Boolean binding"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_nested_bindings(env: &Env) -> Result<(), Error> {
    println!("  Testing nested cel.bind functionality");
    
    let activation = Activation::new()
        .bind_variable("price", 29.99)?;
    
    // Nested bindings example
    let nested_expr = r#"
        cel.bind(base_price, price,
          cel.bind(tax_rate, 0.08,
            cel.bind(tax_amount, base_price * tax_rate,
              cel.bind(total, base_price + tax_amount,
                {
                  "base_price": base_price,
                  "tax_rate": tax_rate,
                  "tax_amount": tax_amount,
                  "total": total
                }
              )
            )
          )
        )
    "#;
    
    let program = env.compile(nested_expr.trim())?;
    let result = program.evaluate(&activation)?;
    println!("    Nested price calculation: {result}");
    
    Ok(())
}

fn demo_performance_optimization(env: &Env) -> Result<(), Error> {
    println!("  Testing performance optimization with bindings");
    
    let activation = Activation::new()
        .bind_variable("text", "Hello, World! This is a test string.".to_string())?;
    
    // Without bindings (repeated calculations)
    let without_bindings = r#"
        {
          "length": text.size(),
          "uppercase": text.upperAscii(),
          "contains_hello": text.contains('Hello'),
          "starts_with_hello": text.startsWith('Hello'),
          "summary": "Text '" + text + "' has " + string(text.size()) + " characters"
        }
    "#;
    
    // With bindings (optimized)
    let with_bindings = r#"
        cel.bind(input_text, text,
          cel.bind(text_length, input_text.size(),
            cel.bind(text_upper, input_text.upperAscii(),
              {
                "length": text_length,
                "uppercase": text_upper,
                "contains_hello": input_text.contains('Hello'),
                "starts_with_hello": input_text.startsWith('Hello'),
                "summary": "Text '" + input_text + "' has " + string(text_length) + " characters"
              }
            )
          )
        )
    "#;
    
    let program1 = env.compile(without_bindings.trim())?;
    let result1 = program1.evaluate(&activation)?;
    println!("    Without bindings: {result1}");
    
    let program2 = env.compile(with_bindings.trim())?;
    let result2 = program2.evaluate(&activation)?;
    println!("    With bindings: {result2}");
    
    Ok(())
}

fn demo_complex_processing(env: &Env) -> Result<(), Error> {
    println!("  Testing complex data processing with bindings");
    
    let activation = Activation::new()
        .bind_variable("numbers", vec![1i64, 2i64, 3i64, 4i64, 5i64])?;
    
    // Complex list processing
    let complex_expr = r#"
        cel.bind(input_list, numbers,
          cel.bind(filtered_list, input_list.filter(x, x > 2),
            cel.bind(mapped_list, filtered_list.map(x, x * x),
              cel.bind(list_sum, mapped_list[0] + mapped_list[1] + mapped_list[2],
                {
                  "original": input_list,
                  "filtered": filtered_list,
                  "squared": mapped_list,
                  "sum_of_squares": list_sum,
                  "average": double(list_sum) / double(mapped_list.size())
                }
              )
            )
          )
        )
    "#;
    
    let program = env.compile(complex_expr.trim())?;
    let result = program.evaluate(&activation)?;
    println!("    Complex list processing: {result}");
    
    Ok(())
}

fn demo_conditional_logic(env: &Env) -> Result<(), Error> {
    println!("  Testing conditional logic with bindings");
    
    let activation = Activation::new()
        .bind_variable("age", 25i64)?
        .bind_variable("name", "Bob".to_string())?;
    
    // Complex conditional logic
    let conditional_expr = r#"
        cel.bind(person_age, age,
          cel.bind(person_name, name,
            cel.bind(is_adult, person_age >= 18,
              cel.bind(is_senior, person_age >= 65,
                cel.bind(age_category, 
                  is_senior ? "senior" : 
                  is_adult ? "adult" : "minor",
                  {
                    "name": person_name,
                    "age": person_age,
                    "category": age_category,
                    "can_vote": is_adult,
                    "can_retire": is_senior,
                    "greeting": "Hello " + person_name + ", you are a " + age_category
                  }
                )
              )
            )
          )
        )
    "#;
    
    let program = env.compile(conditional_expr.trim())?;
    let result = program.evaluate(&activation)?;
    println!("    Conditional logic result: {result}");
    
    // User status determination
    println!("\n  User status determination example:");
    let status_expr = r#"
        cel.bind(user_age, age,
          cel.bind(user_name, name,
            cel.bind(name_length, user_name.size(),
              cel.bind(status_info, {
                "valid_name": name_length > 0 && name_length <= 50,
                "valid_age": user_age > 0 && user_age < 150,
                "age_group": user_age < 13 ? "child" : 
                           user_age < 20 ? "teen" : 
                           user_age < 60 ? "adult" : "senior"
              },
                {
                  "user": user_name,
                  "age": user_age,
                  "valid": status_info.valid_name && status_info.valid_age,
                  "group": status_info.age_group,
                  "message": status_info.valid_name && status_info.valid_age ? 
                           "Welcome " + user_name + " (" + status_info.age_group + ")" : 
                           "Invalid user data"
                }
              )
            )
          )
        )
    "#;
    
    let program = env.compile(status_expr.trim())?;
    let result = program.evaluate(&activation)?;
    println!("    User status: {result}");
    
    Ok(())
} 