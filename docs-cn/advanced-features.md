# 高级特性

## 异步支持

当启用 `async` 功能时，您可以异步求值表达式：

```rust,no_run
# #[cfg(feature = "async")]
# async fn example() -> Result<(), cel_cxx::Error> {
use cel_cxx::*;

let env = Env::builder()
    .use_tokio()
    .register_global_function("async_fetch", async |id: i64| -> Result<String, Error> {
        // 模拟异步数据库调用
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(format!("Data for ID: {}", id))
    })?
    .build()?;

let program = env.compile("async_fetch(42)")?;
let result = program.evaluate(()).await?;
# Ok(())
# }
```

### 异步架构设计

在 CEL 中支持 Rust 异步函数面临独特的挑战，因为 CEL-CPP 本身不支持
异步或基于回调的用户定义函数和变量提供者。当 Rust 异步函数返回 `Future` 时，
它已经退出了当前堆栈帧，C++ CEL 求值引擎无法调度或等待 Rust future。

**cel-cxx** 通过创新的双线程架构解决了这个问题：

1. **异步到阻塞桥接**: 当注册异步函数或变量提供者时，
   整个程序求值会移动到阻塞线程，使用 `Runtime::spawn_blocking()`。
   主异步上下文接收一个 future，在求值完成时解析。

2. **阻塞到异步桥接**: 当在阻塞线程内调用异步回调时，
   返回的 future 会被分派回异步运行时执行，而阻塞线程
   使用 `Runtime::block_on()` 等待完成。

### 实现细节

- **生命周期管理**: 由于用户提供的函数和变量提供者可能是捕获
  复杂生命周期的闭包，cel-cxx 使用 [`async-scoped`](https://crates.io/crates/async-scoped)
  crate 在线程边界安全地管理这些生命周期。

- **多线程运行时要求**: 使用 Tokio 时，运行时必须是多线程的，
  因为实现依赖于 [`tokio::task::block_in_place()`](https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html)，
  这在单线程运行时中会 panic。

这种设计实现了异步 Rust 代码与同步 CEL-CPP 求值引擎的无缝集成，
在运行时边界之间保持性能和正确性。

## 函数重载

该库支持函数重载，具有自动类型解析：

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // 多个同名函数，不同签名
    .register_global_function("process", |x: i64| x * 2)?
    .register_global_function("process", |x: f64| x * 2.0)?
    .register_global_function("process", |x: String| x.to_uppercase())?
    .build()?;

// CEL 会根据参数类型自动选择正确的重载
let program1 = env.compile("process(42)")?;      // 调用 i64 版本
let program2 = env.compile("process(3.14)")?;    // 调用 f64 版本  
let program3 = env.compile("process('hello')")?; // 调用 String 版本
# Ok::<(), cel_cxx::Error>(())
```

## 智能引用处理

该库自动管理引用类型，具有安全的生命周期处理：

```rust,no_run
use cel_cxx::*;
use std::collections::HashMap;

// ✅ 这些引用模式自动工作：
let env = Env::builder()
    .declare_variable::<Vec<&str>>("string_refs")?        // 借用字符串
    .declare_variable::<HashMap<i64, &str>>("lookup")?    // 借用值
    .declare_variable::<Option<&str>>("maybe_str")?       // 可选借用
    .build()?;

// 库在编译时防止不安全的模式：
// ❌ .declare_variable::<&Vec<String>>("invalid")?  // 编译错误
# Ok::<(), cel_cxx::Error>(())
```

