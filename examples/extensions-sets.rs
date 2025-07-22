use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("🔧 CEL Sets Extensions Example");
    println!("==============================");
    
    // Build environment with sets extensions enabled
    let env = Env::builder()
        .with_ext_sets(true)
        .with_ext_bindings(true)
        .declare_variable::<Vec<String>>("user_permissions")?
        .declare_variable::<Vec<String>>("required_permissions")?
        .declare_variable::<Vec<String>>("admin_permissions")?
        .declare_variable::<Vec<String>>("group_a_permissions")?
        .declare_variable::<Vec<String>>("group_b_permissions")?
        .declare_variable::<Vec<String>>("allowed_categories")?
        .declare_variable::<Vec<String>>("submitted_categories")?
        .declare_variable::<Vec<i64>>("set_a")?
        .declare_variable::<Vec<i64>>("set_b")?
        .declare_variable::<Vec<String>>("user_roles")?
        .declare_variable::<Vec<String>>("editor_roles")?
        .build()?;

    // Demo 1: sets.contains function
    println!("\n📌 Demo 1: sets.contains Function");
    demo_sets_contains(&env)?;
    
    // Demo 2: sets.equivalent function
    println!("\n📌 Demo 2: sets.equivalent Function");
    demo_sets_equivalent(&env)?;
    
    // Demo 3: sets.intersects function
    println!("\n📌 Demo 3: sets.intersects Function");
    demo_sets_intersects(&env)?;
    
    // Demo 4: Permission checking scenarios
    println!("\n📌 Demo 4: Permission Checking Scenarios");
    demo_permission_checking(&env)?;
    
    // Demo 5: Data validation scenarios
    println!("\n📌 Demo 5: Data Validation Scenarios");
    demo_data_validation(&env)?;
    
    // Demo 6: Role-based access control
    println!("\n📌 Demo 6: Role-Based Access Control");
    demo_rbac(&env)?;
    
    println!("\n✅ All CEL Sets Extensions demos completed!");
    Ok(())
}

