# 可选类型

- [可选类型](#可选类型)
  - [1. 概述](#1-概述)
  - [2. 语法](#2-语法)
    - [2.1 可选字段选择 (`.?`)](#21-可选字段选择-)
    - [2.2 可选索引 (`[?_]`)](#22-可选索引-_)
      - [2.2.1 映射键访问](#221-映射键访问)
      - [2.2.2 列表索引访问](#222-列表索引访问)
    - [2.3 可选构造 (`{?key: ...}`)](#23-可选构造-key-)
      - [2.3.1 消息字段构造](#231-消息字段构造)
      - [2.3.2 映射条目构造](#232-映射条目构造)
      - [2.3.3 可选列表元素构造](#233-可选列表元素构造)
    - [2.4 链式行为](#24-链式行为)
  - [3. 函数](#3-函数)
    - [3.1 创建函数](#31-创建函数)
      - [3.1.1 `optional.of(<value>)`](#311-optionalofvalue)
      - [3.1.2 `optional.ofNonZeroValue(<value>)`](#312-optionalofnonzerovaluevalue)
      - [3.1.3 `optional.none()`](#313-optionalnone)
    - [3.2 访问函数](#32-访问函数)
      - [3.2.1 `<optional>.hasValue()`](#321-optionalhasvalue)
      - [3.2.2 `<optional>.value()`](#322-optionalvalue)
    - [3.3 链式函数](#33-链式函数)
      - [3.3.1 `<optional>.or(<optional>)`](#331-optionaloroptional)
      - [3.3.2 `<optional>.orValue(<default>)`](#332-optionalorvaluedefault)
    - [3.4 转换函数](#34-转换函数)
      - [3.4.1 `<optional>.optMap(<var>, <expr>)`](#341-optionaloptmapvar-expr)
      - [3.4.2 `<optional>.optFlatMap(<var>, <expr>)`](#342-optionaloptflatmapvar-expr)
    - [3.5 实用函数](#35-实用函数)
      - [3.5.1 `optional.unwrap(<list>)`](#351-optionalunwraplist)
  - [4. 类型系统](#4-类型系统)
    - [4.1 可选类型声明](#41-可选类型声明)
    - [4.2 类型兼容性](#42-类型兼容性)
  - [5. 示例](#5-示例)
    - [5.1 基本用法](#51-基本用法)
    - [5.2 链式操作](#52-链式操作)
    - [5.3 条件构造](#53-条件构造)
    - [5.4 数据处理](#54-数据处理)
    - [5.5 错误处理](#55-错误处理)
  - [6. 最佳实践](#6-最佳实践)
  - [7. 迁移指南](#7-迁移指南)
    - [从显式空值检查：](#从显式空值检查)
    - [从容易出错的索引：](#从容易出错的索引)
    - [从复杂的条件构造：](#从复杂的条件构造)

## 1. 概述

可选类型扩展引入了表示和交互可能在运行时存在或不存在的值的能力。这对于处理未明确标记为 `optional` 的 protobuf 字段、访问可能不存在的映射键或安全地索引列表特别有用。

**主要优势：**
- **空值安全操作**：避免访问潜在缺失数据时的运行时错误
- **表达性语法**：清晰区分存在和缺失的值
- **可组合操作**：在可选值上链式操作而无需显式空值检查
- **类型安全**：编译时感知可选值与必需值

**要求：**
- 需要 `RuntimeOptions.enable_qualified_type_identifiers = true`
- 需要 `RuntimeOptions.enable_heterogeneous_equality = true`

可选类型实现为 `optional(T)`，其中 `T` 是包含值的类型。

## 2. 语法

可选类型引入了用于选择、索引和消息/映射构造的新语法。

### 2.1 可选字段选择 (`.?`)

可选字段选择语法 `.?` 允许您访问可能缺失的消息字段。如果字段存在，它返回包含字段值的可选值。如果缺失，它返回 `optional.none()`。

**语义：** `has(msg.field) ? optional.of(msg.field) : optional.none()`

```cel
// 如果 msg.field 存在，结果是 optional(msg.field_value)。
// 否则，结果是 optional.none()。
let result = msg.?field;

// 适用于嵌套字段
let nested = msg.?user.?profile.?name;
```

**支持的类型：**
- Protocol Buffer 消息
- 映射值（用作结构时）
- 可选值（自动链式）

### 2.2 可选索引 (`[?_]`)

可选索引对映射和列表的工作方式类似。它提供了一种安全的方式来访问元素，而不会因越界索引或缺失键而出现错误。

#### 2.2.1 映射键访问

**语义：** `key in map ? optional.of(map[key]) : optional.none()`

```cel
// 如果 'my_key' 在 my_map 中，结果是 optional(my_map['my_key'])。
// 否则，结果是 optional.none()。
let result = my_map[?'my_key'];

// 适用于不同的键类型
let int_key = my_map[?42];
let bool_key = my_map[?true];

// 数值键的自动类型转换
let converted = my_map[?1];  // 尝试 int(1) 和 uint(1)
```

#### 2.2.2 列表索引访问

**语义：** `index >= 0 && index < list.size() ? optional.of(list[index]) : optional.none()`

```cel
// 如果索引 1 对 my_list 有效，结果是 optional(my_list[1])。
// 否则，结果是 optional.none()。
let result = my_list[?1];

// 负索引总是 optional.none()
let invalid = my_list[?-1];  // 总是 optional.none()
```

### 2.3 可选构造 (`{?key: ...}`)

可选构造语法允许您有条件地在消息或映射字面量中包含字段。只有当右侧的值表达式不是 `optional.none()` 时，条目才会被包含。

#### 2.3.1 消息字段构造

**语义：** `<expr>.hasValue() ? Msg{field: <expr>.value()} : Msg{}`

`<expr>` 必须是类型 `optional(T)`，其中 `T` 是消息字段的类型。

```cel
// 只有当 opt_age 有值时，`age` 字段才会在消息上设置。
MyMessage{
    name: 'John Doe',
    ?age: opt_age,
    ?email: user.?profile.?email
}
```

#### 2.3.2 映射条目构造

**语义：** `<expr>.hasValue() ? {key: <expr>.value()} : {}`

`<expr>` 必须是可选类型。

```cel
// 只有当 opt_age 不是 none 时，'age' 条目才会包含在映射中。
{
    'name': 'John Doe',
    ?'age': opt_age,
    ?'email': user.?profile.?email
}
```

#### 2.3.3 可选列表元素构造

列表字面量中的元素可以根据表达式是否评估为 `optional.none()` 来可选包含。

```cel
// 根据哪些可选值有值，创建包含 1-4 个元素的列表
[
    'always_included',
    ?opt_value1,
    ?opt_value2,
    ?opt_value3
]
```

### 2.4 链式行为

可选选择和索引操作是"病毒性的" - 一旦使用可选操作，后续操作自动变为可选：

```cel
// 这些是等价的：
obj.?field.subfield
obj.?field.?subfield

// 也是等价的：
list[?0].field
list[?0].?field

// 混合链式
obj.?field[?'key'].?subfield
```

## 3. 函数

可选类型附带了一套全面的实用函数来创建和交互可选值。

### 3.1 创建函数

#### 3.1.1 `optional.of(<value>)`

创建包含给定 `<value>` 的可选值。

**签名：** `optional.of(T) -> optional(T)`

```cel
let opt_name = optional.of('Alice');        // optional(string)
let opt_age = optional.of(30);              // optional(int)
let opt_list = optional.of([1, 2, 3]);      // optional(list(int))
```

#### 3.1.2 `optional.ofNonZeroValue(<value>)`

如果给定的值不是零值（默认空值），则创建包含该值的可选值。如果值是零值，返回 `optional.none()`。

**签名：** `optional.ofNonZeroValue(T) -> optional(T)`

**按类型的零值：**
- 数字：`0`、`0u`、`0.0`
- 字符串：`""`
- 字节：`b""`
- 列表：`[]`
- 映射：`{}`
- 消息：所有字段未设置
- 布尔值：`false`

```cel
optional.ofNonZeroValue([1, 2, 3])  // optional(list(int))
optional.ofNonZeroValue([])         // optional.none()
optional.ofNonZeroValue(0)          // optional.none()
optional.ofNonZeroValue("")         // optional.none()
optional.ofNonZeroValue("hello")    // optional(string)
optional.ofNonZeroValue(false)      // optional.none()
optional.ofNonZeroValue(true)       // optional(bool)
```

#### 3.1.3 `optional.none()`

创建空的可选值。

**签名：** `optional.none() -> optional(T)`

```cel
let opt_age = optional.none();  // 表示缺失值
```

### 3.2 访问函数

#### 3.2.1 `<optional>.hasValue()`

如果可选值包含值，返回 `true`，否则返回 `false`。

**签名：** `optional(T).hasValue() -> bool`

```cel
optional.of('Alice').hasValue()     // true
optional.none().hasValue()          // false
optional.ofNonZeroValue("").hasValue()  // false
```

#### 3.2.2 `<optional>.value()`

返回可选值中包含的值。如果可选值没有值，结果将是 CEL 错误。

**签名：** `optional(T).value() -> T`

```cel
optional.of('Alice').value()        // 'Alice'
optional.none().value()             // 错误：可选值没有值
```

**⚠️ 警告：** 如果在空可选值上调用此函数，将导致运行时错误。使用 `hasValue()` 先检查，或优先使用 `orValue()` 进行更安全的访问。

### 3.3 链式函数

#### 3.3.1 `<optional>.or(<optional>)`

如果左侧可选值为空（`optional.none()`），返回右侧可选值。如果左侧有值，返回它。此操作是短路的。

**签名：** `optional(T).or(optional(T)) -> optional(T)`

```cel
optional.of('Alice').or(optional.of('Bob'))     // optional('Alice')
optional.none().or(optional.of('Bob'))          // optional('Bob')
optional.none().or(optional.none())             // optional.none()

// 链式多个备选
user.?name.or(user.?nickname).or(optional.of('Anonymous'))
```

#### 3.3.2 `<optional>.orValue(<default>)`

如果可选值存在，返回其包含的值；否则，返回 `<default>` 值。

**签名：** `optional(T).orValue(T) -> T`

```cel
optional.of('Alice').orValue('Guest')   // 'Alice'
optional.none().orValue('Guest')        // 'Guest'

// 提供默认值的常见模式
let display_name = user.?profile.?name.orValue('Anonymous');
```

### 3.4 转换函数

#### 3.4.1 `<optional>.optMap(<var>, <expr>)`

如果可选值不为空，对其底层值应用转换，并基于转换返回可选类型的结果。转换表达式类型必须返回类型 `T`，它会自动包装到 `optional(T)` 中。

**签名：** `optional(A).optMap(var, expr) -> optional(B)`，其中 `expr: A -> B`

```cel
// 如果存在则转换值
optional.of("hello").optMap(s, s.size())                    // optional(5)
optional.none().optMap(s, s.size())                         // optional.none()

// 链式转换
optional.of([1, 2, 3])
    .optMap(list, list.size())
    .optMap(size, size * 2)                                 // optional(6)

// 实际示例
user.?profile.?name.optMap(name, name.upperAscii()).orValue("UNKNOWN")
```

#### 3.4.2 `<optional>.optFlatMap(<var>, <expr>)`

如果可选值不为空，对其底层值应用转换并返回结果。转换表达式必须返回 `optional(T)` 而不是类型 `T`。当转换本身可能失败或返回空结果时，这很有用。

**签名：** `optional(A).optFlatMap(var, expr) -> optional(B)`，其中 `expr: A -> optional(B)`

```cel
// 使用另一个可选操作进行转换
optional.of([1, 2, 3])
    .optFlatMap(list, list[?0])                             // optional(1)

optional.of([])
    .optFlatMap(list, list[?0])                             // optional.none()

// 使用条件逻辑进行链式
optional.of("user@example.com")
    .optFlatMap(email, email.contains("@") ? 
        optional.of(email.split("@")[0]) : 
        optional.none())                                     // optional("user")
```

### 3.5 实用函数

#### 3.5.1 `optional.unwrap(<list>)`

接受可选值列表并返回仅包含非空可选值的新列表。空可选值被过滤掉。

**签名：** `optional.unwrap(list(optional(T))) -> list(T)`

```cel
optional.unwrap([
    optional.of(1),
    optional.none(),
    optional.of(3),
    optional.none(),
    optional.of(5)
])  // [1, 3, 5]

// 也可作为方法使用
[
    optional.of("a"),
    optional.none(),
    optional.of("c")
].unwrapOpt()  // ["a", "c"]
```

## 4. 类型系统

### 4.1 可选类型声明

可选类型表示为 `optional(T)`，其中 `T` 是包装的类型。类型系统理解可选类型并提供适当的类型检查。

```cel
// 类型注解（概念性）
optional.of(42)         // optional(int)
optional.of("hello")    // optional(string)
optional.of([1, 2, 3])  // optional(list(int))
optional.none()         // optional(dyn) - 可以是任何可选类型
```

### 4.2 类型兼容性

可选类型遵循以下兼容性规则：

- `optional(T)` 不能直接赋值给 `T`
- `T` 可以使用 `optional.of()` 包装到 `optional(T)` 中
- 可选操作通过链保持类型信息
- 类型检查确保可选构造语法接收 `optional(T)` 值

## 5. 示例

### 5.1 基本用法

```cel
// 安全字段访问
let user_email = request.?user.?email.orValue("no-email@example.com");

// 安全列表索引
let first_item = items[?0].orValue("default");

// 安全映射访问
let config_value = config[?"timeout"].orValue(30);
```

### 5.2 链式操作

```cel
// 复杂的链式转换
request.?user.?profile.?preferences.?theme
    .optMap(theme, theme.lowerAscii())
    .or(optional.of("default"))
    .value()

// 多个备选
primary_config[?"setting"]
    .or(fallback_config[?"setting"])
    .or(optional.of("default_value"))
    .value()
```

### 5.3 条件构造

```cel
// 带可选字段的消息构造
UserProfile{
    name: "John Doe",
    ?email: request.?user.?email,
    ?age: request.?user.?age,
    ?avatar_url: request.?user.?profile.?avatar
}

// 带可选条目的映射构造
{
    "name": "John Doe",
    ?"email": request.?user.?email,
    ?"preferences": request.?user.?preferences
}

// 带可选元素的列表
[
    "required_item",
    ?optional_item1,
    ?optional_item2,
    "another_required_item"
]
```

### 5.4 数据处理

```cel
// 处理可选值列表
let valid_emails = users
    .map(user, user.?email)
    .filter(opt_email, opt_email.hasValue())
    .map(opt_email, opt_email.value());

// 或使用 unwrap
let valid_emails = optional.unwrap(users.map(user, user.?email));

// 转换并提供默认值
let display_names = users.map(user, 
    user.?profile.?display_name.orValue(user.?name.orValue("Anonymous"))
);
```

### 5.5 错误处理

```cel
// 无错误的安全访问
let result = data.?results[?0].?value.orValue("no data");

// 使用可选类型进行验证
let is_valid = request.?user.?email
    .optMap(email, email.matches(r'^[^@]+@[^@]+\.[^@]+$'))
    .orValue(false);

// 条件处理
let processed = input.?data.hasValue() ? 
    processData(input.data.value()) : 
    "no data to process";
```

## 6. 最佳实践

1. **优先使用 `orValue()` 而不是 `value()`**：使用 `orValue()` 提供默认值，而不是冒 `value()` 的运行时错误风险。

2. **使用 `ofNonZeroValue()` 进行验证**：当您想将空/零值视为缺失时。

3. **高效地链式操作**：利用可选操作的病毒性质来避免显式空值检查。

4. **提供有意义的默认值**：始终考虑在您的上下文中什么默认值是有意义的。

5. **使用可选构造来编写清晰代码**：利用可选构造语法来避免复杂的条件逻辑。

6. **记录可选行为**：明确说明函数或数据结构何时可能返回可选值。

## 7. 迁移指南

### 从显式空值检查：

```cel
// 之前
has(request.user) && has(request.user.email) ? request.user.email : "default"

// 之后
request.?user.?email.orValue("default")
```

### 从容易出错的索引：

```cel
// 之前（可能导致运行时错误）
items.size() > 0 ? items[0] : default_item

// 之后
items[?0].orValue(default_item)
```

### 从复杂的条件构造：

```cel
// 之前
has(user.email) ? UserProfile{name: user.name, email: user.email} : UserProfile{name: user.name}

// 之后
UserProfile{
    name: user.name,
    ?email: user.?email
}
``` 