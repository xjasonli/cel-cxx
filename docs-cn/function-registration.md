# 零注解函数注册

该库的核心特性使用**泛型关联类型（GATs）**自动推断函数签名，
无需手动类型注解：

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // ✨ 函数签名自动从 Rust 类型推断！
    .register_global_function("add", |a: i64, b: i64| a + b)?
    .register_global_function("concat", |a: String, b: &str| a + b)?
    .register_global_function("length", |s: &str| s.len() as i64)?
    .register_global_function("parse", |s: &str| s.parse::<i64>())?  // Result<i64, _> 自动处理
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

该系统支持多种函数模式：

## 拥有类型参数

函数可以接受拥有值，这些值会自动从 CEL 类型转换：

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // 基本拥有类型
    .register_global_function("add", |a: i64, b: i64| a + b)?
    .register_global_function("concat", |a: String, b: String| a + &b)?
    .register_global_function("sum_list", |nums: Vec<i64>| nums.iter().sum::<i64>())?
    
    // 复杂拥有类型
    .register_global_function("process_map", |data: std::collections::HashMap<String, i64>| {
        data.values().sum::<i64>()
    })?
    .register_global_function("handle_optional", |maybe_val: Option<String>| {
        maybe_val.unwrap_or_else(|| "default".to_string())
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## 引用类型参数

引用参数支持零拷贝操作，适用于性能关键代码：

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // 字符串引用 - 无需复制
    .register_global_function("length", |s: &str| s.len() as i64)?
    .register_global_function("starts_with", |text: &str, prefix: &str| text.starts_with(prefix))?
    
    // 集合元素引用 - 容器持有拥有值
    .register_global_function("first", |items: Vec<i64>| items.first().copied().unwrap_or(0))?
    .register_global_function("contains", |haystack: Vec<&str>, needle: &str| {
        haystack.iter().any(|&s| s == needle)
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## 引用类型返回值

函数可以返回其参数内数据的引用，实现高效的数据访问：

```rust,no_run
use cel_cxx::*;

// 定义返回引用的函数，使用适当的生命周期注解
fn get_domain(email: &str) -> &str {
    email.split('@').nth(1).unwrap_or("")
}

fn get_substring(text: &str, start: i64) -> &str {
    let start = start as usize;
    if start < text.len() { &text[start..] } else { "" }
}

let env = Env::builder()
    // 使用命名函数从借用参数返回字符串切片
    .register_global_function("get_domain", get_domain)?
    
    // 使用闭包从拥有容器返回拥有值
    .register_global_function("get_first", |items: Vec<String>| {
        items.into_iter().next().unwrap_or_default()
    })?
    
    // 使用命名函数返回参数数据的引用
    .register_global_function("get_substring", get_substring)?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## 直接返回值 vs Result 类型

系统支持直接返回值和 `Result<T, E>` 用于错误处理：

```rust,no_run
use cel_cxx::*;
use std::num::ParseIntError;
use std::io;

let env = Env::builder()
    // 直接返回值 - 总是成功
    .register_global_function("double", |x: i64| x * 2)?
    .register_global_function("format_name", |first: &str, last: &str| {
        format!("{}, {}", last, first)
    })?
    
    // Result 返回值 - 可以使用标准库错误优雅地失败
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
    
    // 具有拥有返回值和具体错误类型的 Result
    .register_global_function("safe_index", |items: Vec<String>, idx: i64| -> Result<String, io::Error> {
        let index = idx as usize;
        items.get(index)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Index out of bounds"))
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## 同步 vs 异步函数

同步和异步函数都得到无缝支持：

```rust,no_run
# #[cfg(feature = "async")]
# async fn example() -> Result<(), cel_cxx::Error> {
use cel_cxx::*;

let env = Env::builder()
    .use_tokio()
    
    // 同步函数 - 立即执行
    .register_global_function("sync_add", |a: i64, b: i64| a + b)?
    .register_global_function("sync_format", |name: &str| format!("Hello, {}", name))?
    
    // 异步函数 - 返回 future
    .register_global_function("async_fetch", async |id: i64| -> Result<String, std::io::Error> {
        // 模拟异步数据库调用
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(format!("Data for ID: {}", id))
    })?
    .register_global_function("async_validate", async |email: &str| -> Result<bool, std::fmt::Error> {
        // 模拟异步验证服务
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        Ok(email.contains('@'))
    })?
    
    // 在同一环境中混合同步和异步
    .register_global_function("process", |data: String| data.to_uppercase())?
    .register_global_function("async_process", async |data: String| {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        data.to_lowercase()
    })?
    .build()?;

// 当使用任何异步函数时进行异步求值
let program = env.compile("async_fetch(42) + ' - ' + async_validate('user@example.com')")?;
let result = program.evaluate(()).await?;
# Ok(())
# }
```

## 函数签名示例

以下是支持的函数签名的全面概述：

```rust,no_run
use cel_cxx::*;

// 定义返回引用的函数，使用适当的生命周期注解
fn substring_fn(s: &str, start: i64, len: i64) -> &str {
    let start = start as usize;
    let end = (start + len as usize).min(s.len());
    &s[start..end]
}

// 所有这些函数签名都是自动推断的：
let env = Env::builder()
    // 无参数
    .register_global_function("now", || std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64)?
    .register_global_function("pi", || std::f64::consts::PI)?
    
    // 单参数 - 各种类型
    .register_global_function("abs", |x: i64| x.abs())?
    .register_global_function("uppercase", |s: String| s.to_uppercase())?
    .register_global_function("len", |s: &str| s.len() as i64)?
    
    // 多参数 - 混合类型（使用命名函数处理生命周期）
    .register_global_function("substring", substring_fn)?
    
    // 泛型集合 - 拥有容器
    .register_global_function("join", |items: Vec<String>, sep: &str| items.join(sep))?
    .register_global_function("filter_positive", |nums: Vec<i64>| {
        nums.into_iter().filter(|&x| x > 0).collect::<Vec<_>>()
    })?
    
    // 可选类型
    .register_global_function("unwrap_or", |opt: Option<String>, default: String| {
        opt.unwrap_or(default)
    })?
    
    // 用于错误处理的 Result 类型，使用标准库错误
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

