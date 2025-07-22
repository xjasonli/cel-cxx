use cel_cxx::*;

#[test]
fn test_char_at() -> Result<(), Error> {
    println!("test charAt function");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("str")?
        .declare_variable::<i64>("index")?
        .build()?;

    // Test charAt with valid index
    let program = env.compile("str.charAt(index)")?;
    let activation = Activation::new()
        .bind_variable("str", "Hello".to_string())?
        .bind_variable("index", 0i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("H".into()));

    // Test charAt with last character
    let program = env.compile("str.charAt(index)")?;
    let activation = Activation::new()
        .bind_variable("str", "World".to_string())?
        .bind_variable("index", 4i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("d".into()));

    // Test charAt with middle character
    let program = env.compile("str.charAt(index)")?;
    let activation = Activation::new()
        .bind_variable("str", "Programming".to_string())?
        .bind_variable("index", 7i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("m".into()));



    Ok(())
}

#[test]
fn test_index_of() -> Result<(), Error> {
    println!("test indexOf function");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("str")?
        .declare_variable::<String>("substr")?
        .declare_variable::<i64>("start")?
        .build()?;

    // Test indexOf with single character
    let program = env.compile("str.indexOf(substr)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("substr", "o".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(4));

    // Test indexOf with substring
    let program = env.compile("str.indexOf(substr)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("substr", "world".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(6));

    // Test indexOf with start index
    let program = env.compile("str.indexOf(substr, start)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("substr", "o".to_string())?
        .bind_variable("start", 5i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(7));

    // Test indexOf with not found substring
    let program = env.compile("str.indexOf(substr)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("substr", "xyz".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(-1));

    Ok(())
}

#[test]
fn test_last_index_of() -> Result<(), Error> {
    println!("test lastIndexOf function");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("str")?
        .declare_variable::<String>("substr")?
        .declare_variable::<i64>("start")?
        .build()?;

    // Test lastIndexOf with single character
    let program = env.compile("str.lastIndexOf(substr)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("substr", "o".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(7));

    // Test lastIndexOf with substring
    let program = env.compile("str.lastIndexOf(substr)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello hello".to_string())?
        .bind_variable("substr", "hello".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(6));

    // Test lastIndexOf with start index
    let program = env.compile("str.lastIndexOf(substr, start)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("substr", "o".to_string())?
        .bind_variable("start", 5i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(4));

    // Test lastIndexOf with not found substring
    let program = env.compile("str.lastIndexOf(substr)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("substr", "xyz".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Int(-1));

    Ok(())
}

#[test]
fn test_substring() -> Result<(), Error> {
    println!("test substring function");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("str")?
        .declare_variable::<i64>("start")?
        .declare_variable::<i64>("end")?
        .build()?;

    // Test substring with start and end
    let program = env.compile("str.substring(start, end)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("start", 0i64)?
        .bind_variable("end", 5i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("hello".into()));

    // Test substring with only start
    let program = env.compile("str.substring(start)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("start", 6i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("world".into()));

    // Test substring with middle portion
    let program = env.compile("str.substring(start, end)")?;
    let activation = Activation::new()
        .bind_variable("str", "programming".to_string())?
        .bind_variable("start", 3i64)?
        .bind_variable("end", 7i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("gram".into()));



    Ok(())
}

#[test]
fn test_strings_quote() -> Result<(), Error> {
    println!("test strings.quote function");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("str")?
        .build()?;

    // Test strings.quote with simple string
    let program = env.compile("strings.quote(str)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("\"hello\"".into()));

    // Test strings.quote with string containing quotes
    let program = env.compile("strings.quote(str)")?;
    let activation = Activation::new()
        .bind_variable("str", "say \"hello\"".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("\"say \\\"hello\\\"\"".into()));

    // Test strings.quote with empty string
    let program = env.compile("strings.quote(str)")?;
    let activation = Activation::new()
        .bind_variable("str", "".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("\"\"".into()));

    Ok(())
}

#[test]
fn test_trim() -> Result<(), Error> {
    println!("test trim function");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("str")?
        .build()?;

    // Test trim with leading and trailing spaces
    let program = env.compile("str.trim()")?;
    let activation = Activation::new()
        .bind_variable("str", "  hello world  ".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("hello world".into()));

    // Test trim with only leading spaces
    let program = env.compile("str.trim()")?;
    let activation = Activation::new()
        .bind_variable("str", "  hello".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("hello".into()));

    // Test trim with only trailing spaces
    let program = env.compile("str.trim()")?;
    let activation = Activation::new()
        .bind_variable("str", "hello  ".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("hello".into()));

    // Test trim with no spaces
    let program = env.compile("str.trim()")?;
    let activation = Activation::new()
        .bind_variable("str", "hello".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("hello".into()));

    // Test trim with tabs and newlines
    let program = env.compile("str.trim()")?;
    let activation = Activation::new()
        .bind_variable("str", "\t\nhello\n\t".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("hello".into()));

    Ok(())
}

#[test]
fn test_join() -> Result<(), Error> {
    println!("test join function");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<Vec<String>>("items")?
        .declare_variable::<String>("separator")?
        .build()?;

    // Test join with comma separator
    let program = env.compile("items.join(separator)")?;
    let activation = Activation::new()
        .bind_variable("items", vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()])?
        .bind_variable("separator", ", ".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("apple, banana, cherry".into()));

    // Test join with empty separator
    let program = env.compile("items.join(separator)")?;
    let activation = Activation::new()
        .bind_variable("items", vec!["a".to_string(), "b".to_string(), "c".to_string()])?
        .bind_variable("separator", "".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("abc".into()));

    // Test join with single element
    let program = env.compile("items.join(separator)")?;
    let activation = Activation::new()
        .bind_variable("items", vec!["single".to_string()])?
        .bind_variable("separator", ", ".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("single".into()));

    // Test join with empty list
    let program = env.compile("items.join(separator)")?;
    let activation = Activation::new()
        .bind_variable("items", Vec::<String>::new())?
        .bind_variable("separator", ", ".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("".into()));

    Ok(())
}

#[test]
fn test_split() -> Result<(), Error> {
    println!("test split function");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("str")?
        .declare_variable::<String>("separator")?
        .declare_variable::<i64>("limit")?
        .build()?;

    // Test split with comma separator
    let program = env.compile("str.split(separator)")?;
    let activation = Activation::new()
        .bind_variable("str", "apple,banana,cherry".to_string())?
        .bind_variable("separator", ",".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![
        Value::String("apple".into()),
        Value::String("banana".into()),
        Value::String("cherry".into())
    ]));

    // Test split with space separator
    let program = env.compile("str.split(separator)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world test".to_string())?
        .bind_variable("separator", " ".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![
        Value::String("hello".into()),
        Value::String("world".into()),
        Value::String("test".into())
    ]));

    // Test split with limit
    let program = env.compile("str.split(separator, limit)")?;
    let activation = Activation::new()
        .bind_variable("str", "a,b,c,d".to_string())?
        .bind_variable("separator", ",".to_string())?
        .bind_variable("limit", 2i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![
        Value::String("a".into()),
        Value::String("b,c,d".into())
    ]));

    // Test split with non-existent separator
    let program = env.compile("str.split(separator)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello".to_string())?
        .bind_variable("separator", ",".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![Value::String("hello".into())]));

    Ok(())
}

#[test]
fn test_case_conversion() -> Result<(), Error> {
    println!("test case conversion functions");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("str")?
        .build()?;

    // Test lowerAscii
    let program = env.compile("str.lowerAscii()")?;
    let activation = Activation::new()
        .bind_variable("str", "Hello World".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("hello world".into()));

    // Test upperAscii
    let program = env.compile("str.upperAscii()")?;
    let activation = Activation::new()
        .bind_variable("str", "Hello World".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("HELLO WORLD".into()));

    // Test lowerAscii with mixed case
    let program = env.compile("str.lowerAscii()")?;
    let activation = Activation::new()
        .bind_variable("str", "CamelCase123".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("camelcase123".into()));

    // Test upperAscii with mixed case
    let program = env.compile("str.upperAscii()")?;
    let activation = Activation::new()
        .bind_variable("str", "CamelCase123".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("CAMELCASE123".into()));

    Ok(())
}

#[test]
fn test_replace() -> Result<(), Error> {
    println!("test replace function");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("str")?
        .declare_variable::<String>("old")?
        .declare_variable::<String>("new")?
        .declare_variable::<i64>("count")?
        .build()?;

    // Test replace all occurrences
    let program = env.compile("str.replace(old, new)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world hello".to_string())?
        .bind_variable("old", "hello".to_string())?
        .bind_variable("new", "hi".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("hi world hi".into()));

    // Test replace with count limit
    let program = env.compile("str.replace(old, new, count)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world hello".to_string())?
        .bind_variable("old", "hello".to_string())?
        .bind_variable("new", "hi".to_string())?
        .bind_variable("count", 1i64)?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("hi world hello".into()));

    // Test replace with single character
    let program = env.compile("str.replace(old, new)")?;
    let activation = Activation::new()
        .bind_variable("str", "abcabc".to_string())?
        .bind_variable("old", "a".to_string())?
        .bind_variable("new", "x".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("xbcxbc".into()));

    // Test replace with empty string
    let program = env.compile("str.replace(old, new)")?;
    let activation = Activation::new()
        .bind_variable("str", "hello world".to_string())?
        .bind_variable("old", " ".to_string())?
        .bind_variable("new", "".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("helloworld".into()));

    Ok(())
}

//#[test]
//fn test_format() -> Result<(), Error> {
//    println!("test format function");
//
//    let env = Env::builder()
//        .with_ext_strings(true)
//        .declare_variable::<String>("template")?
//        .declare_variable::<Vec<String>>("args")?
//        .build()?;
//
//    // Test format with string arguments
//    let program = env.compile("template.format(args)")?;
//    let activation = Activation::new()
//        .bind_variable("template", "Hello %s, you are %d years old".to_string())?
//        .bind_variable("args", vec![Value::String("Alice".into()), Value::Int(30)])?;
//    let res = program.evaluate(&activation)?;
//    assert_eq!(res, Value::String("Hello Alice, you are 30 years old".into()));
//
//    // Test format with mixed arguments
//    let program = env.compile("template.format(args)")?;
//    let activation = Activation::new()
//        .bind_variable("template", "Value: %v, Number: %d, Float: %.2f".to_string())?
//        .bind_variable("args", vec![Value::String("test".into()), Value::Int(42), Value::Double(3.14159)])?;
//    let res = program.evaluate(&activation)?;
//    assert_eq!(res, Value::String("Value: test, Number: 42, Float: 3.14".into()));
//
//    // Test format with no arguments
//    let program = env.compile("template.format(args)")?;
//    let activation = Activation::new()
//        .bind_variable("template", "Hello World".to_string())?
//        .bind_variable("args", Vec::<Value>::new())?;
//    let res = program.evaluate(&activation)?;
//    assert_eq!(res, Value::String("Hello World".into()));
//
//    Ok(())
//}

#[test]
fn test_reverse() -> Result<(), Error> {
    println!("test reverse function");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("str")?
        .build()?;

    // Test reverse with simple string
    let program = env.compile("str.reverse()")?;
    let activation = Activation::new()
        .bind_variable("str", "hello".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("olleh".into()));

    // Test reverse with palindrome
    let program = env.compile("str.reverse()")?;
    let activation = Activation::new()
        .bind_variable("str", "racecar".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("racecar".into()));

    // Test reverse with empty string
    let program = env.compile("str.reverse()")?;
    let activation = Activation::new()
        .bind_variable("str", "".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("".into()));

    // Test reverse with single character
    let program = env.compile("str.reverse()")?;
    let activation = Activation::new()
        .bind_variable("str", "a".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("a".into()));

    Ok(())
}

#[test]
fn test_comprehensive_string_operations() -> Result<(), Error> {
    println!("test comprehensive string operations");

    let env = Env::builder()
        .with_ext_strings(true)
        .declare_variable::<String>("email")?
        .declare_variable::<String>("csv")?
        .declare_variable::<String>("text")?
        .build()?;

    // Email processing example
    let program = env.compile("email.split('@')[0].lowerAscii()")?;
    let activation = Activation::new()
        .bind_variable("email", "John.Doe@EXAMPLE.COM".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("john.doe".into()));

    // CSV processing example
    let program = env.compile("csv.split(',').map(x, x.trim())")?;
    let activation = Activation::new()
        .bind_variable("csv", "apple, banana , cherry ".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::List(vec![
        Value::String("apple".into()),
        Value::String("banana".into()),
        Value::String("cherry".into())
    ]));

    // Text normalization example
    let program = env.compile("text.trim().lowerAscii().replace(' ', '_')")?;
    let activation = Activation::new()
        .bind_variable("text", "  Hello World  ".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::String("hello_world".into()));

    // String validation example
    let program = env.compile("text.indexOf('@') > 0 && text.indexOf('.') > text.indexOf('@')")?;
    let activation = Activation::new()
        .bind_variable("text", "user@example.com".to_string())?;
    let res = program.evaluate(&activation)?;
    assert_eq!(res, Value::Bool(true));

    Ok(())
} 
