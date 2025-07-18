# Math Extension

The math extension provides mathematical functions and operations beyond basic arithmetic available in the CEL standard library.

## Table of Contents

- [Math Extension](#math-extension)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Min and Max Operations](#2-min-and-max-operations)
    - [2.1 math.min()](#21-mathmin)
    - [2.2 math.max()](#22-mathmax)
  - [3. Absolute Value - `math.abs()`](#3-absolute-value---mathabs)
    - [math.abs()](#mathabs)
  - [4. Sign Function - `math.sign()`](#4-sign-function---mathsign)
    - [math.sign()](#mathsign)
  - [5. Rounding Functions](#5-rounding-functions)
    - [5.1 math.ceil()](#51-mathceil)
    - [5.2 math.floor()](#52-mathfloor)
    - [5.3 math.round()](#53-mathround)
    - [5.4 math.trunc()](#54-mathtrunc)
  - [6. Bitwise Operations](#6-bitwise-operations)
    - [6.1 math.bitAnd()](#61-mathbitand)
    - [6.2 math.bitOr()](#62-mathbitor)
    - [6.3 math.bitXor()](#63-mathbitxor)
    - [6.4 math.bitNot()](#64-mathbitnot)
    - [6.5 math.bitShiftLeft()](#65-mathbitshiftleft)
    - [6.6 math.bitShiftRight()](#66-mathbitshiftright)
  - [7. Usage Examples](#7-usage-examples)
    - [Range Validation](#range-validation)
    - [Statistical Operations](#statistical-operations)
    - [Rounding and Formatting](#rounding-and-formatting)
    - [Bitwise Flags](#bitwise-flags)
    - [Sign-based Logic](#sign-based-logic)

## 1. Overview

The math extension enhances CEL with additional mathematical functions that are commonly needed in expressions. All functions are deterministic and side-effect free.

**Enabling the Extension**:
```rust
let env = Env::builder()
    .with_ext_math(true)
    .build()?;
```

## 2. Min and Max Operations

### 2.1 math.min()

Returns the minimum value from a list or multiple arguments.

**Syntax:**
- `math.min(list)`
- `math.min(a, b, ...)`

**Parameters:**
- `list`: List of numeric values
- `a, b, ...`: Multiple numeric arguments

**Return Type:** Same as input type (int or double)

**Examples:**
```cel
math.min([1, 2, 3])         // 1
math.min(5, 2, 8, 1)        // 1
math.min([3.14, 2.71])      // 2.71
math.min([-5, -2, -8])      // -8
math.min([42])              // 42
```

### 2.2 math.max()

Returns the maximum value from a list or multiple arguments.

**Syntax:**
- `math.max(list)`
- `math.max(a, b, ...)`

**Parameters:**
- `list`: List of numeric values
- `a, b, ...`: Multiple numeric arguments

**Return Type:** Same as input type (int or double)

**Examples:**
```cel
math.max([1, 2, 3])         // 3
math.max(5, 2, 8, 1)        // 8
math.max([3.14, 2.71])      // 3.14
math.max([-5, -2, -8])      // -2
math.max([42])              // 42
```

## 3. Absolute Value - `math.abs()`

### math.abs()

Returns the absolute value of a number.

**Syntax:** `math.abs(number)`

**Parameters:**
- `number`: Numeric value (int or double)

**Return Type:** Same as input type

**Examples:**
```cel
math.abs(-5)                // 5
math.abs(3.14)              // 3.14
math.abs(-2.71)             // 2.71
math.abs(0)                 // 0
math.abs(42)                // 42
```

## 4. Sign Function - `math.sign()`

### math.sign()

Returns the sign of a number.

**Syntax:** `math.sign(number)`

**Parameters:**
- `number`: Numeric value (int or double)

**Return Type:** int (-1, 0, or 1)

**Examples:**
```cel
math.sign(-5)               // -1
math.sign(0)                // 0
math.sign(3.14)             // 1
math.sign(-2.71)            // -1
math.sign(42)               // 1
```

**Return Values:**
- `-1` for negative numbers
- `0` for zero
- `1` for positive numbers

## 5. Rounding Functions

### 5.1 math.ceil()

Rounds a number up to the nearest integer.

**Syntax:** `math.ceil(number)`

**Parameters:**
- `number`: Numeric value

**Return Type:** double

**Examples:**
```cel
math.ceil(3.14)             // 4.0
math.ceil(-2.71)            // -2.0
math.ceil(5.0)              // 5.0
math.ceil(-5.0)             // -5.0
math.ceil(0.1)              // 1.0
```

### 5.2 math.floor()

Rounds a number down to the nearest integer.

**Syntax:** `math.floor(number)`

**Parameters:**
- `number`: Numeric value

**Return Type:** double

**Examples:**
```cel
math.floor(3.14)            // 3.0
math.floor(-2.71)           // -3.0
math.floor(5.0)             // 5.0
math.floor(-5.0)            // -5.0
math.floor(0.9)             // 0.0
```

### 5.3 math.round()

Rounds a number to the nearest integer.

**Syntax:** `math.round(number)`

**Parameters:**
- `number`: Numeric value

**Return Type:** double

**Examples:**
```cel
math.round(3.14)            // 3.0
math.round(3.64)            // 4.0
math.round(-2.71)           // -3.0
math.round(-2.49)           // -2.0
math.round(5.5)             // 6.0
```

**Rounding Rule:** Uses "round half away from zero" (banker's rounding)

### 5.4 math.trunc()

Truncates a number to its integer part.

**Syntax:** `math.trunc(number)`

**Parameters:**
- `number`: Numeric value

**Return Type:** double

**Examples:**
```cel
math.trunc(3.14)            // 3.0
math.trunc(-2.71)           // -2.0
math.trunc(5.0)             // 5.0
math.trunc(-5.0)            // -5.0
math.trunc(0.9)             // 0.0
```

## 6. Bitwise Operations

### 6.1 math.bitAnd()

Performs bitwise AND operation.

**Syntax:** `math.bitAnd(a, b)`

**Parameters:**
- `a`: First integer
- `b`: Second integer

**Return Type:** int

**Examples:**
```cel
math.bitAnd(12, 10)         // 8 (1100 & 1010 = 1000)
math.bitAnd(15, 7)          // 7 (1111 & 0111 = 0111)
math.bitAnd(0, 255)         // 0
```

### 6.2 math.bitOr()

Performs bitwise OR operation.

**Syntax:** `math.bitOr(a, b)`

**Parameters:**
- `a`: First integer
- `b`: Second integer

**Return Type:** int

**Examples:**
```cel
math.bitOr(12, 10)          // 14 (1100 | 1010 = 1110)
math.bitOr(8, 4)            // 12 (1000 | 0100 = 1100)
math.bitOr(0, 255)          // 255
```

### 6.3 math.bitXor()

Performs bitwise XOR operation.

**Syntax:** `math.bitXor(a, b)`

**Parameters:**
- `a`: First integer
- `b`: Second integer

**Return Type:** int

**Examples:**
```cel
math.bitXor(12, 10)         // 6 (1100 ^ 1010 = 0110)
math.bitXor(15, 15)         // 0 (same numbers)
math.bitXor(0, 255)         // 255
```

### 6.4 math.bitNot()

Performs bitwise NOT operation.

**Syntax:** `math.bitNot(a)`

**Parameters:**
- `a`: Integer value

**Return Type:** int

**Examples:**
```cel
math.bitNot(12)             // -13 (~1100 = ...11110011)
math.bitNot(0)              // -1
math.bitNot(-1)             // 0
```

### 6.5 math.bitShiftLeft()

Performs left bit shift operation.

**Syntax:** `math.bitShiftLeft(a, positions)`

**Parameters:**
- `a`: Integer value to shift
- `positions`: Number of positions to shift left

**Return Type:** int

**Examples:**
```cel
math.bitShiftLeft(5, 2)     // 20 (101 << 2 = 10100)
math.bitShiftLeft(1, 3)     // 8 (1 << 3 = 1000)
math.bitShiftLeft(0, 5)     // 0
```

### 6.6 math.bitShiftRight()

Performs right bit shift operation.

**Syntax:** `math.bitShiftRight(a, positions)`

**Parameters:**
- `a`: Integer value to shift
- `positions`: Number of positions to shift right

**Return Type:** int

**Examples:**
```cel
math.bitShiftRight(20, 2)   // 5 (10100 >> 2 = 101)
math.bitShiftRight(8, 3)    // 1 (1000 >> 3 = 1)
math.bitShiftRight(7, 1)    // 3 (111 >> 1 = 11)
```

## 7. Usage Examples

### Range Validation
```cel
// Check if value is within acceptable range
cel.bind(value, 75,
  cel.bind(min_val, 0,
    cel.bind(max_val, 100,
      value >= min_val && value <= max_val &&
      math.abs(value - 50) <= 25
    )
  )
)
// Result: true
```

### Statistical Operations
```cel
// Calculate basic statistics
cel.bind(numbers, [1, 2, 3, 4, 5],
  {
    "min": math.min(numbers),
    "max": math.max(numbers),
    "range": math.max(numbers) - math.min(numbers),
    "abs_values": numbers.map(n, math.abs(n))
  }
)
// Result: {"min": 1, "max": 5, "range": 4, "abs_values": [1, 2, 3, 4, 5]}
```

### Rounding and Formatting
```cel
// Round currency values
cel.bind(price, 19.99,
  {
    "floor": math.floor(price),
    "ceil": math.ceil(price),
    "round": math.round(price),
    "trunc": math.trunc(price)
  }
)
// Result: {"floor": 19.0, "ceil": 20.0, "round": 20.0, "trunc": 19.0}
```

### Bitwise Flags
```cel
// Check permission flags
cel.bind(permissions, 7,  // binary: 111
  cel.bind(read_flag, 1,  // binary: 001
    cel.bind(write_flag, 2, // binary: 010
      cel.bind(exec_flag, 4, // binary: 100
        {
          "can_read": math.bitAnd(permissions, read_flag) != 0,
          "can_write": math.bitAnd(permissions, write_flag) != 0,
          "can_execute": math.bitAnd(permissions, exec_flag) != 0,
          "full_access": permissions == math.bitOr(math.bitOr(read_flag, write_flag), exec_flag)
        }
      )
    )
  )
)
// Result: {"can_read": true, "can_write": true, "can_execute": true, "full_access": true}
```

### Sign-based Logic
```cel
// Categorize numbers by sign
cel.bind(values, [-5, 0, 3, -2, 7],
  values.map(v, 
    math.sign(v) == -1 ? "negative" :
    math.sign(v) == 0 ? "zero" :
    "positive"
  )
)
// Result: ["negative", "zero", "positive", "negative", "positive"]
```

The math extension provides essential mathematical operations that complement CEL's basic arithmetic, enabling more sophisticated numerical computations in expressions. 