# CEL Standard Library

## Table of Contents

- [CEL Standard Library](#cel-standard-library)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Presence and Comprehension Macros](#2-presence-and-comprehension-macros)
    - [2.1 Has Function (has)](#21-has-function-has)
    - [2.2 All Function (all)](#22-all-function-all)
    - [2.3 Exists Function (exists)](#23-exists-function-exists)
    - [2.4 Exists One Function (exists\_one)](#24-exists-one-function-exists_one)
    - [2.5 Filter Function (filter)](#25-filter-function-filter)
    - [2.6 Map Function (map)](#26-map-function-map)
  - [3. Logical Operators](#3-logical-operators)
    - [3.1 Logical NOT (!)](#31-logical-not-)
    - [3.2 Logical AND (\&\&)](#32-logical-and-)
    - [3.3 Logical OR (||)](#33-logical-or-)
    - [3.4 Conditional Operator (? :)](#34-conditional-operator--)
  - [4. Arithmetic Operators](#4-arithmetic-operators)
    - [4.1 Negation (-)](#41-negation--)
    - [4.2 Addition (+)](#42-addition-)
    - [4.3 Subtraction (-)](#43-subtraction--)
    - [4.4 Multiplication (\*)](#44-multiplication-)
    - [4.5 Division (/)](#45-division-)
    - [4.6 Modulo (%)](#46-modulo-)
  - [5. Comparison Operators](#5-comparison-operators)
    - [5.1 Equality (==)](#51-equality-)
    - [5.2 Inequality (!=)](#52-inequality-)
    - [5.3 Less Than (\<)](#53-less-than-)
    - [5.4 Less Than or Equal (\<=)](#54-less-than-or-equal-)
    - [5.5 Greater Than (\>)](#55-greater-than-)
    - [5.6 Greater Than or Equal (\>=)](#56-greater-than-or-equal-)
  - [6. List Operators](#6-list-operators)
    - [6.1 List Indexing (\[\])](#61-list-indexing-)
    - [6.2 List Membership (in)](#62-list-membership-in)
    - [6.3 List Size (size)](#63-list-size-size)
  - [7. Map Operators](#7-map-operators)
    - [7.1 Map Indexing (\[\])](#71-map-indexing-)
    - [7.2 Map Key Membership (in)](#72-map-key-membership-in)
    - [7.3 Map Size (size)](#73-map-size-size)
  - [8. Bytes Functions](#8-bytes-functions)
    - [8.1 Bytes Size (size)](#81-bytes-size-size)
  - [9. String Functions](#9-string-functions)
    - [9.1 Contains (contains)](#91-contains-contains)
    - [9.2 Starts With (startsWith)](#92-starts-with-startswith)
    - [9.3 Ends With (endsWith)](#93-ends-with-endswith)
    - [9.4 Regular Expression Match (matches)](#94-regular-expression-match-matches)
    - [9.5 String Size (size)](#95-string-size-size)
  - [10. Date/Time Functions](#10-datetime-functions)
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
  - [11. Types and Conversions](#11-types-and-conversions)
    - [11.1 String Conversion (string)](#111-string-conversion-string)
    - [11.2 Integer Conversion (int)](#112-integer-conversion-int)
    - [11.3 Unsigned Integer Conversion (uint)](#113-unsigned-integer-conversion-uint)
    - [11.4 Double Conversion (double)](#114-double-conversion-double)
    - [11.5 Boolean Conversion (bool)](#115-boolean-conversion-bool)
    - [11.6 Bytes Conversion (bytes)](#116-bytes-conversion-bytes)
    - [11.7 Duration Conversion (duration)](#117-duration-conversion-duration)
    - [11.8 Timestamp Conversion (timestamp)](#118-timestamp-conversion-timestamp)
    - [11.9 Type Function (type)](#119-type-function-type)
    - [11.10 Dynamic Type (dyn)](#1110-dynamic-type-dyn)

## 1. Overview

The CEL standard library provides a comprehensive set of built-in functions and operators for working with various data types. These functions are automatically available in all CEL environments and cover common operations for strings, numbers, collections, and more.

The standard library is organized into the following categories:
- **Presence and Comprehension Macros** for testing field presence and list/map comprehensions
- **Logical operators** for boolean operations
- **Arithmetic operators** for numeric calculations
- **Comparison operators** for value comparisons
- **List operators** for working with lists
- **Map operators** for working with maps
- **Bytes functions** for byte sequence operations
- **String functions** for text manipulation
- **Date/Time functions** for working with timestamps and durations
- **Types and conversions** for type checking and conversion

## 2. Presence and Comprehension Macros

### 2.1 Has Function (has)

**Syntax:** `has(message.field)`

**Description:** Checks if a field exists within a message. This macro supports proto2, proto3, and map key accesses. Only map accesses using the select notation are supported.

**Parameters:**
- `message.field`: Field access expression

**Return Type:** `bool`

**Examples:**
```cel
// `true` if the 'address' field exists in the 'user' message
has(user.address)
// `true` if map 'm' has a key named 'key_name' defined. The value may be null
// as null does not connote absence in CEL.
has(m.key_name)
// `false` if the 'items' field is not set in the 'order' message
has(order.items)
// `false` if the 'user_id' key is not present in the 'sessions' map
has(sessions.user_id)
```

### 2.2 All Function (all)

**Syntax:** `list.all(var, predicate)` or `map.all(var, predicate)`

**Description:** Tests whether all elements in the input list or all keys in a map satisfy the given predicate. The all macro behaves in a manner consistent with the Logical AND operator including in how it absorbs errors and short-circuits.

**Parameters:**
- `var`: Variable name for each element/key
- `predicate`: Boolean expression using the variable

**Return Type:** `bool`

**Examples:**
```cel
[1, 2, 3].all(x, x > 0) // true
[1, 2, 0].all(x, x > 0) // false
['apple', 'banana', 'cherry'].all(fruit, fruit.size() > 3) // true
[3.14, 2.71, 1.61].all(num, num < 3.0) // false
{'a': 1, 'b': 2, 'c': 3}.all(key, key != 'b') // false
```

### 2.3 Exists Function (exists)

**Syntax:** `list.exists(var, predicate)` or `map.exists(var, predicate)`

**Description:** Tests whether any value in the list or any key in the map satisfies the predicate expression. The exists macro behaves in a manner consistent with the Logical OR operator including in how it absorbs errors and short-circuits.

**Parameters:**
- `var`: Variable name for each element/key
- `predicate`: Boolean expression using the variable

**Return Type:** `bool`

**Examples:**
```cel
[1, 2, 3].exists(i, i % 2 != 0) // true
[].exists(i, i > 0) // false
[0, -1, 5].exists(num, num < 0) // true
{'x': 'foo', 'y': 'bar'}.exists(key, key.startsWith('z')) // false
```

### 2.4 Exists One Function (exists_one)

**Syntax:** `list.exists_one(var, predicate)` or `map.exists_one(var, predicate)`

**Description:** Tests whether exactly one list element or map key satisfies the predicate expression. This macro does not short-circuit in order to remain consistent with logical operators being the only operators which can absorb errors within CEL.

**Parameters:**
- `var`: Variable name for each element/key
- `predicate`: Boolean expression using the variable

**Return Type:** `bool`

**Examples:**
```cel
[1, 2, 2].exists_one(i, i < 2) // true
{'a': 'hello', 'aa': 'hellohello'}.exists_one(k, k.startsWith('a')) // false
[1, 2, 3, 4].exists_one(num, num % 2 == 0) // false
```

### 2.5 Filter Function (filter)

**Syntax:** `list.filter(var, predicate)` or `map.filter(var, predicate)`

**Description:** Returns a list containing only the elements from the input list that satisfy the given predicate, or for maps, returns a list of keys that satisfy the predicate.

**Parameters:**
- `var`: Variable name for each element/key
- `predicate`: Boolean expression using the variable

**Return Type:** `list`

**Examples:**
```cel
[1, 2, 3].filter(x, x > 1) // [2, 3]
['cat', 'dog', 'bird', 'fish'].filter(pet, pet.size() == 3) // ['cat', 'dog']
[{'a': 10, 'b': 5, 'c': 20}].map(m, m.filter(key, m[key] > 10)) // [['c']]
```

### 2.6 Map Function (map)

**Syntax:** `list.map(var, transform)` or `map.map(var, transform)`

**Description:** Returns a list where each element is the result of applying the transform expression to the corresponding input list element or input map key.

There are two forms of the map macro:
- The three argument form transforms all elements.
- The four argument form transforms only elements which satisfy the predicate.

**Parameters:**
- `var`: Variable name for each element/key
- `transform`: Expression to transform each element
- `predicate` (optional): Boolean expression to filter elements

**Return Type:** `list`

**Examples:**
```cel
[1, 2, 3].map(x, x * 2) // [2, 4, 6]
[5, 10, 15].map(x, x / 5) // [1, 2, 3]
['apple', 'banana'].map(fruit, fruit.upperAscii()) // ['APPLE', 'BANANA']
[1, 2, 3, 4].map(num, num % 2 == 0, num * 2) // [4, 8]
```

## 3. Logical Operators

### 3.1 Logical NOT (!)

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

### 3.2 Logical AND (&&)

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

### 3.3 Logical OR (||)

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

### 3.4 Conditional Operator (? :)

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

## 4. Arithmetic Operators

### 4.1 Negation (-)

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

### 4.2 Addition (+)

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

### 4.3 Subtraction (-)

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

### 4.4 Multiplication (*)

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

### 4.5 Division (/)

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

### 4.6 Modulo (%)

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

## 5. Comparison Operators

### 5.1 Equality (==)

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

### 5.2 Inequality (!=)

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

### 5.3 Less Than (<)

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

### 5.4 Less Than or Equal (<=)

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

### 5.5 Greater Than (>)

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

### 5.6 Greater Than or Equal (>=)

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

## 6. List Operators

### 6.1 List Indexing ([])

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

### 6.2 List Membership (in)

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

### 6.3 List Size (size)

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

## 7. Map Operators

### 7.1 Map Indexing ([])

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

### 7.2 Map Key Membership (in)

**Syntax:** `key in container`

**Description:** Returns `true` if the key exists in the map.

**Parameters:**
- `key`: Key to check
- `container`: Map

**Return Type:** `bool`

**Examples:**
```cel
"key" in {"key": "value"}  // true
"missing" in {"key": "value"}  // false
```

### 7.3 Map Size (size)

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

## 8. Bytes Functions

### 8.1 Bytes Size (size)

**Syntax:** `bytes.size()` or `size(bytes)`

**Description:** Determine the number of bytes in the sequence.

**Parameters:**
- `bytes`: Byte sequence

**Return Type:** `int`

**Examples:**
```cel
b'hello'.size() // 5
size(b'world!') // 6
size(b'\xF0\x9F\xA4\xAA') // 4
```

## 9. String Functions

### 9.1 Contains (contains)

**Syntax:** `string.contains(substring)`

**Description:** Tests whether the string operand contains the substring. Time complexity is proportional to the product of the sizes of the arguments.

**Parameters:**
- `substring`: String to search for

**Return Type:** `bool`

**Examples:**
```cel
"hello world".contains("world") // true
"foobar".contains("baz") // false
```

### 9.2 Starts With (startsWith)

**Syntax:** `string.startsWith(prefix)`

**Description:** Tests whether the string operand starts with the specified prefix. Average time complexity is linear with respect to the size of the prefix. Worst-case time complexity is proportional to the product of the sizes of the arguments.

**Parameters:**
- `prefix`: String prefix to check for

**Return Type:** `bool`

**Examples:**
```cel
"hello world".startsWith("hello") // true
"foobar".startsWith("foo") // true
```

### 9.3 Ends With (endsWith)

**Syntax:** `string.endsWith(suffix)`

**Description:** Tests whether the string operand ends with the specified suffix. Average time complexity is linear with respect to the size of the suffix string. Worst-case time complexity is proportional to the product of the sizes of the arguments.

**Parameters:**
- `suffix`: String suffix to check for

**Return Type:** `bool`

**Examples:**
```cel
"hello world".endsWith("world") // true
"foobar".endsWith("bar") // true
```

### 9.4 Regular Expression Match (matches)

**Syntax:** `matches(string, pattern)` or `string.matches(pattern)`

**Description:** Tests whether a string matches a given RE2 regular expression. Time complexity is proportional to the product of the sizes of the arguments as guaranteed by the RE2 design.

**Regular Expression Syntax:** For detailed syntax reference, see: [RE2 Syntax Documentation](https://github.com/google/re2/wiki/Syntax)

**Parameters:**
- `pattern`: Regular expression pattern

**Return Type:** `bool`

**Examples:**
```cel
matches("foobar", "foo.*") // true
"foobar".matches("foo.*") // true
```

### 9.5 String Size (size)

**Syntax:** `string.size()` or `size(string)`

**Description:** Determine the length of the string in terms of the number of Unicode codepoints.

**Parameters:**
- `string`: String to measure

**Return Type:** `int`

**Examples:**
```cel
"hello".size() // 5
size("world!") // 6
"fiance\u0301".size() // 7
size(string(b'\xF0\x9F\xA4\xAA')) // 1
```

## 10. Date/Time Functions

All timestamp functions which take accept a timezone argument can use any of the supported [Joda Timezones](https://www.joda.org/joda-time/timezones.html) either using the numeric format or the geographic region.

### 10.1 getDate()

**Syntax:** `timestamp.getDate()` or `timestamp.getDate(timezone)`

**Description:** Get the day of the month from a timestamp (one-based indexing).

**Parameters:**
- `timezone` (optional): Timezone string

**Return Type:** `int`

**Examples:**
```cel
timestamp("2023-12-25T00:00:00Z").getDate() // 25
timestamp("2023-12-25T00:00:00Z").getDate("America/Los_Angeles") // 24
```

### 10.2 getDayOfMonth()

**Syntax:** `timestamp.getDayOfMonth()` or `timestamp.getDayOfMonth(timezone)`

**Description:** Get the day of the month from a timestamp (zero-based indexing).

**Parameters:**
- `timezone` (optional): Timezone string

**Return Type:** `int`

**Examples:**
```cel
timestamp("2023-12-25T00:00:00Z").getDayOfMonth() // 24
timestamp("2023-12-25T00:00:00Z").getDayOfMonth("America/Los_Angeles") // 23
```

### 10.3 getDayOfWeek()

**Syntax:** `timestamp.getDayOfWeek()` or `timestamp.getDayOfWeek(timezone)`

**Description:** Get the day of the week from a timestamp (zero-based, zero for Sunday).

**Parameters:**
- `timezone` (optional): Timezone string

**Return Type:** `int`

**Examples:**
```cel
timestamp("2023-12-25T12:00:00Z").getDayOfWeek() // 1 (Monday)
```

### 10.4 getDayOfYear()

**Syntax:** `timestamp.getDayOfYear()` or `timestamp.getDayOfYear(timezone)`

**Description:** Get the day of the year from a timestamp (zero-based indexing).

**Parameters:**
- `timezone` (optional): Timezone string

**Return Type:** `int`

**Examples:**
```cel
timestamp("2023-12-25T12:00:00Z").getDayOfYear() // 358
```

### 10.5 getFullYear()

**Syntax:** `timestamp.getFullYear()` or `timestamp.getFullYear(timezone)`

**Description:** Get the year from a timestamp.

**Parameters:**
- `timezone` (optional): Timezone string

**Return Type:** `int`

**Examples:**
```cel
timestamp("2023-12-25T12:00:00Z").getFullYear() // 2023
```

### 10.6 getHours()

**Syntax:** `timestamp.getHours()` or `timestamp.getHours(timezone)` or `duration.getHours()`

**Description:** Get the hour from a timestamp or convert the duration to hours.

**Parameters:**
- `timezone` (optional): Timezone string (for timestamp)

**Return Type:** `int`

**Examples:**
```cel
timestamp("2023-12-25T12:00:00Z").getHours() // 12
duration("3h").getHours() // 3
```

### 10.7 getMilliseconds()

**Syntax:** `timestamp.getMilliseconds()` or `timestamp.getMilliseconds(timezone)` or `duration.getMilliseconds()`

**Description:** Get the milliseconds from a timestamp or the milliseconds portion of the duration.

**Parameters:**
- `timezone` (optional): Timezone string (for timestamp)

**Return Type:** `int`

**Examples:**
```cel
timestamp("2023-12-25T12:00:00.500Z").getMilliseconds() // 500
duration("1.234s").getMilliseconds() // 234
```

### 10.8 getMinutes()

**Syntax:** `timestamp.getMinutes()` or `timestamp.getMinutes(timezone)` or `duration.getMinutes()`

**Description:** Get the minutes from a timestamp or convert a duration to minutes.

**Parameters:**
- `timezone` (optional): Timezone string (for timestamp)

**Return Type:** `int`

**Examples:**
```cel
timestamp("2023-12-25T12:30:00Z").getMinutes() // 30
duration("1h30m").getMinutes() // 90
```

### 10.9 getMonth()

**Syntax:** `timestamp.getMonth()` or `timestamp.getMonth(timezone)`

**Description:** Get the month from a timestamp (zero-based, 0 for January).

**Parameters:**
- `timezone` (optional): Timezone string

**Return Type:** `int`

**Examples:**
```cel
timestamp("2023-12-25T12:00:00Z").getMonth() // 11 (December)
```

### 10.10 getSeconds()

**Syntax:** `timestamp.getSeconds()` or `timestamp.getSeconds(timezone)` or `duration.getSeconds()`

**Description:** Get the seconds from a timestamp or convert the duration to seconds.

**Parameters:**
- `timezone` (optional): Timezone string (for timestamp)

**Return Type:** `int`

**Examples:**
```cel
timestamp("2023-12-25T12:30:30Z").getSeconds() // 30
duration("1m30s").getSeconds() // 90
```

## 11. Types and Conversions

### 11.1 String Conversion (string)

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

### 11.2 Integer Conversion (int)

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

### 11.3 Unsigned Integer Conversion (uint)

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

### 11.4 Double Conversion (double)

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

### 11.5 Boolean Conversion (bool)

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

### 11.6 Bytes Conversion (bytes)

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

### 11.7 Duration Conversion (duration)

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

### 11.8 Timestamp Conversion (timestamp)

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

### 11.9 Type Function (type)

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

### 11.10 Dynamic Type (dyn)

**Syntax:** `dyn(value)`

**Description:** Returns the dynamic type of a value.

**Parameters:**
- `value`: Any value

**Return Type:** `dyn`

**Examples:**
```cel
dyn(42)        // int
dyn("hello")   // string
dyn([1, 2, 3]) // list(int)
dyn({"a": 1})  // map(string, int)
```

This comprehensive standard library provides all the essential functions and operators needed for most CEL expressions. The functions are designed to be intuitive and follow common programming patterns while maintaining CEL's focus on safety and determinism. 