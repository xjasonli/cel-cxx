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
      - [`timestamp`](#timestamp)
      - [`duration`](#duration)
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
      - [Triple-Quoted Strings](#triple-quoted-strings)
      - [Raw Strings](#raw-strings)
      - [Escape Sequences](#escape-sequences)
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

CEL's syntax is defined using EBNF notation, where `|` represents alternatives, `[]` for optional elements, `{}` for repeated elements, and `()` for grouping:

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

**Implementation Requirements:**

CEL implementations are required to support at least:
- **24-32 repetitions** of repeating rules:
  - 32 terms separated by `||` in a row
  - 32 terms separated by `&&` in a row
  - 32 function call arguments
  - List literals with 32 elements
  - Map or message literals with 32 fields
  - 24 consecutive ternary conditionals `?:`
  - 24 binary arithmetic operators of the same precedence in a row
  - 24 relations in a row
- **12 repetitions** of recursive rules:
  - 12 nested function calls
  - 12 selection (`.`) operators in a row
  - 12 indexing (`[_]`) operators in a row
  - 12 nested list, map, or message literals

### 2.2 Operator Precedence

CEL operators are listed in order of precedence from highest to lowest:

| Precedence | Operator        | Description                    | Associativity |
|------------|-----------------|--------------------------------|---------------|
| 1          | ()              | Function call                  | Left-to-right |
|            | .               | Qualified name or field access |               |
|            | []              | Indexing                       |               |
|            | {}              | Field initialization           |               |
| 2          | - (unary)       | Negation                       | Right-to-left |
|            | !               | Logical NOT                    |               |
| 3          | *               | Multiplication                 | Left-to-right |
|            | /               | Division                       |               |
|            | %               | Remainder                      |               |
| 4          | +               | Addition                       | Left-to-right |
|            | - (binary)      | Subtraction                    |               |
| 5          | == != < > <= >= | Relations                      | Left-to-right |
|            | in              | Inclusion test                 |               |
| 6          | &&              | Logical AND                    | Left-to-right |
| 7          | \|\|            | Logical OR                     | Left-to-right |
| 8          | ?:              | Conditional                    | Right-to-left |

### 2.3 Lexical Rules

The lexical structure of CEL is defined as follows:

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

#### Identifiers
- Must start with a letter (`a-z`, `A-Z`) or underscore (`_`)
- Can contain letters, digits (`0-9`), and underscores
- Case-sensitive
- Cannot be reserved words

#### Comments
- Single-line comments start with `//` and continue to end of line
- Multi-line comments are not supported in the standard

#### Whitespace
- Spaces, tabs, newlines, form feeds, and carriage returns are ignored except for separating tokens

### 2.4 Reserved Words

**Language Keywords** (cannot be used as identifiers, function names, selectors, struct name segments, or field names):
- `false`, `in`, `null`, `true`

**Host Language Reserved Words** (cannot be used as identifiers or function names, but allowed for receiver-call-style functions):
- `as`, `break`, `const`, `continue`, `else`, `for`, `function`, `if`, `import`, `let`, `loop`, `package`, `namespace`, `return`, `var`, `void`, `while`

**Note:** It's generally recommended to avoid using identifiers that are reserved words in programming languages that might embed CEL.

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

#### `timestamp`
Represents a point in time (implemented as `google.protobuf.Timestamp`)

```cel
timestamp("2023-12-25T12:00:00Z")
timestamp("2023-01-01T00:00:00-08:00")  // with timezone
```

**Properties:**
- Range: ["0001-01-01T00:00:00Z", "9999-12-31T23:59:59.999999999Z"]
- Precision: nanoseconds
- Timezone-aware

**Common operations:**
```cel
// Arithmetic with durations
timestamp("2023-01-01T00:00:00Z") + duration("1h")  // Add duration
timestamp("2023-01-01T01:00:00Z") - duration("1h")  // Subtract duration
timestamp("2023-01-01T01:00:00Z") - timestamp("2023-01-01T00:00:00Z")  // Duration between timestamps

// Comparisons
timestamp("2023-01-01T00:00:00Z") < timestamp("2023-01-02T00:00:00Z")  // true

// Field access
timestamp("2023-12-25T12:30:45Z").getFullYear()  // 2023
timestamp("2023-12-25T12:30:45Z").getMonth()     // 11 (December, 0-based)
timestamp("2023-12-25T12:30:45Z").getDate()      // 25
```

#### `duration`
Represents a span of time (implemented as `google.protobuf.Duration`)

```cel
duration("1h")
duration("30m")
duration("1h30m45s")
duration("1.5s")
```

**Properties:**
- Range: approximately ±290 years
- Precision: nanoseconds
- Can be negative

**Supported units:**
- `h` (hours)
- `m` (minutes)
- `s` (seconds)
- `ms` (milliseconds)
- `us` (microseconds)
- `ns` (nanoseconds)

**Common operations:**
```cel
// Arithmetic
duration("1h") + duration("30m")     // duration("1h30m")
duration("2h") - duration("30m")     // duration("1h30m")

// Comparisons
duration("1h") < duration("2h")      // true
duration("60m") == duration("1h")    // true

// Conversions
duration("1h30m").getHours()         // 1 (truncated)
duration("1h30m").getMinutes()       // 90 (total minutes)
duration("1h30m").getSeconds()       // 5400 (total seconds)
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
Key-value associations with `int`, `uint`, `bool`, or `string` keys

```cel
{"key": "value", "number": 42}
{1: "one", 2: "two"}        // int keys
{true: "yes", false: "no"}  // bool keys
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
Represents the absence of a value (type: `null_type`)

```cel
null
```

#### `type`
Represents types themselves as first-class values

```cel
type(42)           // int
type("hello")      // string
type([1, 2, 3])    // list(int)
type({"a": 1})     // map(string, int)

// Type comparisons
type(x) == int
type(y) == string
type(z) == list(int)
```

**Type hierarchy:**
- Basic types: `int`, `uint`, `double`, `bool`, `string`, `bytes`
- Time types: `timestamp`, `duration`
- Container types: `list(T)`, `map(K, V)`
- Special types: `null_type`, `type`
- Message types: Protocol Buffer message names

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

CEL supports several types of string literals:

#### Regular Strings
Quoted string literals are delimited by either single- or double-quote characters. The closing delimiter must match the opening one, and can contain any unescaped character except the delimiter or newlines (CR or LF).

```cel
"hello world"
'single quotes'
"mixed 'quotes' work"
'mixed "quotes" work'
```

#### Triple-Quoted Strings
Triple-quoted string literals are delimited by three single-quotes or three double-quotes, and may contain any unescaped characters except for the delimiter sequence. Triple-quoted strings may contain newlines.

```cel
"""
Multi-line string
with newlines
"""

'''
Another multi-line
string format
'''
```

#### Raw Strings
If preceded by an `r` or `R` character, the string is a _raw_ string and does not interpret escape sequences. Raw strings are useful for expressing strings which themselves must use escape sequences, such as regular expressions or program text.

```cel
r"raw string with \n literal backslashes"
R'another raw string \t with literal escapes'
```

#### Escape Sequences
Strings (except raw strings) can include escape sequences, which are a backslash (`\`) followed by one of the following:

**Punctuation marks representing themselves:**
- `\\`: backslash
- `\?`: question mark
- `\"`: double quote
- `\'`: single quote
- `\``: backtick

**Whitespace codes:**
- `\a`: bell
- `\b`: backspace
- `\f`: form feed
- `\n`: line feed (newline)
- `\r`: carriage return
- `\t`: horizontal tab
- `\v`: vertical tab

**Unicode escapes:**
- `\uXXXX`: Unicode code point in the BMP (4 hex digits)
- `\UXXXXXXXX`: Unicode code point in any plane (8 hex digits)
- `\xXX` or `\XXX`: Unicode code point (2 hex digits)
- `\000` to `\377`: Unicode code point (3 octal digits)

All hexadecimal digits in escape sequences are case-insensitive.

**Examples:**

| CEL Literal | Meaning |
|-------------|---------|
| `""` | Empty string |
| `'""'` | String of two double-quote characters |
| `'''x''x'''` | String of four characters "`x''x`" |
| `"\""` | String of one double-quote character |
| `"\\"` | String of one backslash character |
| `r"\\"` | String of two backslash characters |
| `"line1\nline2"` | Two-line string |
| `"tab\there"` | String with tab character |
| `"unicode: \u0048"` | String with Unicode 'H' |

### 4.3 Bytes Literals

Bytes literals are represented by string literals preceded by a `b` or `B` character. The bytes literal is the sequence of bytes given by the UTF-8 representation of the string literal. In addition, octal escape sequences are interpreted as octet values rather than as Unicode code points. Both raw and multiline string literals can be used for byte literals.

```cel
b"hello"                    // Byte sequence of 97, 98, 99, 100, 101
b"ÿ"                        // Sequence of bytes 195 and 191 (UTF-8 of ÿ)
b"\303\277"                 // Also sequence of bytes 195 and 191
b"\377"                     // Sequence of byte 255 (not UTF-8 of ÿ)
b"\xff"                     // Sequence of byte 255 (not UTF-8 of ÿ)
B'raw bytes with \n'        // Raw bytes literal
```

**Note:** For strings, `\377` represents Unicode code point 255, but for bytes, `b"\377"` represents the byte value 255.

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

A CEL expression is parsed in the scope of a specific protocol buffer package or message, which controls the interpretation of names. The scope is set by the application context of an expression. A CEL expression can contain simple names (e.g., `a`) or qualified names (e.g., `a.b`).

### 6.1 Resolution Rules

The meaning of names in CEL expressions is determined by one or more of the following:

- **Variables and Functions**: Simple names refer to variables in the execution context, standard functions, or other name bindings provided by the CEL application
- **Field selection**: Appending a period and identifier to an expression indicates accessing a field within a protocol buffer or map
- **Protocol buffer package names**: A simple or qualified name could represent an absolute or relative name in the protocol buffer package namespace. Package names must be followed by a message type or enum constant
- **Protocol buffer message types and enum constants**: Following an optional protocol buffer package name, a simple or qualified name could refer to a message type or an enum constant in the package's namespace

**Resolution Process:**

If `a.b` is a name to be resolved in the context of a protobuf declaration with scope `A.B`, then resolution is attempted in the following order:
1. `A.B.a.b`
2. `A.a.b`
3. `a.b`

### 6.2 Absolute Names

To override the default resolution behavior, you can use absolute names by prefixing with a dot (`.`). For example, `.a.b` will only be resolved in the root scope as `a.b`.

### 6.3 Name Types

**Priority Rule**: If name qualification is mixed with field selection, the longest prefix of the name which resolves in the current lexical scope is used. For example, if `a.b.c` resolves to a message declaration, and `a.b` also resolves with `c` as a possible field selection, then `a.b.c` takes priority over the interpretation `(a.b).c`.

#### Variable Names
Variables are bound in the execution environment and can be referenced by their simple names:

```cel
user.name               // Access 'name' field of 'user' variable
request.auth.token      // Nested field access
```

#### Function Names
Functions can be called in two styles:

**Global function style:**
```cel
size(list)              // Global function call
contains(string, substring)
```

**Receiver function style:**
```cel
list.size()             // Receiver-style function call
string.contains(substring)
```

#### Type Names
Type names are used for type conversions and message construction:

```cel
int(value)              // Type conversion
string(42)              // Convert to string
MyMessage{field: value} // Message construction
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
