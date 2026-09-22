//! Macro example demonstrating CEL compile-time expression expansion
//!
//! This example shows how to create and use CEL macros to extend the language
//! with custom syntax and optimizations. Macros transform expressions at compile
//! time, before type checking and evaluation.
//!
//! Run with `cargo run --example macros`.

use cel_cxx::macros::Macro;
use cel_cxx::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

fn main() -> Result<(), Error> {
    println!("CEL-CXX Macro Example\n");
    println!("======================\n");

    // Example 1: Constant folding optimization
    println!("Example 1: Constant Folding - twice(x)");
    println!("  Macro doubles integer constants at compile time\n");

    let twice = Macro::new_global("twice", 1, |factory, mut args| {
        let arg = args.pop()?;

        // Try constant folding
        if let Some(kind) = arg.kind() {
            if let Some(constant) = kind.as_constant() {
                if let Constant::Int(int_val) = constant {
                    if let Some(doubled) = int_val.checked_mul(2) {
                        println!("  [Compile time] Folding constant: {int_val} -> {doubled}");
                        return Some(factory.new_const(doubled));
                    }
                }
            }
        }

        // Variables and overflowing constants use CEL multiplication, which
        // reports overflow as an evaluation error instead of panicking in Rust.
        Some(factory.new_call("_*_", &[arg, factory.new_const(2i64)]))
    })?;

    let env = Env::builder()
        .declare_variable::<i64>("x")?
        .register_macro(twice)?
        .build()?;

    println!("  Compiling: twice(21)");
    let program = env.compile("twice(21)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, 42i64.into_value());
    println!("  Result: {}\n", result);

    let activation = Activation::new().bind_variable("x", 21i64)?;
    assert_eq!(
        env.compile("twice(x)")?.evaluate(&activation)?,
        42i64.into_value()
    );
    let overflow = env
        .compile("twice(9223372036854775807)")?
        .evaluate(())
        .expect_err("overflow must be reported by CEL");
    assert!(!overflow.message().is_empty());
    println!("  Overflow is an evaluation error: {overflow}");
    // Using a separate macro name leaves the standard double conversion intact.
    assert_eq!(env.compile("double(21)")?.evaluate(())?, 21f64.into_value());

    // Example 2: Creating lists from arguments
    println!("Example 2: Variable-Argument Macro - make_list(args...)");
    println!("  Creates a list from any number of arguments\n");

    let make_list = Macro::new_global_var_arg("make_list", |factory, args| {
        let elements: Vec<_> = args
            .iter()
            .map(|arg| factory.new_list_element(arg, false))
            .collect();
        Some(factory.new_list(&elements))
    })?;

    let env = Env::builder().register_macro(make_list)?.build()?;

    for (expr, expected) in [
        ("make_list()", vec![]),
        ("make_list(1)", vec![1i64]),
        ("make_list(1, 2, 3)", vec![1i64, 2, 3]),
    ] {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
        assert_eq!(result, expected.into_value());
        println!("  {} = {}", expr, result);
    }

    // Example 3: Wrapping values
    println!("\nExample 3: Wrapping Macro - wrap(x)");
    println!("  Wraps any value in a single-element list\n");

    let wrap = Macro::new_global("wrap", 1, |factory, mut args| {
        let arg = args.pop()?;
        let elem = factory.new_list_element(&arg, false);
        Some(factory.new_list(&[elem]))
    })?;

    let env = Env::builder().register_macro(wrap)?.build()?;

    let program = env.compile("wrap(42)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, vec![42i64].into_value());
    println!("  wrap(42) = {}\n", result);

    // Example 4: Duplicating values
    println!("Example 4: Duplication Macro - dup(x)");
    println!("  Duplicates the expression, evaluating it once per list element\n");

    let dup = Macro::new_global("dup", 1, |factory, mut args| {
        let arg = args.pop()?;
        let arg_copy = factory.copy_expr(&arg);
        let elem1 = factory.new_list_element(&arg, false);
        let elem2 = factory.new_list_element(&arg_copy, false);
        Some(factory.new_list(&[elem1, elem2]))
    })?;

    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let env = Env::builder()
        .register_global_function("next_value", move || {
            counter.fetch_add(1, Ordering::SeqCst) as i64 + 1
        })?
        .register_macro(dup)?
        .build()?;

    let program = env.compile("dup(100)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, vec![100i64, 100].into_value());
    println!("  dup(100) = {}\n", result);

    // Copying an AST node does not memoize its result. Avoid duplication macros
    // for expensive or stateful expressions unless this behavior is intended.
    let duplicated = env.compile("dup(next_value())")?.evaluate(())?;
    assert_eq!(duplicated, vec![1i64, 2].into_value());
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    println!("  dup(next_value()) = {duplicated}");

    // Example 5: Creating pairs
    println!("Example 5: Pair Creation - pair(a, b)");
    println!("  Creates a two-element list from two arguments\n");

    let pair = Macro::new_global("pair", 2, |factory, mut args| {
        if args.len() != 2 {
            return None;
        }
        let second = args.pop()?;
        let first = args.pop()?;
        let elem1 = factory.new_list_element(&first, false);
        let elem2 = factory.new_list_element(&second, false);
        Some(factory.new_list(&[elem1, elem2]))
    })?;

    let env = Env::builder().register_macro(pair)?.build()?;

    let program = env.compile("pair(10, 20)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, vec![10i64, 20].into_value());
    println!("  pair(10, 20) = {}\n", result);

    // Example 6: Receiver macro
    println!("Example 6: Receiver Macro - target.wrap()");
    println!("  Wraps the target value in a list\n");

    let wrap_receiver = Macro::new_receiver("wrap", 0, |factory, target, _args| {
        let elem = factory.new_list_element(&target, false);
        Some(factory.new_list(&[elem]))
    })?;

    let env = Env::builder()
        .declare_variable::<i64>("x")?
        .register_macro(wrap_receiver)?
        .build()?;

    let program = env.compile("x.wrap()")?;
    let activation = Activation::new().bind_variable("x", 77i64)?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, vec![77i64].into_value());
    println!("  x.wrap() where x=77: {}\n", result);

    // Example 7: Receiver macro with arguments
    println!("Example 7: Receiver Macro with Args - target.with(value)");
    println!("  Creates a pair [target, value]\n");

    let with_value = Macro::new_receiver("with", 1, |factory, target, mut args| {
        let arg = args.pop()?;
        let elem1 = factory.new_list_element(&target, false);
        let elem2 = factory.new_list_element(&arg, false);
        Some(factory.new_list(&[elem1, elem2]))
    })?;

    let env = Env::builder()
        .declare_variable::<i64>("x")?
        .register_macro(with_value)?
        .build()?;

    let program = env.compile("x.with(999)")?;
    let activation = Activation::new().bind_variable("x", 123i64)?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, vec![123i64, 999].into_value());
    println!("  x.with(999) where x=123: {}\n", result);

    // Example 8: Receiver macro with variable arguments
    println!("Example 8: Receiver Variable Args - target.prepend(args...)");
    println!("  Creates a list with target as first element\n");

    let prepend = Macro::new_receiver_var_arg("prepend", |factory, target, args| {
        let mut elements = vec![factory.new_list_element(&target, false)];
        for arg in args {
            elements.push(factory.new_list_element(&arg, false));
        }
        Some(factory.new_list(&elements))
    })?;

    let env = Env::builder()
        .declare_variable::<i64>("x")?
        .register_macro(prepend)?
        .build()?;

    let program = env.compile("x.prepend(2, 3, 4)")?;
    let activation = Activation::new().bind_variable("x", 1i64)?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, vec![1i64, 2, 3, 4].into_value());
    println!("  x.prepend(2, 3, 4) where x=1: {}\n", result);

    // Example 9: String manipulation
    println!("Example 9: String Macro - quote(string)");
    println!("  Wraps string constants in quotes\n");

    let quote = Macro::new_global("quote", 1, |factory, mut args| {
        let arg = args.pop()?;

        if let Some(kind) = arg.kind() {
            if let Some(constant) = kind.as_constant() {
                if let Constant::String(s) = constant {
                    let quoted = format!("'{}'", s.as_ref());
                    return Some(factory.new_const(quoted));
                }
            }
        }

        None
    })?;

    let env = Env::builder().register_macro(quote)?.build()?;

    let program = env.compile(r#"quote("hello world")"#)?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, "'hello world'".into_value());
    println!("  quote(\"hello world\") = {}\n", result);

    // Example 10: Nested macro calls
    println!("Example 10: Nested Macro Calls");
    println!("  Using macros in complex expressions\n");

    let triple = Macro::new_global("triple", 1, |factory, mut args| {
        let arg = args.pop()?;

        if let Some(kind) = arg.kind() {
            if let Some(constant) = kind.as_constant() {
                if let Constant::Int(int_val) = constant {
                    return int_val.checked_mul(3).map(|value| factory.new_const(value));
                }
            }
        }

        None
    })?;

    let env = Env::builder().register_macro(triple)?.build()?;

    let program = env.compile("[triple(1), triple(2), triple(3)]")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, vec![3i64, 6, 9].into_value());
    println!("  [triple(1), triple(2), triple(3)] = {}\n", result);
    assert_eq!(
        env.compile("triple(triple(2))")?.evaluate(())?,
        18i64.into_value()
    );
    // Returning None declines expansion. Without a matching function, checking fails.
    assert!(env.compile("triple('not an integer')").is_err());
    assert!(env.compile("triple(9223372036854775807)").is_err());

    // Example 11: Different arities
    println!("Example 11: Same Name, Different Arities");
    println!("  Same macro name with different argument counts\n");

    let test1 = Macro::new_global("test", 1, |factory, _args| {
        Some(factory.new_const("one argument"))
    })?;
    let test2 = Macro::new_global("test", 2, |factory, _args| {
        Some(factory.new_const("two arguments"))
    })?;

    let env = Env::builder()
        .register_macro(test1)?
        .register_macro(test2)?
        .build()?;

    let program1 = env.compile("test(0)")?;
    let result1 = program1.evaluate(&Activation::new())?;
    assert_eq!(result1, "one argument".into_value());
    println!("  test(0) = {}", result1);

    let program2 = env.compile("test(0, 0)")?;
    let result2 = program2.evaluate(&Activation::new())?;
    assert_eq!(result2, "two arguments".into_value());
    println!("  test(0, 0) = {}\n", result2);

    assert!(env.compile("test()").is_err());
    assert!(env.compile("test(1, 2, 3)").is_err());
    println!("All macro examples completed successfully!");

    Ok(())
}
