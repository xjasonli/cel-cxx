//! Preserve nanosecond precision while comparing timestamps and durations.
//!
//! Run with: `cargo run --example time-boundaries`

use cel_cxx::*;
use chrono::{DateTime, Duration, Utc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::builder()
        .declare_variable::<DateTime<Utc>>("instant")?
        .declare_variable::<Duration>("delta")?
        .declare_variable::<String>("text")?
        .build()?;
    let shift = env.compile("instant + delta")?;

    // Reuse one compiled expression on either side of the Unix epoch and a leap day.
    for (instant, delta, expected) in [
        (
            "1969-12-31T23:59:59.999999999Z",
            Duration::nanoseconds(1),
            "1970-01-01T00:00:00Z",
        ),
        (
            "1970-01-01T00:00:00Z",
            Duration::nanoseconds(-1),
            "1969-12-31T23:59:59.999999999Z",
        ),
        (
            "2024-02-29T23:59:59.999999999Z",
            Duration::nanoseconds(1),
            "2024-03-01T00:00:00Z",
        ),
    ] {
        let activation = Activation::new()
            .bind_variable("instant", instant.parse::<DateTime<Utc>>()?)?
            .bind_variable("delta", delta)?;
        let result = shift.evaluate(activation)?;
        assert_eq!(result, Value::Timestamp(expected.parse()?));
        println!("{instant} + {delta} = {result}");
    }

    let difference = env.compile(
        "timestamp('1969-12-31T23:59:59.999999999Z') - timestamp('1970-01-01T00:00:00Z')",
    )?;
    assert_eq!(
        difference.evaluate(())?,
        Value::Duration(Duration::nanoseconds(-1))
    );

    let normalized = env.compile("timestamp(text)")?;
    let activation = Activation::new().bind_variable("text", "1970-01-01T01:00:00+01:00")?;
    let result = normalized.evaluate(activation)?;
    assert_eq!(result, Value::Timestamp("1970-01-01T00:00:00Z".parse()?));
    println!("Offset timestamp normalized to UTC: {result}");

    let invalid = Activation::new().bind_variable("text", "2023-02-29T00:00:00Z")?;
    let error = normalized
        .evaluate(invalid)
        .expect_err("2023 is not a leap year");
    println!("Invalid calendar date: {error}");

    // A failed evaluation does not prevent the same program from being reused.
    let valid = Activation::new().bind_variable("text", "2024-02-29T00:00:00Z")?;
    assert_eq!(
        normalized.evaluate(valid)?,
        Value::Timestamp("2024-02-29T00:00:00Z".parse()?)
    );
    Ok(())
}
