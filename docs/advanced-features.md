# Advanced Features

## Async Support

When the `async` feature is enabled, you can evaluate expressions asynchronously:

```rust,no_run
# #[cfg(feature = "async")]
# async fn example() -> Result<(), cel_cxx::Error> {
use cel_cxx::*;

let env = Env::builder()
    .use_tokio()
    .register_global_function("async_fetch", async |id: i64| -> Result<String, Error> {
        // Simulate async database call
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(format!("Data for ID: {}", id))
    })?
    .build()?;

let program = env.compile("async_fetch(42)")?;
let result = program.evaluate(()).await?;
# Ok(())
# }
```

### Async Architecture Design

Supporting Rust async functions in CEL presents unique challenges since CEL-CPP doesn't
natively support asynchronous or callback-based user-defined functions and variable providers.
When a Rust async function returns a `Future`, it has already exited the current stack frame,
and the C++ CEL evaluation engine cannot schedule or await Rust futures.

**cel-cxx** solves this through an innovative dual-threading architecture:

1. **Async-to-Blocking Bridge**: When async functions or variable providers are registered,
   the entire program evaluation is moved to a blocking thread using `Runtime::spawn_blocking()`.
   The main async context receives a future that resolves when evaluation completes.

2. **Blocking-to-Async Bridge**: When async callbacks are invoked within the blocking thread,
   the returned futures are dispatched back to the async runtime for execution, while the
   blocking thread waits for completion using `Runtime::block_on()`.

### Implementation Details

- **Lifetime Management**: Since user-provided functions and variable providers can be capturing
  closures with complex lifetimes, cel-cxx uses the [`async-scoped`](https://crates.io/crates/async-scoped)
  crate to safely manage these lifetimes across thread boundaries.

- **Multi-threaded Runtime Requirement**: When using Tokio, the runtime must be multi-threaded
  because the implementation relies on [`tokio::task::block_in_place()`](https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html),
  which panics in single-threaded runtimes.

This design enables seamless integration of async Rust code with the synchronous CEL-CPP
evaluation engine, maintaining both performance and correctness across runtime boundaries.

## Function Overloads

The library supports function overloading with automatic type resolution:

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // Multiple functions with same name, different signatures
    .register_global_function("process", |x: i64| x * 2)?
    .register_global_function("process", |x: f64| x * 2.0)?
    .register_global_function("process", |x: String| x.to_uppercase())?
    .build()?;

// CEL will automatically choose the right overload based on argument types
let program1 = env.compile("process(42)")?;      // Calls i64 version
let program2 = env.compile("process(3.14)")?;    // Calls f64 version  
let program3 = env.compile("process('hello')")?; // Calls String version
# Ok::<(), cel_cxx::Error>(())
```

## Smart Reference Handling

The library automatically manages reference types with safe lifetime handling:

```rust,no_run
use cel_cxx::*;
use std::collections::HashMap;

// ✅ These reference patterns work automatically:
let env = Env::builder()
    .declare_variable::<Vec<&str>>("string_refs")?        // Borrowed strings
    .declare_variable::<HashMap<i64, &str>>("lookup")?    // Borrowed values
    .declare_variable::<Option<&str>>("maybe_str")?       // Optional borrows
    .build()?;

// The library prevents unsafe patterns at compile time:
// ❌ .declare_variable::<&Vec<String>>("invalid")?  // Compiler error
# Ok::<(), cel_cxx::Error>(())
```
