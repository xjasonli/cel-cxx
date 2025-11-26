# Math 扩展

Math 扩展为 CEL 标准库提供了基本算术运算之外的数学函数和操作。

## 目录

- [Math 扩展](#math-扩展)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. 最小值和最大值操作](#2-最小值和最大值操作)
    - [2.1 math.greatest()](#21-mathgreatest)
    - [2.2 math.least()](#22-mathleast)
  - [3. 绝对值和符号](#3-绝对值和符号)
    - [3.1 math.abs()](#31-mathabs)
    - [3.2 math.sign()](#32-mathsign)
  - [4. 舍入函数](#4-舍入函数)
    - [4.1 math.ceil()](#41-mathceil)
    - [4.2 math.floor()](#42-mathfloor)
    - [4.3 math.round()](#43-mathround)
    - [4.4 math.trunc()](#44-mathtrunc)
  - [5. 位运算操作](#5-位运算操作)
    - [5.1 math.bitAnd()](#51-mathbitand)
    - [5.2 math.bitOr()](#52-mathbitor)
    - [5.3 math.bitXor()](#53-mathbitxor)
    - [5.4 math.bitNot()](#54-mathbitnot)
    - [5.5 math.bitShiftLeft()](#55-mathbitshiftleft)
    - [5.6 math.bitShiftRight()](#56-mathbitshiftright)
  - [6. 浮点数辅助函数](#6-浮点数辅助函数)
    - [6.1 math.isInf()](#61-mathisinf)
    - [6.2 math.isNaN()](#62-mathisnan)
    - [6.3 math.isFinite()](#63-mathisfinite)
  - [7. 平方根](#7-平方根)
    - [7.1 math.sqrt()](#71-mathsqrt)
  - [8. 使用示例](#8-使用示例)
    - [范围验证](#范围验证)
    - [统计操作](#统计操作)
    - [舍入和格式化](#舍入和格式化)
    - [位标志](#位标志)
    - [浮点数验证](#浮点数验证)

## 1. 概述

Math 扩展通过表达式中常用的附加数学函数增强了 CEL。所有函数都是确定性的且无副作用。

**注意**：所有宏都使用 'math' 命名空间；但是，在宏展开时，命名空间看起来就像任何其他标识符一样。如果您当前使用名为 'math' 的变量，宏可能会按预期工作；但是，存在一些冲突的可能性。

**启用扩展**：
```rust
let env = Env::builder()
    .with_ext_math(true)
    .build()?;
```

## 2. 最小值和最大值操作

### 2.1 math.greatest()

返回宏参数中存在的最大值数字。

**语法：**
- `math.greatest(<arg>, ...)`

**参数：**
- 变参数量宏，必须至少接受一个参数
- 支持简单的数值和列表字面量作为有效参数类型
- 其他字面量在宏展开期间会被标记为错误

**返回类型：** 根据输入返回 `double`、`int` 或 `uint`

**示例：**
```cel
math.greatest(1)                             // 1
math.greatest(1u, 2u)                        // 2u
math.greatest(-42.0, -21.5, -100.0)         // -21.5
math.greatest([-42.0, -21.5, -100.0])       // -21.5
math.greatest(numbers)                       // numbers 必须是 list(numeric)
```

**错误情况：**
```cel
math.greatest()                              // 解析错误
math.greatest('string')                      // 解析错误
math.greatest(a, b)                          // 如果 a 或 b 不是数值类型，则为检查时错误
math.greatest(dyn('string'))                 // 运行时错误
```

### 2.2 math.least()

返回宏参数中存在的最小值数字。

**语法：**
- `math.least(<arg>, ...)`

**参数：**
- 变参数量宏，必须至少接受一个参数
- 支持简单的数值和列表字面量作为有效参数类型
- 其他字面量在宏展开期间会被标记为错误

**返回类型：** 根据输入返回 `double`、`int` 或 `uint`

**示例：**
```cel
math.least(1)                                // 1
math.least(1u, 2u)                           // 1u
math.least(-42.0, -21.5, -100.0)            // -100.0
math.least([-42.0, -21.5, -100.0])          // -100.0
math.least(numbers)                          // numbers 必须是 list(numeric)
```

**错误情况：**
```cel
math.least()                                 // 解析错误
math.least('string')                         // 解析错误
math.least(a, b)                             // 如果 a 或 b 不是数值类型，则为检查时错误
math.least(dyn('string'))                    // 运行时错误
```

## 3. 绝对值和符号

### 3.1 math.abs()

返回作为输入提供的数值类型的绝对值。如果值是 NaN，输出也是 NaN。如果输入是 int64 最小值，函数将导致溢出错误。

**语法：** `math.abs(number)`

**参数：**
- `number`：数值（int、uint 或 double）

**返回类型：** 与输入类型相同

**示例：**
```cel
math.abs(-1)                                 // 1
math.abs(1)                                  // 1
math.abs(-5.5)                               // 5.5
math.abs(0)                                  // 0
math.abs(42u)                                // 42u
```

**错误情况：**
```cel
math.abs(-9223372036854775808)               // 溢出错误（int64 最小值）
```

### 3.2 math.sign()

返回数值类型的符号，根据重载返回 -1、0 或 1 的 int、double 或 uint。对于浮点值，如果输入 NaN，输出也是 NaN。实现不区分正零和负零。

**语法：** `math.sign(number)`

**参数：**
- `number`：数值（int、uint 或 double）

**返回类型：** 与输入类型相同（-1、0 或 1）

**示例：**
```cel
math.sign(-42)                               // -1
math.sign(0)                                 // 0
math.sign(42)                                // 1
math.sign(-3.14)                             // -1.0
math.sign(0.0)                               // 0.0
math.sign(2.71)                              // 1.0
```

## 4. 舍入函数

### 4.1 math.ceil()

计算 double 值的上限。

**语法：** `math.ceil(number)`

**参数：**
- `number`：Double 值

**返回类型：** `double`

**示例：**
```cel
math.ceil(1.2)                               // 2.0
math.ceil(-1.2)                              // -1.0
math.ceil(5.0)                               // 5.0
math.ceil(0.1)                               // 1.0
```

### 4.2 math.floor()

计算 double 值的下限。

**语法：** `math.floor(number)`

**参数：**
- `number`：Double 值

**返回类型：** `double`

**示例：**
```cel
math.floor(1.2)                              // 1.0
math.floor(-1.2)                             // -2.0
math.floor(5.0)                              // 5.0
math.floor(0.9)                              // 0.0
```

### 4.3 math.round()

将 double 值舍入到最接近的整数，平局时远离零舍入，例如 1.5 -> 2.0，-1.5 -> -2.0。

**语法：** `math.round(number)`

**参数：**
- `number`：Double 值

**返回类型：** `double`

**示例：**
```cel
math.round(1.2)                              // 1.0
math.round(1.5)                              // 2.0
math.round(-1.5)                             // -2.0
math.round(3.14)                             // 3.0
math.round(3.64)                             // 4.0
```

### 4.4 math.trunc()

截断 double 值的小数部分。

**语法：** `math.trunc(number)`

**参数：**
- `number`：Double 值

**返回类型：** `double`

**示例：**
```cel
math.trunc(-1.3)                             // -1.0
math.trunc(1.3)                              // 1.0
math.trunc(3.14)                             // 3.0
math.trunc(-2.71)                            // -2.0
```

## 5. 位运算操作

### 5.1 math.bitAnd()

对两个 int 或 uint 值执行按位与操作。

**语法：**
- `math.bitAnd(<int>, <int>)` -> `<int>`
- `math.bitAnd(<uint>, <uint>)` -> `<uint>`

**参数：**
- 两个相同类型的整数（int 或 uint）

**返回类型：** 与输入类型相同

**示例：**
```cel
math.bitAnd(3u, 2u)                          // 2u
math.bitAnd(3, 5)                            // 1
math.bitAnd(-3, -5)                          // -7
math.bitAnd(12, 10)                          // 8 (1100 & 1010 = 1000)
```

### 5.2 math.bitOr()

对两个 int 或 uint 值执行按位或操作。

**语法：**
- `math.bitOr(<int>, <int>)` -> `<int>`
- `math.bitOr(<uint>, <uint>)` -> `<uint>`

**参数：**
- 两个相同类型的整数（int 或 uint）

**返回类型：** 与输入类型相同

**示例：**
```cel
math.bitOr(1u, 2u)                           // 3u
math.bitOr(-2, -4)                           // -2
math.bitOr(12, 10)                           // 14 (1100 | 1010 = 1110)
```

### 5.3 math.bitXor()

对两个 int 或 uint 值执行按位异或操作。

**语法：**
- `math.bitXor(<int>, <int>)` -> `<int>`
- `math.bitXor(<uint>, <uint>)` -> `<uint>`

**参数：**
- 两个相同类型的整数（int 或 uint）

**返回类型：** 与输入类型相同

**示例：**
```cel
math.bitXor(3u, 5u)                          // 6u
math.bitXor(1, 3)                            // 2
math.bitXor(12, 10)                          // 6 (1100 ^ 1010 = 0110)
```

### 5.4 math.bitNot()

接受单个 int 或 uint 并对给定二进制值执行按位非（一的补码）操作的函数。

**语法：**
- `math.bitNot(<int>)` -> `<int>`
- `math.bitNot(<uint>)` -> `<uint>`

**参数：**
- 单个整数（int 或 uint）

**返回类型：** 与输入类型相同

**示例：**
```cel
math.bitNot(1)                               // -2
math.bitNot(-1)                              // 0
math.bitNot(0u)                              // 18446744073709551615u
```

### 5.5 math.bitShiftLeft()

对第一个参数执行位左移操作，移位数量由第二个参数指定。第一个参数是 uint 或 int。第二个参数必须是 int。

当第二个参数为 64 或更大时，总是返回 0，因为移位的位数大于或等于被移位数字的总位长度。负值位移将导致运行时错误。

**语法：**
- `math.bitShiftLeft(<int>, <int>)` -> `<int>`
- `math.bitShiftLeft(<uint>, <int>)` -> `<uint>`

**参数：**
- 第一个参数：要移位的整数值（int 或 uint）
- 第二个参数：左移的位数（int）

**返回类型：** 与第一个参数类型相同

**示例：**
```cel
math.bitShiftLeft(1, 2)                      // 4
math.bitShiftLeft(-1, 2)                     // -4
math.bitShiftLeft(1u, 2)                     // 4u
math.bitShiftLeft(1u, 200)                   // 0u
math.bitShiftLeft(5, 2)                      // 20 (101 << 2 = 10100)
```

### 5.6 math.bitShiftRight()

对第一个参数执行位右移操作，移位数量由第二个参数指定。第一个参数是 uint 或 int。第二个参数必须是 int。

当第二个参数为 64 或更大时，总是返回 0，因为移位的位数大于或等于被移位数字的总位长度。负值位移将导致运行时错误。

此操作不会保留符号位扩展：左侧的空位用 0 填充。

**语法：**
- `math.bitShiftRight(<int>, <int>)` -> `<int>`
- `math.bitShiftRight(<uint>, <int>)` -> `<uint>`

**参数：**
- 第一个参数：要移位的整数值（int 或 uint）
- 第二个参数：右移的位数（int）

**返回类型：** 与第一个参数类型相同

**示例：**
```cel
math.bitShiftRight(1024, 2)                  // 256
math.bitShiftRight(1024u, 2)                 // 256u
math.bitShiftRight(1024u, 64)                // 0u
math.bitShiftRight(20, 2)                    // 5 (10100 >> 2 = 101)
```

## 6. 浮点数辅助函数

### 6.1 math.isInf()

如果输入的 double 值是 -Inf 或 +Inf，返回 true。

**语法：** `math.isInf(<double>)` -> `<bool>`

**参数：**
- `number`：Double 值

**返回类型：** `bool`

**示例：**
```cel
math.isInf(1.0/0.0)                          // true
math.isInf(-1.0/0.0)                         // true
math.isInf(1.2)                              // false
math.isInf(0.0)                              // false
```

### 6.2 math.isNaN()

如果输入的 double 值是 NaN，返回 true，否则返回 false。

**语法：** `math.isNaN(<double>)` -> `<bool>`

**参数：**
- `number`：Double 值

**返回类型：** `bool`

**示例：**
```cel
math.isNaN(0.0/0.0)                          // true
math.isNaN(1.2)                              // false
math.isNaN(1.0/0.0)                          // false（这是 Inf，不是 NaN）
```

### 6.3 math.isFinite()

如果值是有限数字，返回 true。行为等同于：`!math.isNaN(double) && !math.isInf(double)`

**语法：** `math.isFinite(<double>)` -> `<bool>`

**参数：**
- `number`：Double 值

**返回类型：** `bool`

**示例：**
```cel
math.isFinite(0.0/0.0)                       // false（NaN）
math.isFinite(1.0/0.0)                       // false（Inf）
math.isFinite(1.2)                           // true
math.isFinite(-42.5)                         // true
```

## 7. 平方根

### 7.1 math.sqrt()

返回给定输入的平方根作为 double。对于负数或非数值输入抛出错误。

**语法：**
- `math.sqrt(<double>)` -> `<double>`
- `math.sqrt(<int>)` -> `<double>`
- `math.sqrt(<uint>)` -> `<double>`

**参数：**
- `number`：数值（int、uint 或 double）

**返回类型：** `double`

**示例：**
```cel
math.sqrt(81)                                // 9.0
math.sqrt(985.25)                            // 31.388692231439016
math.sqrt(0)                                 // 0.0
math.sqrt(4u)                                // 2.0
```

**错误情况：**
```cel
math.sqrt(-15)                               // 返回 NaN
```

## 8. 使用示例

### 范围验证
```cel
// 使用 greatest/least 检查值是否在可接受范围内
cel.bind(value, 75,
  cel.bind(bounds, [0, 100],
    value >= math.least(bounds) && value <= math.greatest(bounds) &&
    math.abs(value - 50) <= 25
  )
)
// 结果：true
```

### 统计操作
```cel
// 计算基本统计信息
cel.bind(numbers, [1, 2, 3, 4, 5],
  {
    "min": math.least(numbers),
    "max": math.greatest(numbers),
    "range": math.greatest(numbers) - math.least(numbers),
    "mean_approx": math.trunc((math.greatest(numbers) + math.least(numbers)) / 2.0),
    "abs_values": numbers.map(n, math.abs(n))
  }
)
```

### 舍入和格式化
```cel
// 舍入货币值
cel.bind(price, 19.99,
  {
    "floor": math.floor(price),
    "ceil": math.ceil(price),
    "round": math.round(price),
    "trunc": math.trunc(price),
    "sqrt_price": math.sqrt(price)
  }
)
// 结果：{"floor": 19.0, "ceil": 20.0, "round": 20.0, "trunc": 19.0, "sqrt_price": 4.47...}
```

### 位标志
```cel
// 检查权限标志
cel.bind(permissions, 7,  // 二进制：111
  cel.bind(read_flag, 1,  // 二进制：001
    cel.bind(write_flag, 2, // 二进制：010
      cel.bind(exec_flag, 4, // 二进制：100
        {
          "can_read": math.bitAnd(permissions, read_flag) != 0,
          "can_write": math.bitAnd(permissions, write_flag) != 0,
          "can_execute": math.bitAnd(permissions, exec_flag) != 0,
          "full_access": permissions == math.bitOr(math.bitOr(read_flag, write_flag), exec_flag),
          "shifted_perms": math.bitShiftLeft(permissions, 1)
        }
      )
    )
  )
)
```

### 浮点数验证
```cel
// 验证浮点数
cel.bind(values, [1.5, 1.0/0.0, 0.0/0.0, -42.7],
  values.map(v, {
    "value": v,
    "is_finite": math.isFinite(v),
    "is_inf": math.isInf(v),
    "is_nan": math.isNaN(v),
    "sign": math.isFinite(v) ? math.sign(v) : 0.0,
    "sqrt": v >= 0 && math.isFinite(v) ? math.sqrt(v) : 0.0
  })
)
```

Math 扩展提供了补充 CEL 基本算术的基本数学操作，使表达式能够进行更复杂的数值计算。 