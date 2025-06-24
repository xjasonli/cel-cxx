# cel-cxx

[![Crates.io](https://img.shields.io/crates/v/cel-cxx.svg)](https://crates.io/crates/cel-cxx)
[![Docs.rs](https://docs.rs/cel-cxx/badge.svg)](https://docs.rs/cel-cxx)
[![CI](https://github.com/xjasonli/cel-cxx/actions/workflows/rust.yml/badge.svg)](https://github.com/xjasonli/cel-cxx/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

A modern, high-performance Rust interface for [Common Expression Language (CEL)](https://github.com/google/cel-spec), built on top of [google/cel-cpp](https://github.com/google/cel-cpp) with zero-cost FFI bindings via [cxx](https://github.com/dtolnay/cxx).

## 🚀 Overview

`cel-cxx` provides a **type-safe**, **ergonomic**, and **high-performance** Rust API for CEL that leverages advanced Rust language features like **Generic Associated Types (GATs)** and **zero-annotation function registration** to deliver an exceptional developer experience while maintaining full CEL specification compatibility.

### Why cel-cxx?

- **🔥 Zero-Annotation Functions**: Register Rust functions without manual type annotations - the library automatically infers parameter and return types
- **⚡ Zero-Cost FFI**: Direct C++ interop with no runtime overhead through the cxx crate
- **🔄 First-Class Async**: Native async/await support with multiple runtime backends (Tokio, async-std)
- **🛡️ Memory Safe**: Rust's ownership system prevents common CEL integration bugs
- **🎯 Type-Safe**: Compile-time type checking for variables, functions, and expressions
- **🔧 Ergonomic API**: Intuitive builder patterns and derive macros

## 🖥️ Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| **Linux** | ✅ Supported | Fully tested and supported |
| **macOS** | ⚠️ Untested | Should work but not regularly tested |
| **Windows** | ❌ Not Supported | CEL-CPP Bazel build scripts don't support Windows |

## 📋 CEL Feature Support

### ✅ Supported Features

| Feature | Status | Description |
|---------|--------|-------------|
| **Basic Types** | ✅ | `null`, `bool`, `int`, `uint`, `double`, `string`, `bytes` |
| **Collections** | ✅ | `list<T>`, `map<K,V>` with full indexing and comprehensions |
| **Time Types** | ✅ | `duration`, `timestamp` with full arithmetic support |
| **Operators** | ✅ | Arithmetic, logical, comparison, and membership operators |
| **Functions** | ✅ | Built-in functions and custom function registration |
| **Variables** | ✅ | Variable binding and scoping |
| **Conditionals** | ✅ | Ternary operator and logical short-circuiting |
| **Comprehensions** | ✅ | List and map comprehensions with filtering |
| **Optional Types** | ✅ | `optional<T>` with safe navigation |
| **Custom Types** | ✅ | Opaque types via `#[derive(Opaque)]` |
| **Extensions** | ✅ | CEL language extensions and custom operators |
| **Macros** | ✅ | CEL macro expansion support |
| **Async Support** | ✅ | Async function calls and evaluation |
| **Function Overloads** | ✅ | Multiple function signatures with automatic resolution |
| **Type Checking** | ✅ | Compile-time type validation |

### 🚧 Planned Features

| Feature | Status | Description |
|---------|--------|-------------|
| **Protocol Buffer Integration** | 🚧 Planned | Direct support for protobuf messages and enums as native CEL types |
| **Windows Support** | 🚧 Planned | Requires CEL-CPP Windows build support |

## 🎯 Key Design Features

### 1. Zero-Annotation Function System

The library's **flagship feature** uses **Generic Associated Types (GATs)** to automatically infer function signatures:

```rust
use cel_cxx::*;

// ✨ Function signatures automatically inferred from Rust types!
let env = Env::builder()
    .register_global_function("add", |a: i64, b: i64| a + b)?
    .register_global_function("concat", |a: String, b: &str| a + b)?
    .register_global_function("length", |s: &str| s.len() as i64)?
    .register_global_function("parse", |s: &str| s.parse::<i64>())?  // Result<i64, _> auto-handled
    .build()?;
```

### 2. Dual Function Architecture

The library separates **function declarations** (compile-time type signatures) from **function implementations** (runtime callable code):

```rust
// Compile-time: Type checking and signature validation
env.declare_variable::<String>("user_name")?
   .register_global_function("validate_email", email_validator)?;

// Runtime: Actual function execution during evaluation  
let result = program.evaluate(&activation)?;
```

### 3. Advanced Type System Integration

Seamless integration between Rust and CEL type systems with automatic conversions:

```rust
use cel_cxx::*;
use std::collections::HashMap;

#[derive(Opaque, Debug, Clone, PartialEq)]
#[cel_cxx(type = "myapp.User")]
struct User {
    name: String,
    age: i32,
    roles: Vec<String>,
}

impl User {
    fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
}

let env = Env::builder()
    .declare_variable::<User>("current_user")?
    .declare_variable::<HashMap<String, Vec<i64>>>("permissions")?
    // ✨ Register struct method directly using RustType::method_name syntax
    .register_member_function("has_role", User::has_role)?
    .build()?;
```

### 4. Smart Reference Handling

The library automatically manages Rust reference types with controlled lifetime erasure:

```rust
// ✅ These work automatically:
Vec<&str>           // Borrows from source data
HashMap<i64, &str>  // String keys with borrowed values
Option<&str>        // Optional borrowed strings

// ❌ These are prevented at compile time:
&Vec<String>        // Prevents dangling references
&HashMap<String, i64>
```

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cel-cxx = "0.1.0"

# Optional features
cel-cxx = { version = "0.1.0", features = ["async", "derive", "tokio"] }
```

### Basic Usage

```rust
use cel_cxx::*;

fn main() -> Result<(), Error> {
    // 1. Build environment with variables and functions
    let env = Env::builder()
        .declare_variable::<String>("name")?
        .declare_variable::<i64>("age")?
        .register_global_function("adult", |age: i64| age >= 18)?
        .build()?;

    // 2. Compile expression
    let program = env.compile("'Hello ' + name + '! You are ' + (adult(age) ? 'an adult' : 'a minor')")?;

    // 3. Create activation with variable bindings
    let activation = Activation::new()
        .bind_variable("name", "Alice")?
        .bind_variable("age", 25i64)?;

    // 4. Evaluate
    let result = program.evaluate(&activation)?;
    println!("{}", result); // "Hello Alice! You are an adult"
    
    Ok(())
}
```

## 📚 Comprehensive Examples

### 1. Custom Types with Derive Macros

```rust
use cel_cxx::*;

#[derive(Opaque, Debug, Clone, PartialEq)]
#[cel_cxx(type = "ecommerce.Product")]
struct Product {
    id: i64,
    name: String,
    price: f64,
    categories: Vec<String>,
}

impl Product {
    // Struct methods that can be registered directly as CEL member functions
    fn in_budget(&self, budget: f64) -> bool {
        self.price <= budget
    }
    
    fn has_category(&self, category: &str) -> bool {
        self.categories.iter().any(|c| c == category)
    }
    
    fn discounted_price(&self, discount: f64) -> f64 {
        self.price * (1.0 - discount)
    }
    
    fn get_category_count(&self) -> i64 {
        self.categories.len() as i64
    }
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.id)
    }
}

fn main() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Vec<Product>>("products")?
        .declare_variable::<f64>("budget")?
        
        // ✨ Register struct methods directly - &self becomes CEL receiver
        .register_member_function("in_budget", Product::in_budget)?
        .register_member_function("has_category", Product::has_category)?
        .register_member_function("discounted_price", Product::discounted_price)?
        .register_member_function("get_category_count", Product::get_category_count)?
        
        // You can still use closures for more complex logic
        .register_member_function("formatted_name", |product: Product| {
            format!("{} (ID: {})", product.name, product.id)
        })?
        
        // Register global functions
        .register_global_function("affordable_products", |products: Vec<Product>, budget: f64| {
            products.into_iter()
                .filter(|p| p.price <= budget)
                .collect::<Vec<_>>()
        })?
        .build()?;

    // Test expressions using struct methods
    let test_expressions = vec![
        "products.filter(p, p.in_budget(budget) && p.has_category('electronics')).size() > 0",
        "products[0].discounted_price(0.1)",
        "products[0].get_category_count()",
        "products[0].formatted_name()",
    ];

    let products = vec![
        Product { id: 1, name: "Laptop".to_string(), price: 999.99, categories: vec!["electronics".to_string()] },
        Product { id: 2, name: "Book".to_string(), price: 29.99, categories: vec!["education".to_string()] },
    ];

    let activation = Activation::new()
        .bind_variable("products", products)?
        .bind_variable("budget", 1000.0)?;

    for expr in test_expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("{} = {}", expr, result);
    }
    // Output:
    // products.filter(p, p.in_budget(budget) && p.has_category('electronics')).size() > 0 = true
    // products[0].discounted_price(0.1) = 899.991
    // products[0].get_category_count() = 1
    // products[0].formatted_name() = Laptop (ID: 1)

    Ok(())
}
```

### 2. Advanced Function Registration

```rust
use cel_cxx::*;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let env = Env::builder()
        // Simple functions with automatic type inference
        .register_global_function("square", |x: i64| x * x)?
        .register_global_function("is_even", |x: i64| x % 2 == 0)?
        
        // Functions returning Results (error handling)
        .register_global_function("safe_divide", |a: f64, b: f64| -> Result<f64, Error> {
            if b == 0.0 {
                Err(Error::invalid_argument("Division by zero".to_string()))
            } else {
                Ok(a / b)
            }
        })?
        
        // Generic functions (need type annotations when registering)
        .register_global_function("count_items", count_items::<String>)?
        .register_global_function("count_numbers", count_items::<i64>)?
        
        // Complex container operations
        .register_global_function("group_by_length", |strings: Vec<String>| {
            let mut groups: HashMap<usize, Vec<String>> = HashMap::new();
            for s in strings {
                groups.entry(s.len()).or_default().push(s);
            }
            groups
        })?
        
        .build()?;

    // Test various function types
    let test_cases = vec![
        ("square(8)", "64"),
        ("is_even(7)", "false"),
        ("safe_divide(10.0, 2.0)", "5"),
        ("count_items(['hello', 'world'])", "2"),
    ];

    for (expr, expected) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(())?;
        println!("{} = {} (expected: {})", expr, result, expected);
    }

    Ok(())
}

// Generic helper function
fn count_items<T>(items: Vec<T>) -> usize {
    items.len()
}
```

### 3. Function Overloads and Generic Functions

```rust
use cel_cxx::*;

fn main() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Vec<i64>>("numbers")?
        .declare_variable::<Vec<String>>("strings")?
        
        // Multiple functions with same name, different signatures
        .register_global_function("process", |x: i64| x * 2)?
        .register_global_function("process", |x: f64| (x * 2.0).round())?
        .register_global_function("process", |x: String| x.to_uppercase())?
        
        // Overloaded member functions for different container types
        .register_member_function("sum", |numbers: Vec<i64>| {
            numbers.iter().sum::<i64>()
        })?
        
        .register_member_function("join", |strings: Vec<String>| {
            strings.join(", ")
        })?
        
        // Generic functions (need explicit type annotation when registering)
        .register_global_function("count_items", count_items::<i64>)?
        .register_global_function("count_strings", count_items::<String>)?
        
        .build()?;

    let test_cases = vec![
        ("process(42)", "Process integer"),
        ("process(3.14)", "Process float"),
        ("process('hello')", "Process string"),
        ("numbers.sum()", "Sum integers"),
        ("strings.join()", "Join strings"),
        ("count_items(numbers)", "Count integers"),
        ("count_strings(strings)", "Count strings"),
    ];

    let activation = Activation::new()
        .bind_variable("numbers", vec![1i64, 2, 3, 4, 5])?
        .bind_variable("strings", vec!["hello".to_string(), "world".to_string()])?;

    for (expr, description) in test_cases {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("  {} = {} ({})", expr, result, description);
    }

    Ok(())
}

