# Math Extension

The math extension provides mathematical functions and operations beyond basic arithmetic available in the CEL standard library.

## Table of Contents

- [Math Extension](#math-extension)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Min and Max Operations](#2-min-and-max-operations)
    - [2.1 math.greatest()](#21-mathgreatest)
    - [2.2 math.least()](#22-mathleast)
  - [3. Absolute Value and Sign](#3-absolute-value-and-sign)
    - [3.1 math.abs()](#31-mathabs)
    - [3.2 math.sign()](#32-mathsign)
  - [4. Rounding Functions](#4-rounding-functions)
    - [4.1 math.ceil()](#41-mathceil)
    - [4.2 math.floor()](#42-mathfloor)
    - [4.3 math.round()](#43-mathround)
    - [4.4 math.trunc()](#44-mathtrunc)
  - [5. Bitwise Operations](#5-bitwise-operations)
    - [5.1 math.bitAnd()](#51-mathbitand)
    - [5.2 math.bitOr()](#52-mathbitor)
    - [5.3 math.bitXor()](#53-mathbitxor)
    - [5.4 math.bitNot()](#54-mathbitnot)
    - [5.5 math.bitShiftLeft()](#55-mathbitshiftleft)
    - [5.6 math.bitShiftRight()](#56-mathbitshiftright)
  - [6. Floating Point Helpers](#6-floating-point-helpers)
    - [6.1 math.isInf()](#61-mathisinf)
    - [6.2 math.isNaN()](#62-mathisnan)
    - [6.3 math.isFinite()](#63-mathisfinite)
  - [7. Square Root](#7-square-root)
    - [7.1 math.sqrt()](#71-mathsqrt)
  - [8. Usage Examples](#8-usage-examples)
    - [Range Validation](#range-validation)
    - [Statistical Operations](#statistical-operations)
    - [Rounding and Formatting](#rounding-and-formatting)
    - [Bitwise Flags](#bitwise-flags)
    - [Floating Point Validation](#floating-point-validation)

## 1. Overview

The math extension enhances CEL with additional mathematical functions that are commonly needed in expressions. All functions are deterministic and side-effect free.

**Note**: All macros use the 'math' namespace; however, at the time of macro expansion the namespace looks just like any other identifier. If you are currently using a variable named 'math', the macro will likely work just as intended; however, there is some chance for collision.

**Enabling the Extension**:
```rust
let env = Env::builder()
    .with_ext_math(true)
    .build()?;
```

## 2. Min and Max Operations

### 2.1 math.greatest()

Returns the greatest valued number present in the arguments to the macro.

**Syntax:**
- `math.greatest(<arg>, ...)`

**Parameters:**
- Variable argument count macro which must take at least one argument
- Simple numeric and list literals are supported as valid argument types
- Other literals will be flagged as errors during macro expansion

**Return Type:** `double`, `int`, or `uint` depending on input

**Examples:**
```cel
math.greatest(1)                             // 1
math.greatest(1u, 2u)                        // 2u
math.greatest(-42.0, -21.5, -100.0)         // -21.5
math.greatest([-42.0, -21.5, -100.0])       // -21.5
math.greatest(numbers)                       // numbers must be list(numeric)
```

**Error Cases:**
```cel
math.greatest()                              // parse error
math.greatest('string')                      // parse error
math.greatest(a, b)                          // check-time error if a or b is non-numeric
math.greatest(dyn('string'))                 // runtime error
```

### 2.2 math.least()

Returns the least valued number present in the arguments to the macro.

**Syntax:**
- `math.least(<arg>, ...)`

**Parameters:**
- Variable argument count macro which must take at least one argument
- Simple numeric and list literals are supported as valid argument types
- Other literals will be flagged as errors during macro expansion

**Return Type:** `double`, `int`, or `uint` depending on input

**Examples:**
```cel
math.least(1)                                // 1
math.least(1u, 2u)                           // 1u
math.least(-42.0, -21.5, -100.0)            // -100.0
math.least([-42.0, -21.5, -100.0])          // -100.0
math.least(numbers)                          // numbers must be list(numeric)
```

**Error Cases:**
```cel
math.least()                                 // parse error
math.least('string')                         // parse error
math.least(a, b)                             // check-time error if a or b is non-numeric
math.least(dyn('string'))                    // runtime error
```

## 3. Absolute Value and Sign

### 3.1 math.abs()

Returns the absolute value of the numeric type provided as input. If the value is NaN, the output is NaN. If the input is int64 min, the function will result in an overflow error.

**Syntax:** `math.abs(number)`

**Parameters:**
- `number`: Numeric value (int, uint, or double)

**Return Type:** Same as input type

**Examples:**
```cel
math.abs(-1)                                 // 1
math.abs(1)                                  // 1
math.abs(-5.5)                               // 5.5
math.abs(0)                                  // 0
math.abs(42u)                                // 42u
```

**Error Cases:**
```cel
math.abs(-9223372036854775808)               // overflow error (int64 min)
```

### 3.2 math.sign()

Returns the sign of the numeric type, either -1, 0, or 1 as an int, double, or uint depending on the overload. For floating point values, if NaN is provided as input, the output is also NaN. The implementation does not differentiate between positive and negative zero.

**Syntax:** `math.sign(number)`

**Parameters:**
- `number`: Numeric value (int, uint, or double)

**Return Type:** Same as input type (-1, 0, or 1)

**Examples:**
```cel
math.sign(-42)                               // -1
math.sign(0)                                 // 0
math.sign(42)                                // 1
math.sign(-3.14)                             // -1.0
math.sign(0.0)                               // 0.0
math.sign(2.71)                              // 1.0
```

## 4. Rounding Functions

### 4.1 math.ceil()

Compute the ceiling of a double value.

**Syntax:** `math.ceil(number)`

**Parameters:**
- `number`: Double value

**Return Type:** `double`

**Examples:**
```cel
math.ceil(1.2)                               // 2.0
math.ceil(-1.2)                              // -1.0
math.ceil(5.0)                               // 5.0
math.ceil(0.1)                               // 1.0
```

### 4.2 math.floor()

Compute the floor of a double value.

**Syntax:** `math.floor(number)`

**Parameters:**
- `number`: Double value

**Return Type:** `double`

**Examples:**
```cel
math.floor(1.2)                              // 1.0
math.floor(-1.2)                             // -2.0
math.floor(5.0)                              // 5.0
math.floor(0.9)                              // 0.0
```

### 4.3 math.round()

Rounds the double value to the nearest whole number with ties rounding away from zero, e.g. 1.5 -> 2.0, -1.5 -> -2.0.

**Syntax:** `math.round(number)`

**Parameters:**
- `number`: Double value

**Return Type:** `double`

**Examples:**
```cel
math.round(1.2)                              // 1.0
math.round(1.5)                              // 2.0
math.round(-1.5)                             // -2.0
math.round(3.14)                             // 3.0
math.round(3.64)                             // 4.0
```

### 4.4 math.trunc()

Truncates the fractional portion of the double value.

**Syntax:** `math.trunc(number)`

**Parameters:**
- `number`: Double value

**Return Type:** `double`

**Examples:**
```cel
math.trunc(-1.3)                             // -1.0
math.trunc(1.3)                              // 1.0
math.trunc(3.14)                             // 3.0
math.trunc(-2.71)                            // -2.0
```

## 5. Bitwise Operations

### 5.1 math.bitAnd()

Performs a bitwise-AND operation over two int or uint values.

**Syntax:**
- `math.bitAnd(<int>, <int>)` -> `<int>`
- `math.bitAnd(<uint>, <uint>)` -> `<uint>`

**Parameters:**
- Two integers of the same type (int or uint)

**Return Type:** Same as input type

**Examples:**
```cel
math.bitAnd(3u, 2u)                          // 2u
math.bitAnd(3, 5)                            // 1
math.bitAnd(-3, -5)                          // -7
math.bitAnd(12, 10)                          // 8 (1100 & 1010 = 1000)
```

### 5.2 math.bitOr()

Performs a bitwise-OR operation over two int or uint values.

**Syntax:**
- `math.bitOr(<int>, <int>)` -> `<int>`
- `math.bitOr(<uint>, <uint>)` -> `<uint>`

**Parameters:**
- Two integers of the same type (int or uint)

**Return Type:** Same as input type

**Examples:**
```cel
math.bitOr(1u, 2u)                           // 3u
math.bitOr(-2, -4)                           // -2
math.bitOr(12, 10)                           // 14 (1100 | 1010 = 1110)
```

### 5.3 math.bitXor()

Performs a bitwise-XOR operation over two int or uint values.

**Syntax:**
- `math.bitXor(<int>, <int>)` -> `<int>`
- `math.bitXor(<uint>, <uint>)` -> `<uint>`

**Parameters:**
- Two integers of the same type (int or uint)

**Return Type:** Same as input type

**Examples:**
```cel
math.bitXor(3u, 5u)                          // 6u
math.bitXor(1, 3)                            // 2
math.bitXor(12, 10)                          // 6 (1100 ^ 1010 = 0110)
```

### 5.4 math.bitNot()

Function which accepts a single int or uint and performs a bitwise-NOT ones-complement of the given binary value.

**Syntax:**
- `math.bitNot(<int>)` -> `<int>`
- `math.bitNot(<uint>)` -> `<uint>`

**Parameters:**
- Single integer (int or uint)

**Return Type:** Same as input type

**Examples:**
```cel
math.bitNot(1)                               // -2
math.bitNot(-1)                              // 0
math.bitNot(0u)                              // 18446744073709551615u
```

### 5.5 math.bitShiftLeft()

Perform a left shift of bits on the first parameter, by the amount of bits specified in the second parameter. The first parameter is either a uint or an int. The second parameter must be an int.

When the second parameter is 64 or greater, 0 will always be returned since the number of bits shifted is greater than or equal to the total bit length of the number being shifted. Negative valued bit shifts will result in a runtime error.

**Syntax:**
- `math.bitShiftLeft(<int>, <int>)` -> `<int>`
- `math.bitShiftLeft(<uint>, <int>)` -> `<uint>`

**Parameters:**
- First parameter: Integer value to shift (int or uint)
- Second parameter: Number of bit positions to shift left (int)

**Return Type:** Same as first parameter type

**Examples:**
```cel
math.bitShiftLeft(1, 2)                      // 4
math.bitShiftLeft(-1, 2)                     // -4
math.bitShiftLeft(1u, 2)                     // 4u
math.bitShiftLeft(1u, 200)                   // 0u
math.bitShiftLeft(5, 2)                      // 20 (101 << 2 = 10100)
```

### 5.6 math.bitShiftRight()

Perform a right shift of bits on the first parameter, by the amount of bits specified in the second parameter. The first parameter is either a uint or an int. The second parameter must be an int.

When the second parameter is 64 or greater, 0 will always be returned since the number of bits shifted is greater than or equal to the total bit length of the number being shifted. Negative valued bit shifts will result in a runtime error.

The sign bit extension will not be preserved for this operation: vacant bits on the left are filled with 0.

**Syntax:**
- `math.bitShiftRight(<int>, <int>)` -> `<int>`
- `math.bitShiftRight(<uint>, <int>)` -> `<uint>`

**Parameters:**
- First parameter: Integer value to shift (int or uint)
- Second parameter: Number of bit positions to shift right (int)

**Return Type:** Same as first parameter type

**Examples:**
```cel
math.bitShiftRight(1024, 2)                  // 256
math.bitShiftRight(1024u, 2)                 // 256u
math.bitShiftRight(1024u, 64)                // 0u
math.bitShiftRight(20, 2)                    // 5 (10100 >> 2 = 101)
```

## 6. Floating Point Helpers

### 6.1 math.isInf()

Returns true if the input double value is -Inf or +Inf.

**Syntax:** `math.isInf(<double>)` -> `<bool>`

**Parameters:**
- `number`: Double value

**Return Type:** `bool`

**Examples:**
```cel
math.isInf(1.0/0.0)                          // true
math.isInf(-1.0/0.0)                         // true
math.isInf(1.2)                              // false
math.isInf(0.0)                              // false
```

### 6.2 math.isNaN()

Returns true if the input double value is NaN, false otherwise.

**Syntax:** `math.isNaN(<double>)` -> `<bool>`

**Parameters:**
- `number`: Double value

**Return Type:** `bool`

**Examples:**
```cel
math.isNaN(0.0/0.0)                          // true
math.isNaN(1.2)                              // false
math.isNaN(1.0/0.0)                          // false (this is Inf, not NaN)
```

### 6.3 math.isFinite()

Returns true if the value is a finite number. Equivalent in behavior to: `!math.isNaN(double) && !math.isInf(double)`

**Syntax:** `math.isFinite(<double>)` -> `<bool>`

**Parameters:**
- `number`: Double value

**Return Type:** `bool`

**Examples:**
```cel
math.isFinite(0.0/0.0)                       // false (NaN)
math.isFinite(1.0/0.0)                       // false (Inf)
math.isFinite(1.2)                           // true
math.isFinite(-42.5)                         // true
```

## 7. Square Root

### 7.1 math.sqrt()

Returns the square root of the given input as double. Throws error for negative or non-numeric inputs.

**Syntax:**
- `math.sqrt(<double>)` -> `<double>`
- `math.sqrt(<int>)` -> `<double>`
- `math.sqrt(<uint>)` -> `<double>`

**Parameters:**
- `number`: Numeric value (int, uint, or double)

**Return Type:** `double`

**Examples:**
```cel
math.sqrt(81)                                // 9.0
math.sqrt(985.25)                            // 31.388692231439016
math.sqrt(0)                                 // 0.0
math.sqrt(4u)                                // 2.0
```

**Error Cases:**
```cel
math.sqrt(-15)                               // returns NaN
```

## 8. Usage Examples

### Range Validation
```cel
// Check if value is within acceptable range using greatest/least
cel.bind(value, 75,
  cel.bind(bounds, [0, 100],
    value >= math.least(bounds) && value <= math.greatest(bounds) &&
    math.abs(value - 50) <= 25
  )
)
// Result: true
```

### Statistical Operations
```cel
// Calculate basic statistics
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

### Rounding and Formatting
```cel
// Round currency values
cel.bind(price, 19.99,
  {
    "floor": math.floor(price),
    "ceil": math.ceil(price),
    "round": math.round(price),
    "trunc": math.trunc(price),
    "sqrt_price": math.sqrt(price)
  }
)
// Result: {"floor": 19.0, "ceil": 20.0, "round": 20.0, "trunc": 19.0, "sqrt_price": 4.47...}
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
          "full_access": permissions == math.bitOr(math.bitOr(read_flag, write_flag), exec_flag),
          "shifted_perms": math.bitShiftLeft(permissions, 1)
        }
      )
    )
  )
)
```

### Floating Point Validation
```cel
// Validate floating point numbers
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

The math extension provides essential mathematical operations that complement CEL's basic arithmetic, enabling more sophisticated numerical computations in expressions. 