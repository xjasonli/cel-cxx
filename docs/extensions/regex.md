# Regular Expression Extension

The regex extension provides advanced regular expression operations for pattern matching and text extraction.

## Table of Contents

- [Regular Expression Extension](#regular-expression-extension)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Pattern Extraction](#2-pattern-extraction)
    - [regex.extract()](#regexextract)
    - [regex.extractAll()](#regexextractall)
  - [3. Pattern Replacement](#3-pattern-replacement)
    - [regex.replace()](#regexreplace)
  - [4. Usage Examples](#4-usage-examples)
    - [Email Validation and Parsing](#email-validation-and-parsing)
    - [Phone Number Extraction](#phone-number-extraction)
    - [Data Cleaning and Transformation](#data-cleaning-and-transformation)
    - [URL Processing](#url-processing)
    - [Text Processing with Capture Groups](#text-processing-with-capture-groups)
    - [Log Parsing](#log-parsing)
    - [Data Validation](#data-validation)
    - [Text Replacement with Limits](#text-replacement-with-limits)
    - [Pattern Matching with Optional Results](#pattern-matching-with-optional-results)

## 1. Overview

The regex extension enables powerful pattern matching and text extraction using regular expressions. All functions use standard regex syntax and provide safe, deterministic operations.

**Note**: This library depends on the CEL optional type. Please ensure that the `cel.OptionalTypes()` is enabled when using regex extensions.

**Enabling the Extension**:
```rust
let env = Env::builder()
    .with_ext_regex(true)
    .build()?;
```

## 2. Pattern Extraction

### regex.extract()

Returns the first match of a regex pattern in a string. If no match is found, it returns an optional none value. An error will be thrown for invalid regex or for multiple capture groups.

**Syntax:** `regex.extract(target, pattern)`

**Parameters:**
- `target`: The input string to search
- `pattern`: Regular expression pattern (raw string recommended)

**Return Type:** `optional<string>` (first match, or optional.none if no match)

**Examples:**
```cel
regex.extract('hello world', 'hello(.*)') == optional.of(' world')
regex.extract('item-A, item-B', 'item-(\\w+)') == optional.of('A')
regex.extract('HELLO', 'hello') == optional.none
regex.extract('testuser@testdomain', '(.*)@([^.]*)') // Runtime Error: multiple capture groups
```

**Behavior:**
- If there is a capturing group, returns the first captured group
- If no capturing group, returns the whole match
- If optional group is empty, returns optional.none
- Supports at most one capturing group (multiple groups cause an error)

### regex.extractAll()

Returns a list of all matches of a regex pattern in a target string. If no matches are found, it returns an empty list. An error will be thrown for invalid regex or for multiple capture groups.

**Syntax:** `regex.extractAll(target, pattern)`

**Parameters:**
- `target`: The input string to search
- `pattern`: Regular expression pattern (raw string recommended)

**Return Type:** `list<string>` (all matches)

**Examples:**
```cel
regex.extractAll('id:123, id:456', 'id:\\d+') == ['id:123', 'id:456']
regex.extractAll('id:123, id:456', 'assa') == []
regex.extractAll('testuser@testdomain', '(.*)@([^.]*)') // Runtime Error: multiple capture groups
```

**Behavior:**
- If there is one capturing group, returns all captured groups
- If no capturing group, returns all whole matches
- Empty captured groups are excluded from results
- Supports at most one capturing group (multiple groups cause an error)

## 3. Pattern Replacement

### regex.replace()

Replaces all non-overlapping substrings of a regex pattern in the target string with a replacement string. Optionally, you can limit the number of replacements by providing a count argument.

**Syntax:**
- `regex.replace(target, pattern, replacement)`
- `regex.replace(target, pattern, replacement, count)`

**Parameters:**
- `target`: The input string
- `pattern`: Regular expression pattern
- `replacement`: The replacement string (supports capture group references)
- `count`: Optional maximum number of replacements (default: all)

**Return Type:** `string`

**Examples:**
```cel
regex.replace('hello world hello', 'hello', 'hi') == 'hi world hi'
regex.replace('banana', 'a', 'x', 0) == 'banana'
regex.replace('banana', 'a', 'x', 1) == 'bxnana'
regex.replace('banana', 'a', 'x', 2) == 'bxnxna'
regex.replace('banana', 'a', 'x', -12) == 'bxnxnx'
regex.replace('foo bar', '(fo)o (ba)r', r'\2 \1') == 'ba fo'
```

**Capture Group References:**
- Use `\1`, `\2`, etc. to reference capture groups in the replacement string
- `\0` refers to the entire match
- Use `\\` to include a literal backslash
- Invalid references (e.g., `\9` when only 2 groups exist) cause errors

**Error Handling:**
```cel
regex.replace('test', '(.)', r'\2') // Runtime Error: invalid replace string
regex.replace('foo bar', '(', '$2 $1') // Runtime Error: invalid regex string
regex.replace('id=123', r'id=(?P<value>\d+)', r'value: \values') // Runtime Error: invalid replace string
```

**Behavior:**
- When count is 0, returns the original string unchanged
- When count is negative, replaces all occurrences
- When count exceeds available matches, replaces all available matches
- Only numeric capture group references (`\1` to `\9`) are supported

## 4. Usage Examples

### Email Validation and Parsing
```cel
// Extract username from email
cel.bind(email, "john.doe@company.com",
  cel.bind(username, regex.extract(email, r"([^@]+)@"),
    username.hasValue() ? {
      "valid": true,
      "username": username.value(),
      "full_email": email
    } : {"valid": false}
  )
)
// Result: {"valid": true, "username": "john.doe", "full_email": "john.doe@company.com"}
```

### Phone Number Extraction
```cel
// Extract all phone numbers from text
cel.bind(text, "Call me at 555-123-4567 or 555-987-6543",
  cel.bind(phones, regex.extractAll(text, r"\d{3}-\d{3}-\d{4}"),
    {
      "found_count": phones.size(),
      "phone_numbers": phones
    }
  )
)
// Result: {"found_count": 2, "phone_numbers": ["555-123-4567", "555-987-6543"]}
```

### Data Cleaning and Transformation
```cel
// Clean and transform user input
cel.bind(messy_input, "  Hello    World!!!  ",
  cel.bind(cleaned, regex.replace(messy_input.trim(), r"\s+", " "),
    cel.bind(final, regex.replace(cleaned, r"!+", ""),
      {
        "original": messy_input,
        "cleaned": final
      }
    )
  )
)
// Result: {"original": "  Hello    World!!!  ", "cleaned": "Hello World"}
```

### URL Processing
```cel
// Extract domain from URLs
cel.bind(urls, [
  "https://www.example.com/path",
  "http://subdomain.test.org/page",
  "https://another-site.net"
],
  urls.map(url, 
    cel.bind(domain, regex.extract(url, r"https?://([^/]+)"),
      domain.hasValue() ? domain.value() : "unknown"
    )
  )
)
// Result: ["www.example.com", "subdomain.test.org", "another-site.net"]
```

### Text Processing with Capture Groups
```cel
// Transform date format using capture groups
cel.bind(date_text, "Today is 2023-12-25 and tomorrow is 2023-12-26",
  regex.replace(date_text, r"(\d{4})-(\d{2})-(\d{2})", r"\3/\2/\1")
)
// Result: "Today is 25/12/2023 and tomorrow is 26/12/2023"
```

### Log Parsing
```cel
// Parse log entries
cel.bind(log_line, "2023-12-25 14:30:15 [ERROR] Database connection failed",
  cel.bind(timestamp, regex.extract(log_line, r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})"),
    cel.bind(level, regex.extract(log_line, r"\[([A-Z]+)\]"),
      cel.bind(message, regex.extract(log_line, r"\] (.+)$"),
        {
          "timestamp": timestamp.orValue("unknown"),
          "level": level.orValue("INFO"),
          "message": message.orValue("no message")
        }
      )
    )
  )
)
// Result: {"timestamp": "2023-12-25 14:30:15", "level": "ERROR", "message": "Database connection failed"}
```

### Data Validation
```cel
// Validate various data formats
cel.bind(data, {
  "email": "user@example.com",
  "phone": "555-123-4567",
  "zip": "12345"
},
  {
    "email_valid": regex.extract(data.email, r"^[^@]+@[^@]+\.[^@]+$").hasValue(),
    "phone_valid": regex.extract(data.phone, r"^\d{3}-\d{3}-\d{4}$").hasValue(),
    "zip_valid": regex.extract(data.zip, r"^\d{5}$").hasValue()
  }
)
// Result: {"email_valid": true, "phone_valid": true, "zip_valid": true}
```

### Text Replacement with Limits
```cel
// Replace with different count limits
cel.bind(text, "foo foo foo foo",
  {
    "replace_all": regex.replace(text, "foo", "bar"),
    "replace_first": regex.replace(text, "foo", "bar", 1),
    "replace_two": regex.replace(text, "foo", "bar", 2),
    "replace_none": regex.replace(text, "foo", "bar", 0)
  }
)
// Result: {
//   "replace_all": "bar bar bar bar",
//   "replace_first": "bar foo foo foo",
//   "replace_two": "bar bar foo foo",
//   "replace_none": "foo foo foo foo"
// }
```

### Pattern Matching with Optional Results
```cel
// Handle optional extraction results
cel.bind(text, "No numbers here",
  cel.bind(number, regex.extract(text, r"\d+"),
    {
      "text": text,
      "has_number": number.hasValue(),
      "number": number.orValue("none found"),
      "message": number.hasValue() ? 
        "Found: " + number.value() : 
        "No numbers detected"
    }
  )
)
// Result: {"text": "No numbers here", "has_number": false, "number": "none found", "message": "No numbers detected"}
```

The regex extension provides powerful pattern matching capabilities that are essential for text processing, data validation, and information extraction tasks in CEL expressions. The integration with CEL's optional types makes it safe and convenient to handle cases where patterns may or may not match. 
// Parse configuration key-value pairs
cel.bind(config_text, "database.host=localhost\ndatabase.port=5432\napp.debug=true",
  cel.bind(matches, config_text.findAll(r"([^=\n]+)=([^\n]+)"),
    matches.map(match, {
      "key": match[1],
      "value": match[2]
    })
  )
)
// Result: [{"key": "database.host", "value": "localhost"}, {"key": "database.port", "value": "5432"}, {"key": "app.debug", "value": "true"}]
```

The regex extension provides powerful pattern matching capabilities that are essential for text processing, data validation, and information extraction tasks in CEL expressions. 