// Generic helper function
fn count_items<T>(items: Vec<T>) -> usize {
    items.len()
}
```

### 4. Direct Struct Method Registration

The library supports registering struct methods directly as CEL member functions, where `&self` becomes the CEL receiver:

```rust
use cel_cxx::*;

#[derive(Opaque, Debug, Clone, PartialEq)]
#[cel_cxx(type = "banking.Account")]
struct Account {
    id: String,
    balance: f64,
    currency: String,
    frozen: bool,
}

impl Account {
    // These methods can be registered directly as CEL member functions
    fn is_active(&self) -> bool {
        !self.frozen && self.balance >= 0.0
    }
    
    fn can_withdraw(&self, amount: f64) -> bool {
        self.is_active() && self.balance >= amount
    }
    
    fn get_balance_in_cents(&self) -> i64 {
        (self.balance * 100.0) as i64
    }
    
    fn format_balance(&self) -> String {
        format!("{:.2} {}", self.balance, self.currency)
    }
    
    // Methods with Result return types work too
    fn validate_currency(&self, expected: &str) -> Result<bool, Error> {
        if self.currency == expected {
            Ok(true)
        } else {
            Err(Error::invalid_argument(format!("Expected {}, got {}", expected, self.currency)))
        }
    }
}

fn main() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<Account>("account")?
        .declare_variable::<f64>("amount")?
        
        // ✨ Register struct methods directly - no closures needed!
        .register_member_function("is_active", Account::is_active)?
        .register_member_function("can_withdraw", Account::can_withdraw)?
        .register_member_function("get_balance_in_cents", Account::get_balance_in_cents)?
        .register_member_function("format_balance", Account::format_balance)?
        .register_member_function("validate_currency", Account::validate_currency)?
        
        .build()?;

    let account = Account {
        id: "ACC123".to_string(),
        balance: 1500.75,
        currency: "USD".to_string(),
        frozen: false,
    };

    let test_expressions = vec![
        "account.is_active()",
        "account.can_withdraw(amount)",
        "account.get_balance_in_cents()",
        "account.format_balance()",
        "account.validate_currency('USD')",
    ];

    let activation = Activation::new()
        .bind_variable("account", account)?
        .bind_variable("amount", 500.0)?;

    for expr in test_expressions {
        let program = env.compile(expr)?;
        let result = program.evaluate(&activation)?;
        println!("{} = {}", expr, result);
    }
    // Output:
    // account.is_active() = true
    // account.can_withdraw(amount) = true  
    // account.get_balance_in_cents() = 150075
    // account.format_balance() = 1500.75 USD
    // account.validate_currency('USD') = true

    Ok(())
}
```

**Key Benefits of Direct Method Registration:**

- **🎯 Natural Rust Patterns**: Use standard `&self` receiver syntax
- **🔄 Zero Boilerplate**: No need to write wrapper closures
- **📝 Better Readability**: Method names directly correspond to CEL functions
- **🛡️ Type Safety**: Compile-time verification of method signatures
- **⚡ Performance**: Direct function pointers, no closure overhead

### 5. Error Handling and Validation

```rust
use cel_cxx::*;

