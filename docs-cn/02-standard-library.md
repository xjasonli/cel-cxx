# CEL 标准库

## 目录

- [CEL 标准库](#cel-标准库)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. 存在性和推导宏](#2-存在性和推导宏)
    - [2.1 Has 函数 (has)](#21-has-函数-has)
    - [2.2 All 函数 (all)](#22-all-函数-all)
    - [2.3 Exists 函数 (exists)](#23-exists-函数-exists)
    - [2.4 Exists One 函数 (exists\_one)](#24-exists-one-函数-exists_one)
    - [2.5 Filter 函数 (filter)](#25-filter-函数-filter)
    - [2.6 Map 函数 (map)](#26-map-函数-map)
  - [3. 逻辑操作符](#3-逻辑操作符)
    - [3.1 逻辑非 (!)](#31-逻辑非-)
    - [3.2 逻辑与 (\&\&)](#32-逻辑与-)
    - [3.3 逻辑或 (||)](#33-逻辑或-)
    - [3.4 条件操作符 (? :)](#34-条件操作符--)
  - [4. 算术操作符](#4-算术操作符)
    - [4.1 取负 (-)](#41-取负--)
    - [4.2 加法 (+)](#42-加法-)
    - [4.3 减法 (-)](#43-减法--)
    - [4.4 乘法 (\*)](#44-乘法-)
    - [4.5 除法 (/)](#45-除法-)
    - [4.6 取模 (%)](#46-取模-)
  - [5. 比较操作符](#5-比较操作符)
    - [5.1 等于 (==)](#51-等于-)
    - [5.2 不等于 (!=)](#52-不等于-)
    - [5.3 小于 (\<)](#53-小于-)
    - [5.4 小于等于 (\<=)](#54-小于等于-)
    - [5.5 大于 (\>)](#55-大于-)
    - [5.6 大于等于 (\>=)](#56-大于等于-)
  - [6. 列表操作符](#6-列表操作符)
    - [6.1 列表索引 (\[\])](#61-列表索引-)
    - [6.2 列表成员检查 (in)](#62-列表成员检查-in)
    - [6.3 列表大小 (size)](#63-列表大小-size)
  - [7. 映射操作符](#7-映射操作符)
    - [7.1 映射索引 (\[\])](#71-映射索引-)
    - [7.2 映射键成员检查 (in)](#72-映射键成员检查-in)
    - [7.3 映射大小 (size)](#73-映射大小-size)
  - [8. 字节函数](#8-字节函数)
    - [8.1 字节大小 (size)](#81-字节大小-size)
  - [9. 字符串函数](#9-字符串函数)
    - [9.1 包含 (contains)](#91-包含-contains)
    - [9.2 开始于 (startsWith)](#92-开始于-startswith)
    - [9.3 结束于 (endsWith)](#93-结束于-endswith)
    - [9.4 正则表达式匹配 (matches)](#94-正则表达式匹配-matches)
    - [9.5 字符串大小 (size)](#95-字符串大小-size)
  - [10. 日期/时间函数](#10-日期时间函数)
    - [10.1 getDate()](#101-getdate)
    - [10.2 getDayOfMonth()](#102-getdayofmonth)
    - [10.3 getDayOfWeek()](#103-getdayofweek)
    - [10.4 getDayOfYear()](#104-getdayofyear)
    - [10.5 getFullYear()](#105-getfullyear)
    - [10.6 getHours()](#106-gethours)
    - [10.7 getMilliseconds()](#107-getmilliseconds)
    - [10.8 getMinutes()](#108-getminutes)
    - [10.9 getMonth()](#109-getmonth)
    - [10.10 getSeconds()](#1010-getseconds)
  - [11. 类型和转换](#11-类型和转换)
    - [11.1 字符串转换 (string)](#111-字符串转换-string)
    - [11.2 整数转换 (int)](#112-整数转换-int)
    - [11.3 无符号整数转换 (uint)](#113-无符号整数转换-uint)
    - [11.4 双精度转换 (double)](#114-双精度转换-double)
    - [11.5 布尔转换 (bool)](#115-布尔转换-bool)
    - [11.6 字节转换 (bytes)](#116-字节转换-bytes)
    - [11.7 持续时间转换 (duration)](#117-持续时间转换-duration)
    - [11.8 时间戳转换 (timestamp)](#118-时间戳转换-timestamp)
    - [11.9 类型函数 (type)](#119-类型函数-type)
    - [11.10 动态类型 (dyn)](#1110-动态类型-dyn)

## 1. 概述

CEL 标准库提供了一套全面的内置函数和操作符，用于处理各种数据类型。这些函数在所有 CEL 环境中自动可用，涵盖了字符串、数字、集合等的常见操作。

标准库组织为以下类别：
- **存在性和推导宏** 用于测试字段存在性和列表/映射推导
- **逻辑操作符** 用于布尔运算
- **算术操作符** 用于数值计算
- **比较操作符** 用于值比较
- **列表操作符** 用于处理列表
- **映射操作符** 用于处理映射
- **字节函数** 用于字节序列操作
- **字符串函数** 用于文本操作
- **日期/时间函数** 用于处理时间戳和持续时间
- **类型和转换** 用于类型检查和转换

## 2. 存在性和推导宏

### 2.1 Has 函数 (has)

**语法：** `has(message.field)`

**描述：** 检查消息中是否存在字段。此宏支持 proto2、proto3 和映射键访问。仅支持使用选择表示法的映射访问。

**参数：**
- `message.field`：字段访问表达式

**返回类型：** `bool`

**示例：**
```cel
// 如果 'user' 消息中存在 'address' 字段，返回 `true`
has(user.address)
// 如果映射 'm' 定义了名为 'key_name' 的键，返回 `true`。值可能为 null，
// 因为 null 在 CEL 中不表示缺失。
has(m.key_name)
// 如果 'order' 消息中未设置 'items' 字段，返回 `false`
has(order.items)
// 如果 'sessions' 映射中不存在 'user_id' 键，返回 `false`
has(sessions.user_id)
```

### 2.2 All 函数 (all)

**语法：** `list.all(var, predicate)` 或 `map.all(var, predicate)`

**描述：** 测试输入列表中的所有元素或映射中的所有键是否都满足给定的谓词。all 宏的行为与逻辑与操作符一致，包括如何吸收错误和短路。

**参数：**
- `var`：每个元素/键的变量名称
- `predicate`：使用变量的布尔表达式

**返回类型：** `bool`

**示例：**
```cel
[1, 2, 3].all(x, x > 0) // true
[1, 2, 0].all(x, x > 0) // false
['apple', 'banana', 'cherry'].all(fruit, fruit.size() > 3) // true
[3.14, 2.71, 1.61].all(num, num < 3.0) // false
{'a': 1, 'b': 2, 'c': 3}.all(key, key != 'b') // false
```

### 2.3 Exists 函数 (exists)

**语法：** `list.exists(var, predicate)` 或 `map.exists(var, predicate)`

**描述：** 测试列表中的任何值或映射中的任何键是否满足谓词表达式。exists 宏的行为与逻辑或操作符一致，包括如何吸收错误和短路。

**参数：**
- `var`：每个元素/键的变量名称
- `predicate`：使用变量的布尔表达式

**返回类型：** `bool`

**示例：**
```cel
[1, 2, 3].exists(i, i % 2 != 0) // true
[2, 4, 6].exists(i, i % 2 != 0) // false
['apple', 'banana'].exists(fruit, fruit.startsWith('b')) // true
{'x': 10, 'y': 20}.exists(key, key == 'z') // false
```

### 2.4 Exists One 函数 (exists_one)

**语法：** `list.exists_one(var, predicate)` 或 `map.exists_one(var, predicate)`

**描述：** 测试列表中是否恰好有一个值或映射中恰好有一个键满足谓词表达式。

**参数：**
- `var`：每个元素/键的变量名称
- `predicate`：使用变量的布尔表达式

**返回类型：** `bool`

**示例：**
```cel
[1, 2, 3].exists_one(x, x > 2) // true（只有 3）
[1, 3, 5].exists_one(x, x > 2) // false（3 和 5 都 > 2）
[1, 2].exists_one(x, x > 2) // false（没有元素 > 2）
```

### 2.5 Filter 函数 (filter)

**语法：** `list.filter(var, predicate)`

**描述：** 返回一个新列表，包含满足谓词条件的所有元素。

**参数：**
- `var`：每个元素的变量名称
- `predicate`：使用变量的布尔表达式

**返回类型：** `list`

**示例：**
```cel
[1, 2, 3, 4, 5].filter(x, x > 3) // [4, 5]
['apple', 'banana', 'cherry'].filter(fruit, fruit.size() > 5) // ['banana', 'cherry']
[1, -2, 3, -4, 5].filter(x, x > 0) // [1, 3, 5]
```

### 2.6 Map 函数 (map)

**语法：** `list.map(var, expression)`

**描述：** 将表达式应用到列表的每个元素，返回结果列表。

**参数：**
- `var`：每个元素的变量名称
- `expression`：使用变量的表达式

**返回类型：** `list`

**示例：**
```cel
[1, 2, 3].map(x, x * 2) // [2, 4, 6]
['hello', 'world'].map(s, s.size()) // [5, 5]
[1, 2, 3].map(x, x + " items") // ["1 items", "2 items", "3 items"]
```

## 3. 逻辑操作符

### 3.1 逻辑非 (!)

**语法：** `!expression`

**描述：** 返回布尔表达式的逻辑否定。

**示例：**
```cel
!true      // false
!false     // true
!(5 > 3)   // false
```

### 3.2 逻辑与 (&&)

**语法：** `expression1 && expression2`

**描述：** 如果两个表达式都为 true，返回 true。支持短路评估。

**示例：**
```cel
true && true    // true
true && false   // false
false && true   // false
```

### 3.3 逻辑或 (||)

**语法：** `expression1 || expression2`

**描述：** 如果任一表达式为 true，返回 true。支持短路评估。

**示例：**
```cel
true || false   // true
false || true   // true
false || false  // false
```

### 3.4 条件操作符 (? :)

**语法：** `condition ? value_if_true : value_if_false`

**描述：** 基于条件返回两个值中的一个。

**示例：**
```cel
5 > 3 ? "yes" : "no"    // "yes"
x > 0 ? x : -x          // 绝对值
```

## 4. 算术操作符

### 4.1 取负 (-)

**语法：** `-expression`

**描述：** 返回数值表达式的负值。

**示例：**
```cel
-5      // -5
-(-3)   // 3
-0      // 0
```

### 4.2 加法 (+)

**语法：** `expression1 + expression2`

**描述：** 执行算术加法或字符串连接。

**示例：**
```cel
5 + 3           // 8
"hello" + " world"  // "hello world"
```

### 4.3 减法 (-)

**语法：** `expression1 - expression2`

**描述：** 执行算术减法。

**示例：**
```cel
10 - 3  // 7
5 - 8   // -3
```

### 4.4 乘法 (*)

**语法：** `expression1 * expression2`

**描述：** 执行算术乘法。

**示例：**
```cel
4 * 3   // 12
-2 * 5  // -10
```

### 4.5 除法 (/)

**语法：** `expression1 / expression2`

**描述：** 执行算术除法。

**示例：**
```cel
10 / 2  // 5
7 / 2   // 3（整数除法）
7.0 / 2 // 3.5（浮点除法）
```

### 4.6 取模 (%)

**语法：** `expression1 % expression2`

**描述：** 返回除法的余数。

**示例：**
```cel
10 % 3  // 1
7 % 2   // 1
8 % 4   // 0
```

## 5. 比较操作符

### 5.1 等于 (==)

**语法：** `expression1 == expression2`

**描述：** 测试两个值是否相等。

**示例：**
```cel
5 == 5      // true
"a" == "a"  // true
5 == 3      // false
```

### 5.2 不等于 (!=)

**语法：** `expression1 != expression2`

**描述：** 测试两个值是否不相等。

**示例：**
```cel
5 != 3      // true
"a" != "b"  // true
5 != 5      // false
```

### 5.3 小于 (<)

**语法：** `expression1 < expression2`

**描述：** 测试第一个值是否小于第二个值。

**示例：**
```cel
3 < 5       // true
5 < 3       // false
"a" < "b"   // true
```

### 5.4 小于等于 (<=)

**语法：** `expression1 <= expression2`

**描述：** 测试第一个值是否小于或等于第二个值。

**示例：**
```cel
3 <= 5  // true
5 <= 5  // true
5 <= 3  // false
```

### 5.5 大于 (>)

**语法：** `expression1 > expression2`

**描述：** 测试第一个值是否大于第二个值。

**示例：**
```cel
5 > 3   // true
3 > 5   // false
```

### 5.6 大于等于 (>=)

**语法：** `expression1 >= expression2`

**描述：** 测试第一个值是否大于或等于第二个值。

**示例：**
```cel
5 >= 3  // true
5 >= 5  // true
3 >= 5  // false
```

## 6. 列表操作符

### 6.1 列表索引 ([])

**语法：** `list[index]`

**描述：** 通过索引访问列表元素。

**示例：**
```cel
[1, 2, 3][0]    // 1
[1, 2, 3][2]    // 3
["a", "b"][1]   // "b"
```

### 6.2 列表成员检查 (in)

**语法：** `element in list`

**描述：** 检查元素是否在列表中。

**示例：**
```cel
1 in [1, 2, 3]      // true
4 in [1, 2, 3]      // false
"a" in ["a", "b"]   // true
```

### 6.3 列表大小 (size)

**语法：** `size(list)`

**描述：** 返回列表中元素的数量。

**示例：**
```cel
size([1, 2, 3])     // 3
size([])            // 0
size(["a", "b"])    // 2
```

## 7. 映射操作符

### 7.1 映射索引 ([])

**语法：** `map[key]`

**描述：** 通过键访问映射值。

**示例：**
```cel
{"a": 1, "b": 2}["a"]   // 1
{"x": 10, "y": 20}["y"] // 20
```

### 7.2 映射键成员检查 (in)

**语法：** `key in map`

**描述：** 检查键是否在映射中。

**示例：**
```cel
"a" in {"a": 1, "b": 2}     // true
"c" in {"a": 1, "b": 2}     // false
```

### 7.3 映射大小 (size)

**语法：** `size(map)`

**描述：** 返回映射中键值对的数量。

**示例：**
```cel
size({"a": 1, "b": 2})  // 2
size({})                // 0
```

## 8. 字节函数

### 8.1 字节大小 (size)

**语法：** `size(bytes)`

**描述：** 返回字节序列的长度。

**示例：**
```cel
size(b"hello")  // 5
size(b"")       // 0
```

## 9. 字符串函数

### 9.1 包含 (contains)

**语法：** `string.contains(substring)`

**描述：** 检查字符串是否包含子字符串。

**示例：**
```cel
"hello world".contains("world")  // true
"hello".contains("xyz")          // false
```

### 9.2 开始于 (startsWith)

**语法：** `string.startsWith(prefix)`

**描述：** 检查字符串是否以指定前缀开头。

**示例：**
```cel
"hello world".startsWith("hello")  // true
"world".startsWith("hello")        // false
```

### 9.3 结束于 (endsWith)

**语法：** `string.endsWith(suffix)`

**描述：** 检查字符串是否以指定后缀结尾。

**示例：**
```cel
"hello world".endsWith("world")  // true
"hello".endsWith("world")        // false
```

### 9.4 正则表达式匹配 (matches)

**语法：** `string.matches(pattern)`

**描述：** 检查字符串是否匹配正则表达式模式。使用 RE2 正则表达式语法。

**正则表达式语法：** 详细语法参考请见：[RE2 语法文档](https://github.com/google/re2/wiki/Syntax)

**示例：**
```cel
"hello123".matches("[a-z]+[0-9]+")  // true
"HELLO".matches("[a-z]+")           // false
```

### 9.5 字符串大小 (size)

**语法：** `size(string)`

**描述：** 返回字符串的字符数。

**示例：**
```cel
size("hello")   // 5
size("")        // 0
size("世界")     // 2
```

## 10. 日期/时间函数

### 10.1 getDate()

**语法：** `timestamp.getDate()`

**描述：** 返回时间戳的日期（月份中的第几天）。

**示例：**
```cel
timestamp("2023-12-25T10:30:00Z").getDate()  // 25
```

### 10.2 getDayOfMonth()

**语法：** `timestamp.getDayOfMonth()`

**描述：** 返回时间戳的月份中的第几天（与 getDate() 相同）。

### 10.3 getDayOfWeek()

**语法：** `timestamp.getDayOfWeek()`

**描述：** 返回时间戳的星期几（0=星期日，1=星期一，...，6=星期六）。

### 10.4 getDayOfYear()

**语法：** `timestamp.getDayOfYear()`

**描述：** 返回时间戳的年份中的第几天。

### 10.5 getFullYear()

**语法：** `timestamp.getFullYear()`

**描述：** 返回时间戳的年份。

**示例：**
```cel
timestamp("2023-12-25T10:30:00Z").getFullYear()  // 2023
```

### 10.6 getHours()

**语法：** `timestamp.getHours()`

**描述：** 返回时间戳的小时（0-23）。

### 10.7 getMilliseconds()

**语法：** `timestamp.getMilliseconds()`

**描述：** 返回时间戳的毫秒数。

### 10.8 getMinutes()

**语法：** `timestamp.getMinutes()`

**描述：** 返回时间戳的分钟数（0-59）。

### 10.9 getMonth()

**语法：** `timestamp.getMonth()`

**描述：** 返回时间戳的月份（0-11，0=一月）。

### 10.10 getSeconds()

**语法：** `timestamp.getSeconds()`

**描述：** 返回时间戳的秒数（0-59）。

## 11. 类型和转换

### 11.1 字符串转换 (string)

**语法：** `string(value)`

**描述：** 将值转换为字符串。

**示例：**
```cel
string(42)      // "42"
string(3.14)    // "3.14"
string(true)    // "true"
```

### 11.2 整数转换 (int)

**语法：** `int(value)`

**描述：** 将值转换为整数。

**示例：**
```cel
int(3.14)   // 3
int("42")   // 42
int(true)   // 1
```

### 11.3 无符号整数转换 (uint)

**语法：** `uint(value)`

**描述：** 将值转换为无符号整数。

**示例：**
```cel
uint(42)    // 42u
uint("100") // 100u
```

### 11.4 双精度转换 (double)

**语法：** `double(value)`

**描述：** 将值转换为双精度浮点数。

**示例：**
```cel
double(42)      // 42.0
double("3.14")  // 3.14
```

### 11.5 布尔转换 (bool)

**语法：** `bool(value)`

**描述：** 将值转换为布尔值。

**示例：**
```cel
bool(1)     // true
bool(0)     // false
bool("true") // true
```

### 11.6 字节转换 (bytes)

**语法：** `bytes(string)`

**描述：** 将字符串转换为字节序列。

**示例：**
```cel
bytes("hello")  // b"hello"
```

### 11.7 持续时间转换 (duration)

**语法：** `duration(string)`

**描述：** 将字符串转换为持续时间。

**示例：**
```cel
duration("1h30m")   // 1小时30分钟
duration("300s")    // 300秒
```

### 11.8 时间戳转换 (timestamp)

**语法：** `timestamp(string)`

**描述：** 将字符串转换为时间戳。

**示例：**
```cel
timestamp("2023-01-01T00:00:00Z")
```

### 11.9 类型函数 (type)

**语法：** `type(value)`

**描述：** 返回值的类型。

**示例：**
```cel
type(42)        // int
type("hello")   // string
type([1, 2, 3]) // list
```

### 11.10 动态类型 (dyn)

**语法：** `dyn(value)`

**描述：** 将值转换为动态类型。

**示例：**
```cel
dyn(42)     // 动态包装的 42
dyn("test") // 动态包装的 "test"
```

CEL 标准库提供了强大而全面的函数集合，使您能够编写表达式来处理各种数据类型和操作场景。这些函数都经过优化，确保高性能和类型安全。 