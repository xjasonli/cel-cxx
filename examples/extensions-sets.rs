//! Set operations on CEL lists, including permission policies and empty inputs.
//! Run with `cargo run --example extensions-sets`.

use cel_cxx::*;

fn main() -> Result<(), Error> {
    println!("CEL sets extension");
    let env = Env::builder()
        .with_ext_sets(true)
        .with_ext_bindings(true)
        .declare_variable::<Vec<String>>("user_permissions")?
        .declare_variable::<Vec<String>>("required_permissions")?
        .declare_variable::<Vec<String>>("submitted_tags")?
        .build()?;

    // contains asks whether every element of the second list appears in the first.
    // These functions treat lists as sets: duplicate counts and ordering do not matter.
    for (expression, expected) in [
        ("sets.contains([1, 2, 3], [1, 3])", true),
        ("sets.contains([1, 2, 3], [1, 4])", false),
        ("sets.contains(['a', 'b', 'c'], ['b'])", true),
        ("sets.contains([1], [1, 1, 1])", true),
        ("sets.contains([1, 2, 3], [])", true),
        ("sets.contains([], [1])", false),
        ("sets.contains([], [])", true),
        ("sets.equivalent([1, 2, 2, 3], [3, 2, 1])", true),
        ("sets.equivalent(['x', 'y'], ['y', 'x'])", true),
        ("sets.equivalent([1, 2, 3], [1, 2, 4])", false),
        ("sets.equivalent([], [])", true),
        ("sets.intersects([1, 2, 3], [3, 4, 5])", true),
        ("sets.intersects([1, 2, 3], [4, 5, 6])", false),
        ("sets.intersects(['a', 'b'], ['b', 'c'])", true),
        ("sets.intersects([1, 2], [])", false),
        ("sets.intersects([], [1, 2])", false),
        // Nested lists are elements; their own ordering still matters for equality.
        ("sets.intersects([[1, 2]], [[1, 2]])", true),
        ("sets.intersects([[1, 2]], [[2, 1]])", false),
    ] {
        let result = env.compile(expression)?.evaluate(())?;
        assert_eq!(result, Value::Bool(expected));
        println!("{expression} => {result}");
    }

    // An empty required list passes contains. Require it to be nonempty when an
    // operation must specify at least one permission; do not assume contains does so.
    let authorize = env.compile(
        "required_permissions.size() > 0 && \
         sets.contains(user_permissions, required_permissions)",
    )?;
    for (user, required, expected) in [
        (vec!["read", "write"], vec!["read"], true),
        (vec!["read"], vec!["write"], false),
        (vec!["read"], vec!["read", "read"], true),
        (vec![], vec!["read"], false),
        (vec!["read"], vec![], false),
    ] {
        let activation = Activation::new()
            .bind_variable("user_permissions", user)?
            .bind_variable("required_permissions", required)?;
        let result = authorize.evaluate(&activation)?;
        assert_eq!(result, Value::Bool(expected));
        println!("Authorized: {result}");
    }

    // Validate both an allowlist and a required tag without relying on input order.
    let validate_tags = env.compile(
        "cel.bind(allowed, ['frontend', 'backend', 'api'], \
         sets.contains(allowed, submitted_tags) && \
         sets.contains(submitted_tags, ['api']))",
    )?;
    for (tags, expected) in [
        (vec!["api", "frontend"], true),
        (vec!["api", "api"], true),
        (vec!["frontend"], false),
        (vec!["api", "unknown"], false),
        (vec![], false),
    ] {
        let activation = Activation::new().bind_variable("submitted_tags", tags)?;
        assert_eq!(validate_tags.evaluate(&activation)?, Value::Bool(expected));
    }

    // Permission groups express application policy; sets itself has no role hierarchy.
    let access = env.compile(
        "sets.intersects(user_permissions, ['admin', 'super-admin']) ? 'admin' : \
         sets.intersects(user_permissions, ['editor', 'author']) ? 'editor' : 'reader'",
    )?;
    for (roles, expected) in [
        (vec!["reviewer", "editor"], "editor"),
        (vec!["editor", "admin"], "admin"),
        (vec!["guest"], "reader"),
        (vec![], "reader"),
    ] {
        let activation = Activation::new().bind_variable("user_permissions", roles)?;
        assert_eq!(access.evaluate(&activation)?, Value::from(expected));
    }
    Ok(())
}