fn main() -> Result<(), Error> {
    let env = Env::builder()
        .declare_variable::<String>("email")?
        .declare_variable::<i64>("age")?
        
        .register_global_function("validate_email", |email: &str| -> Result<bool, Error> {
            if email.contains('@') && email.contains('.') {
                Ok(true)
            } else {
                Err(Error::invalid_argument(format!("Invalid email format: {}", email)))
            }
        })?
        
        .register_global_function("validate_age", |age: i64| -> Result<String, Error> {
            match age {
                0..=17 => Err(Error::invalid_argument("Must be 18 or older".to_string())),
                18..=120 => Ok("Valid age".to_string()),
                _ => Err(Error::invalid_argument("Invalid age range".to_string())),
            }
        })?
        
        .build()?;

    // Test validation expressions
    let validation_expr = "validate_email(email) && validate_age(age) != ''";
    let program = env.compile(validation_expr)?;

    // Test cases
    let test_cases = vec![
        ("valid@email.com", 25i64, true),
        ("invalid-email", 25i64, false),
        ("valid@email.com", 16i64, false),
    ];

    for (email, age, should_pass) in test_cases {
        let activation = Activation::new()
            .bind_variable("email", email)?
            .bind_variable("age", age)?;

        match program.evaluate(&activation) {
            Ok(result) => {
                let passed = result == Value::Bool(true);
                println!("Email: {}, Age: {} -> {} (expected: {})", 
                    email, age, if passed { "PASS" } else { "FAIL" }, should_pass);
            }
            Err(e) => {
                println!("Email: {}, Age: {} -> ERROR: {} (expected: {})", 
                    email, age, e, should_pass);
            }
        }
    }

    Ok(())
}
```

### 6. Async Support with Multiple Runtimes

```rust
#[cfg(feature = "async")]
use cel_cxx::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = Env::builder()
        .use_tokio()  // or .use_async_std()
        .declare_variable::<String>("user_id")?
        
        // Async global function
        .register_global_function("fetch_user_score", async |user_id: String| -> Result<i64, Error> {
            // Simulate async database call
            tokio::time::sleep(Duration::from_millis(100)).await;
            if user_id == "user123" {
                Ok(95)
            } else {
                Err(Error::not_found("User not found".to_string()))
            }
        })?
        
        // Mixed sync/async functions
        .register_global_function("calculate_grade", |score: i64| -> String {
            match score {
                90..=100 => "A".to_string(),
                80..=89 => "B".to_string(),
                70..=79 => "C".to_string(),
                _ => "F".to_string(),
            }
        })?
        
        .build()?;

    let program = env.compile("calculate_grade(fetch_user_score(user_id))")?;

    let activation = Activation::new()
        .bind_variable("user_id", "user123".to_string())?;

    let result = program.evaluate(&activation).await?;
    println!("Grade: {}", result); // "A"

    Ok(())
}
```

#### Async Architecture Design

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

#### Implementation Details

- **Lifetime Management**: Since user-provided functions and variable providers can be capturing
  closures with complex lifetimes, cel-cxx uses the [async-scoped](https://crates.io/crates/async-scoped) 
  crate to safely manage these lifetimes across thread boundaries.

- **Multi-threaded Runtime Requirement**: When using Tokio, the runtime must be multi-threaded
  because the implementation relies on [`tokio::task::block_in_place()`](https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html),
  which panics in single-threaded runtimes.

This design enables seamless integration of async Rust code with the synchronous CEL-CPP
evaluation engine, maintaining both performance and correctness across runtime boundaries.

## 🔧 Advanced Features

### Feature Flags

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `async` | Async/await support for expressions and functions | `async-scoped`, `futures` |
| `derive` | Derive macros for custom types (`#[derive(Opaque)]`) | `cel-cxx-macros` |
| `tokio` | Tokio async runtime integration | `tokio`, `async` |
| `async-std` | async-std runtime integration | `async-std`, `async` |

