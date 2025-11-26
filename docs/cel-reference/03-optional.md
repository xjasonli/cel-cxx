# Optional Types

- [Optional Types](#optional-types)
  - [1. Overview](#1-overview)
  - [2. Syntax](#2-syntax)
    - [2.1 Optional Field Selection (`.?`)](#21-optional-field-selection-)
    - [2.2 Optional Indexing (`[?_]`)](#22-optional-indexing-_)
      - [2.2.1 Map Key Access](#221-map-key-access)
      - [2.2.2 List Index Access](#222-list-index-access)
    - [2.3 Optional Construction (`{?key: ...}`)](#23-optional-construction-key-)
      - [2.3.1 Message Field Construction](#231-message-field-construction)
      - [2.3.2 Map Entry Construction](#232-map-entry-construction)
      - [2.3.3 Optional List Element Construction](#233-optional-list-element-construction)
    - [2.4 Chaining Behavior](#24-chaining-behavior)
  - [3. Functions](#3-functions)
    - [3.1 Creation Functions](#31-creation-functions)
      - [3.1.1 `optional.of(<value>)`](#311-optionalofvalue)
      - [3.1.2 `optional.ofNonZeroValue(<value>)`](#312-optionalofnonzerovaluevalue)
      - [3.1.3 `optional.none()`](#313-optionalnone)
    - [3.2 Access Functions](#32-access-functions)
      - [3.2.1 `<optional>.hasValue()`](#321-optionalhasvalue)
      - [3.2.2 `<optional>.value()`](#322-optionalvalue)
    - [3.3 Chaining Functions](#33-chaining-functions)
      - [3.3.1 `<optional>.or(<optional>)`](#331-optionaloroptional)
      - [3.3.2 `<optional>.orValue(<default>)`](#332-optionalorvaluedefault)
    - [3.4 Transformation Functions](#34-transformation-functions)
      - [3.4.1 `<optional>.optMap(<var>, <expr>)`](#341-optionaloptmapvar-expr)
      - [3.4.2 `<optional>.optFlatMap(<var>, <expr>)`](#342-optionaloptflatmapvar-expr)
    - [3.5 Utility Functions](#35-utility-functions)
      - [3.5.1 `optional.unwrap(<list>)`](#351-optionalunwraplist)
  - [4. Type System](#4-type-system)
    - [4.1 Optional Type Declaration](#41-optional-type-declaration)
    - [4.2 Type Compatibility](#42-type-compatibility)
  - [5. Examples](#5-examples)
    - [5.1 Basic Usage](#51-basic-usage)
    - [5.2 Chaining Operations](#52-chaining-operations)
    - [5.3 Conditional Construction](#53-conditional-construction)
    - [5.4 Data Processing](#54-data-processing)
    - [5.5 Error Handling](#55-error-handling)
  - [6. Best Practices](#6-best-practices)
  - [7. Migration Guide](#7-migration-guide)
    - [From explicit null checks:](#from-explicit-null-checks)
    - [From error-prone indexing:](#from-error-prone-indexing)
    - [From complex conditional construction:](#from-complex-conditional-construction)

## 1. Overview

The optional type extension introduces the ability to represent and interact with values that may or may not be present at runtime. This is particularly useful for handling protobuf fields that are not explicitly marked as `optional`, accessing map keys that may not exist, or indexing into lists safely.

**Key Benefits:**
- **Null-safe operations**: Avoid runtime errors when accessing potentially missing data
- **Expressive syntax**: Clear distinction between present and absent values
- **Composable operations**: Chain operations on optional values without explicit null checks
- **Type safety**: Compile-time awareness of optional vs. required values

**Requirements:**
- Requires `RuntimeOptions.enable_qualified_type_identifiers = true`
- Requires `RuntimeOptions.enable_heterogeneous_equality = true`

The optional type is implemented as `optional(T)` where `T` is the type of the contained value.

## 2. Syntax

Optional types introduce new syntax for selection, indexing, and message/map construction.

### 2.1 Optional Field Selection (`.?`)

The optional field selection syntax `.?` allows you to access a field on a message that might be absent. If the field is present, it returns an optional value containing the field's value. If absent, it returns an `optional.none()`.

**Semantics:** `has(msg.field) ? optional.of(msg.field) : optional.none()`

```cel
// If msg.field exists, result is optional(msg.field_value).
// Otherwise, result is optional.none().
let result = msg.?field;

// Works with nested fields
let nested = msg.?user.?profile.?name;
```

**Supported on:**
- Protocol Buffer messages
- Map values (when used as structs)
- Optional values (chains automatically)

### 2.2 Optional Indexing (`[?_]`)

Optional indexing works similarly for maps and lists. It provides a safe way to access elements without risking an error for out-of-bounds indices or missing keys.

#### 2.2.1 Map Key Access

**Semantics:** `key in map ? optional.of(map[key]) : optional.none()`

```cel
// If 'my_key' is in my_map, result is optional(my_map['my_key']).
// Otherwise, result is optional.none().
let result = my_map[?'my_key'];

// Works with different key types
let int_key = my_map[?42];
let bool_key = my_map[?true];

// Automatic type conversion for numeric keys
let converted = my_map[?1];  // Tries both int(1) and uint(1)
```

#### 2.2.2 List Index Access

**Semantics:** `index >= 0 && index < list.size() ? optional.of(list[index]) : optional.none()`

```cel
// If index 1 is valid for my_list, result is optional(my_list[1]).
// Otherwise, result is optional.none().
let result = my_list[?1];

// Negative indices are always optional.none()
let invalid = my_list[?-1];  // Always optional.none()
```

### 2.3 Optional Construction (`{?key: ...}`)

Optional construction syntax allows you to conditionally include fields in a message or entries in a map literal. The entry is only included if the value expression on the right-hand side is not `optional.none()`.

#### 2.3.1 Message Field Construction

**Semantics:** `<expr>.hasValue() ? Msg{field: <expr>.value()} : Msg{}`

The `<expr>` must be of type `optional(T)`, where `T` is the type of the message field.

```cel
// The `age` field will only be set on the message if opt_age has a value.
MyMessage{
    name: 'John Doe',
    ?age: opt_age,
    ?email: user.?profile.?email
}
```

#### 2.3.2 Map Entry Construction

**Semantics:** `<expr>.hasValue() ? {key: <expr>.value()} : {}`

The `<expr>` must be an optional type.

```cel
// The 'age' entry is only included in the map if opt_age is not none.
{
    'name': 'John Doe',
    ?'age': opt_age,
    ?'email': user.?profile.?email
}
```

#### 2.3.3 Optional List Element Construction

Elements in list literals can be optionally included based on whether the expression evaluates to `optional.none()`.

```cel
// Creates a list with 1-4 elements depending on which optionals have values
[
    'always_included',
    ?opt_value1,
    ?opt_value2,
    ?opt_value3
]
```

### 2.4 Chaining Behavior

Optional selection and indexing operations are "viral" - once an optional operation is used, subsequent operations automatically become optional:

```cel
// These are equivalent:
obj.?field.subfield
obj.?field.?subfield

// Also equivalent:
list[?0].field
list[?0].?field

// Mixed chaining
obj.?field[?'key'].?subfield
```

## 3. Functions

The optional type comes with a comprehensive set of utility functions to create and interact with optional values.

### 3.1 Creation Functions

#### 3.1.1 `optional.of(<value>)`

Creates an optional value that contains the given `<value>`.

**Signature:** `optional.of(T) -> optional(T)`

```cel
let opt_name = optional.of('Alice');        // optional(string)
let opt_age = optional.of(30);              // optional(int)
let opt_list = optional.of([1, 2, 3]);      // optional(list(int))
```

#### 3.1.2 `optional.ofNonZeroValue(<value>)`

Creates an optional value containing the given value if it is not a zero-value (default empty value). If the value is a zero-value, returns `optional.none()`.

**Signature:** `optional.ofNonZeroValue(T) -> optional(T)`

**Zero values by type:**
- Numbers: `0`, `0u`, `0.0`
- Strings: `""`
- Bytes: `b""`
- Lists: `[]`
- Maps: `{}`
- Messages: all fields unset
- Booleans: `false`

```cel
optional.ofNonZeroValue([1, 2, 3])  // optional(list(int))
optional.ofNonZeroValue([])         // optional.none()
optional.ofNonZeroValue(0)          // optional.none()
optional.ofNonZeroValue("")         // optional.none()
optional.ofNonZeroValue("hello")    // optional(string)
optional.ofNonZeroValue(false)      // optional.none()
optional.ofNonZeroValue(true)       // optional(bool)
```

#### 3.1.3 `optional.none()`

Creates an empty optional value.

**Signature:** `optional.none() -> optional(T)`

```cel
let opt_age = optional.none();  // Represents an absent value
```

### 3.2 Access Functions

#### 3.2.1 `<optional>.hasValue()`

Returns `true` if the optional value contains a value, and `false` otherwise.

**Signature:** `optional(T).hasValue() -> bool`

```cel
optional.of('Alice').hasValue()     // true
optional.none().hasValue()          // false
optional.ofNonZeroValue("").hasValue()  // false
```

#### 3.2.2 `<optional>.value()`

Returns the value contained within the optional. If the optional does not have a value, the result will be a CEL error.

**Signature:** `optional(T).value() -> T`

```cel
optional.of('Alice').value()        // 'Alice'
optional.none().value()             // ERROR: optional has no value
```

**⚠️ Warning:** This function will cause a runtime error if called on an empty optional. Use `hasValue()` to check first, or prefer `orValue()` for safer access.

### 3.3 Chaining Functions

#### 3.3.1 `<optional>.or(<optional>)`

If the left-hand side optional is empty (`optional.none()`), returns the right-hand side optional. If the left-hand side has a value, returns it. This operation is short-circuiting.

**Signature:** `optional(T).or(optional(T)) -> optional(T)`

```cel
optional.of('Alice').or(optional.of('Bob'))     // optional('Alice')
optional.none().or(optional.of('Bob'))          // optional('Bob')
optional.none().or(optional.none())             // optional.none()

// Chaining multiple alternatives
user.?name.or(user.?nickname).or(optional.of('Anonymous'))
```

#### 3.3.2 `<optional>.orValue(<default>)`

Returns the value contained within the optional if it's present; otherwise, returns the `<default>` value.

**Signature:** `optional(T).orValue(T) -> T`

```cel
optional.of('Alice').orValue('Guest')   // 'Alice'
optional.none().orValue('Guest')        // 'Guest'

// Common pattern for providing defaults
let display_name = user.?profile.?name.orValue('Anonymous');
```

### 3.4 Transformation Functions

#### 3.4.1 `<optional>.optMap(<var>, <expr>)`

Apply a transformation to the optional's underlying value if it is not empty and return an optional-typed result based on the transformation. The transformation expression type must return a type `T` which is automatically wrapped into `optional(T)`.

**Signature:** `optional(A).optMap(var, expr) -> optional(B)` where `expr: A -> B`

```cel
// Transform the value if present
optional.of("hello").optMap(s, s.size())                    // optional(5)
optional.none().optMap(s, s.size())                         // optional.none()

// Chain transformations
optional.of([1, 2, 3])
    .optMap(list, list.size())
    .optMap(size, size * 2)                                 // optional(6)

// Real-world example
user.?profile.?name.optMap(name, name.upperAscii()).orValue("UNKNOWN")
```

#### 3.4.2 `<optional>.optFlatMap(<var>, <expr>)`

Apply a transformation to the optional's underlying value if it is not empty and return the result. The transform expression must return an `optional(T)` rather than type `T`. This is useful when the transformation itself might fail or return an empty result.

**Signature:** `optional(A).optFlatMap(var, expr) -> optional(B)` where `expr: A -> optional(B)`

```cel
// Transform with another optional operation
optional.of([1, 2, 3])
    .optFlatMap(list, list[?0])                             // optional(1)

optional.of([])
    .optFlatMap(list, list[?0])                             // optional.none()

// Chain with conditional logic
optional.of("user@example.com")
    .optFlatMap(email, email.contains("@") ? 
        optional.of(email.split("@")[0]) : 
        optional.none())                                     // optional("user")
```

### 3.5 Utility Functions

#### 3.5.1 `optional.unwrap(<list>)`

Takes a list of optional values and returns a new list containing only the values from non-empty optionals. Empty optionals are filtered out.

**Signature:** `optional.unwrap(list(optional(T))) -> list(T)`

```cel
optional.unwrap([
    optional.of(1),
    optional.none(),
    optional.of(3),
    optional.none(),
    optional.of(5)
])  // [1, 3, 5]

// Also available as a method
[
    optional.of("a"),
    optional.none(),
    optional.of("c")
].unwrapOpt()  // ["a", "c"]
```

## 4. Type System

### 4.1 Optional Type Declaration

Optional types are represented as `optional(T)` where `T` is the wrapped type. The type system understands optional types and provides appropriate type checking.

```cel
// Type annotations (conceptual)
optional.of(42)         // optional(int)
optional.of("hello")    // optional(string)
optional.of([1, 2, 3])  // optional(list(int))
optional.none()         // optional(dyn) - can be any optional type
```

### 4.2 Type Compatibility

Optional types follow these compatibility rules:

- `optional(T)` is not directly assignable to `T`
- `T` can be wrapped into `optional(T)` using `optional.of()`
- Optional operations preserve type information through the chain
- Type checking ensures optional construction syntax receives `optional(T)` values

## 5. Examples

### 5.1 Basic Usage

```cel
// Safe field access
let user_email = request.?user.?email.orValue("no-email@example.com");

// Safe list indexing
let first_item = items[?0].orValue("default");

// Safe map access
let config_value = config[?"timeout"].orValue(30);
```

### 5.2 Chaining Operations

```cel
// Complex chaining with transformations
request.?user.?profile.?preferences.?theme
    .optMap(theme, theme.lowerAscii())
    .or(optional.of("default"))
    .value()

// Multiple fallbacks
primary_config[?"setting"]
    .or(fallback_config[?"setting"])
    .or(optional.of("default_value"))
    .value()
```

### 5.3 Conditional Construction

```cel
// Message construction with optional fields
UserProfile{
    name: "John Doe",
    ?email: request.?user.?email,
    ?age: request.?user.?age,
    ?avatar_url: request.?user.?profile.?avatar
}

// Map construction with optional entries
{
    "name": "John Doe",
    ?"email": request.?user.?email,
    ?"preferences": request.?user.?preferences
}

// List with optional elements
[
    "required_item",
    ?optional_item1,
    ?optional_item2,
    "another_required_item"
]
```

### 5.4 Data Processing

```cel
// Process a list of optional values
let valid_emails = users
    .map(user, user.?email)
    .filter(opt_email, opt_email.hasValue())
    .map(opt_email, opt_email.value());

// Or using unwrap
let valid_emails = optional.unwrap(users.map(user, user.?email));

// Transform and provide defaults
let display_names = users.map(user, 
    user.?profile.?display_name.orValue(user.?name.orValue("Anonymous"))
);
```

### 5.5 Error Handling

```cel
// Safe access without errors
let result = data.?results[?0].?value.orValue("no data");

// Validation with optional types
let is_valid = request.?user.?email
    .optMap(email, email.matches(r'^[^@]+@[^@]+\.[^@]+$'))
    .orValue(false);

// Conditional processing
let processed = input.?data.hasValue() ? 
    processData(input.data.value()) : 
    "no data to process";
```

## 6. Best Practices

1. **Prefer `orValue()` over `value()`**: Use `orValue()` to provide defaults instead of risking runtime errors with `value()`.

2. **Use `ofNonZeroValue()` for validation**: When you want to treat empty/zero values as absent.

3. **Chain operations efficiently**: Take advantage of the viral nature of optional operations to avoid explicit null checks.

4. **Provide meaningful defaults**: Always consider what default value makes sense in your context.

5. **Use optional construction for clean code**: Leverage optional construction syntax to avoid complex conditional logic.

6. **Document optional behavior**: Make it clear when functions or data structures might return optional values.

## 7. Migration Guide

### From explicit null checks:

```cel
// Before
has(request.user) && has(request.user.email) ? request.user.email : "default"

// After
request.?user.?email.orValue("default")
```

### From error-prone indexing:

```cel
// Before (could cause runtime errors)
items.size() > 0 ? items[0] : default_item

// After
items[?0].orValue(default_item)
```

### From complex conditional construction:

```cel
// Before
has(user.email) ? UserProfile{name: user.name, email: user.email} : UserProfile{name: user.name}

// After
UserProfile{
    name: user.name,
    ?email: user.?email
}
``` 