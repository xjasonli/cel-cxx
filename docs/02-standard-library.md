# CEL Standard Library

## Table of Contents

- [CEL Standard Library](#cel-standard-library)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Logical Operators](#2-logical-operators)
    - [2.1 Logical NOT (!)](#21-logical-not-)
    - [2.2 Logical AND (\&\&)](#22-logical-and-)
    - [2.3 Logical OR (||)](#23-logical-or-)
    - [2.4 Conditional Operator (? :)](#24-conditional-operator--)
  - [3. Arithmetic Operators](#3-arithmetic-operators)
    - [3.1 Negation (-)](#31-negation--)
    - [3.2 Addition (+)](#32-addition-)
    - [3.3 Subtraction (-)](#33-subtraction--)
    - [3.4 Multiplication (\*)](#34-multiplication-)
    - [3.5 Division (/)](#35-division-)
    - [3.6 Modulo (%)](#36-modulo-)
  - [4. Comparison Operators](#4-comparison-operators)
    - [4.1 Equality (==)](#41-equality-)
    - [4.2 Inequality (!=)](#42-inequality-)
    - [4.3 Less Than (\<)](#43-less-than-)
    - [4.4 Less Than or Equal (\<=)](#44-less-than-or-equal-)
    - [4.5 Greater Than (\>)](#45-greater-than-)
    - [4.6 Greater Than or Equal (\>=)](#46-greater-than-or-equal-)
  - [5. Container Operators](#5-container-operators)
    - [5.1 Contains (in)](#51-contains-in)
    - [5.2 Index (\[\])](#52-index-)
    - [5.3 Size (size)](#53-size-size)
  - [6. String Functions](#6-string-functions)
    - [6.1 Contains (contains)](#61-contains-contains)
    - [6.2 Starts With (startsWith)](#62-starts-with-startswith)
    - [6.3 Ends With (endsWith)](#63-ends-with-endswith)
    - [6.4 Regular Expression Match (matches)](#64-regular-expression-match-matches)
  - [7. Type Conversion Functions](#7-type-conversion-functions)
    - [7.1 String Conversion (string)](#71-string-conversion-string)
    - [7.2 Integer Conversion (int)](#72-integer-conversion-int)
    - [7.3 Unsigned Integer Conversion (uint)](#73-unsigned-integer-conversion-uint)
    - [7.4 Double Conversion (double)](#74-double-conversion-double)
    - [7.5 Boolean Conversion (bool)](#75-boolean-conversion-bool)
    - [7.6 Bytes Conversion (bytes)](#76-bytes-conversion-bytes)
    - [7.7 Duration Conversion (duration)](#77-duration-conversion-duration)
    - [7.8 Timestamp Conversion (timestamp)](#78-timestamp-conversion-timestamp)
  - [8. Time Functions](#8-time-functions)
    - [8.1 Duration Functions](#81-duration-functions)
    - [8.2 Timestamp Functions](#82-timestamp-functions)
  - [9. Utility Functions](#9-utility-functions)
    - [9.1 Type Function (type)](#91-type-function-type)
    - [9.2 Has Function (has)](#92-has-function-has)
    - [9.3 All Function (all)](#93-all-function-all)
    - [9.4 Exists Function (exists)](#94-exists-function-exists)
    - [9.5 Exists One Function (exists\_one)](#95-exists-one-function-exists_one)
    - [9.6 Map Function (map)](#96-map-function-map)
    - [9.7 Filter Function (filter)](#97-filter-function-filter)

## 1. Overview

The CEL standard library provides a comprehensive set of built-in functions and operators for working with various data types. These functions are automatically available in all CEL environments and cover common operations for strings, numbers, collections, and more.

The standard library is organized into the following categories:
- **Logical operators** for boolean operations
- **Arithmetic operators** for numeric calculations
- **Comparison operators** for value comparisons
- **Container operators** for working with lists and maps
- **String functions** for text manipulation
- **Type conversion functions** for converting between types
- **Time functions** for working with timestamps and durations
- **Utility functions** for advanced operations

## 2. Logical Operators

### 2.1 Logical NOT (!)

**Syntax:** `!expr`

**Description:** Returns the logical negation of a boolean expression.

**Parameters:**
- `expr`: A boolean expression

**Return Type:** `bool`

**Examples:**
```cel
!true           // false
!false          // true
!(x > 5)        // true if x <= 5
!has(msg.field) // true if field is not present
```

### 2.2 Logical AND (&&)

**Syntax:** `expr1 && expr2`

**Description:** Returns `true` if both expressions are `true`. Uses short-circuit evaluation.

**Parameters:**
- `expr1`: First boolean expression
- `expr2`: Second boolean expression

**Return Type:** `bool`

**Examples:**
```cel
true && true    // true
true && false   // false
false && true   // false
x > 0 && x < 10 // true if x is between 0 and 10
```

### 2.3 Logical OR (||)

**Syntax:** `expr1 || expr2`

**Description:** Returns `true` if at least one expression is `true`. Uses short-circuit evaluation.

**Parameters:**
- `expr1`: First boolean expression
- `expr2`: Second boolean expression

**Return Type:** `bool`

**Examples:**
```cel
true || false   // true
false || true   // true
false || false  // false
x < 0 || x > 10 // true if x is outside the range [0, 10]
```

### 2.4 Conditional Operator (? :)

**Syntax:** `condition ? true_expr : false_expr`

**Description:** Returns `true_expr` if `condition` is `true`, otherwise returns `false_expr`.

**Parameters:**
- `condition`: Boolean expression
- `true_expr`: Expression to evaluate if condition is true
- `false_expr`: Expression to evaluate if condition is false

**Return Type:** Type of the selected expression

**Examples:**
```cel
x > 0 ? "positive" : "non-positive"
age >= 18 ? "adult" : "minor"
has(user.name) ? user.name : "anonymous"
```

## 3. Arithmetic Operators

### 3.1 Negation (-)

**Syntax:** `-expr`

**Description:** Returns the arithmetic negation of a numeric expression.

**Parameters:**
- `expr`: Numeric expression (`int`, `uint`, or `double`)

**Return Type:** Same as input type

**Examples:**
```cel
-42      // -42
-3.14    // -3.14
-x       // negation of variable x
```

### 3.2 Addition (+)

**Syntax:** `expr1 + expr2`

**Description:** Adds two numeric values or concatenates strings.

**Parameters:**
- `expr1`, `expr2`: Numeric expressions or strings

**Return Type:** 
- For numbers: `int`, `uint`, or `double` (following promotion rules)
- For strings: `string`

**Examples:**
```cel
// Numeric addition
1 + 2           // 3
3.14 + 2.86     // 6.0
42 + 8u         // 50u

// String concatenation
"Hello, " + "World!"  // "Hello, World!"
```

### 3.3 Subtraction (-)

**Syntax:** `expr1 - expr2`

**Description:** Subtracts the second numeric value from the first.

**Parameters:**
- `expr1`, `expr2`: Numeric expressions

**Return Type:** `int`, `uint`, or `double` (following promotion rules)

**Examples:**
```cel
10 - 3          // 7
5.5 - 2.5       // 3.0
100u - 25u      // 75u
```

### 3.4 Multiplication (*)

**Syntax:** `expr1 * expr2`

**Description:** Multiplies two numeric values.

**Parameters:**
- `expr1`, `expr2`: Numeric expressions

**Return Type:** `int`, `uint`, or `double` (following promotion rules)

**Examples:**
```cel
3 * 4           // 12
2.5 * 4.0       // 10.0
6u * 7u         // 42u
```

### 3.5 Division (/)

**Syntax:** `expr1 / expr2`

**Description:** Divides the first numeric value by the second.

**Parameters:**
- `expr1`, `expr2`: Numeric expressions

**Return Type:** `int`, `uint`, or `double` (following promotion rules)

**Examples:**
```cel
10 / 3          // 3 (integer division)
10.0 / 3.0      // 3.333...
20u / 4u        // 5u
```

### 3.6 Modulo (%)

**Syntax:** `expr1 % expr2`

**Description:** Returns the remainder of dividing the first value by the second.

**Parameters:**
- `expr1`, `expr2`: Numeric expressions

**Return Type:** `int`, `uint`, or `double`

**Examples:**
```cel
10 % 3          // 1
17 % 5          // 2
20u % 6u        // 2u
```

## 4. Comparison Operators

### 4.1 Equality (==)

**Syntax:** `expr1 == expr2`

**Description:** Returns `true` if both expressions are equal.

**Parameters:**
- `expr1`, `expr2`: Expressions of compatible types

**Return Type:** `bool`

**Examples:**
```cel
42 == 42        // true
"hello" == "hello"  // true
[1, 2] == [1, 2]    // true
null == null    // true
```

### 4.2 Inequality (!=)

**Syntax:** `expr1 != expr2`

**Description:** Returns `true` if the expressions are not equal.

**Parameters:**
- `expr1`, `expr2`: Expressions of compatible types

**Return Type:** `bool`

**Examples:**
```cel
42 != 43        // true
"hello" != "world"  // true
[1, 2] != [2, 1]    // true
```

### 4.3 Less Than (<)

**Syntax:** `expr1 < expr2`

**Description:** Returns `true` if the first expression is less than the second.

**Parameters:**
- `expr1`, `expr2`: Comparable expressions (numbers, strings, timestamps, durations)

**Return Type:** `bool`

**Examples:**
```cel
5 < 10          // true
"apple" < "banana"  // true (lexicographic)
timestamp("2023-01-01T00:00:00Z") < timestamp("2023-01-02T00:00:00Z")  // true
```

### 4.4 Less Than or Equal (<=)

**Syntax:** `expr1 <= expr2`

**Description:** Returns `true` if the first expression is less than or equal to the second.

**Parameters:**
- `expr1`, `expr2`: Comparable expressions

**Return Type:** `bool`

**Examples:**
```cel
5 <= 5          // true
5 <= 10         // true
"apple" <= "apple"  // true
```

### 4.5 Greater Than (>)

**Syntax:** `expr1 > expr2`

**Description:** Returns `true` if the first expression is greater than the second.

**Parameters:**
- `expr1`, `expr2`: Comparable expressions

**Return Type:** `bool`

**Examples:**
```cel
10 > 5          // true
"banana" > "apple"  // true
duration("2h") > duration("1h")  // true
```

### 4.6 Greater Than or Equal (>=)

**Syntax:** `expr1 >= expr2`

**Description:** Returns `true` if the first expression is greater than or equal to the second.

**Parameters:**
- `expr1`, `expr2`: Comparable expressions

**Return Type:** `bool`

**Examples:**
```cel
10 >= 10        // true
10 >= 5         // true
"banana" >= "banana"  // true
```

## 5. Container Operators

### 5.1 Contains (in)

**Syntax:** `element in container`

**Description:** Returns `true` if the element is contained in the container.

**Parameters:**
- `element`: Element to search for
- `container`: List, map, or string to search in

**Return Type:** `bool`

**Examples:**
```cel
// List membership
1 in [1, 2, 3]          // true
"x" in ["a", "b", "c"]  // false

// Map key existence
"key" in {"key": "value"}  // true
"missing" in {"key": "value"}  // false

// String substring
"ell" in "hello"        // true
"xyz" in "hello"        // false
```

### 5.2 Index ([])

**Syntax:** `container[index]`

**Description:** Accesses an element in a list or map by index/key.

**Parameters:**
- `container`: List or map
- `index`: Integer index (for lists) or key (for maps)

**Return Type:** Type of the container element

**Examples:**
```cel
// List indexing
[1, 2, 3][0]           // 1
["a", "b", "c"][2]     // "c"

// Map access
{"name": "Alice", "age": 30}["name"]  // "Alice"
{"x": 10, "y": 20}["x"]               // 10
```

### 5.3 Size (size)

**Syntax:** `size(container)`

**Description:** Returns the number of elements in a container.

**Parameters:**
- `container`: List, map, string, or bytes

**Return Type:** `int`

**Examples:**
```cel
size([1, 2, 3])         // 3
size({"a": 1, "b": 2})  // 2
size("hello")           // 5
size(b"data")           // 4
size([])                // 0
```

## 6. String Functions

### 6.1 Contains (contains)

**Syntax:** `string.contains(substring)`

**Description:** Returns `true` if the string contains the specified substring.

**Parameters:**
- `string`: String to search in
- `substring`: Substring to search for

**Return Type:** `bool`

**Examples:**
```cel
"hello world".contains("world")  // true
"hello world".contains("xyz")    // false
"".contains("")                  // true
```

### 6.2 Starts With (startsWith)

**Syntax:** `string.startsWith(prefix)`

**Description:** Returns `true` if the string starts with the specified prefix.

**Parameters:**
- `string`: String to check
- `prefix`: Prefix to check for

**Return Type:** `bool`

**Examples:**
```cel
"hello world".startsWith("hello")  // true
"hello world".startsWith("world")  // false
"".startsWith("")                  // true
```

### 6.3 Ends With (endsWith)

**Syntax:** `string.endsWith(suffix)`

**Description:** Returns `true` if the string ends with the specified suffix.

**Parameters:**
- `string`: String to check
- `suffix`: Suffix to check for

**Return Type:** `bool`

**Examples:**
```cel
"hello world".endsWith("world")  // true
"hello world".endsWith("hello")  // false
"".endsWith("")                  // true
```

### 6.4 Regular Expression Match (matches)

**Syntax:** `string.matches(pattern)`

**Description:** Returns `true` if the string matches the regular expression pattern.

**Parameters:**
- `string`: String to match against
- `pattern`: Regular expression pattern

**Return Type:** `bool`

**Examples:**
```cel
"hello123".matches(r"[a-z]+\d+")     // true
"user@example.com".matches(r".*@.*") // true
"abc".matches(r"\d+")                // false
```

## 7. Type Conversion Functions

### 7.1 String Conversion (string)

**Syntax:** `string(value)`

**Description:** Converts a value to its string representation.

**Parameters:**
- `value`: Value to convert

**Return Type:** `string`

**Examples:**
```cel
string(42)        // "42"
string(3.14)      // "3.14"
string(true)      // "true"
string([1, 2, 3]) // "[1, 2, 3]"
```

### 7.2 Integer Conversion (int)

**Syntax:** `int(value)`

**Description:** Converts a value to an integer.

**Parameters:**
- `value`: Numeric value or string

**Return Type:** `int`

**Examples:**
```cel
int(3.14)    // 3
int("42")    // 42
int(true)    // 1
int(false)   // 0
```

### 7.3 Unsigned Integer Conversion (uint)

**Syntax:** `uint(value)`

**Description:** Converts a value to an unsigned integer.

**Parameters:**
- `value`: Numeric value or string

**Return Type:** `uint`

**Examples:**
```cel
uint(42)     // 42u
uint("123")  // 123u
uint(3.14)   // 3u
```

### 7.4 Double Conversion (double)

**Syntax:** `double(value)`

**Description:** Converts a value to a double-precision floating-point number.

**Parameters:**
- `value`: Numeric value or string

**Return Type:** `double`

**Examples:**
```cel
double(42)     // 42.0
double("3.14") // 3.14
double(true)   // 1.0
```

### 7.5 Boolean Conversion (bool)

**Syntax:** `bool(value)`

**Description:** Converts a value to a boolean.

**Parameters:**
- `value`: Value to convert

**Return Type:** `bool`

**Examples:**
```cel
bool(1)        // true
bool(0)        // false
bool("true")   // true
bool("false")  // false
```

### 7.6 Bytes Conversion (bytes)

**Syntax:** `bytes(value)`

**Description:** Converts a string to bytes.

**Parameters:**
- `value`: String value

**Return Type:** `bytes`

**Examples:**
```cel
bytes("hello")  // b"hello"
bytes("")       // b""
```

### 7.7 Duration Conversion (duration)

**Syntax:** `duration(value)`

**Description:** Converts a string to a duration.

**Parameters:**
- `value`: String representation of duration

**Return Type:** `duration`

**Examples:**
```cel
duration("1h")      // 1 hour
duration("30m")     // 30 minutes
duration("1h30m")   // 1 hour 30 minutes
duration("1.5s")    // 1.5 seconds
```

### 7.8 Timestamp Conversion (timestamp)

**Syntax:** `timestamp(value)`

**Description:** Converts a string to a timestamp.

**Parameters:**
- `value`: String representation of timestamp (RFC3339 format)

**Return Type:** `timestamp`

**Examples:**
```cel
timestamp("2023-01-01T00:00:00Z")
timestamp("2023-12-25T12:30:45.123Z")
```

## 8. Time Functions

### 8.1 Duration Functions

**getHours():** Returns the hours component of a duration.
```cel
duration("2h30m").getHours()  // 2
```

**getMinutes():** Returns the minutes component of a duration.
```cel
duration("2h30m").getMinutes()  // 30
```

**getSeconds():** Returns the seconds component of a duration.
```cel
duration("1m30s").getSeconds()  // 30
```

**getMilliseconds():** Returns the milliseconds component of a duration.
```cel
duration("1.5s").getMilliseconds()  // 500
```

### 8.2 Timestamp Functions

**getFullYear():** Returns the year of a timestamp.
```cel
timestamp("2023-01-01T00:00:00Z").getFullYear()  // 2023
```

**getMonth():** Returns the month (0-11) of a timestamp.
```cel
timestamp("2023-01-01T00:00:00Z").getMonth()  // 0
```

**getDate():** Returns the day of the month of a timestamp.
```cel
timestamp("2023-01-15T00:00:00Z").getDate()  // 15
```

**getHours():** Returns the hours of a timestamp.
```cel
timestamp("2023-01-01T15:30:00Z").getHours()  // 15
```

**getMinutes():** Returns the minutes of a timestamp.
```cel
timestamp("2023-01-01T15:30:45Z").getMinutes()  // 30
```

**getSeconds():** Returns the seconds of a timestamp.
```cel
timestamp("2023-01-01T15:30:45Z").getSeconds()  // 45
```

**getMilliseconds():** Returns the milliseconds of a timestamp.
```cel
timestamp("2023-01-01T15:30:45.123Z").getMilliseconds()  // 123
```

## 9. Utility Functions

### 9.1 Type Function (type)

**Syntax:** `type(value)`

**Description:** Returns the type of a value.

**Parameters:**
- `value`: Any value

**Return Type:** `type`

**Examples:**
```cel
type(42)        // int
type("hello")   // string
type([1, 2, 3]) // list(int)
type({"a": 1})  // map(string, int)
```

### 9.2 Has Function (has)

**Syntax:** `has(message.field)`

**Description:** Returns `true` if a message field is set.

**Parameters:**
- `message.field`: Field access expression

**Return Type:** `bool`

**Examples:**
```cel
has(user.name)     // true if user.name is set
has(user.age)      // true if user.age is set
has(request.auth)  // true if request.auth is set
```

### 9.3 All Function (all)

**Syntax:** `list.all(var, condition)`

**Description:** Returns `true` if the condition is true for all elements in the list.

**Parameters:**
- `list`: List to iterate over
- `var`: Variable name for each element
- `condition`: Boolean expression to evaluate

**Return Type:** `bool`

**Examples:**
```cel
[1, 2, 3, 4].all(x, x > 0)           // true
[2, 4, 6, 8].all(x, x % 2 == 0)      // true
["a", "b", "c"].all(s, size(s) == 1) // true
```

### 9.4 Exists Function (exists)

**Syntax:** `list.exists(var, condition)`

**Description:** Returns `true` if the condition is true for at least one element in the list.

**Parameters:**
- `list`: List to iterate over
- `var`: Variable name for each element
- `condition`: Boolean expression to evaluate

**Return Type:** `bool`

**Examples:**
```cel
[1, 2, 3, 4].exists(x, x > 3)        // true
[1, 3, 5, 7].exists(x, x % 2 == 0)   // false
["apple", "banana"].exists(s, s.startsWith("a"))  // true
```

### 9.5 Exists One Function (exists_one)

**Syntax:** `list.exists_one(var, condition)`

**Description:** Returns `true` if the condition is true for exactly one element in the list.

**Parameters:**
- `list`: List to iterate over
- `var`: Variable name for each element
- `condition`: Boolean expression to evaluate

**Return Type:** `bool`

**Examples:**
```cel
[1, 2, 3, 4].exists_one(x, x > 3)    // true (only 4 > 3)
[1, 2, 3, 4].exists_one(x, x > 2)    // false (both 3 and 4 > 2)
```

### 9.6 Map Function (map)

**Syntax:** `list.map(var, expression)`

**Description:** Creates a new list by applying an expression to each element.

**Parameters:**
- `list`: List to iterate over
- `var`: Variable name for each element
- `expression`: Expression to apply to each element

**Return Type:** `list`

**Examples:**
```cel
[1, 2, 3].map(x, x * 2)              // [2, 4, 6]
["a", "b", "c"].map(s, s + "!")      // ["a!", "b!", "c!"]
[1, 2, 3].map(x, string(x))          // ["1", "2", "3"]
```

### 9.7 Filter Function (filter)

**Syntax:** `list.filter(var, condition)`

**Description:** Creates a new list containing only elements that satisfy the condition.

**Parameters:**
- `list`: List to iterate over
- `var`: Variable name for each element
- `condition`: Boolean expression to filter by

**Return Type:** `list`

**Examples:**
```cel
[1, 2, 3, 4, 5].filter(x, x > 3)     // [4, 5]
[1, 2, 3, 4, 5].filter(x, x % 2 == 0) // [2, 4]
["apple", "banana", "cherry"].filter(s, size(s) > 5)  // ["banana", "cherry"]
```

This comprehensive standard library provides all the essential functions and operators needed for most CEL expressions. The functions are designed to be intuitive and follow common programming patterns while maintaining CEL's focus on safety and determinism. 