### Memory Management

The library provides sophisticated memory management for reference types:

```rust
// ✅ Supported reference patterns
let data = vec!["hello".to_string(), "world".to_string()];
let borrowed: Vec<&str> = data.iter().map(|s| s.as_str()).collect();

// Container with borrowed contents
let map: HashMap<i64, &str> = HashMap::from([(1, "one"), (2, "two")]);

// Optional borrowed types  
let opt: Option<&str> = Some("value");
```

### Type Conversion System

Comprehensive type mapping between CEL and Rust with automatic conversions:

| CEL Type | | Rust Type | | |
|----------|---|-----------|---|---|
| | | **Declare** | **To CEL** | **From CEL** |
| | | `TypedValue` | `IntoValue` | `FromValue` |
| `null` | | `()` | ✅ | ✅ | ✅ |
| `bool` | | `bool` | ✅ | ✅ | ✅ |
| `int` | | `i64`, `i32`, `i16`, `isize` | ✅ | ✅ | ✅ |
| `uint` | | `u64`, `u32`, `u16`, `usize` | ✅ | ✅ | ✅ |
| `double` | | `f64`, `f32` | ✅ | ✅ | ✅ |
| `string` | | `String`, `ArcStr`, `Box<str>`, `str` | ✅ | ✅ | ✅ |
| `bytes` | | `Vec<u8>`, `ArcBytes`, `Box<[u8]>`, `[u8]` | ✅ | ✅ | ✅ |
| `duration` | | `chrono::Duration` | ✅ | ✅ | ✅ |
| `timestamp` | | `chrono::DateTime<Utc>`, `SystemTime` | ✅ | ✅ | ✅ |
| `list<T>` | | `Vec<T>`, `VecDeque<T>`, `LinkedList<T>`, `[T]` | ✅ | ✅ | ✅ |
| `map<K,V>` | | `HashMap<K,V>`, `BTreeMap<K,V>`, `Vec<(K,V)>` | ✅ | ✅ | ✅ |
| `optional<T>` | | `Option<T>`, `Optional<T>` | ✅ | ✅ | ✅ |
| `type` | | `ValueType` | ✅ | ✅ | ✅ |
| `error` | | `Error` | ✅ | ✅ | ✅ |
| `opaque` | | `#[derive(Opaque)] struct` | ✅ | ✅ | ✅ |

