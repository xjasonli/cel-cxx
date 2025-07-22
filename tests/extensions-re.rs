use cel_cxx::*;
use std::collections::HashMap;

#[test]
fn test_re_extract() -> Result<(), Error> {
    println!("test re.extract function");

    let env = Env::builder()
        .with_ext_re(true)
        .declare_variable::<String>("text")?
        .declare_variable::<String>("pattern")?
        .declare_variable::<String>("rewrite")?
        .build()?;

    // Test extracting and rewriting "Hello World" to "World, Hello"
    let program = env.compile("re.extract(text, pattern, rewrite)")?;
    let activation = Activation::new()
        .bind_variable("text", "Hello World".to_string())?
        .bind_variable("pattern", r"(\w+) (\w+)".to_string())?
        .bind_variable("rewrite", r"\2, \1".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("World, Hello".into()));

    // Test date format conversion
    let program = env.compile("re.extract(text, pattern, rewrite)")?;
    let activation = Activation::new()
        .bind_variable("text", "2023-12-25".to_string())?
        .bind_variable("pattern", r"(\d{4})-(\d{2})-(\d{2})".to_string())?
        .bind_variable("rewrite", r"\2/\3/\1".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("12/25/2023".into()));

    // Test log entry reformatting
    let program = env.compile("re.extract(text, pattern, rewrite)")?;
    let activation = Activation::new()
        .bind_variable("text", "[INFO] User login successful".to_string())?
        .bind_variable("pattern", r"\[(\w+)\] (.+)".to_string())?
        .bind_variable("rewrite", r"\1: \2".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("INFO: User login successful".into()));

    Ok(())
}

#[test]
fn test_re_capture() -> Result<(), Error> {
    println!("test re.capture function");

    let env = Env::builder()
        .with_ext_re(true)
        .declare_variable::<String>("text")?
        .declare_variable::<String>("pattern")?
        .build()?;

    // Test capturing username from email (must match full string)
    let program = env.compile("re.capture(text, pattern)")?;
    let activation = Activation::new()
        .bind_variable("text", "user@example.com".to_string())?
        .bind_variable("pattern", r"(\w+)@[\w.]+".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("user".into()));

    // Test capturing price from text (must match full string)
    let program = env.compile("re.capture(text, pattern)")?;
    let activation = Activation::new()
        .bind_variable("text", "Price: $29.99".to_string())?
        .bind_variable("pattern", r"Price: \$(\d+\.\d+)".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("29.99".into()));

    // Test capturing first name (must match full string)
    let program = env.compile("re.capture(text, pattern)")?;
    let activation = Activation::new()
        .bind_variable("text", "John Doe".to_string())?
        .bind_variable("pattern", r"(\w+) \w+".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("John".into()));

    Ok(())
}

#[test]
fn test_re_capture_n() -> Result<(), Error> {
    println!("test re.captureN function");

    let env = Env::builder()
        .with_ext_re(true)
        .declare_variable::<String>("text")?
        .declare_variable::<String>("pattern")?
        .build()?;

    // Test capturing all groups from email (unnamed groups, must match full string)
    let program = env.compile("re.captureN(text, pattern)")?;
    let activation = Activation::new()
        .bind_variable("text", "user@example.com".to_string())?
        .bind_variable("pattern", r"(\w+)@([\w.]+)".to_string())?;
    let res = program.evaluate(&activation)?;
    // Should return map with 1-based indexing for unnamed groups
    let expected_map = HashMap::from([
        (MapKey::String("1".into()), Value::String("user".into())),
        (MapKey::String("2".into()), Value::String("example.com".into())),
    ]);
    assert_eq!(res, Value::Map(expected_map));

    // Test capturing named groups
    let program = env.compile("re.captureN(text, pattern)")?;
    let activation = Activation::new()
        .bind_variable("text", "John Doe".to_string())?
        .bind_variable("pattern", r"(?P<first>\w+) (?P<last>\w+)".to_string())?;
    let res = program.evaluate(&activation)?;
    // Should return map with named groups
    let expected_map = HashMap::from([
        (MapKey::String("first".into()), Value::String("John".into())),
        (MapKey::String("last".into()), Value::String("Doe".into())),
    ]);
    assert_eq!(res, Value::Map(expected_map));

    // Test capturing date components
    let program = env.compile("re.captureN(text, pattern)")?;
    let activation = Activation::new()
        .bind_variable("text", "2023-12-25".to_string())?
        .bind_variable("pattern", r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})".to_string())?;
    let res = program.evaluate(&activation)?;
    let expected_map = HashMap::from([
        (MapKey::String("year".into()), Value::String("2023".into())),
        (MapKey::String("month".into()), Value::String("12".into())),
        (MapKey::String("day".into()), Value::String("25".into())),
    ]);
    assert_eq!(res, Value::Map(expected_map));

    Ok(())
}

#[test]
fn test_re_comprehensive_operations() -> Result<(), Error> {
    println!("test comprehensive regex operations");

    let env = Env::builder()
        .with_ext_re(true)
        .declare_variable::<String>("text")?
        .declare_variable::<String>("pattern")?
        .declare_variable::<String>("rewrite")?
        .build()?;

    // Test extract with word reversal
    let program = env.compile("re.extract(text, pattern, rewrite)")?;
    let activation = Activation::new()
        .bind_variable("text", "Hello World".to_string())?
        .bind_variable("pattern", r"(\w+) (\w+)".to_string())?
        .bind_variable("rewrite", r"\2, \1".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("World, Hello".into()));

    // Test captureN with log parsing (we know this works)
    let program = env.compile("re.captureN(text, pattern)")?;
    let activation = Activation::new()
        .bind_variable("text", "[ERROR] Database connection failed".to_string())?
        .bind_variable("pattern", r"\[(.+)\] (.+)".to_string())?;
    let res = program.evaluate(&activation)?;
    let expected_map = HashMap::from([
        (MapKey::String("1".into()), Value::String("ERROR".into())),
        (MapKey::String("2".into()), Value::String("Database connection failed".into())),
    ]);
    assert_eq!(res, Value::Map(expected_map));

    // Test captureN with email domain extraction (must match full string)
    let program = env.compile("re.captureN(text, pattern)")?;
    let activation = Activation::new()
        .bind_variable("text", "user@mail.example.com".to_string())?
        .bind_variable("pattern", r"[^@]+@([\w]+)\.([\w.]+)".to_string())?;
    let res = program.evaluate(&activation)?;
    let expected_map = HashMap::from([
        (MapKey::String("1".into()), Value::String("mail".into())),
        (MapKey::String("2".into()), Value::String("example.com".into())),
    ]);
    assert_eq!(res, Value::Map(expected_map));

    Ok(())
}

 