# RE Extension

The RE extension provides C++ specific regular expression functions built on top of the RE2 library. This extension offers additional functionality for string pattern matching, extraction, and capture operations.

## Table of Contents

- [RE Extension](#re-extension)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Pattern Extraction - `re.extract()`](#2-pattern-extraction---reextract)
  - [3. Group Capture - `re.capture()` and `re.captureN()`](#3-group-capture---recapture-and-recapturen)
    - [re.capture()](#recapture)
    - [re.captureN()](#recapturen)
  - [4. Usage Examples](#4-usage-examples)
    - [Date Format Conversion](#date-format-conversion)
    - [Email Information Extraction](#email-information-extraction)
    - [Phone Number Formatting](#phone-number-formatting)
    - [URL Parsing](#url-parsing)
    - [Log Parsing](#log-parsing)
    - [Text Cleaning and Formatting](#text-cleaning-and-formatting)
    - [Credit Card Number Masking](#credit-card-number-masking)
    - [Version Number Parsing](#version-number-parsing)
    - [Code Extraction](#code-extraction)

## 1. Overview

The RE extension is based on the high-performance RE2 regular expression library, providing advanced pattern matching and text processing capabilities. It is designed for performance and safety, avoiding catastrophic backtracking issues in regular expressions.

**Key Features:**
- Pattern extraction and rewriting (`re.extract`)
- Group capture (`re.capture`, `re.captureN`)
- Direct error handling
- RE2 library performance and safety

**Note**: This extension is specific to the C++ CEL implementation and requires the RE2 library.

**Regular Expression Syntax**: All regex functions use RE2 syntax. For detailed syntax reference, see: [RE2 Syntax Documentation](https://github.com/google/re2/wiki/Syntax)

**Enabling the Extension**:
```rust
use cel_cxx::env::EnvBuilder;

let env = EnvBuilder::new()
    .with_ext_re(true)
    .build()
    .unwrap();
```

**Error Handling**: The `re` extension functions return errors in the following cases:

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

**Performance Considerations**:
- The RE2 library is used for all regex operations, providing consistent performance and safety
- Regular expressions are compiled for each function call, so consider caching compiled patterns for frequently used expressions
- Complex regex patterns may have performance implications for large input strings

**Comparison with Standard Regex Extension**:

| Feature | `re` Extension (C++) | Standard `regex` Extension |
|---------|---------------------|---------------------------|
| Library | RE2 (C++ specific) | Implementation dependent |
| Functions | `extract`, `capture`, `captureN` | `matches`, `find`, `findAll`, `split` |
| Return Types | String/Map | String/List/Optional |
| Error Handling | Direct errors | Optional types |
| Availability | C++ CEL only | Cross-platform |

## 2. Pattern Extraction - `re.extract()`

**Syntax:** `re.extract(target, regex, rewrite) -> string`

**Description:** Extracts and rewrites matched groups from a target string using a regular expression pattern.

**Parameters:**
- `target` (string): The input string to search
- `regex` (string): The regular expression pattern (supports capture groups)
- `rewrite` (string): The rewrite template using captured groups (use `\1`, `\2`, etc.)

**Returns:** A string with the extracted and rewritten content

**Examples:**
```cel
re.extract("Hello World", r"(\w+) (\w+)", r"\2, \1")
// Returns: "World, Hello"

re.extract("user@example.com", r"(\w+)@(\w+\.\w+)", r"User: \1, Domain: \2")
// Returns: "User: user, Domain: example.com"

re.extract("2023-12-25", r"(\d{4})-(\d{2})-(\d{2})", r"\2/\3/\1")
// Returns: "12/25/2023"
```

## 3. Group Capture - `re.capture()` and `re.captureN()`

### re.capture()

**Syntax:** `re.capture(target, regex) -> string`

**Description:** Captures the first unnamed or named group from a regular expression match.

**Parameters:**
- `target` (string): The input string to search
- `regex` (string): The regular expression pattern with capturing groups

**Returns:** A string containing the first captured group

**Examples:**
```cel
re.capture("user@example.com", r"(\w+)@[\w.]+")
// Returns: "user"

re.capture("Price: $29.99", r"Price: \$(\d+\.\d+)")
// Returns: "29.99"

re.capture("Hello World", r"Hello (\w+)")
// Returns: "World"
```

### re.captureN()

**Syntax:** `re.captureN(target, regex) -> map<string, string>`

**Description:** Captures all groups from a regular expression match and returns them as a map.

**Parameters:**
- `target` (string): The input string to search
- `regex` (string): The regular expression pattern with capturing groups

**Returns:** A map where:
- For named groups: `{group_name: captured_value}`
- For unnamed groups: `{group_index: captured_value}` (1-based indexing)

**Examples:**
```cel
re.captureN("user@example.com", r"(\w+)@([\w.]+)")
// Returns: {"1": "user", "2": "example.com"}

re.captureN("John Doe", r"(?P<first>\w+) (?P<last>\w+)")
// Returns: {"first": "John", "last": "Doe"}

re.captureN("2023-12-25", r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})")
// Returns: {"year": "2023", "month": "12", "day": "25"}

re.captureN("user@example.com", r"(?P<username>[\w.]+)@(?P<domain>[\w.]+)")
// Returns: {"username": "user", "domain": "example.com"}
```

## 4. Usage Examples

### Date Format Conversion
```cel
// Convert date from YYYY-MM-DD to DD/MM/YYYY
cel.bind(date, "2023-12-25",
  re.extract(date, r"(\d{4})-(\d{2})-(\d{2})", r"\3/\2/\1")
)
// Result: "25/12/2023"

// Extract date components
re.captureN("2023-12-25", r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})")
// Returns: {"year": "2023", "month": "12", "day": "25"}
```

### Email Information Extraction
```cel
// Extract username from email
re.capture("john.doe@company.com", r"([^@]+)@")
// Returns: "john.doe"

// Extract username and domain from email address
cel.bind(email, "john.doe@company.com",
  re.captureN(email, r"(?P<username>[\w.]+)@(?P<domain>[\w.]+)")
)
// Result: {"username": "john.doe", "domain": "company.com"}

// Extract domain parts
re.captureN("user@mail.example.com", r"[^@]+@([\w]+)\.([\w.]+)")
// Returns: {"1": "mail", "2": "example.com"}
```

### Phone Number Formatting
```cel
// Format phone number
cel.bind(phone, "1234567890",
  re.extract(phone, r"(\d{3})(\d{3})(\d{4})", r"(\1) \2-\3")
)
// Result: "(123) 456-7890"
```

### URL Parsing
```cel
// Parse URL components
cel.bind(url, "https://www.example.com:8080/path/to/resource?param=value",
  re.captureN(url, r"(?P<protocol>https?)://(?P<host>[\w.]+)(:(?P<port>\d+))?(?P<path>/[^?]*)?(\?(?P<query>.*))?")
)
// Result: {"protocol": "https", "host": "www.example.com", "port": "8080", "path": "/path/to/resource", "query": "param=value"}
```

### Log Parsing
```cel
// Extract log level and message
re.captureN("[ERROR] Database connection failed", r"\[(\w+)\] (.+)")
// Returns: {"1": "ERROR", "2": "Database connection failed"}

// Parse log entry
cel.bind(log, "2023-12-25 10:30:45 [ERROR] Connection timeout in module auth",
  re.captureN(log, r"(?P<date>\d{4}-\d{2}-\d{2}) (?P<time>\d{2}:\d{2}:\d{2}) \[(?P<level>\w+)\] (?P<message>.*)")
)
// Result: {"date": "2023-12-25", "time": "10:30:45", "level": "ERROR", "message": "Connection timeout in module auth"}

// Reformat log entry
re.extract("[INFO] User login successful", r"\[(\w+)\] (.+)", r"\1: \2")
// Returns: "INFO: User login successful"
```

### Text Cleaning and Formatting
```cel
// Clean and reformat text
cel.bind(text, "  Hello,    World!   ",
  cel.bind(cleaned, re.extract(text, r"\s*(.+?)\s*", r"\1"),
    re.extract(cleaned, r"(\w+),\s+(\w+)!", r"\1 \2")
  )
)
// Result: "Hello World"
```

### Credit Card Number Masking
```cel
// Mask credit card number, showing only last four digits
cel.bind(card, "1234567890123456",
  re.extract(card, r"(\d{4})(\d{4})(\d{4})(\d{4})", r"****-****-****-\4")
)
// Result: "****-****-****-3456"
```

### Version Number Parsing
```cel
// Parse semantic version number
cel.bind(version, "v1.2.3-beta.1",
  re.captureN(version, r"v(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(-(?P<prerelease>[\w.]+))?")
)
// Result: {"major": "1", "minor": "2", "patch": "3", "prerelease": "beta.1"}
```

### Code Extraction
```cel
// Extract code blocks from text
cel.bind(text, "The function `calculateSum(a, b)` returns the sum.",
  re.capture(text, r"`([^`]+)`")
)
// Result: "calculateSum(a, b)"
```

The RE extension provides powerful regular expression functionality based on the high-performance RE2 library, offering safe and reliable solutions for complex text processing and data extraction tasks. 