# CEL C++ Regular Expression Extension (re)

## Table of Contents

- [CEL C++ Regular Expression Extension (re)](#cel-c-regular-expression-extension-re)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Enabling the Extension](#enabling-the-extension)
  - [Functions](#functions)
    - [re.extract](#reextract)
    - [re.capture](#recapture)
    - [re.captureN](#recapturen)
  - [Error Handling](#error-handling)
  - [Performance Considerations](#performance-considerations)
  - [Comparison with Standard Regex Extension](#comparison-with-standard-regex-extension)
  - [Examples](#examples)
    - [Email Validation and Extraction](#email-validation-and-extraction)
    - [Date Format Conversion](#date-format-conversion)
    - [Log Parsing](#log-parsing)


## Overview

The `re` extension provides C++ specific regular expression functions that are built on top of the RE2 library. This extension is distinct from the standard regex extension and offers additional functionality for string pattern matching, extraction, and capture operations.

**Note**: This extension is specific to the C++ CEL implementation and requires the RE2 library.

## Enabling the Extension

To enable the `re` extension in your CEL environment:

```rust
use cel_cxx::env::EnvBuilder;

let env = EnvBuilder::new()
    .with_ext_re(true)
    .build()
    .unwrap();
```

## Functions

### re.extract

Extracts and rewrites matched groups from a target string using a regular expression pattern.

**Syntax**: `re.extract(target, regex, rewrite) -> string`

**Parameters**:
- `target` (string): The input string to search
- `regex` (string): The regular expression pattern
- `rewrite` (string): The rewrite template using captured groups

**Returns**: A string with the extracted and rewritten content

**Example**:
```cel
re.extract("Hello World", r"(\w+) (\w+)", r"\2, \1")
// Returns: "World, Hello"

re.extract("2023-12-25", r"(\d{4})-(\d{2})-(\d{2})", r"\2/\3/\1")
// Returns: "12/25/2023"
```

### re.capture

Captures the first unnamed or named group from a regular expression match.

**Syntax**: `re.capture(target, regex) -> string`

**Parameters**:
- `target` (string): The input string to search
- `regex` (string): The regular expression pattern with capturing groups

**Returns**: A string containing the first captured group

**Example**:
```cel
re.capture("user@example.com", r"(\w+)@[\w.]+")
// Returns: "user"

re.capture("Price: $29.99", r"Price: \$(\d+\.\d+)")
// Returns: "29.99"
```

### re.captureN

Captures all groups from a regular expression match and returns them as a map.

**Syntax**: `re.captureN(target, regex) -> map<string, string>`

**Parameters**:
- `target` (string): The input string to search
- `regex` (string): The regular expression pattern with capturing groups

**Returns**: A map where:
- For named groups: `{group_name: captured_value}`
- For unnamed groups: `{group_index: captured_value}` (1-based indexing)

**Example**:
```cel
re.captureN("user@example.com", r"(\w+)@([\w.]+)")
// Returns: {"1": "user", "2": "example.com"}

re.captureN("John Doe", r"(?P<first>\w+) (?P<last>\w+)")
// Returns: {"first": "John", "last": "Doe"}
```

## Error Handling

The `re` extension functions return errors in the following cases:

1. **Invalid Regex**: When the provided regular expression is malformed
2. **No Match**: When the pattern doesn't match the target string
3. **No Capturing Groups**: When using `re.captureN` with a regex that has no capturing groups

**Example Error Cases**:
```cel
re.extract("test", "[", "replacement")
// Error: "Given Regex is Invalid"

re.capture("no match", r"(\d+)")
// Error: "Unable to capture groups for the given regex"

re.captureN("test", r"test")
// Error: "Capturing groups were not found in the given regex"
```

## Performance Considerations

- The RE2 library is used for all regex operations, providing consistent performance and safety
- Regular expressions are compiled for each function call, so consider caching compiled patterns for frequently used expressions
- Complex regex patterns may have performance implications for large input strings

## Comparison with Standard Regex Extension

| Feature | `re` Extension (C++) | Standard `regex` Extension |
|---------|---------------------|---------------------------|
| Library | RE2 (C++ specific) | Implementation dependent |
| Functions | `extract`, `capture`, `captureN` | `matches`, `find`, `findAll`, `split` |
| Return Types | String/Map | String/List/Optional |
| Error Handling | Direct errors | Optional types |
| Availability | C++ CEL only | Cross-platform |

## Examples

### Email Validation and Extraction
```cel
// Extract username from email
re.capture("john.doe@company.com", r"([^@]+)@")
// Returns: "john.doe"

// Extract domain parts
re.captureN("user@mail.example.com", r"[^@]+@([\w]+)\.([\w.]+)")
// Returns: {"1": "mail", "2": "example.com"}
```

### Date Format Conversion
```cel
// Convert ISO date to US format
re.extract("2023-12-25", r"(\d{4})-(\d{2})-(\d{2})", r"\2/\3/\1")
// Returns: "12/25/2023"

// Extract date components
re.captureN("2023-12-25", r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})")
// Returns: {"year": "2023", "month": "12", "day": "25"}
```

### Log Parsing
```cel
// Extract log level and message
re.captureN("[ERROR] Database connection failed", r"\[(\w+)\] (.+)")
// Returns: {"1": "ERROR", "2": "Database connection failed"}

// Reformat log entry
re.extract("[INFO] User login successful", r"\[(\w+)\] (.+)", r"\1: \2")
// Returns: "INFO: User login successful"
``` 