fn demo_sets_contains(env: &Env) -> Result<(), Error> {
    println!("  Testing sets.contains function");
    
    // Basic containment check
    let activation = Activation::new()
        .bind_variable("set_a", vec![1, 2, 3, 4, 5])?
        .bind_variable("set_b", vec![2, 3])?;
    
    let test_cases = vec![
        ("sets.contains(set_a, set_b)", "Check if [1,2,3,4,5] contains all of [2,3]"),
        ("sets.contains([1, 2, 3], [1, 3])", "Check if [1,2,3] contains all of [1,3]"),
        ("sets.contains([1, 2, 3], [1, 4])", "Check if [1,2,3] contains all of [1,4]"),
        ("sets.contains(['a', 'b', 'c'], ['b'])", "Check string containment"),
        ("sets.contains([1, 2, 3], [])", "Empty set is subset of any set"),
        ("sets.contains([], [1])", "Empty set cannot contain non-empty set"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_sets_equivalent(env: &Env) -> Result<(), Error> {
    println!("  Testing sets.equivalent function");
    
    let activation = Activation::new()
        .bind_variable("set_a", vec![1, 2, 3])?
        .bind_variable("set_b", vec![3, 2, 1])?;
    
    let test_cases = vec![
        ("sets.equivalent(set_a, set_b)", "Check if [1,2,3] is equivalent to [3,2,1]"),
        ("sets.equivalent([1, 2, 2, 3], [1, 2, 3])", "Duplicates are ignored"),
        ("sets.equivalent(['x', 'y'], ['y', 'x'])", "String set equivalence"),
        ("sets.equivalent([1, 2, 3], [1, 2, 4])", "Different sets are not equivalent"),
        ("sets.equivalent([], [])", "Empty sets are equivalent"),
        ("sets.equivalent([1], [1, 1, 1])", "Single element vs duplicates"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_sets_intersects(env: &Env) -> Result<(), Error> {
    println!("  Testing sets.intersects function");
    
    let activation = Activation::new()
        .bind_variable("set_a", vec![1, 2, 3])?
        .bind_variable("set_b", vec![3, 4, 5])?;
    
    let test_cases = vec![
        ("sets.intersects(set_a, set_b)", "Check if [1,2,3] intersects with [3,4,5]"),
        ("sets.intersects([1, 2, 3], [4, 5, 6])", "No intersection"),
        ("sets.intersects(['a', 'b'], ['b', 'c'])", "String intersection"),
        ("sets.intersects([1, 2], [])", "Empty set intersects with nothing"),
        ("sets.intersects([], [1, 2])", "Empty set intersects with nothing"),
        ("sets.intersects([[1], [2]], [[2], [3]])", "Nested list intersection"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    Ok(())
}

fn demo_permission_checking(env: &Env) -> Result<(), Error> {
    println!("  Testing permission checking scenarios");
    
    // Scenario 1: User permission validation
    let activation = Activation::new()
        .bind_variable("user_permissions", vec!["read".to_string(), "write".to_string(), "execute".to_string()])?
        .bind_variable("required_permissions", vec!["read".to_string(), "write".to_string()])?
        .bind_variable("admin_permissions", vec!["delete".to_string(), "admin".to_string(), "sudo".to_string()])?;
    
    let test_cases = vec![
        ("sets.contains(user_permissions, required_permissions)", "User has all required permissions"),
        ("sets.intersects(user_permissions, admin_permissions)", "User has any admin permissions"),
        ("sets.contains(['read', 'write', 'delete', 'admin'], ['read', 'write'])", "Admin user has basic permissions"),
        ("sets.equivalent(['read', 'write'], ['write', 'read'])", "Permission sets are equivalent regardless of order"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    // Scenario 2: Advanced permission checking with CEL bind
    println!("  Advanced permission checking:");
    
    let complex_expr = r#"
        cel.bind(user_perms, ['read', 'write', 'execute'],
          cel.bind(required_perms, ['read', 'write'],
            cel.bind(admin_perms, ['delete', 'admin', 'sudo'],
              {
                'has_required': sets.contains(user_perms, required_perms),
                'has_admin': sets.intersects(user_perms, admin_perms),
                'is_power_user': sets.contains(user_perms, ['read', 'write', 'execute'])
              }
            )
          )
        )
    "#;
    
    let program = env.compile(complex_expr)?;
    let result = program.evaluate(&Activation::new())?;
    println!("    Complex permission analysis: {result}");
    
    Ok(())
}

fn demo_data_validation(env: &Env) -> Result<(), Error> {
    println!("  Testing data validation scenarios");
    
    let activation = Activation::new()
        .bind_variable("allowed_categories", vec!["tech".to_string(), "health".to_string(), "finance".to_string(), "education".to_string()])?
        .bind_variable("submitted_categories", vec!["tech".to_string(), "health".to_string()])?;
    
    let test_cases = vec![
        ("sets.contains(allowed_categories, submitted_categories)", "All submitted categories are allowed"),
        ("sets.contains(['red', 'green', 'blue'], ['red', 'yellow'])", "Invalid color submitted"),
        ("sets.equivalent(['draft', 'published'], ['published', 'draft'])", "Status sets are equivalent"),
        ("sets.intersects(['urgent', 'normal'], ['low', 'medium'])", "Priority levels don't overlap"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    // Complex validation scenario
    println!("  Complex validation scenario:");
    
    let validation_expr = r#"
        cel.bind(allowed_tags, ['frontend', 'backend', 'database', 'api', 'mobile'],
          cel.bind(submitted_tags, ['frontend', 'api', 'mobile'],
            cel.bind(required_tags, ['frontend'],
              {
                'all_tags_valid': sets.contains(allowed_tags, submitted_tags),
                'has_required_tags': sets.contains(submitted_tags, required_tags),
                'has_backend_tags': sets.intersects(submitted_tags, ['backend', 'database']),
                'validation_passed': sets.contains(allowed_tags, submitted_tags) && sets.contains(submitted_tags, required_tags)
              }
            )
          )
        )
    "#;
    
    let program = env.compile(validation_expr)?;
    let result = program.evaluate(&Activation::new())?;
    println!("    Tag validation result: {result}");
    
    Ok(())
}

fn demo_rbac(env: &Env) -> Result<(), Error> {
    println!("  Testing Role-Based Access Control scenarios");
    
    let activation = Activation::new()
        .bind_variable("user_roles", vec!["editor".to_string(), "reviewer".to_string()])?
        .bind_variable("editor_roles", vec!["editor".to_string(), "author".to_string()])?;
    
    let test_cases = vec![
        ("sets.intersects(user_roles, editor_roles)", "User has editor-level access"),
        ("sets.contains(['admin', 'moderator', 'editor'], ['editor'])", "Role hierarchy check"),
        ("sets.equivalent(['user', 'member'], ['member', 'user'])", "Role sets are equivalent"),
        ("sets.intersects(['guest'], ['admin', 'moderator'])", "Guest has no elevated privileges"),
    ];
    
    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("    {description}: `{expr}` -> {result}");
    }
    
    // Complex RBAC scenario
    println!("  Complex RBAC scenario:");
    
    let rbac_expr = r#"
        cel.bind(user_roles, ['editor', 'reviewer'],
          cel.bind(admin_roles, ['admin', 'super-admin'],
            cel.bind(editor_roles, ['editor', 'author'],
              cel.bind(required_roles, ['editor'],
                {
                  'is_admin': sets.intersects(user_roles, admin_roles),
                  'can_edit': sets.contains(user_roles, required_roles),
                  'has_editor_access': sets.intersects(user_roles, editor_roles),
                  'access_level': sets.intersects(user_roles, admin_roles) ? 'admin' :
                                 sets.intersects(user_roles, editor_roles) ? 'editor' : 'user'
                }
              )
            )
          )
        )
    "#;
    
    let program = env.compile(rbac_expr)?;
    let result = program.evaluate(&Activation::new())?;
    println!("    RBAC analysis result: {result}");
    
    Ok(())
} 