**Special Reference Support**: All `&T` types support **Declare** and **To CEL** operations,
enabling zero-copy function arguments like `&str`, `&[u8]`, `&MyStruct`, etc.

#### Conversion Examples

```rust
// Rust -> CEL (IntoValue)
let rust_value = vec![1, 2, 3];
let cel_value: Value = rust_value.into_value();

// CEL -> Rust (FromValue)  
let cel_value = Value::List(vec![Value::Int(1), Value::Int(2)]);
let rust_value: Vec<i64> = cel_value.try_into()?;

// Standard Rust conversions also work
let cel_value = Value::Int(42);
let rust_value: i32 = cel_value.try_into()?;  // TryFrom

// Reference types for zero-copy operations
let data = "hello world";
let cel_value: Value = data.into_value();  // &str -> CEL string
```

## 🏗️ Architecture Overview

### Core Components

1. **Environment (`Env`)**: Compilation context with type declarations and function registry
2. **Program**: Compiled CEL expression ready for evaluation  
3. **Activation**: Runtime context with variable bindings and providers
4. **Value System**: Type-safe value representation with automatic conversions
5. **Function System**: Dual-layer architecture for type checking and execution
6. **Variable System**: Compile-time declarations with runtime bindings

### Type System Hierarchy

```
CEL Type System
├── Primitive Types (bool, int, uint, double, string, bytes)
├── Container Types (list, map)  
├── Well-Known Types (Duration, Timestamp, Struct)
├── Protocol Buffer Types (Any, Value, etc.)
└── Custom Opaque Types (via #[derive(Opaque)])
```

