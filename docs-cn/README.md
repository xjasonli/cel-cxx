# 文档

欢迎阅读 cel-cxx 文档！本指南涵盖了有效使用 cel-cxx 所需了解的所有内容。

## 目录

- [文档](#文档)
  - [目录](#目录)
  - [快速开始](#快速开始)
    - [前置要求](#前置要求)
  - [架构](#架构)
    - [核心设计原则](#核心设计原则)
    - [集成架构](#集成架构)
  - [类型系统](#类型系统)
    - [类型转换示例](#类型转换示例)
  - [功能标志](#功能标志)
  - [性能](#性能)
  - [示例](#示例)
  - [指南](#指南)
    - [函数注册](#函数注册)
    - [高级特性](#高级特性)
  - [参考](#参考)
  - [语言文档](#语言文档)

## 快速开始

### 前置要求

**系统要求：**
- **Rust**: 1.80+
- **C++ 工具链**: C++17 兼容的编译器
  - Linux: GCC 9+ 或 Clang 15+
  - macOS: Xcode 10+ 或 Clang 15+
  - Windows: MSVC 2022+

**安装验证：**
```bash
# 克隆并测试
git clone https://github.com/xjasonli/cel-cxx.git
cd cel-cxx
cargo test --all-targets

# 运行示例
cargo run --example comprehensive
cargo run --example tokio --features="tokio"
```

快速开始指南，请参阅[主 README](../README.md#quick-start)。

## 架构

### 核心设计原则

- **类型安全**: 编译时验证 CEL 表达式和函数签名
- **零成本抽象**: 直接 FFI 调用 CEL-CPP，开销最小
- **内存安全**: Rust 所有权系统防止常见的集成错误
- **符合人体工程学的 API**: 构建器模式和自动类型推断减少样板代码
- **可扩展性**: 支持自定义类型和异步操作

### 集成架构

该库提供了一个分层架构，连接 Rust 和 CEL-CPP：

- **应用层**: 用于环境构建和表达式求值的高级 API
- **类型系统层**: Rust 和 CEL 类型之间的自动转换
- **FFI 层**: 通过 `cxx` crate 实现零成本绑定到 CEL-CPP
- **CEL-CPP 层**: Google 的解析和求值参考实现

## 类型系统

该 crate 提供了全面的类型支持，支持 CEL 和 Rust 类型之间的自动转换。
所有类型都支持三个核心 trait，实现无缝集成：

| CEL 类型 | | Rust 类型 | | |
|----------|---|-----------|---|---|
| | | **声明** | **转换为 CEL** | **从 CEL 转换** |
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

**特殊引用支持**: 所有 `&T` 类型都支持**声明**和**转换为 CEL**操作，
支持零拷贝函数参数，如 `&str`、`&[u8]`、`&MyStruct` 等。

### 类型转换示例

```rust,no_run
use cel_cxx::*;
use std::collections::VecDeque;

// 自动转换无缝工作
let env = Env::builder()
    // 不同的整数类型都映射到 CEL int
    .register_global_function("process_i32", |x: i32| x * 2)?
    .register_global_function("process_i64", |x: i64| x * 2)?
    
    // 字符串类型可以互换
    .register_global_function("process_string", |s: String| s.to_uppercase())?
    .register_global_function("process_str", |s: &str| s.len() as i64)?
    
    // 容器类型可以与任何兼容的 Rust 集合一起工作
    .register_global_function("sum_vec", |nums: Vec<i64>| nums.iter().sum::<i64>())?
    .register_global_function("sum_deque", |nums: VecDeque<i64>| nums.iter().sum::<i64>())?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## 功能标志

| 功能 | 描述 | 默认 |
|---------|-------------|---------|
| `derive` | 自定义类型的派生宏 (`#[derive(Opaque)]`) | ✅ |
| `async` | 表达式和函数的异步/等待支持 | ❌ |
| `tokio` | Tokio 异步运行时集成（需要 `async`） | ❌ |
| `smol` | smol 运行时集成（需要 `async`） | ❌ |
| `async-std` | async-std 运行时集成（需要 `async`） | ❌ |

## 性能

- **零成本 FFI**: 直接 C++ 函数调用，无编组开销
- **编译时优化**: 函数签名在编译时解析
- **内存高效**: 通过智能引用处理实现最小分配
- **异步开销**: 仅在显式使用异步功能时产生
- **类型安全**: 编译时防止常见的集成错误

## 示例

该 crate 包含全面的示例，演示各种功能：

- **基本用法**: 变量绑定、函数注册、表达式求值
- **自定义类型**: 派生宏、成员函数、类型集成
- **异步支持**: Tokio/smol/async-std 集成示例
- **高级特性**: 函数重载、错误处理、复杂类型转换

运行示例：
```bash
cargo run --example comprehensive
cargo run --example tokio --features="tokio"
```

## 指南

### [函数注册](function-registration.md)

使用泛型关联类型（GATs）进行零注解函数注册的完整指南。
学习如何注册具有拥有类型、引用类型、异步函数等的函数。

### [高级特性](advanced-features.md)

深入了解高级特性，包括异步支持架构、函数重载和智能引用处理。

## 参考

- [CEL 参考](cel-reference/) - CEL 语言特性和标准库文档
- [API 文档](https://docs.rs/cel-cxx) - docs.rs 上的完整 API 参考
- [GitHub 仓库](https://github.com/xjasonli/cel-cxx) - 源代码和问题跟踪器
- [Crates.io](https://crates.io/crates/cel-cxx) - 包注册表

## 语言文档

- [英文文档](../README.md) (主要)
- [中文文档](../docs-cn/) (中文)

