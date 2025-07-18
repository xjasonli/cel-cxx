# Strings Extension

The strings extension provides advanced string manipulation functions that go beyond the basic string operations in the standard library.

## Table of Contents

- [Strings Extension](#strings-extension)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Character Access - `charAt()`](#2-character-access---charat)
    - [charAt()](#charat)
  - [3. String Search](#3-string-search)
    - [3.1 indexOf()](#31-indexof)
    - [3.2 lastIndexOf()](#32-lastindexof)
  - [4. String Extraction - `substring()`](#4-string-extraction---substring)
    - [substring()](#substring)
  - [5. String Quoting - `strings.quote()`](#5-string-quoting---stringsquote)
    - [strings.quote()](#stringsquote)
  - [6. String Trimming - `trim()`](#6-string-trimming---trim)
    - [trim()](#trim)
  - [7. String Joining - `join()`](#7-string-joining---join)
    - [join()](#join)
  - [8. String Splitting - `split()`](#8-string-splitting---split)
    - [split()](#split)
  - [9. Case Conversion](#9-case-conversion)
    - [9.1 lowerAscii()](#91-lowerascii)
    - [9.2 upperAscii()](#92-upperascii)
  - [10. String Replacement - `replace()`](#10-string-replacement---replace)
    - [replace()](#replace)
  - [11. String Formatting - `format()`](#11-string-formatting---format)
    - [format()](#format)
  - [12. String Reversal - `reverse()`](#12-string-reversal---reverse)
    - [reverse()](#reverse)
  - [13. Usage Examples](#13-usage-examples)
    - [Email Processing](#email-processing)
    - [CSV Data Processing](#csv-data-processing)
    - [Text Normalization](#text-normalization)
    - [String Validation](#string-validation)
    - [Template Processing](#template-processing)

## 1. Overview

The strings extension enhances CEL with powerful string manipulation capabilities. All functions are designed to be safe, efficient, and consistent with CEL's philosophy of deterministic, side-effect-free evaluation.

**Enabling the Extension**:
```rust
let env = Env::builder()
    .with_ext_strings(true)
    .build()?;
```

## 2. Character Access - `charAt()`

### charAt()

Gets the character at a specific index.

**Syntax:** `string.charAt(index)`

**Parameters:**
- `string`: The input string
- `index`: Zero-based index of the character to retrieve

**Return Type:** `string` (single character)

**Examples:**
```cel
"Hello".charAt(0)           // "H"
"World".charAt(4)           // "d"
"CEL".charAt(1)             // "E"
"Programming".charAt(7)     // "m"
```

**Error Handling:**
- Returns empty string for out-of-bounds indices
- Negative indices are treated as out-of-bounds

## 3. String Search

### 3.1 indexOf()

Finds the first occurrence of a substring.

**Syntax:** 
- `string.indexOf(substring)`
- `string.indexOf(substring, startIndex)`

**Parameters:**
- `string`: The input string to search in
- `substring`: The substring to find
- `startIndex`: Optional starting position for the search

**Return Type:** `int` (index of first occurrence, or -1 if not found)

**Examples:**
```cel
"hello world".indexOf("o")        // 4
"hello world".indexOf("world")    // 6
"hello world".indexOf("o", 5)     // 7
"hello world".indexOf("xyz")      // -1
"programming".indexOf("gram")     // 3
```

### 3.2 lastIndexOf()

Finds the last occurrence of a substring.

**Syntax:**
- `string.lastIndexOf(substring)`
- `string.lastIndexOf(substring, endIndex)`

**Parameters:**
- `string`: The input string to search in
- `substring`: The substring to find
- `endIndex`: Optional ending position for the search

**Return Type:** `int` (index of last occurrence, or -1 if not found)

**Examples:**
```cel
"hello world".lastIndexOf("o")    // 7
"hello world".lastIndexOf("l")    // 9
"hello world".lastIndexOf("l", 5) // 3
"hello world".lastIndexOf("xyz")  // -1
"banana".lastIndexOf("a")         // 5
```

## 4. String Extraction - `substring()`

### substring()

Extracts a portion of a string.

**Syntax:**
- `string.substring(start)`
- `string.substring(start, end)`

**Parameters:**
- `string`: The input string
- `start`: Starting index (inclusive)
- `end`: Ending index (exclusive, optional)

**Return Type:** `string`

**Examples:**
```cel
"Hello World".substring(0, 5)     // "Hello"
"Hello World".substring(6)        // "World"
"Programming".substring(0, 7)     // "Program"
"CEL Language".substring(4)       // "Language"
```

**Behavior:**
- If `end` is omitted, extracts to the end of the string
- Out-of-bounds indices are clamped to valid ranges
- If `start >= end`, returns empty string

## 5. String Quoting - `strings.quote()`

### strings.quote()

Escapes special characters in a string.

**Syntax:** `strings.quote(string)`

**Parameters:**
- `string`: The input string to quote

**Return Type:** `string` (escaped string)

**Examples:**
```cel
strings.quote("Hello World")       // "Hello World"
strings.quote("Line 1\nLine 2")    // "Line 1\\nLine 2"
strings.quote("Tab\there")         // "Tab\\there"
strings.quote("Quote: \"Hello\"")  // "Quote: \\\"Hello\\\""
strings.quote("Backslash: \\")     // "Backslash: \\\\"
strings.quote("")                  // ""
```

**Escaped Characters:**
- `\n` → `\\n` (newline)
- `\r` → `\\r` (carriage return)
- `\t` → `\\t` (tab)
- `\\` → `\\\\` (backslash)
- `\"` → `\\\"` (double quote)

## 6. String Trimming - `trim()`

### trim()

Removes whitespace from both ends of a string.

**Syntax:** `string.trim()`

**Parameters:**
- `string`: The input string

**Return Type:** `string`

**Examples:**
```cel
"  hello  ".trim()          // "hello"
"\t\nworld\r\n".trim()      // "world"
"   ".trim()                // ""
"NoSpaces".trim()           // "NoSpaces"
"  Leading".trim()          // "Leading"
"Trailing  ".trim()         // "Trailing"
```

**Whitespace Characters:**
- Space (` `)
- Tab (`\t`)
- Newline (`\n`)
- Carriage return (`\r`)
- Form feed (`\f`)
- Vertical tab (`\v`)

## 7. String Joining - `join()`

### join()

Joins a list of strings with a separator.

**Syntax:**
- `list.join()` (no separator)
- `list.join(separator)`

**Parameters:**
- `list`: List of strings to join
- `separator`: Optional separator string

**Return Type:** `string`

**Examples:**
```cel
["a", "b", "c"].join()              // "abc"
["a", "b", "c"].join(", ")          // "a, b, c"
["hello", "world"].join(" ")        // "hello world"
["one", "two", "three"].join(" | ") // "one | two | three"
["single"].join(",")                // "single"
[].join(",")                        // ""
```

## 8. String Splitting - `split()`

### split()

Splits a string into a list of substrings.

**Syntax:**
- `string.split(separator)`
- `string.split(separator, limit)`

**Parameters:**
- `string`: The input string to split
- `separator`: The separator string
- `limit`: Optional maximum number of splits

**Return Type:** `list(string)`

**Examples:**
```cel
"a,b,c".split(",")                  // ["a", "b", "c"]
"a,b,c,d".split(",", 2)             // ["a", "b", "c,d"]
"hello world".split(" ")            // ["hello", "world"]
"a::b::c".split("::")               // ["a", "b", "c"]
"no-separators".split(",")          // ["no-separators"]
"".split(",")                       // [""]
```

**Behavior:**
- If `limit` is specified, at most `limit` splits are performed
- Empty strings between separators are preserved
- If separator is not found, returns list with original string

## 9. Case Conversion

### 9.1 lowerAscii()

Converts ASCII characters to lowercase.

**Syntax:** `string.lowerAscii()`

**Parameters:**
- `string`: The input string

**Return Type:** `string`

**Examples:**
```cel
"HELLO".lowerAscii()        // "hello"
"Hello World".lowerAscii()  // "hello world"
"MiXeD CaSe".lowerAscii()   // "mixed case"
"123ABC".lowerAscii()       // "123abc"
"already lowercase".lowerAscii() // "already lowercase"
```

### 9.2 upperAscii()

Converts ASCII characters to uppercase.

**Syntax:** `string.upperAscii()`

**Parameters:**
- `string`: The input string

**Return Type:** `string`

**Examples:**
```cel
"hello".upperAscii()        // "HELLO"
"Hello World".upperAscii()  // "HELLO WORLD"
"123abc".upperAscii()       // "123ABC"
"ALREADY UPPERCASE".upperAscii() // "ALREADY UPPERCASE"
```

**Note:** Only ASCII characters (A-Z, a-z) are converted. Unicode characters are left unchanged.

## 10. String Replacement - `replace()`

### replace()

Replaces occurrences of a substring with another string.

**Syntax:**
- `string.replace(old, new)` (replace all)
- `string.replace(old, new, count)` (replace up to count)

**Parameters:**
- `string`: The input string
- `old`: The substring to replace
- `new`: The replacement string
- `count`: Optional maximum number of replacements

**Return Type:** `string`

**Examples:**
```cel
"hello world".replace("world", "CEL")     // "hello CEL"
"foo foo foo".replace("foo", "bar")       // "bar bar bar"
"foo foo foo".replace("foo", "bar", 1)    // "bar foo foo"
"no match".replace("xyz", "abc")          // "no match"
"".replace("a", "b")                      // ""
```

## 11. String Formatting - `format()`

### format()

Formats a string using printf-style placeholders.

**Syntax:** `string.format(args)`

**Parameters:**
- `string`: Format string with placeholders
- `args`: List of arguments to substitute

**Return Type:** `string`

**Supported Placeholders:**
- `%s` - String
- `%d` - Integer
- `%f` - Float
- `%.Nf` - Float with N decimal places

**Examples:**
```cel
"Hello, %s!".format(["World"])           // "Hello, World!"
"Number: %d".format([42])                // "Number: 42"
"Float: %.2f".format([3.14159])          // "Float: 3.14"
"%s has %d apples".format(["Alice", 5])  // "Alice has 5 apples"
"No placeholders".format([])             // "No placeholders"
```

## 12. String Reversal - `reverse()`

### reverse()

Reverses the characters in a string.

**Syntax:** `string.reverse()`

**Parameters:**
- `string`: The input string

**Return Type:** `string`

**Examples:**
```cel
"hello".reverse()           // "olleh"
"world".reverse()           // "dlrow"
"racecar".reverse()         // "racecar"
"12345".reverse()           // "54321"
"a".reverse()               // "a"
"".reverse()                // ""
"Hello, World!".reverse()   // "!dlroW ,olleH"
```

## 13. Usage Examples

### Email Processing
```cel
// Extract username and domain from email
cel.bind(email, "  User@Example.Com  ",
  cel.bind(clean, email.trim().lowerAscii(),
    {
      "username": clean.substring(0, clean.indexOf("@")),
      "domain": clean.substring(clean.indexOf("@") + 1)
    }
  )
)
// Result: {"username": "user", "domain": "example.com"}
```

### CSV Data Processing
```cel
// Parse and transform CSV data
"name:John,age:30,city:NYC".split(",")
  .map(item, item.split(":"))
  .map(pair, pair[0].upperAscii() + "=" + pair[1])
  .join(" AND ")
// Result: "NAME=John AND AGE=30 AND CITY=NYC"
```

### Text Normalization
```cel
// Clean and normalize user input
cel.bind(input, "  Hello    World!  ",
  input.trim()
    .replace("  ", " ")
    .replace("!", "")
    .lowerAscii()
)
// Result: "hello world"
```

### String Validation
```cel
// Check if string is a palindrome
cel.bind(text, "racecar",
  text.lowerAscii() == text.lowerAscii().reverse()
)
// Result: true
```

### Template Processing
```cel
// Simple template substitution
"Hello, %s! You have %d new messages.".format([
  user.name.charAt(0).upperAscii() + user.name.substring(1).lowerAscii(),
  user.messageCount
])
```

The strings extension provides a comprehensive set of tools for string manipulation, making it easy to process text data in CEL expressions while maintaining safety and performance. 