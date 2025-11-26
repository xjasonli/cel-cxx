# Strings 扩展

Strings 扩展提供了超越标准库基本字符串操作的高级字符串操作函数。

## 目录

- [Strings 扩展](#strings-扩展)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. 字符访问 - `charAt()`](#2-字符访问---charat)
    - [charAt()](#charat)
  - [3. 字符串搜索](#3-字符串搜索)
    - [3.1 indexOf()](#31-indexof)
    - [3.2 lastIndexOf()](#32-lastindexof)
  - [4. 字符串提取 - `substring()`](#4-字符串提取---substring)
    - [substring()](#substring)
  - [5. 字符串引用 - `strings.quote()`](#5-字符串引用---stringsquote)
    - [strings.quote()](#stringsquote)
  - [6. 字符串修剪 - `trim()`](#6-字符串修剪---trim)
    - [trim()](#trim)
  - [7. 字符串连接 - `join()`](#7-字符串连接---join)
    - [join()](#join)
  - [8. 字符串分割 - `split()`](#8-字符串分割---split)
    - [split()](#split)
  - [9. 大小写转换](#9-大小写转换)
    - [9.1 lowerAscii()](#91-lowerascii)
    - [9.2 upperAscii()](#92-upperascii)
  - [10. 字符串替换 - `replace()`](#10-字符串替换---replace)
    - [replace()](#replace)
  - [11. 字符串格式化 - `format()`](#11-字符串格式化---format)
    - [format()](#format)
  - [12. 字符串反转 - `reverse()`](#12-字符串反转---reverse)
    - [reverse()](#reverse)
  - [13. 使用示例](#13-使用示例)
    - [电子邮件处理](#电子邮件处理)
    - [CSV 数据处理](#csv-数据处理)
    - [文本规范化](#文本规范化)
    - [字符串验证](#字符串验证)
    - [模板处理](#模板处理)

## 1. 概述

Strings 扩展通过强大的字符串操作功能增强了 CEL。所有函数都设计为安全、高效，并与 CEL 确定性、无副作用评估的哲学保持一致。

**启用扩展**：
```rust
let env = Env::builder()
    .with_ext_strings(true)
    .build()?;
```

## 2. 字符访问 - `charAt()`

### charAt()

获取特定索引处的字符。

**语法：** `string.charAt(index)`

**参数：**
- `string`：输入字符串
- `index`：要检索字符的从零开始的索引

**返回类型：** `string`（单个字符）

**示例：**
```cel
"Hello".charAt(0)           // "H"
"World".charAt(4)           // "d"
"CEL".charAt(1)             // "E"
"Programming".charAt(7)     // "m"
```

**错误处理：**
- 对于越界索引返回空字符串
- 负索引被视为越界

## 3. 字符串搜索

### 3.1 indexOf()

查找子字符串的第一次出现。

**语法：** 
- `string.indexOf(substring)`
- `string.indexOf(substring, startIndex)`

**参数：**
- `string`：要搜索的输入字符串
- `substring`：要查找的子字符串
- `startIndex`：可选的搜索起始位置

**返回类型：** `int`（第一次出现的索引，如果未找到则返回 -1）

**示例：**
```cel
"hello world".indexOf("o")        // 4
"hello world".indexOf("world")    // 6
"hello world".indexOf("o", 5)     // 7
"hello world".indexOf("xyz")      // -1
```

### 3.2 lastIndexOf()

查找子字符串的最后一次出现。

**语法：** 
- `string.lastIndexOf(substring)`
- `string.lastIndexOf(substring, startIndex)`

**参数：**
- `string`：要搜索的输入字符串
- `substring`：要查找的子字符串
- `startIndex`：可选的搜索起始位置（向前搜索）

**返回类型：** `int`（最后一次出现的索引，如果未找到则返回 -1）

**示例：**
```cel
"hello world".lastIndexOf("o")    // 7
"hello world".lastIndexOf("l")    // 9
"hello world".lastIndexOf("x")    // -1
```

## 4. 字符串提取 - `substring()`

### substring()

提取字符串的一部分。

**语法：**
- `string.substring(start)`
- `string.substring(start, end)`

**参数：**
- `string`：输入字符串
- `start`：起始索引（包含）
- `end`：结束索引（不包含，可选）

**返回类型：** `string`

**示例：**
```cel
"Hello World".substring(0, 5)     // "Hello"
"Hello World".substring(6)        // "World"
"Programming".substring(0, 7)     // "Program"
"CEL Language".substring(4)       // "Language"
```

**行为：**
- 如果省略 `end`，则提取到字符串末尾
- 越界索引会被限制在有效范围内
- 如果 `start >= end`，返回空字符串

## 5. 字符串引用 - `strings.quote()`

### strings.quote()

转义字符串中的特殊字符。

**语法：** `strings.quote(string)`

**参数：**
- `string`：要引用的输入字符串

**返回类型：** `string`（转义后的字符串）

**示例：**
```cel
strings.quote("Hello World")       // "Hello World"
strings.quote("Line 1\nLine 2")    // "Line 1\\nLine 2"
strings.quote("Tab\there")         // "Tab\\there"
strings.quote("Quote: \"Hello\"")  // "Quote: \\\"Hello\\\""
strings.quote("Backslash: \\")     // "Backslash: \\\\"
strings.quote("")                  // ""
```

**转义字符：**
- `\n` → `\\n`（换行）
- `\r` → `\\r`（回车）
- `\t` → `\\t`（制表符）
- `\\` → `\\\\`（反斜杠）
- `\"` → `\\\"`（双引号）

## 6. 字符串修剪 - `trim()`

### trim()

从字符串两端移除空白字符。

**语法：** `string.trim()`

**参数：**
- `string`：输入字符串

**返回类型：** `string`

**示例：**
```cel
"  hello  ".trim()          // "hello"
"\t\nworld\r\n".trim()      // "world"
"   ".trim()                // ""
"NoSpaces".trim()           // "NoSpaces"
"  Leading".trim()          // "Leading"
"Trailing  ".trim()         // "Trailing"
```

**空白字符：**
- 空格（` `）
- 制表符（`\t`）
- 换行符（`\n`）
- 回车符（`\r`）
- 换页符（`\f`）
- 垂直制表符（`\v`）

## 7. 字符串连接 - `join()`

### join()

使用分隔符连接字符串列表。

**语法：**
- `list.join()`（无分隔符）
- `list.join(separator)`

**参数：**
- `list`：要连接的字符串列表
- `separator`：可选分隔符字符串

**返回类型：** `string`

**示例：**
```cel
["a", "b", "c"].join()              // "abc"
["a", "b", "c"].join(", ")          // "a, b, c"
["hello", "world"].join(" ")        // "hello world"
["one", "two", "three"].join(" | ") // "one | two | three"
["single"].join(",")                // "single"
[].join(",")                        // ""
```

## 8. 字符串分割 - `split()`

### split()

将字符串分割为子字符串列表。

**语法：**
- `string.split(separator)`
- `string.split(separator, limit)`

**参数：**
- `string`：要分割的输入字符串
- `separator`：分隔符字符串
- `limit`：可选的最大分割次数

**返回类型：** `list(string)`

**示例：**
```cel
"a,b,c".split(",")                  // ["a", "b", "c"]
"a,b,c,d".split(",", 2)             // ["a", "b", "c,d"]
"hello world".split(" ")            // ["hello", "world"]
"a::b::c".split("::")               // ["a", "b", "c"]
"no-separators".split(",")          // ["no-separators"]
"".split(",")                       // [""]
```

**行为：**
- 如果指定了 `limit`，最多执行 `limit` 次分割
- 分隔符之间的空字符串会被保留
- 如果找不到分隔符，返回包含原字符串的列表

## 9. 大小写转换

### 9.1 lowerAscii()

将 ASCII 字符转换为小写。

**语法：** `string.lowerAscii()`

**参数：**
- `string`：输入字符串

**返回类型：** `string`

**示例：**
```cel
"HELLO".lowerAscii()        // "hello"
"Hello World".lowerAscii()  // "hello world"
"MiXeD CaSe".lowerAscii()   // "mixed case"
"123ABC".lowerAscii()       // "123abc"
"already lowercase".lowerAscii() // "already lowercase"
```

### 9.2 upperAscii()

将 ASCII 字符转换为大写。

**语法：** `string.upperAscii()`

**参数：**
- `string`：输入字符串

**返回类型：** `string`

**示例：**
```cel
"hello".upperAscii()        // "HELLO"
"Hello World".upperAscii()  // "HELLO WORLD"
"123abc".upperAscii()       // "123ABC"
"ALREADY UPPERCASE".upperAscii() // "ALREADY UPPERCASE"
```

**注意：** 只有 ASCII 字符（A-Z、a-z）会被转换。Unicode 字符保持不变。

## 10. 字符串替换 - `replace()`

### replace()

用另一个字符串替换子字符串的出现。

**语法：**
- `string.replace(old, new)`（替换全部）
- `string.replace(old, new, count)`（最多替换 count 次）

**参数：**
- `string`：输入字符串
- `old`：要替换的子字符串
- `new`：替换字符串
- `count`：可选的最大替换次数

**返回类型：** `string`

**示例：**
```cel
"hello world".replace("world", "CEL")     // "hello CEL"
"foo foo foo".replace("foo", "bar")       // "bar bar bar"
"foo foo foo".replace("foo", "bar", 1)    // "bar foo foo"
"no match".replace("xyz", "abc")          // "no match"
"".replace("a", "b")                      // ""
```

## 11. 字符串格式化 - `format()`

### format()

使用 printf 风格的占位符格式化字符串。

**语法：** `string.format(args)`

**参数：**
- `string`：带占位符的格式字符串
- `args`：要替换的参数列表

**返回类型：** `string`

**支持的占位符：**
- `%s` - 字符串
- `%d` - 整数
- `%f` - 浮点数
- `%.Nf` - N 位小数的浮点数

**示例：**
```cel
"Hello, %s!".format(["World"])           // "Hello, World!"
"Number: %d".format([42])                // "Number: 42"
"Float: %.2f".format([3.14159])          // "Float: 3.14"
"%s has %d apples".format(["Alice", 5])  // "Alice has 5 apples"
"No placeholders".format([])             // "No placeholders"
```

## 12. 字符串反转 - `reverse()`

### reverse()

反转字符串中的字符。

**语法：** `string.reverse()`

**参数：**
- `string`：输入字符串

**返回类型：** `string`

**示例：**
```cel
"hello".reverse()           // "olleh"
"world".reverse()           // "dlrow"
"racecar".reverse()         // "racecar"
"12345".reverse()           // "54321"
"a".reverse()               // "a"
"".reverse()                // ""
"Hello, World!".reverse()   // "!dlroW ,olleH"
```

## 13. 使用示例

### 电子邮件处理
```cel
// 从电子邮件中提取用户名和域名
cel.bind(email, "  User@Example.Com  ",
  cel.bind(clean, email.trim().lowerAscii(),
    {
      "username": clean.substring(0, clean.indexOf("@")),
      "domain": clean.substring(clean.indexOf("@") + 1)
    }
  )
)
// 结果：{"username": "user", "domain": "example.com"}
```

### CSV 数据处理
```cel
// 解析和转换 CSV 数据
"name:John,age:30,city:NYC".split(",")
  .map(item, item.split(":"))
  .map(pair, pair[0].upperAscii() + "=" + pair[1])
  .join(" AND ")
// 结果："NAME=John AND AGE=30 AND CITY=NYC"
```

### 文本规范化
```cel
// 清理和规范化用户输入
cel.bind(input, "  Hello    World!  ",
  input.trim()
    .replace("  ", " ")
    .replace("!", "")
    .lowerAscii()
)
// 结果："hello world"
```

### 字符串验证
```cel
// 检查字符串是否为回文
cel.bind(text, "racecar",
  text.lowerAscii() == text.lowerAscii().reverse()
)
// 结果：true
```

### 模板处理
```cel
// 简单模板替换
"Hello, %s! You have %d new messages.".format([
  user.name.charAt(0).upperAscii() + user.name.substring(1).lowerAscii(),
  user.messageCount
])
```

Strings 扩展提供了一套全面的字符串操作工具，使在 CEL 表达式中处理文本数据变得容易，同时保持安全性和性能。