## 🚀 Performance Characteristics

- **Zero-cost FFI**: Direct C++ calls with no marshaling overhead
- **Compile-time optimization**: Function signatures resolved at compile time
- **Memory efficient**: Minimal allocations through smart reference handling
- **Async overhead**: Only when async features are used

## 📋 Prerequisites

### System Requirements

- **Rust**: 1.70+ (for GATs support)
- **C++ Toolchain**: C++17 compatible compiler
  - Linux: GCC 7+ or Clang 6+
  - macOS: Xcode 10+ or Clang 6+
  - Windows: MSVC 2019+ or Clang 6+

### Dependencies

- **google/cel-cpp**: Follow the [installation guide](https://github.com/google/cel-cpp#building)
- **pkg-config**: For library discovery (Linux/macOS)

### Installation Verification

```bash
# Clone and test
git clone https://github.com/xjasonli/cel-cxx.git
cd cel-cxx
cargo test

# Run examples
cargo run --example comprehensive
cargo run --example tokio --features="async,tokio"
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Setup development environment
git clone https://github.com/xjasonli/cel-cxx.git
cd cel-cxx

# Install dependencies
cargo build

# Run tests
cargo test --all-features

# Run examples
cargo run --example comprehensive
```

### Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Ensure all tests pass (`cargo test`)
- Add documentation for public APIs
- Include examples for new features

## 📄 License

Licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgements

- [google/cel-cpp](https://github.com/google/cel-cpp) - The foundational C++ CEL implementation
- [cxx](https://github.com/dtolnay/cxx) - Safe and efficient Rust-C++ interop
- The CEL community for the excellent specification and ecosystem

---

## 📋 Summary

**cel-cxx** represents a significant advancement in CEL integration for Rust applications. By leveraging cutting-edge Rust language features like GATs and sophisticated type system integration, it provides:

### 🎯 **Core Innovations**
- **Zero-Annotation Function System**: Automatic type inference eliminates boilerplate
- **Dual Architecture**: Separates compile-time checking from runtime execution
- **Smart Memory Management**: Safe reference handling with lifetime erasure
- **Type-Safe Integration**: Compile-time prevention of common integration errors

### 🏆 **Key Benefits**
- **Developer Experience**: Intuitive API with minimal learning curve
- **Performance**: Zero-cost abstractions with direct C++ integration
- **Safety**: Rust's ownership system prevents memory safety issues
- **Flexibility**: Support for sync/async, custom types, and function overloads

### 🔮 **Use Cases**
- **Policy Engines**: Business rule evaluation and validation
- **Configuration Systems**: Dynamic configuration with type safety
- **API Gateways**: Request/response filtering and transformation
- **Workflow Engines**: Conditional logic and decision making
- **Security Systems**: Access control and permission evaluation

**cel-cxx** makes CEL integration in Rust not just possible, but **enjoyable** and **productive**.

---

<div align="center">

**[📖 Documentation](https://docs.rs/cel-cxx)** | **[🦀 Crates.io](https://crates.io/crates/cel-cxx)** | **[💻 GitHub](https://github.com/xjasonli/cel-cxx)**

</div>
