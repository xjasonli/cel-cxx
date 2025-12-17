//! Macro example demonstrating CEL compile-time expression expansion
//!
//! This example shows how to create and use CEL macros to extend the language
//! with custom syntax and optimizations. Macros transform expressions at compile
//! time, before type checking and evaluation.

use cel_cxx::*;
use cel_cxx::macros::Macro;

fn main() -> Result<(), Error> {
    println!("CEL-CXX Macro Example\n");
    println!("======================\n");

    // Example 1: Constant folding optimization
    println!("Example 1: Constant Folding - double(x)");
    println!("  Macro doubles integer constants at compile time\n");
    
    let double = Macro::new_global("double", 1, |factory, mut args| {
        let arg = args.pop()?;
        
        // Try constant folding
        if let Some(kind) = arg.kind() {
            if let Some(constant) = kind.as_constant() {
                if let Constant::Int(int_val) = constant {
                    println!("  [Compile time] Folding constant: {} -> {}", int_val, int_val * 2);
                    return Some(factory.new_const(int_val * 2));
                }
            }
        }
        
        println!("  [Compile time] Not a constant, keeping original expression");
        None
    })?;

    let env = Env::builder()
        .register_macro(double)?
        .build()?;

    println!("  Compiling: double(21)");
    let program = env.compile("double(21)")?;
    let result = program.evaluate(&Activation::new())?;
    println!("  Result: {}\n", result);

    // Example 2: Creating lists from arguments
    println!("Example 2: Variable-Argument Macro - make_list(args...)");
    println!("  Creates a list from any number of arguments\n");

    let make_list = Macro::new_global_var_arg("make_list", |factory, args| {
        let elements: Vec<_> = args.iter()
            .map(|arg| factory.new_list_element(arg, false))
            .collect();
        Some(factory.new_list(&elements))
    })?;

    let env = Env::builder()
        .register_macro(make_list)?
        .build()?;

    for expr in ["make_list()", "make_list(1)", "make_list(1, 2, 3)"] {
        let program = env.compile(expr)?;
        let result = program.evaluate(&Activation::new())?;
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

    let env = Env::builder()
        .register_macro(wrap)?
        .build()?;

    let program = env.compile("wrap(42)")?;
    let result = program.evaluate(&Activation::new())?;
    println!("  wrap(42) = {}\n", result);

    // Example 4: Duplicating values
    println!("Example 4: Duplication Macro - dup(x)");
    println!("  Creates a two-element list with the same value\n");

    let dup = Macro::new_global("dup", 1, |factory, mut args| {
        let arg = args.pop()?;
        let arg_copy = factory.copy_expr(&arg);
        let elem1 = factory.new_list_element(&arg, false);
        let elem2 = factory.new_list_element(&arg_copy, false);
        Some(factory.new_list(&[elem1, elem2]))
    })?;

    let env = Env::builder()
        .register_macro(dup)?
        .build()?;

    let program = env.compile("dup(100)")?;
    let result = program.evaluate(&Activation::new())?;
    println!("  dup(100) = {}\n", result);

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

    let env = Env::builder()
        .register_macro(pair)?
        .build()?;

    let program = env.compile("pair(10, 20)")?;
    let result = program.evaluate(&Activation::new())?;
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

    let env = Env::builder()
        .register_macro(quote)?
        .build()?;

    let program = env.compile(r#"quote("hello world")"#)?;
    let result = program.evaluate(&Activation::new())?;
    println!("  quote(\"hello world\") = {}\n", result);

    // Example 10: Nested macro calls
    println!("Example 10: Nested Macros");
    println!("  Using macros in complex expressions\n");

    let triple = Macro::new_global("triple", 1, |factory, mut args| {
        let arg = args.pop()?;
        
        if let Some(kind) = arg.kind() {
            if let Some(constant) = kind.as_constant() {
                if let Constant::Int(int_val) = constant {
                    return Some(factory.new_const(int_val * 3));
                }
            }
        }
        
        None
    })?;

    let env = Env::builder()
        .register_macro(triple)?
        .build()?;

    let program = env.compile("[triple(1), triple(2), triple(3)]")?;
    let result = program.evaluate(&Activation::new())?;
    println!("  [triple(1), triple(2), triple(3)] = {}\n", result);

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
    println!("  test(0) = {}", result1);

    let program2 = env.compile("test(0, 0)")?;
    let result2 = program2.evaluate(&Activation::new())?;
    println!("  test(0, 0) = {}\n", result2);

    println!("All macro examples completed successfully!");
    
    Ok(())
}
