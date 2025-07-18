# Optional Types

- [Optional Types](#optional-types)
  - [1. Syntax](#1-syntax)
    - [1.1 Optional Field Selection (`.?`)](#11-optional-field-selection-)
    - [1.2 Optional Indexing (`[?_]`)](#12-optional-indexing-_)
      - [1.2.1 Map Key Access](#121-map-key-access)
      - [1.2.2 List Index Access](#122-list-index-access)
    - [1.3 Optional Construction (`{?key: ...}`)](#13-optional-construction-key-)
      - [1.3.1 Message Field Construction](#131-message-field-construction)
      - [1.3.2 Map Entry Construction](#132-map-entry-construction)
  - [2. Functions](#2-functions)
    - [2.1 `optional.of(<value>)`](#21-optionalofvalue)
    - [2.2 `optional.none()`](#22-optionalnone)
    - [2.3 `<optional>.hasValue()`](#23-optionalhasvalue)
    - [2.4 `<optional>.orValue(<default>)`](#24-optionalorvaluedefault)


The optional type extension introduces the ability to represent and interact with values that may or may not be present at runtime. This is particularly useful for handling protobuf fields that are not explicitly marked as `optional`, accessing map keys that may not exist, or indexing into lists safely.

The primary motivation is to provide a robust way to deal with missing data without causing evaluation errors, enabling more resilient and expressive policies.

## 1. Syntax

Optional types introduce new syntax for selection, indexing, and message/map construction.

### 1.1 Optional Field Selection (`.?`)

The optional field selection syntax `.?` allows you to access a field on a message that might be absent. If the field is present, it returns an optional value containing the field's value. If absent, it returns an `optional.none()`.

**Semantics:** `has(msg.field) ? optional.of(msg.field) : optional.none()`

```cel
// If msg.field exists, result is optional(msg.field_value).
// Otherwise, result is optional.none().
let result = msg.?field;
```

### 1.2 Optional Indexing (`[?_]`)

Optional indexing works similarly for maps and lists. It provides a safe way to access elements without risking an error for out-of-bounds indices or missing keys.

#### 1.2.1 Map Key Access

**Semantics:** `key in map ? optional.of(map[key]) : optional.none()`

```cel
// If 'my_key' is in my_map, result is optional(my_map['my_key']).
// Otherwise, result is optional.none().
let result = my_map[?'my_key'];
```

#### 1.2.2 List Index Access

**Semantics:** `index >= 0 && index < list.size() ? optional.of(list[index]) : optional.none()`

```cel
// If index 1 is valid for my_list, result is optional(my_list[1]).
// Otherwise, result is optional.none().
let result = my_list[?1];
```

### 1.3 Optional Construction (`{?key: ...}`)

Optional construction syntax allows you to conditionally include fields in a message or entries in a map literal. The entry is only included if the value expression on the right-hand side is not `optional.none()`.

#### 1.3.1 Message Field Construction

**Semantics (pseudo-code):** `<expr>.hasValue() ? Msg{field: <expr>.value()} : Msg{}`

The `<expr>` must be of type `optional(T)`, where `T` is the type of the message field.

```cel
// The `age` field will only be set on the message if opt_age has a value.
MyMessage{
    name: 'John Doe',
    ?age: opt_age 
}
```

#### 1.3.2 Map Entry Construction

**Semantics (pseudo-code):** `<expr>.hasValue() ? {key: <expr>.value()} : {}`

The `<expr>` must be an optional type.

```cel
// The 'age' entry is only included in the map if opt_age is not none.
{
    'name': 'John Doe',
    ?'age': opt_age
}
```

## 2. Functions

The optional type comes with a set of utility functions to create and interact with optional values.

### 2.1 `optional.of(<value>)`

Creates an optional value that contains the given `<value>`.

```cel
let opt_name = optional.of('Alice'); // Contains 'Alice'
```

### 2.2 `optional.none()`

Creates an empty optional value.

```cel
let opt_age = optional.none(); // Represents an absent value
```

### 2.3 `<optional>.hasValue()`

Returns `true` if the optional value contains a value, and `false` otherwise.

```cel
optional.of('Alice').hasValue() // returns true
optional.none().hasValue()      // returns false
```

### 2.4 `<optional>.orValue(<default>)`

Returns the value contained within the optional if it's present; otherwise, it returns the `<default>` value.

```cel
optional.of('Alice').orValue('Guest') // returns 'Alice'
optional.none().orValue(30)           // returns 30
``` 