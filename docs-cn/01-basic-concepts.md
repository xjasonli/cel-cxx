# CEL 基本概念和语法

## 目录

- [CEL 基本概念和语法](#cel-基本概念和语法)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. 语法](#2-语法)
    - [2.1 语法规则](#21-语法规则)
    - [2.2 操作符优先级](#22-操作符优先级)
    - [2.3 词法规则](#23-词法规则)
      - [标识符](#标识符)
      - [注释](#注释)
      - [空白字符](#空白字符)
    - [2.4 保留字](#24-保留字)
  - [3. 类型系统](#3-类型系统)
    - [3.1 基本类型](#31-基本类型)
      - [`bool`](#bool)
      - [`int`](#int)
      - [`uint`](#uint)
      - [`double`](#double)
      - [`string`](#string)
      - [`bytes`](#bytes)
      - [`timestamp`](#timestamp)
    - [3.2 复合类型](#32-复合类型)
      - [`list`](#list)
      - [`map`](#map)
      - [`message`](#message)
    - [3.3 特殊类型](#33-特殊类型)
      - [`null`](#null)
      - [`type`](#type)
  - [4. 字面量](#4-字面量)
    - [4.1 数值字面量](#41-数值字面量)
      - [整数字面量](#整数字面量)
      - [无符号整数字面量](#无符号整数字面量)
      - [浮点数字面量](#浮点数字面量)
    - [4.2 字符串字面量](#42-字符串字面量)
      - [普通字符串](#普通字符串)
      - [三引号字符串](#三引号字符串)
      - [原始字符串](#原始字符串)
      - [转义序列](#转义序列)
    - [4.3 字节字面量](#43-字节字面量)
    - [4.4 布尔值和空值字面量](#44-布尔值和空值字面量)
  - [5. 表达式](#5-表达式)
    - [5.1 基本表达式](#51-基本表达式)
      - [算术操作](#算术操作)
      - [比较操作](#比较操作)
      - [逻辑操作](#逻辑操作)
    - [5.2 条件表达式](#52-条件表达式)
    - [5.3 集合表达式](#53-集合表达式)
      - [列表操作](#列表操作)
      - [映射操作](#映射操作)
    - [5.4 访问表达式](#54-访问表达式)
      - [字段访问](#字段访问)
      - [索引访问](#索引访问)
      - [函数调用](#函数调用)
  - [6. 名称解析](#6-名称解析)
    - [6.1 解析规则](#61-解析规则)
    - [6.2 绝对名称](#62-绝对名称)
    - [6.3 名称类型](#63-名称类型)
      - [变量名称](#变量名称)
      - [函数名称](#函数名称)
      - [类型名称](#类型名称)
  - [7. 示例](#7-示例)
    - [7.1 简单表达式](#71-简单表达式)
    - [7.2 复杂表达式](#72-复杂表达式)
    - [7.3 实际应用示例](#73-实际应用示例)
      - [访问控制](#访问控制)
      - [数据验证](#数据验证)
      - [资源过滤](#资源过滤)

## 1. 概述

通用表达式语言（CEL）是一种基于 Protocol Buffer 类型构建的简单表达式语言。CEL 具有以下特性：

- **内存安全**：程序无法访问无关的内存
- **无副作用**：CEL 程序只从输入计算输出
- **终止性**：CEL 程序不能无限循环
- **强类型**：值具有明确定义的类型，操作符和函数检查参数类型
- **动态类型**：类型与值关联，而不是与变量或表达式关联
- **渐进类型**：可选的类型检查阶段可以在运行时之前检测并拒绝违反类型约束的程序

## 2. 语法

### 2.1 语法规则

CEL 的语法使用 EBNF 表示法定义，其中 `|` 表示选择，`[]` 表示可选元素，`{}` 表示重复元素，`()` 表示分组：

```grammar
Expr           = ConditionalOr ["?" ConditionalOr ":" Expr] ;
ConditionalOr  = [ConditionalOr "||"] ConditionalAnd ;
ConditionalAnd = [ConditionalAnd "&&"] Relation ;
Relation       = [Relation Relop] Addition ;
Relop          = "<" | "<=" | ">=" | ">" | "==" | "!=" | "in" ;
Addition       = [Addition ("+" | "-")] Multiplication ;
Multiplication = [Multiplication ("*" | "/" | "%")] Unary ;
Unary          = Member
               | "!" {"!"} Member
               | "-" {"-"} Member
               ;
Member         = Primary
               | Member "." SELECTOR ["(" [ExprList] ")"]
               | Member "[" Expr "]"
               ;
Primary        = ["."] IDENT ["(" [ExprList] ")"]
               | "(" Expr ")"
               | "[" [ExprList] [","] "]"
               | "{" [MapInits] [","] "}"
               | ["."] SELECTOR { "." SELECTOR } "{" [FieldInits] [","] "}"
               | LITERAL
               ;
ExprList       = Expr {"," Expr} ;
FieldInits     = SELECTOR ":" Expr {"," SELECTOR ":" Expr} ;
MapInits       = Expr ":" Expr {"," Expr ":" Expr} ;
```

**实现要求：**

CEL 实现至少需要支持：
- **24-32 次重复**的重复规则：
  - 连续 32 个由 `||` 分隔的项
  - 连续 32 个由 `&&` 分隔的项
  - 32 个函数调用参数
  - 包含 32 个元素的列表字面量
  - 包含 32 个字段的映射或消息字面量
  - 24 个连续的三元条件运算符 `?:`
  - 24 个相同优先级的二元算术运算符
  - 24 个连续的关系运算符
- **12 次重复**的递归规则：
  - 12 个嵌套函数调用
  - 12 个连续的选择（`.`）操作符
  - 12 个连续的索引（`[_]`）操作符
  - 12 个嵌套的列表、映射或消息字面量

### 2.2 操作符优先级

CEL 操作符按优先级从高到低列出：

| 优先级 | 操作符              | 描述                     | 结合性     |
|--------|---------------------|--------------------------|------------|
| 1      | ()                  | 函数调用                 | 左到右     |
|        | .                   | 限定名称或字段访问       |            |
|        | []                  | 索引                     |            |
|        | {}                  | 字段初始化               |            |
| 2      | - (一元)            | 取负                     | 右到左     |
|        | !                   | 逻辑非                   |            |
| 3      | *                   | 乘法                     | 左到右     |
|        | /                   | 除法                     |            |
|        | %                   | 取余                     |            |
| 4      | +                   | 加法                     | 左到右     |
|        | - (二元)            | 减法                     |            |
| 5      | == != < > <= >=     | 关系运算                 | 左到右     |
|        | in                  | 包含测试                 |            |
| 6      | &&                  | 逻辑与                   | 左到右     |
| 7      | \|\|                | 逻辑或                   | 左到右     |
| 8      | ?:                  | 条件运算                 | 右到左     |

### 2.3 词法规则

CEL 的词法结构定义如下：

```
IDENT          ::= SELECTOR - RESERVED
SELECTOR       ::= [_a-zA-Z][_a-zA-Z0-9]* - KEYWORD
LITERAL        ::= INT_LIT | UINT_LIT | FLOAT_LIT | STRING_LIT | BYTES_LIT
                 | BOOL_LIT | NULL_LIT
INT_LIT        ::= -? DIGIT+ | -? 0x HEXDIGIT+
UINT_LIT       ::= INT_LIT [uU]
FLOAT_LIT      ::= -? DIGIT* . DIGIT+ EXPONENT? | -? DIGIT+ EXPONENT
DIGIT          ::= [0-9]
HEXDIGIT       ::= [0-9abcdefABCDEF]
EXPONENT       ::= [eE] [+-]? DIGIT+
STRING_LIT     ::= [rR]? ( "    ~( " | \r | \n )*  "
                         | '    ~( ' | \r | \n )*  '
                         | """  ~"""*              """
                         | '''  ~'''*              '''
                         )
BYTES_LIT      ::= [bB] STRING_LIT
BOOL_LIT       ::= "true" | "false"
NULL_LIT       ::= "null"
KEYWORD        ::= BOOL_LIT | NULL_LIT | "in"
RESERVED       ::= "as" | "break" | "const" | "continue" | "else"
                 | "for" | "function" | "if" | "import" | "let"
                 | "loop" | "package" | "namespace" | "return"
                 | "var" | "void" | "while"
WHITESPACE     ::= [\t\n\f\r ]+
COMMENT        ::= '//' ~\n* \n
```

#### 标识符
- 必须以字母（`a-z`、`A-Z`）或下划线（`_`）开头
- 可以包含字母、数字（`0-9`）和下划线
- 区分大小写
- 不能是保留字

#### 注释
- 单行注释以 `//` 开头，持续到行尾
- 标准中不支持多行注释

#### 空白字符
- 空格、制表符、换行符、换页符和回车符会被忽略，除非用于分隔标记

### 2.4 保留字

**语言关键字**（不能用作标识符、函数名、选择器、结构名段或字段名）：
- `false`, `in`, `null`, `true`

**宿主语言保留字**（不能用作标识符或函数名，但允许用于接收者调用风格的函数）：
- `as`, `break`, `const`, `continue`, `else`, `for`, `function`, `if`, `import`, `let`, `loop`, `package`, `namespace`, `return`, `var`, `void`, `while`

**注意：** 通常建议避免使用在可能嵌入 CEL 的编程语言中是保留字的标识符。

## 3. 类型系统

### 3.1 基本类型

CEL 支持以下基本类型：

#### `bool`
布尔值：`true` 或 `false`

```cel
true && false  // false
!true          // false
```

#### `int`
64 位有符号整数

```cel
42
-17
0
```

#### `uint`
64 位无符号整数

```cel
42u
0u
```

#### `double`
64 位 IEEE 754 浮点数

```cel
3.14
-2.5
1e10
```

#### `string`
UTF-8 编码的文本

```cel
"hello world"
'single quotes'
"unicode: \u0048\u0065\u006c\u006c\u006f"
```

#### `bytes`
字节序列

```cel
b"hello"
b'\x48\x65\x6c\x6c\x6f'
```

#### `timestamp`
表示时间点（实现为 `google.protobuf.Timestamp`）

```cel
timestamp("2023-12-25T12:00:00Z")
timestamp("2023-01-01T00:00:00-08:00")  // 带时区
```

**属性：**
- 范围：["0001-01-01T00:00:00Z", "9999-12-31T23:59:59.999999999Z"]
- 精度：纳秒
- 时区感知

**常见操作：**
```cel
// 与持续时间的算术运算
timestamp("2023-01-01T00:00:00Z") + duration("1h")  // 添加持续时间
timestamp("2023-01-01T01:00:00Z") - duration("1h")  // 减去持续时间
timestamp("2023-01-01T01:00:00Z") - timestamp("2023-01-01T00:00:00Z")  // 时间戳之间的持续时间

// 比较
timestamp("2023-01-01T00:00:00Z") < timestamp("2023-01-02T00:00:00Z")  // true

// 字段访问
timestamp("2023-12-25T12:30:45Z").getFullYear()  // 2023
timestamp("2023-12-25T12:30:45Z").getMonth()     // 11 (十二月，从 0 开始)
timestamp("2023-12-25T12:30:45Z").getDate()      // 25
```

表示时间跨度（实现为 `google.protobuf.Duration`）

```cel
duration("1h")
duration("30m")
duration("1h30m45s")
duration("1.5s")
```

**属性：**
- 范围：大约 ±290 年
- 精度：纳秒
- 可以为负值

**支持的单位：**
- `h`（小时）
- `m`（分钟）
- `s`（秒）
- `ms`（毫秒）
- `us`（微秒）
- `ns`（纳秒）

**常见操作：**
```cel
// 算术运算
duration("1h") + duration("30m")     // duration("1h30m")
duration("2h") - duration("30m")     // duration("1h30m")

// 比较
duration("1h") < duration("2h")      // true
duration("60m") == duration("1h")    // true

// 转换
duration("1h30m").getHours()         // 1（截断）
duration("1h30m").getMinutes()       // 90（总分钟数）
duration("1h30m").getSeconds()       // 5400（总秒数）
```

### 3.2 复合类型

#### `list`
有序的值集合

```cel
[1, 2, 3]
["a", "b", "c"]
[]  // 空列表
```

#### `map`
键值关联，键可以是 `int`、`uint`、`bool` 或 `string` 类型

```cel
{"key": "value", "number": 42}
{1: "one", 2: "two"}        // int 键
{true: "yes", false: "no"}  // bool 键
{}  // 空映射
```

#### `message`
Protocol Buffer 消息类型

```cel
MyMessage{
  field1: "value",
  field2: 42
}
```

### 3.3 特殊类型

#### `null`
表示值的缺失（类型：`null_type`）

```cel
null
```

#### `type`
将类型本身表示为一等值

```cel
type(42)           // int
type("hello")      // string
type([1, 2, 3])    // list(int)
type({"a": 1})     // map(string, int)

// 类型比较
type(x) == int
type(y) == string
type(z) == list(int)
```

**类型层次结构：**
- 基本类型：`int`、`uint`、`double`、`bool`、`string`、`bytes`
- 时间类型：`timestamp`、`duration`
- 容器类型：`list(T)`、`map(K, V)`
- 特殊类型：`null_type`、`type`
- 消息类型：Protocol Buffer 消息名称

## 4. 字面量

### 4.1 数值字面量

#### 整数字面量
```cel
42          // 十进制
0x2A        // 十六进制
0o52        // 八进制
0b101010    // 二进制
```

#### 无符号整数字面量
```cel
42u         // 十进制无符号
0x2Au       // 十六进制无符号
```

#### 浮点数字面量
```cel
3.14
.5
2.
1e10
1.5e-3
```

### 4.2 字符串字面量

CEL 支持几种类型的字符串字面量：

#### 普通字符串
引号字符串字面量由单引号或双引号字符分隔。结束分隔符必须与开始分隔符匹配，可以包含除分隔符或换行符（CR 或 LF）之外的任何未转义字符。

```cel
"hello world"
'single quotes'
"mixed 'quotes' work"
'mixed "quotes" work'
```

#### 三引号字符串
三引号字符串字面量由三个单引号或三个双引号分隔，可以包含除分隔符序列之外的任何未转义字符。三引号字符串可以包含换行符。

```cel
"""
多行字符串
包含换行符
"""

'''
另一种多行
字符串格式
'''
```

#### 原始字符串
如果前面有 `r` 或 `R` 字符，该字符串是_原始_字符串，不解释转义序列。原始字符串对于表达本身必须使用转义序列的字符串很有用，例如正则表达式或程序文本。

```cel
r"raw string with \n literal backslashes"
R'another raw string \t with literal escapes'
```

#### 转义序列
字符串（除原始字符串外）可以包含转义序列，即反斜杠（`\`）后跟以下之一：

**表示自身的标点符号：**
- `\\`：反斜杠
- `\?`：问号
- `\"`：双引号
- `\'`：单引号
- `\``：反引号

**空白字符代码：**
- `\a`：响铃
- `\b`：退格
- `\f`：换页
- `\n`：换行（新行）
- `\r`：回车
- `\t`：水平制表符
- `\v`：垂直制表符

**Unicode 转义：**
- `\uXXXX`：BMP 中的 Unicode 码点（4 个十六进制数字）
- `\UXXXXXXXX`：任何平面中的 Unicode 码点（8 个十六进制数字）
- `\xXX` 或 `\XXX`：Unicode 码点（2 个十六进制数字）
- `\000` 到 `\377`：Unicode 码点（3 个八进制数字）

转义序列中的所有十六进制数字都不区分大小写。

**示例：**

| CEL 字面量 | 含义 |
|------------|------|
| `""` | 空字符串 |
| `'""'` | 包含两个双引号字符的字符串 |
| `'''x''x'''` | 包含四个字符 "`x''x`" 的字符串 |
| `"\""` | 包含一个双引号字符的字符串 |
| `"\\"` | 包含一个反斜杠字符的字符串 |
| `r"\\"` | 包含两个反斜杠字符的字符串 |
| `"line1\nline2"` | 两行字符串 |
| `"tab\there"` | 包含制表符的字符串 |
| `"unicode: \u0048"` | 包含 Unicode 'H' 的字符串 |

### 4.3 字节字面量

字节字面量由前面带有 `b` 或 `B` 字符的字符串字面量表示。字节字面量是字符串字面量的 UTF-8 表示形式给出的字节序列。此外，八进制转义序列被解释为八位字节值而不是 Unicode 码点。原始字符串和多行字符串字面量都可以用于字节字面量。

```cel
b"hello"                    // 字节序列 97, 98, 99, 100, 101
b"ÿ"                        // 字节序列 195 和 191（ÿ 的 UTF-8）
b"\303\277"                 // 也是字节序列 195 和 191
b"\377"                     // 字节序列 255（不是 ÿ 的 UTF-8）
b"\xff"                     // 字节序列 255（不是 ÿ 的 UTF-8）
B'raw bytes with \n'        // 原始字节字面量
```

**注意：** 对于字符串，`\377` 表示 Unicode 码点 255，但对于字节，`b"\377"` 表示字节值 255。

### 4.4 布尔值和空值字面量

```cel
true
false
null
```

## 5. 表达式

### 5.1 基本表达式

#### 算术操作
```cel
1 + 2           // 3
5 - 3           // 2
4 * 2           // 8
10 / 3          // 3
10 % 3          // 1
```

#### 比较操作
```cel
5 > 3           // true
5 >= 5          // true
3 < 5           // true
3 <= 3          // true
5 == 5          // true
5 != 3          // true
```

#### 逻辑操作
```cel
true && false   // false
true || false   // true
!true           // false
```

### 5.2 条件表达式

```cel
condition ? value_if_true : value_if_false
```

示例：
```cel
5 > 3 ? "yes" : "no"        // "yes"
x > 0 ? x : -x              // 绝对值
```

### 5.3 集合表达式

#### 列表操作
```cel
[1, 2, 3].size()           // 3
[1, 2, 3][0]               // 1
1 in [1, 2, 3]             // true
```

#### 映射操作
```cel
{"a": 1, "b": 2}.size()    // 2
{"a": 1, "b": 2}["a"]      // 1
"a" in {"a": 1, "b": 2}    // true
```

### 5.4 访问表达式

#### 字段访问
```cel
message.field
message.nested.field
```

#### 索引访问
```cel
list[0]
map["key"]
```

#### 函数调用
```cel
size(list)
substring("hello", 1, 3)
```

## 6. 名称解析

CEL 表达式在特定 protocol buffer 包或消息的范围内解析，这控制了名称的解释。范围由表达式的应用程序上下文设置。CEL 表达式可以包含简单名称（例如 `a`）或限定名称（例如 `a.b`）。

### 6.1 解析规则

CEL 表达式中名称的含义由以下一个或多个确定：

- **变量和函数**：简单名称指向执行上下文中的变量、标准函数或 CEL 应用程序提供的其他名称绑定
- **字段选择**：在表达式后附加句点和标识符表示访问 protocol buffer 或映射中的字段
- **Protocol buffer 包名称**：简单或限定名称可以表示 protocol buffer 包命名空间中的绝对或相对名称。包名称后必须跟随消息类型或枚举常量
- **Protocol buffer 消息类型和枚举常量**：在可选的 protocol buffer 包名称之后，简单或限定名称可以指向包命名空间中的消息类型或枚举常量

**解析过程：**

如果 `a.b` 是要在具有范围 `A.B` 的 protobuf 声明上下文中解析的名称，则按以下顺序尝试解析：
1. `A.B.a.b`
2. `A.a.b`
3. `a.b`

### 6.2 绝对名称

要覆盖默认解析行为，您可以通过在前面加点（`.`）来使用绝对名称。例如，`.a.b` 将仅在根范围中解析为 `a.b`。

### 6.3 名称类型

**优先级规则**：如果名称限定与字段选择混合，则使用在当前词法范围中解析的名称的最长前缀。例如，如果 `a.b.c` 解析为消息声明，而 `a.b` 也解析且 `c` 作为可能的字段选择，则 `a.b.c` 优先于解释 `(a.b).c`。

#### 变量名称
变量在执行环境中绑定，可以通过其简单名称引用：

```cel
user.name               // 访问 'user' 变量的 'name' 字段
request.auth.token      // 嵌套字段访问
```

#### 函数名称
函数可以用两种风格调用：

**全局函数风格：**
```cel
size(list)              // 全局函数调用
contains(string, substring)
```

**接收者函数风格：**
```cel
list.size()             // 接收者风格函数调用
string.contains(substring)
```

#### 类型名称
类型名称用于类型转换和消息构造：

```cel
int(value)              // 类型转换
string(42)              // 转换为字符串
MyMessage{field: value} // 消息构造
```

## 7. 示例

### 7.1 简单表达式

```cel
// 基本算术
2 + 3 * 4                   // 14

// 字符串操作
"Hello, " + "World!"        // "Hello, World!"

// 布尔逻辑
true && (false || true)     // true

// 比较
5 > 3 && 2 < 4             // true
```

### 7.2 复杂表达式

```cel
// 带复杂条件的条件表达式
user.age >= 18 && user.status == "active" ? "allowed" : "denied"

// 类似列表推导的操作
[1, 2, 3, 4, 5].filter(x, x > 2).map(x, x * 2)  // [6, 8, 10]

// 映射操作
{"users": [{"name": "Alice"}, {"name": "Bob"}]}.users.map(u, u.name)
```

### 7.3 实际应用示例

#### 访问控制
```cel
// 检查用户是否具有所需角色
user.role in ["admin", "moderator"] && resource.public == false

// 基于时间的访问
now() < resource.expiry_time && user.subscription.active
```

#### 数据验证
```cel
// 电子邮件验证
email.matches(r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$')

// 年龄验证
user.age >= 13 && user.age <= 120
```

#### 资源过滤
```cel
// 基于用户权限的过滤
resource.owner == user.id || 
(resource.shared && user.id in resource.allowed_users)

// 内容过滤
content.tags.exists(tag, tag in user.interests) && 
content.mature_content == false
``` 