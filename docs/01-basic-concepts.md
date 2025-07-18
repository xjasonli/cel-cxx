# CEL Basic Concepts and Syntax

## Table of Contents

- [CEL Basic Concepts and Syntax](#cel-basic-concepts-and-syntax)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Syntax](#2-syntax)
    - [2.1 Grammar Rules](#21-grammar-rules)
    - [2.2 Operator Precedence](#22-operator-precedence)
    - [2.3 Lexical Rules](#23-lexical-rules)
      - [Identifiers](#identifiers)
      - [Comments](#comments)
      - [Whitespace](#whitespace)
    - [2.4 Reserved Words](#24-reserved-words)
  - [3. Type System](#3-type-system)
    - [3.1 Basic Types](#31-basic-types)
      - [`bool`](#bool)
      - [`int`](#int)
      - [`uint`](#uint)
      - [`double`](#double)
      - [`string`](#string)
      - [`bytes`](#bytes)
    - [3.2 Composite Types](#32-composite-types)
      - [`list`](#list)
      - [`map`](#map)
      - [`message`](#message)
    - [3.3 Special Types](#33-special-types)
      - [`null`](#null)
      - [`type`](#type)
  - [4. Literals](#4-literals)
    - [4.1 Numeric Literals](#41-numeric-literals)
      - [Integer Literals](#integer-literals)
      - [Unsigned Integer Literals](#unsigned-integer-literals)
      - [Floating-Point Literals](#floating-point-literals)
    - [4.2 String Literals](#42-string-literals)
      - [Regular Strings](#regular-strings)
      - [Escape Sequences](#escape-sequences)
      - [Raw Strings](#raw-strings)
    - [4.3 Bytes Literals](#43-bytes-literals)
    - [4.4 Boolean and Null Literals](#44-boolean-and-null-literals)
  - [5. Expressions](#5-expressions)
    - [5.1 Basic Expressions](#51-basic-expressions)
      - [Arithmetic Operations](#arithmetic-operations)
      - [Comparison Operations](#comparison-operations)
      - [Logical Operations](#logical-operations)
    - [5.2 Conditional Expressions](#52-conditional-expressions)
    - [5.3 Collection Expressions](#53-collection-expressions)
      - [List Operations](#list-operations)
      - [Map Operations](#map-operations)
    - [5.4 Access Expressions](#54-access-expressions)
      - [Field Access](#field-access)
      - [Index Access](#index-access)
      - [Function Calls](#function-calls)
  - [6. Name Resolution](#6-name-resolution)
    - [6.1 Resolution Rules](#61-resolution-rules)
    - [6.2 Absolute Names](#62-absolute-names)
    - [6.3 Name Types](#63-name-types)
      - [Variable Names](#variable-names)
      - [Function Names](#function-names)
      - [Type Names](#type-names)
  - [7. Examples](#7-examples)
    - [7.1 Simple Expressions](#71-simple-expressions)
    - [7.2 Complex Expressions](#72-complex-expressions)
    - [7.3 Practical Application Examples](#73-practical-application-examples)
      - [Access Control](#access-control)
      - [Data Validation](#data-validation)
      - [Resource Filtering](#resource-filtering)

## 1. Overview

Common Expression Language (CEL) is a simple expression language built on Protocol Buffer types. CEL has the following characteristics:

- **Memory Safe**: Programs cannot access unrelated memory
- **Side-effect Free**: CEL programs only compute output from input
- **Terminating**: CEL programs cannot loop infinitely
- **Strongly Typed**: Values have well-defined types, operators and functions check argument types
- **Dynamically Typed**: Types are associated with values, not variables or expressions
- **Gradually Typed**: Optional type checking phase can detect and reject programs that violate type constraints before runtime

## 2. Syntax

### 2.1 Grammar Rules

CEL's syntax is defined using EBNF notation:

```grammar
expr          = conditionalOr
conditionalOr = conditionalAnd ('||' conditionalAnd)*
conditionalAnd = relation ('&&' relation)*
relation      = calc (('<' | '<=' | '>=' | '>' | '==' | '!=' | 'in') calc)*
calc          = unary (('+' | '-') unary)*
unary         = member
              | '!' member
              | '-' member
member        = primary
              | member '.' IDENT
              | member '.' IDENT '(' exprList? ')'
              | member '[' expr ']'
              | member '{' fieldInits? '}'
primary       = '.'? IDENT
              | '.'? IDENT '(' exprList? ')'
              | '(' expr ')'
              | '[' exprList? ']'
              | '{' mapInits? '}'
              | literal
```

### 2.2 Operator Precedence

CEL operators are listed in order of precedence from highest to lowest:

1. **Member access**: `.`, `[]`, `()` (function calls)
2. **Unary operators**: `!`, `-` (unary minus)
3. **Multiplicative**: `*`, `/`, `%`
4. **Additive**: `+`, `-`
5. **Relational**: `<`, `<=`, `>=`, `>`, `==`, `!=`, `in`
6. **Logical AND**: `&&`
7. **Logical OR**: `||`
8. **Conditional**: `? :`

### 2.3 Lexical Rules

#### Identifiers
- Must start with a letter or underscore
- Can contain letters, digits, and underscores
- Case-sensitive

#### Comments
- Single-line comments start with `//`
- Multi-line comments are enclosed in `/* */`

#### Whitespace
- Spaces, tabs, and newlines are ignored except for separating tokens

### 2.4 Reserved Words

The following words are reserved and cannot be used as identifiers:

- `true`, `false`, `null`
- `in`
- `as`
- `break`, `const`, `continue`, `else`, `for`, `function`, `if`, `import`, `let`, `loop`, `package`, `namespace`, `return`, `var`, `void`, `while`

## 3. Type System

### 3.1 Basic Types

CEL supports the following basic types:

#### `bool`
Boolean values: `true` or `false`

```cel
true && false  // false
!true          // false
```

#### `int`
64-bit signed integers

```cel
42
-17
0
```

#### `uint`
64-bit unsigned integers

```cel
42u
0u
```

#### `double`
64-bit IEEE 754 floating-point numbers

```cel
3.14
-2.5
1e10
```

#### `string`
UTF-8 encoded text

```cel
"hello world"
'single quotes'
"unicode: \u0048\u0065\u006c\u006c\u006f"
```

#### `bytes`
Byte sequences

```cel
b"hello"
b'\x48\x65\x6c\x6c\x6f'
```

### 3.2 Composite Types

#### `list`
Ordered collections of values

```cel
[1, 2, 3]
["a", "b", "c"]
[]  // empty list
```

#### `map`
Key-value associations

```cel
{"key": "value", "number": 42}
{}  // empty map
```

#### `message`
Protocol Buffer message types

```cel
MyMessage{
  field1: "value",
  field2: 42
}
```

### 3.3 Special Types

#### `null`
Represents the absence of a value

```cel
null
```

#### `type`
Represents types themselves

```cel
int
string
list(int)
```

## 4. Literals

### 4.1 Numeric Literals

#### Integer Literals
```cel
42          // decimal
0x2A        // hexadecimal
0o52        // octal
0b101010    // binary
```

#### Unsigned Integer Literals
```cel
42u         // decimal unsigned
0x2Au       // hexadecimal unsigned
```

#### Floating-Point Literals
```cel
3.14
.5
2.
1e10
1.5e-3
```

### 4.2 String Literals

#### Regular Strings
```cel
"hello world"
'single quotes'
```

#### Escape Sequences
```cel
"line1\nline2"      // newline
"tab\there"         // tab
"quote: \""         // escaped quote
"unicode: \u0048"   // unicode escape
```

#### Raw Strings
```cel
r"raw string with \n literal backslashes"
```

### 4.3 Bytes Literals

```cel
b"hello"
b'\x48\x65\x6c\x6c\x6f'
b"\110\145\154\154\157"  // octal escapes
```

### 4.4 Boolean and Null Literals

```cel
true
false
null
```

## 5. Expressions

### 5.1 Basic Expressions

#### Arithmetic Operations
```cel
1 + 2           // 3
5 - 3           // 2
4 * 2           // 8
10 / 3          // 3
10 % 3          // 1
```

#### Comparison Operations
```cel
5 > 3           // true
5 >= 5          // true
3 < 5           // true
3 <= 3          // true
5 == 5          // true
5 != 3          // true
```

#### Logical Operations
```cel
true && false   // false
true || false   // true
!true           // false
```

### 5.2 Conditional Expressions

```cel
condition ? value_if_true : value_if_false
```

Examples:
```cel
5 > 3 ? "yes" : "no"        // "yes"
x > 0 ? x : -x              // absolute value
```

### 5.3 Collection Expressions

#### List Operations
```cel
[1, 2, 3].size()           // 3
[1, 2, 3][0]               // 1
1 in [1, 2, 3]             // true
```

#### Map Operations
```cel
{"a": 1, "b": 2}.size()    // 2
{"a": 1, "b": 2}["a"]      // 1
"a" in {"a": 1, "b": 2}    // true
```

### 5.4 Access Expressions

#### Field Access
```cel
message.field
message.nested.field
```

#### Index Access
```cel
list[0]
map["key"]
```

#### Function Calls
```cel
size(list)
substring("hello", 1, 3)
```

## 6. Name Resolution

### 6.1 Resolution Rules

CEL resolves names using the following priority order:

1. **Local variables** (in function scope)
2. **Function parameters**
3. **Global variables**
4. **Functions**
5. **Types**
6. **Enum values**

### 6.2 Absolute Names

Names can be qualified with a leading dot to indicate absolute resolution:

```cel
.fully.qualified.name
```

### 6.3 Name Types

#### Variable Names
```cel
my_variable
camelCase
```

#### Function Names
```cel
size()
substring()
matches()
```

#### Type Names
```cel
int
string
MyMessage
```

## 7. Examples

### 7.1 Simple Expressions

```cel
// Basic arithmetic
2 + 3 * 4                   // 14

// String operations
"Hello, " + "World!"        // "Hello, World!"

// Boolean logic
true && (false || true)     // true

// Comparisons
5 > 3 && 2 < 4             // true
```

### 7.2 Complex Expressions

```cel
// Conditional with complex condition
user.age >= 18 && user.status == "active" ? "allowed" : "denied"

// List comprehension-like operations
[1, 2, 3, 4, 5].filter(x, x > 2).map(x, x * 2)  // [6, 8, 10]

// Map operations
{"users": [{"name": "Alice"}, {"name": "Bob"}]}.users.map(u, u.name)
```

### 7.3 Practical Application Examples

#### Access Control
```cel
// Check if user has required role
user.role in ["admin", "moderator"] && resource.public == false

// Time-based access
now() < resource.expiry_time && user.subscription.active
```

#### Data Validation
```cel
// Email validation
email.matches(r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$')

// Age validation
user.age >= 13 && user.age <= 120
```

#### Resource Filtering
```cel
// Filter based on user permissions
resource.owner == user.id || 
(resource.shared && user.id in resource.allowed_users)

// Content filtering
content.tags.exists(tag, tag in user.interests) && 
content.mature_content == false
``` 