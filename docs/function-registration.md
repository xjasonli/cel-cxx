# Zero-Annotation Function Registration

The library's flagship feature uses **Generic Associated Types (GATs)** to automatically infer function signatures,
eliminating the need for manual type annotations:

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // ✨ Function signatures automatically inferred from Rust types!
    .register_global_function("add", |a: i64, b: i64| a + b)?
    .register_global_function("concat", |a: String, b: &str| a + b)?
    .register_global_function("length", |s: &str| s.len() as i64)?
    .register_global_function("parse", |s: &str| s.parse::<i64>())?  // Result<i64, _> auto-handled
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

This system supports a wide variety of function patterns:

## Owned Type Parameters

Functions can accept owned values, which are automatically converted from CEL types:

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // Basic owned types
    .register_global_function("add", |a: i64, b: i64| a + b)?
    .register_global_function("concat", |a: String, b: String| a + &b)?
    .register_global_function("sum_list", |nums: Vec<i64>| nums.iter().sum::<i64>())?
    
    // Complex owned types
    .register_global_function("process_map", |data: std::collections::HashMap<String, i64>| {
        data.values().sum::<i64>()
    })?
    .register_global_function("handle_optional", |maybe_val: Option<String>| {
        maybe_val.unwrap_or_else(|| "default".to_string())
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## Reference Type Parameters

Reference parameters enable zero-copy operations for performance-critical code:

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // String references - no copying required
    .register_global_function("length", |s: &str| s.len() as i64)?
    .register_global_function("starts_with", |text: &str, prefix: &str| text.starts_with(prefix))?
    
    // Collection element references - containers hold owned values
    .register_global_function("first", |items: Vec<i64>| items.first().copied().unwrap_or(0))?
    .register_global_function("contains", |haystack: Vec<&str>, needle: &str| {
        haystack.iter().any(|&s| s == needle)
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## Reference Type Return Values

Functions can return references to data within their parameters, enabling efficient data access:

```rust,no_run
use cel_cxx::*;

// Define functions that return references with proper lifetime annotations
fn get_domain(email: &str) -> &str {
    email.split('@').nth(1).unwrap_or("")
}

fn get_substring(text: &str, start: i64) -> &str {
    let start = start as usize;
    if start < text.len() { &text[start..] } else { "" }
}

let env = Env::builder()
    // Return string slices from borrowed parameters using named functions
    .register_global_function("get_domain", get_domain)?
    
    // Return owned values from owned containers using closures
    .register_global_function("get_first", |items: Vec<String>| {
        items.into_iter().next().unwrap_or_default()
    })?
    
    // Return references to parameter data using named functions
    .register_global_function("get_substring", get_substring)?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## Direct Return Values vs Result Types

The system supports both direct return values and `Result<T, E>` for error handling:

```rust,no_run
use cel_cxx::*;
use std::num::ParseIntError;
use std::io;

let env = Env::builder()
    // Direct return values - always succeed
    .register_global_function("double", |x: i64| x * 2)?
    .register_global_function("format_name", |first: &str, last: &str| {
        format!("{}, {}", last, first)
    })?
    
    // Result return values - can fail gracefully with standard library errors
    .register_global_function("parse_int", |s: &str| -> Result<i64, ParseIntError> {
        s.parse()
    })?
    .register_global_function("divide", |a: f64, b: f64| -> Result<f64, io::Error> {
        if b == 0.0 {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Division by zero"))
        } else {
            Ok(a / b)
        }
    })?
    
    // Result with owned return values and concrete error types
    .register_global_function("safe_index", |items: Vec<String>, idx: i64| -> Result<String, io::Error> {
        let index = idx as usize;
        items.get(index)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Index out of bounds"))
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## Synchronous vs Asynchronous Functions

Both sync and async functions are supported seamlessly:

```rust,no_run
# #[cfg(feature = "async")]
# async fn example() -> Result<(), cel_cxx::Error> {
use cel_cxx::*;

let env = Env::builder()
    .use_tokio()
    
    // Synchronous functions - execute immediately
    .register_global_function("sync_add", |a: i64, b: i64| a + b)?
    .register_global_function("sync_format", |name: &str| format!("Hello, {}", name))?
    
    // Asynchronous functions - return futures
    .register_global_function("async_fetch", async |id: i64| -> Result<String, std::io::Error> {
        // Simulate async database call
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(format!("Data for ID: {}", id))
    })?
    .register_global_function("async_validate", async |email: &str| -> Result<bool, std::fmt::Error> {
        // Simulate async validation service
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        Ok(email.contains('@'))
    })?
    
    // Mixed sync and async in same environment
    .register_global_function("process", |data: String| data.to_uppercase())?
    .register_global_function("async_process", async |data: String| {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        data.to_lowercase()
    })?
    .build()?;

// Async evaluation when any async functions are used
let program = env.compile("async_fetch(42) + ' - ' + async_validate('user@example.com')")?;
let result = program.evaluate(()).await?;
# Ok(())
# }
```

## Function Signature Examples

Here's a comprehensive overview of supported function signatures:

```rust,no_run
use cel_cxx::*;

// Define function that returns reference with proper lifetime annotation
fn substring_fn(s: &str, start: i64, len: i64) -> &str {
    let start = start as usize;
    let end = (start + len as usize).min(s.len());
    &s[start..end]
}

// All of these function signatures are automatically inferred:
let env = Env::builder()
    // No parameters
    .register_global_function("now", || std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64)?
    .register_global_function("pi", || std::f64::consts::PI)?
    
    // Single parameter - various types
    .register_global_function("abs", |x: i64| x.abs())?
    .register_global_function("uppercase", |s: String| s.to_uppercase())?
    .register_global_function("len", |s: &str| s.len() as i64)?
    
    // Multiple parameters - mixed types (using named function for lifetime)
    .register_global_function("substring", substring_fn)?
    
    // Generic collections - owned containers
    .register_global_function("join", |items: Vec<String>, sep: &str| items.join(sep))?
    .register_global_function("filter_positive", |nums: Vec<i64>| {
        nums.into_iter().filter(|&x| x > 0).collect::<Vec<_>>()
    })?
    
    // Optional types
    .register_global_function("unwrap_or", |opt: Option<String>, default: String| {
        opt.unwrap_or(default)
    })?
    
    // Result types for error handling with standard library errors
    .register_global_function("safe_divide", |a: f64, b: f64| -> Result<f64, std::io::Error> {
        if b == 0.0 { 
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Division by zero")) 
        } else { 
            Ok(a / b) 
        }
    })?
    .register_global_function("parse_float", |s: &str| -> Result<f64, std::num::ParseFloatError> {
        s.parse()
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

