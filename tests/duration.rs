use cel_cxx::{Activation, Env, Error, Value};
use chrono::{DateTime, Duration, Utc};

#[test]
fn fractional_durations_preserve_nanoseconds_in_both_directions() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Duration>("value")?
        .build()?;
    let identity = env.compile("value")?;

    for (literal, expected) in [
        ("0s", Duration::zero()),
        ("0.000000001s", Duration::nanoseconds(1)),
        ("-0.000000001s", Duration::nanoseconds(-1)),
        ("0.5s", Duration::milliseconds(500)),
        ("-0.5s", Duration::milliseconds(-500)),
        ("1.23456789s", Duration::nanoseconds(1_234_567_890)),
        ("-1.23456789s", Duration::nanoseconds(-1_234_567_890)),
        ("315576000000s", Duration::seconds(315_576_000_000)),
        ("-315576000000s", Duration::seconds(-315_576_000_000)),
    ] {
        let parsed = env.compile(format!("duration('{literal}')"))?;
        assert_eq!(parsed.evaluate(())?, Value::Duration(expected), "{literal}");

        let activation = Activation::new().bind_variable("value", expected)?;
        let comparison = env.compile(format!("value == duration('{literal}')"))?;
        assert_eq!(
            comparison.evaluate(&activation)?,
            Value::Bool(true),
            "{literal}"
        );
        assert_eq!(
            identity.evaluate(&activation)?,
            Value::Duration(expected),
            "{literal}"
        );
    }

    Ok(())
}

#[test]
fn fractional_timestamps_preserve_nanoseconds_in_both_directions() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<DateTime<Utc>>("value")?
        .build()?;
    let identity = env.compile("value")?;

    for literal in [
        "0001-01-01T00:00:00Z",
        "1969-12-31T23:59:59.123456789Z",
        "1969-12-31T23:59:59.999999999Z",
        "1970-01-01T00:00:00Z",
        "1970-01-01T00:00:00.000000001Z",
        "2026-09-21T12:34:56.987654321Z",
        "9999-12-31T23:59:59.999999999Z",
    ] {
        let expected = literal.parse::<DateTime<Utc>>().unwrap();
        let parsed = env.compile(format!("timestamp('{literal}')"))?;
        assert_eq!(
            parsed.evaluate(())?,
            Value::Timestamp(expected),
            "{literal}"
        );

        let activation = Activation::new().bind_variable("value", expected)?;
        let comparison = env.compile(format!("value == timestamp('{literal}')"))?;
        assert_eq!(
            comparison.evaluate(&activation)?,
            Value::Bool(true),
            "{literal}"
        );
        assert_eq!(
            identity.evaluate(&activation)?,
            Value::Timestamp(expected),
            "{literal}"
        );
    }

    Ok(())
}

#[test]
fn duration_arithmetic_carries_and_borrows_nanoseconds() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Duration>("left")?
        .declare_variable::<Duration>("right")?
        .build()?;
    let sum = env.compile("left + right")?;
    let difference = env.compile("left - right")?;

    for (left, right) in [
        (Duration::nanoseconds(999_999_999), Duration::nanoseconds(1)),
        (
            Duration::nanoseconds(-999_999_999),
            Duration::nanoseconds(-1),
        ),
        (Duration::seconds(1), Duration::nanoseconds(-1)),
        (Duration::seconds(-1), Duration::nanoseconds(1)),
        (Duration::nanoseconds(-1), Duration::nanoseconds(1)),
        (
            Duration::nanoseconds(-1_500_000_001),
            Duration::nanoseconds(500_000_002),
        ),
    ] {
        let activation = Activation::new()
            .bind_variable("left", left)?
            .bind_variable("right", right)?;
        assert_eq!(sum.evaluate(&activation)?, Value::Duration(left + right));
        assert_eq!(
            difference.evaluate(&activation)?,
            Value::Duration(left - right)
        );
    }

    Ok(())
}

#[test]
fn durations_near_the_supported_limits_preserve_subseconds() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Duration>("value")?
        .build()?;

    for (value, expression, expected) in [
        (
            Duration::seconds(315_576_000_000),
            "value - duration('0.000000001s')",
            Duration::seconds(315_576_000_000) - Duration::nanoseconds(1),
        ),
        (
            Duration::seconds(-315_576_000_000),
            "value + duration('0.000000001s')",
            Duration::seconds(-315_576_000_000) + Duration::nanoseconds(1),
        ),
    ] {
        let activation = Activation::new().bind_variable("value", value)?;
        let result = env.compile(expression)?.evaluate(&activation)?;
        assert_eq!(result, Value::Duration(expected), "{expression}");
    }

    Ok(())
}

