//! CEL standard operators, collection macros, conversions, and error boundaries.
//!
//! Run with `cargo run --example standard-library`.

use cel_cxx::*;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    println!("CEL Standard Library Example\n");

    let env = Env::builder()
        .declare_variable::<HashMap<String, String>>("data")?
        .declare_variable::<Vec<i64>>("numbers")?
        .declare_variable::<String>("text")?
        .declare_variable::<Vec<u8>>("bytes_data")?
        .build()?;
    let activation = Activation::new()
        .bind_variable("data", HashMap::from([("name", "Alice"), ("age", "30")]))?
        .bind_variable("numbers", vec![1i64, 2, 3, 4, 5])?
        .bind_variable("text", "Hello, World!")?
        .bind_variable("bytes_data", vec![0u8, 127, 255])?;

    println!("Presence and comprehension macros:");
    show(
        &env,
        &activation,
        &[
            ("has(data.name)", true.into_value()),
            ("has(data.missing)", false.into_value()),
            ("numbers.all(x, x > 0)", true.into_value()),
            ("numbers.all(x, x > 3)", false.into_value()),
            ("numbers.exists(x, x == 3)", true.into_value()),
            ("numbers.exists(x, x > 10)", false.into_value()),
            ("numbers.exists_one(x, x == 3)", true.into_value()),
            ("numbers.exists_one(x, x > 3)", false.into_value()),
            ("numbers.filter(x, x > 2)", vec![3i64, 4, 5].into_value()),
            (
                "numbers.map(x, x * 2)",
                vec![2i64, 4, 6, 8, 10].into_value(),
            ),
            (
                "numbers.map(x, x % 2 == 0, x * 10)",
                vec![20i64, 40].into_value(),
            ),
        ],
    )?;

    println!("\nLogical operators and conditionals:");
    show(
        &env,
        &activation,
        &[
            ("true && false", false.into_value()),
            ("true || false", true.into_value()),
            ("!true", false.into_value()),
            ("true ? 'yes' : 'no'", "yes".into_value()),
            ("false ? 'yes' : 'no'", "no".into_value()),
            ("true && (false || true)", true.into_value()),
            // An unselected conditional branch is not evaluated.
            ("true ? 42 : 1 / 0", 42i64.into_value()),
        ],
    )?;

    println!("\nArithmetic and comparisons:");
    show(
        &env,
        &activation,
        &[
            ("10 + 5", 15i64.into_value()),
            ("10 - 3", 7i64.into_value()),
            ("6 * 7", 42i64.into_value()),
            ("15 / 3", 5i64.into_value()),
            ("17 % 5", 2i64.into_value()),
            ("-42", (-42i64).into_value()),
            ("2 + 3 * 4", 14i64.into_value()),
            ("(2 + 3) * 4", 20i64.into_value()),
            ("-7 / 3", (-2i64).into_value()),
            ("5 == 5", true.into_value()),
            ("5 == 3", false.into_value()),
            ("5 != 3", true.into_value()),
            ("5 < 10", true.into_value()),
            ("5 <= 5", true.into_value()),
            ("10 > 5", true.into_value()),
            ("3 >= 5", false.into_value()),
            ("'apple' < 'banana'", true.into_value()),
        ],
    )?;

    println!("\nLists and maps:");
    show(
        &env,
        &activation,
        &[
            ("numbers[0]", 1i64.into_value()),
            ("numbers[2]", 3i64.into_value()),
            ("numbers[numbers.size() - 1]", 5i64.into_value()),
            ("3 in numbers", true.into_value()),
            ("6 in numbers", false.into_value()),
            ("numbers.size()", 5i64.into_value()),
            ("size(numbers)", 5i64.into_value()),
            ("data['name']", "Alice".into_value()),
            ("data.name", "Alice".into_value()),
            // This map stores strings, including its age field.
            ("int(data.age) >= 18", true.into_value()),
            ("'name' in data", true.into_value()),
            ("'country' in data", false.into_value()),
            ("data.size()", 2i64.into_value()),
            ("size(data)", 2i64.into_value()),
        ],
    )?;

    println!("\nStrings, bytes, and conversions:");
    show(
        &env,
        &activation,
        &[
            ("text.contains('World')", true.into_value()),
            ("text.contains('xyz')", false.into_value()),
            ("text.startsWith('Hello')", true.into_value()),
            ("text.startsWith('Hi')", false.into_value()),
            ("text.endsWith('World!')", true.into_value()),
            ("text.endsWith('Universe')", false.into_value()),
            ("text.matches('Hello.*')", true.into_value()),
            ("text.size()", 13i64.into_value()),
            ("size(text)", 13i64.into_value()),
            ("int(42.7)", 42i64.into_value()),
            ("uint(42)", 42u64.into_value()),
            ("double(42)", 42f64.into_value()),
            ("string(42)", "42".into_value()),
            ("bool('true')", true.into_value()),
            // dyn() erases the static type; the value passes through unchanged.
            ("dyn(42) + 1", 43i64.into_value()),
            ("type(42)", Value::Type(ValueType::Int)),
            ("bytes('hello')", b"hello".to_vec().into_value()),
            ("string(bytes('world'))", "world".into_value()),
            ("bytes_data.size()", 3i64.into_value()),
            ("bytes_data == b'\\x00\\x7f\\xff'", true.into_value()),
            // CEL string size counts Unicode code points; byte size counts UTF-8 bytes.
            ("'你好'.size()", 2i64.into_value()),
            ("bytes('你好').size()", 6i64.into_value()),
        ],
    )?;

    println!("\nEmpty collection semantics:");
    let empty = Activation::new().bind_variable("numbers", Vec::<i64>::new())?;
    show(
        &env,
        &empty,
        &[
            ("numbers.all(x, x > 0)", true.into_value()),
            ("numbers.exists(x, x > 0)", false.into_value()),
            ("numbers.exists_one(x, x > 0)", false.into_value()),
            ("numbers.map(x, x * 2)", Vec::<i64>::new().into_value()),
            ("numbers.size() == 0 ? 0 : numbers[0]", 0i64.into_value()),
        ],
    )?;

    // Timestamp and duration accessors. getDate() is 1-based while
    // getDayOfMonth()/getMonth()/getDayOfWeek()/getDayOfYear() are 0-based.
    // Duration accessors return totals; getMilliseconds() is the sub-second part.
    println!("\nTimestamp and duration accessors:");
    show(
        &env,
        &activation,
        &[
            ("timestamp('2023-07-04T15:04:05Z').getFullYear()", 2023i64.into_value()),
            ("timestamp('2023-07-04T15:04:05Z').getMonth()", 6i64.into_value()),
            ("timestamp('2023-07-04T15:04:05Z').getDate()", 4i64.into_value()),
            ("timestamp('2023-07-04T15:04:05Z').getDayOfWeek()", 2i64.into_value()),
            ("timestamp('2023-07-04T15:04:05Z').getHours()", 15i64.into_value()),
            ("timestamp('2023-07-04T15:04:05Z').getMinutes()", 4i64.into_value()),
            ("timestamp('2023-07-04T15:04:05Z').getSeconds()", 5i64.into_value()),
            // Accessor overloads accept an IANA timezone name.
            (
                "timestamp('2023-07-04T15:04:05Z').getHours('America/New_York')",
                11i64.into_value(),
            ),
            ("duration('1h2m3.5s').getHours()", 1i64.into_value()),
            ("duration('1h2m3.5s').getMinutes()", 62i64.into_value()),
            ("duration('1h2m3.5s').getSeconds()", 3723i64.into_value()),
            ("duration('1h2m3.5s').getMilliseconds()", 500i64.into_value()),
        ],
    )?;

    println!("\nExpected evaluation errors:");
    for expr in [
        "1 / 0",
        "9223372036854775807 + 1",
        "numbers[5]",
        "data['missing']",
        "int('not a number')",
        "uint(-1)",
    ] {
        let error = env
            .compile(expr)?
            .evaluate(&activation)
            .expect_err("invalid operation must return an error");
        assert_ne!(error.code(), Code::Ok);
        assert!(!error.message().is_empty());
        println!("  {expr}: {error}");
    }
    Ok(())
}

fn show(env: &Env, activation: &Activation, cases: &[(&str, Value)]) -> Result<(), Error> {
    for (expr, expected) in cases {
        let result = env.compile(expr)?.evaluate(activation)?;
        assert_eq!(&result, expected, "{expr}");
        println!("  {expr} = {result}");
    }
    Ok(())
}
