# CEL 扩展

## 目录

- [CEL 扩展](#cel-扩展)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. 可用扩展](#2-可用扩展)
    - [2.1 字符串扩展](#21-字符串扩展)
    - [2.2 数学扩展](#22-数学扩展)
    - [2.3 列表扩展](#23-列表扩展)
    - [2.4 集合扩展](#24-集合扩展)
    - [2.5 正则表达式扩展](#25-正则表达式扩展)
    - [2.6 C++ 正则表达式扩展 (re)](#26-c-正则表达式扩展-re)
    - [2.7 编码器扩展](#27-编码器扩展)
    - [2.8 绑定扩展](#28-绑定扩展)
    - [2.9 Protocol Buffers 扩展](#29-protocol-buffers-扩展)
  - [3. 启用扩展](#3-启用扩展)
  - [4. 扩展兼容性](#4-扩展兼容性)

## 1. 概述

CEL 扩展提供了标准库之外的附加功能。这些扩展是可选的，必须在 CEL 环境中显式启用。它们为各种领域的常见用例提供了专门的函数。

扩展设计为：
- **模块化**：每个扩展都可以独立启用
- **安全**：所有扩展都保持 CEL 的安全保证
- **高性能**：扩展在原生代码中高效实现
- **可组合**：扩展可以无缝协作

## 2. 可用扩展

### 2.1 字符串扩展

**文档**：[strings.md](extensions/strings.md)

字符串扩展提供了超越标准库基本字符串操作的高级字符串操作函数。

**主要功能**：
- 字符访问（`charAt`）
- 字符串搜索（`indexOf`、`lastIndexOf`）
- 字符串提取（`substring`）
- 字符串引用和转义（`strings.quote`）
- 字符串修剪（`trim`）
- 字符串连接和分割（`join`、`split`）
- 大小写转换（`lowerAscii`、`upperAscii`）
- 字符串替换（`replace`）
- 字符串格式化（`format`）
- 字符串反转（`reverse`）

**使用示例**：
```cel
"Hello, World!".charAt(0)                    // "H"
"hello world".indexOf("world")               // 6
["a", "b", "c"].join(", ")                   // "a, b, c"
"Hello, %s!".format(["World"])               // "Hello, World!"
```

### 2.2 数学扩展

**文档**：[math.md](extensions/math.md)

数学扩展提供了基本算术运算之外的数学函数和操作。

**主要功能**：
- 最小值/最大值操作（`math.greatest`、`math.least`）
- 绝对值和符号（`math.abs`、`math.sign`）
- 舍入函数（`math.ceil`、`math.floor`、`math.round`、`math.trunc`）
- 位运算操作（`math.bitAnd`、`math.bitOr`、`math.bitXor`、`math.bitNot`、`math.bitShiftLeft`、`math.bitShiftRight`）
- 浮点数辅助函数（`math.isInf`、`math.isNaN`、`math.isFinite`）
- 平方根（`math.sqrt`）

**使用示例**：
```cel
math.greatest(1, 2, 3)                       // 3
math.least([-42.0, -21.5, -100.0])          // -100.0
math.abs(-5)                                 // 5
math.ceil(3.14)                              // 4.0
math.bitAnd(12, 10)                          // 8
math.sqrt(81)                                // 9.0
math.isFinite(1.2)                           // true
```

### 2.3 列表扩展

**文档**：[lists.md](extensions/lists.md)

列表扩展提供了高级列表处理函数。

**主要功能**：
- 列表切片（`slice`）
- 列表扁平化（`flatten`）
- 列表去重（`distinct`）
- 列表反转（`reverse`）
- 列表排序（`sort`、`sortBy`）
- 数字范围（`lists.range`）

**使用示例**：
```cel
[1, 2, 2, 3, 1].distinct()                   // [1, 2, 3]
[[1, 2], [3, 4]].flatten()                   // [1, 2, 3, 4]
[3, 1, 4, 1, 5].sort()                       // [1, 1, 3, 4, 5]
lists.range(5)                               // [0, 1, 2, 3, 4]
```

### 2.4 集合扩展

**文档**：[sets.md](extensions/sets.md)

集合扩展在列表上提供集合操作，将它们视为数学集合。

**主要功能**：
- 集合包含（`sets.contains`）
- 集合等价（`sets.equivalent`）
- 集合相交（`sets.intersects`）

**使用示例**：
```cel
sets.contains([1, 2, 3, 4], [2, 3])          // true
sets.equivalent([1, 2, 3], [3, 2, 1])        // true
sets.intersects([1, 2, 3], [3, 4, 5])        // true
```

### 2.5 正则表达式扩展

**文档**：[regex.md](extensions/regex.md)

正则表达式扩展使用正则表达式提供模式匹配功能。它支持标准的跨平台正则表达式函数和用于安全模式匹配的可选类型。

**主要功能**：
- 模式匹配（`matches`）
- 模式查找（`find`、`findAll`）
- 字符串分割（`split`）
- 安全操作的可选类型支持

**使用示例**：
```cel
"hello@example.com".matches(r"[\w]+@[\w.]+")  // true
"test123".find(r"\d+")                        // optional("123")
"a,b,c".split(",")                           // ["a", "b", "c"]
```

### 2.6 C++ 正则表达式扩展 (re)

**文档**：[re.md](extensions/re.md)

`re` 扩展提供了基于 RE2 库构建的 C++ 特定正则表达式函数。此扩展为字符串模式匹配、提取和捕获操作提供了附加功能。

**主要功能**：
- 模式提取和重写（`re.extract`）
- 组捕获（`re.capture`、`re.captureN`）
- 直接错误处理
- RE2 库性能和安全性

**使用示例**：
```cel
re.extract("Hello World", r"(\w+) (\w+)", r"\2, \1")  // "World, Hello"
re.capture("user@example.com", r"(\w+)@[\w.]+")       // "user"
re.captureN("John Doe", r"(?P<first>\w+) (?P<last>\w+)")  // {"first": "John", "last": "Doe"}
```

### 2.7 编码器扩展

**文档**：[encoders.md](extensions/encoders.md)

编码器扩展为常见格式提供编码和解码函数。

**主要功能**：
- Base64 编码/解码（`base64.encode`、`base64.decode`）

**使用示例**：
```cel
base64.encode(b"hello")                      // "aGVsbG8="
base64.decode("aGVsbG8=")                    // b"hello"
```

### 2.8 绑定扩展

**文档**：[bindings.md](extensions/bindings.md)

绑定扩展提供了 `cel.bind()` 宏，用于在 CEL 表达式中创建局部变量绑定。这通过避免重复计算来实现更可读和高效的表达式。

**主要功能**：
- 局部变量绑定（`cel.bind`）
- 嵌套绑定支持
- 表达式优化
- 提高可读性

**使用示例**：
```cel
cel.bind(x, "hello", x + " world")           // "hello world"
cel.bind(users, getUsers(), users.size())    // 用户数量
```

### 2.9 Protocol Buffers 扩展

**文档**：[proto.md](extensions/proto.md)

protobuf 扩展为使用 Protocol Buffer 消息提供了增强支持。

**主要功能**：
- 使用宏增强消息构造
- 扩展字段访问（`proto.getExt`、`proto.hasExt`）
- 类型安全操作

**使用示例**：
```cel
proto.getExt(msg, google.expr.proto2.test.int32_ext)
proto.hasExt(msg, google.expr.proto2.test.int32_ext)
```

## 3. 启用扩展

使用 `EnvBuilder` 构建 CEL 环境时启用扩展：

```rust
use cel_cxx::*;

let env = Env::builder()
    .with_ext_strings(true)
    .with_ext_math(true)
    .with_ext_lists(true)
    .with_ext_sets(true)
    .with_ext_regex(true)
    .with_ext_re(true)
    .with_ext_encoders(true)
    .with_ext_bindings(true)
    .with_ext_proto(true)
    .build()?;
```

您可以只启用需要的扩展：

```rust
// 只启用字符串和数学扩展
let env = Env::builder()
    .with_ext_strings(true)
    .with_ext_math(true)
    .build()?;
```

## 4. 扩展兼容性

- **所有扩展都相互兼容**，可以一起使用
- **扩展是可选的** - 您只为使用的功能付出代价
- **扩展保持 CEL 的安全保证** - 无内存泄漏，无崩溃
- **扩展是确定性的** - 相同输入总是产生相同输出
- **扩展是无副作用的** - 它们不修改全局状态

有关每个扩展的详细文档，请参阅 [extensions/](extensions/) 目录中的各个文件。