#[test]
fn timestamp_arithmetic_crosses_seconds_epoch_and_calendar_boundaries() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<DateTime<Utc>>("value")?
        .declare_variable::<Duration>("delta")?
        .build()?;
    let add = env.compile("value + delta")?;
    let subtract = env.compile("value - delta")?;

    for (literal, delta) in [
        ("1969-12-31T23:59:59.999999999Z", Duration::nanoseconds(1)),
        ("1970-01-01T00:00:00Z", Duration::nanoseconds(1)),
        ("1970-01-01T00:00:00.000000001Z", Duration::nanoseconds(-2)),
        ("2024-02-29T23:59:59.999999999Z", Duration::nanoseconds(1)),
        ("2000-01-01T00:00:00Z", Duration::nanoseconds(-1)),
        ("0001-01-01T00:00:01Z", Duration::nanoseconds(999_999_999)),
        ("9999-12-31T23:59:59Z", Duration::nanoseconds(999_999_999)),
    ] {
        let value = literal.parse::<DateTime<Utc>>().unwrap();
        let activation = Activation::new()
            .bind_variable("value", value)?
            .bind_variable("delta", delta)?;
        assert_eq!(
            add.evaluate(&activation)?,
            Value::Timestamp(value + delta),
            "{literal}"
        );
        assert_eq!(
            subtract.evaluate(&activation)?,
            Value::Timestamp(value - delta),
            "{literal}"
        );
    }

    Ok(())
}

#[test]
fn timestamp_subtraction_preserves_negative_fractional_durations() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<DateTime<Utc>>("left")?
        .declare_variable::<DateTime<Utc>>("right")?
        .build()?;
    let difference = env.compile("left - right")?;
    let ordered = env.compile("left < right")?;

    for (left, right) in [
        ("1969-12-31T23:59:59.999999999Z", "1970-01-01T00:00:00Z"),
        (
            "1970-01-01T00:00:00.000000001Z",
            "1969-12-31T23:59:59.999999999Z",
        ),
        ("2024-02-29T00:00:00Z", "2024-03-01T00:00:00.000000001Z"),
        ("0001-01-01T00:00:00Z", "9999-12-31T23:59:59.999999999Z"),
        ("1970-01-01T00:00:00Z", "1970-01-01T00:00:00Z"),
    ] {
        let left = left.parse::<DateTime<Utc>>().unwrap();
        let right = right.parse::<DateTime<Utc>>().unwrap();
        let activation = Activation::new()
            .bind_variable("left", left)?
            .bind_variable("right", right)?;
        assert_eq!(
            difference.evaluate(&activation)?,
            Value::Duration(left - right)
        );
        assert_eq!(ordered.evaluate(&activation)?, Value::Bool(left < right));
    }

    Ok(())
}

#[test]
fn timestamp_offsets_normalize_across_utc_date_boundaries() -> Result<(), Error> {
    let env = Env::builder().build()?;
    for (literal, utc) in [
        ("1970-01-01T00:00:00+01:00", "1969-12-31T23:00:00Z"),
        ("1969-12-31T23:30:00-00:30", "1970-01-01T00:00:00Z"),
        (
            "2024-03-01T00:00:00.000000001+01:30",
            "2024-02-29T22:30:00.000000001Z",
        ),
        (
            "2024-12-31T23:59:59.999999999-01:00",
            "2025-01-01T00:59:59.999999999Z",
        ),
    ] {
        assert_eq!(
            env.compile(format!("timestamp('{literal}')"))?
                .evaluate(())?,
            Value::Timestamp(utc.parse::<DateTime<Utc>>().unwrap()),
            "{literal}",
        );
    }

    Ok(())
}

#[test]
fn invalid_time_strings_fail_evaluation_without_poisoning_the_program() -> Result<(), Error> {
    let env = Env::builder().declare_variable::<String>("text")?.build()?;
    for (conversion, invalid, valid) in [
        (
            "duration",
            vec!["", "one second", "1xs", "1..5s", "NaN"],
            "0.000000001s",
        ),
        (
            "timestamp",
            vec![
                "",
                "not a timestamp",
                "2023-02-29T00:00:00Z",
                "2024-13-01T00:00:00Z",
                "2024-01-01T00:00:00",
            ],
            "1970-01-01T00:00:00Z",
        ),
    ] {
        let program = env.compile(format!("{conversion}(text)"))?;
        for text in invalid {
            let activation = Activation::new().bind_variable("text", text.to_owned())?;
            assert!(
                program.evaluate(&activation).is_err(),
                "{conversion}({text:?}) must fail"
            );
        }
        let activation = Activation::new().bind_variable("text", valid.to_owned())?;
        let expected = if conversion == "duration" {
            Value::Duration(Duration::nanoseconds(1))
        } else {
            Value::Timestamp(valid.parse::<DateTime<Utc>>().unwrap())
        };
        assert_eq!(program.evaluate(&activation)?, expected);
    }

    Ok(())
}
