use cel_cxx::*;
use cel_cxx::macros::Macro;

#[test]
fn test_macro_duplicate_registration() -> Result<(), Error> {
    let macro1 = Macro::new_global("test", 1, |_factory, _args| None)?;
    let macro2 = Macro::new_global("test", 1, |_factory, _args| None)?;

    let result = Env::builder()
        .register_macro(macro1)?
        .register_macro(macro2);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already registered"));

    Ok(())
}

#[test]
fn test_macro_constant_folding() -> Result<(), Error> {
    // Macro that doubles integer constants at compile time
    let double = Macro::new_global("double", 1, |factory, mut args| {
        let arg = args.pop()?;
        
        // Try constant folding
        if let Some(kind) = arg.kind() {
            if let Some(constant) = kind.as_constant() {
                if let Constant::Int(int_val) = constant {
                    return Some(factory.new_const(int_val * 2));
                }
            }
        }
        
        // For non-constants, return None to keep original
        None
    })?;

    let env = Env::builder()
        .register_macro(double)?
        .build()?;

    // Test with constant - should be folded at compile time
    let program = env.compile("double(21)")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Int(42));

    Ok(())
}

#[test]
fn test_macro_wrap_in_list() -> Result<(), Error> {
    // Macro that wraps any expression in a single-element list
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
    
    if let Value::List(list) = result {
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], Value::Int(42));
    } else {
        panic!("Expected list");
    }

    Ok(())
}

#[test]
fn test_macro_duplicate_value() -> Result<(), Error> {
    // Macro that creates a list with the same value twice
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
    
    if let Value::List(list) = result {
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], Value::Int(100));
        assert_eq!(list[1], Value::Int(100));
    } else {
        panic!("Expected list");
    }

    Ok(())
}

#[test]
fn test_macro_create_pair() -> Result<(), Error> {
    // Macro that creates a two-element list from two arguments
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

    let program = env.compile("pair(1, 2)")?;
    let result = program.evaluate(&Activation::new())?;
    
    if let Value::List(list) = result {
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], Value::Int(1));
        assert_eq!(list[1], Value::Int(2));
    } else {
        panic!("Expected list");
    }

    Ok(())
}

#[test]
fn test_macro_variable_args_list() -> Result<(), Error> {
    // Macro that creates a list from any number of arguments
    let make_list = Macro::new_global_var_arg("make_list", |factory, args| {
        let elements: Vec<_> = args.iter()
            .map(|arg| factory.new_list_element(arg, false))
            .collect();
        Some(factory.new_list(&elements))
    })?;

    let env = Env::builder()
        .register_macro(make_list)?
        .build()?;

    // Test with no arguments
    let program = env.compile("make_list()")?;
    let result = program.evaluate(&Activation::new())?;
    if let Value::List(list) = result {
        assert_eq!(list.len(), 0);
    } else {
        panic!("Expected list");
    }

    // Test with multiple arguments
    let program = env.compile("make_list(1, 2, 3, 4, 5)")?;
    let result = program.evaluate(&Activation::new())?;
    if let Value::List(list) = result {
        assert_eq!(list.len(), 5);
        for i in 0..5 {
            assert_eq!(list[i], Value::Int((i + 1) as i64));
        }
    } else {
        panic!("Expected list");
    }

    Ok(())
}

#[test]
fn test_receiver_macro_wrap() -> Result<(), Error> {
    // Receiver macro that wraps the target in a list
    let wrap = Macro::new_receiver("wrap", 0, |factory, target, _args| {
        let elem = factory.new_list_element(&target, false);
        Some(factory.new_list(&[elem]))
    })?;

    let env = Env::builder()
        .declare_variable::<i64>("x")?
        .register_macro(wrap)?
        .build()?;

    let program = env.compile("x.wrap()")?;
    let activation = Activation::new().bind_variable("x", 99i64)?;
    let result = program.evaluate(&activation)?;
    
    if let Value::List(list) = result {
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], Value::Int(99));
    } else {
        panic!("Expected list");
    }

    Ok(())
}

#[test]
fn test_receiver_macro_with_arg() -> Result<(), Error> {
    // Receiver macro that creates a pair [target, arg]
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

    let program = env.compile("x.with(200)")?;
    let activation = Activation::new().bind_variable("x", 100i64)?;
    let result = program.evaluate(&activation)?;
    
    if let Value::List(list) = result {
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], Value::Int(100));
        assert_eq!(list[1], Value::Int(200));
    } else {
        panic!("Expected list");
    }

    Ok(())
}

#[test]
fn test_receiver_macro_variable_args() -> Result<(), Error> {
    // Receiver macro that creates a list with target as first element
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
    
    if let Value::List(list) = result {
        assert_eq!(list.len(), 4);
        assert_eq!(list[0], Value::Int(1));
        assert_eq!(list[1], Value::Int(2));
        assert_eq!(list[2], Value::Int(3));
        assert_eq!(list[3], Value::Int(4));
    } else {
        panic!("Expected list");
    }

    Ok(())
}

#[test]
fn test_macro_different_arities() -> Result<(), Error> {
    // Same name but different arities should be allowed
    let test1 = Macro::new_global("test", 1, |factory, _args| {
        Some(factory.new_const(1))
    })?;
    let test2 = Macro::new_global("test", 2, |factory, _args| {
        Some(factory.new_const(2))
    })?;

    let env = Env::builder()
        .register_macro(test1)?
        .register_macro(test2)?
        .build()?;

    let program1 = env.compile("test(0)")?;
    let result1 = program1.evaluate(&Activation::new())?;
    assert_eq!(result1, Value::Int(1));

    let program2 = env.compile("test(0, 0)")?;
    let result2 = program2.evaluate(&Activation::new())?;
    assert_eq!(result2, Value::Int(2));

    Ok(())
}

#[test]
fn test_macro_global_vs_receiver() -> Result<(), Error> {
    // Same name as global and receiver should be different macros
    let global_test = Macro::new_global("test", 0, |factory, _args| {
        Some(factory.new_const(100))
    })?;

    let receiver_test = Macro::new_receiver("test", 0, |factory, target, _args| {
        Some(factory.copy_expr(&target))
    })?;

    let env = Env::builder()
        .declare_variable::<i64>("x")?
        .register_macro(global_test)?
        .register_macro(receiver_test)?
        .build()?;

    // Global call
    let program = env.compile("test()")?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::Int(100));

    // Receiver call
    let program = env.compile("x.test()")?;
    let activation = Activation::new().bind_variable("x", 50i64)?;
    let result = program.evaluate(&activation)?;
    assert_eq!(result, Value::Int(50));

    Ok(())
}

#[test]
fn test_macro_nested_in_expression() -> Result<(), Error> {
    // Test that macro works when nested in other expressions
    let triple = Macro::new_global("triple", 1, |factory, mut args| {
        let arg = args.pop()?;
        
        // Try constant folding
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

    // Nested in list
    let program = env.compile("[triple(1), triple(2), triple(3)]")?;
    let result = program.evaluate(&Activation::new())?;
    
    if let Value::List(list) = result {
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], Value::Int(3));
        assert_eq!(list[1], Value::Int(6));
        assert_eq!(list[2], Value::Int(9));
    } else {
        panic!("Expected list");
    }

    Ok(())
}

#[test]
fn test_macro_with_string_constant() -> Result<(), Error> {
    // Macro that works with string constants
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

    let program = env.compile(r#"quote("hello")"#)?;
    let result = program.evaluate(&Activation::new())?;
    assert_eq!(result, Value::String("'hello'".to_string().into()));

    Ok(())
}
