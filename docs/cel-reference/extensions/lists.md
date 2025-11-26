# Lists Extension

The lists extension provides advanced list processing functions that extend CEL's built-in list capabilities.

## Table of Contents

- [Lists Extension](#lists-extension)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. List Slicing - `slice()`](#2-list-slicing---slice)
  - [3. List Flattening - `flatten()`](#3-list-flattening---flatten)
  - [4. List Deduplication - `distinct()`](#4-list-deduplication---distinct)
  - [5. List Reversal - `reverse()`](#5-list-reversal---reverse)
  - [6. List Sorting](#6-list-sorting)
    - [6.1 sort()](#61-sort)
    - [6.2 sortBy()](#62-sortby)
  - [7. Number Ranges - `range()`](#7-number-ranges---range)
  - [8. Usage Examples](#8-usage-examples)
    - [Data Deduplication and Cleaning](#data-deduplication-and-cleaning)
    - [Nested Data Processing](#nested-data-processing)
    - [Pagination and Slicing](#pagination-and-slicing)
    - [Complex Data Transformation](#complex-data-transformation)
    - [Multi-level Flattening](#multi-level-flattening)
    - [Range-based Operations](#range-based-operations)
    - [Statistical Analysis](#statistical-analysis)

## 1. Overview

The lists extension enhances CEL with powerful list manipulation capabilities. All functions maintain CEL's immutability guarantees and return new lists rather than modifying existing ones. All indices are zero-based.

**Enabling the Extension**:
```rust
let env = Env::builder()
    .with_ext_lists(true)
    .build()?;
```

## 2. List Slicing - `slice()`

**Introduced in version:** 0

Returns a new sub-list using the indexes provided.

**Syntax:** `list.slice(start, end)`

**Parameters:**
- `list`: Input list
- `start`: Starting index (inclusive)
- `end`: Ending index (exclusive)

**Return Type:** List of the same type

**Examples:**
```cel
[1, 2, 3, 4].slice(1, 3)           // [2, 3]
[1, 2, 3, 4].slice(2, 4)           // [3, 4]
["a", "b", "c", "d"].slice(0, 2)   // ["a", "b"]
[1, 2, 3, 4, 5].slice(2, 10)       // [3, 4, 5] (clamped)
[1, 2, 3].slice(1, 1)              // []
```

**Behavior:**
- Negative indices are not supported
- Out-of-bounds indices are clamped to valid ranges
- If `start >= end`, returns empty list

## 3. List Flattening - `flatten()`

**Introduced in version:** 1

Flattens a list recursively. If an optional depth is provided, the list is flattened to the specified level. A negative depth value will result in an error.

**Syntax:**
- `list.flatten()` (flatten completely)
- `list.flatten(depth)` (flatten to specified depth)

**Parameters:**
- `list`: Input list containing nested lists
- `depth`: Optional maximum depth to flatten (default: complete flattening)

**Return Type:** Flattened list

**Examples:**
```cel
[1, [2, 3], [4]].flatten()          // [1, 2, 3, 4]
[1, [2, [3, 4]]].flatten()          // [1, 2, [3, 4]]
[1, 2, [], [], [3, 4]].flatten()    // [1, 2, 3, 4]
[1, [2, [3, [4]]]].flatten(2)       // [1, 2, 3, [4]]
[1, [2, [3, [4]]]].flatten(-1)      // error
```

**Behavior:**
- Flattens only lists, not other nested structures
- Preserves element order
- Non-list elements are preserved as-is
- Negative depth values result in an error

## 4. List Deduplication - `distinct()`

**Introduced in version:** 2

Returns the distinct elements of a list, preserving the order of first occurrence.

**Syntax:** `<list(T)>.distinct() -> <list(T)>`

**Parameters:**
- `list`: Input list of any type

**Return Type:** List of the same type with duplicates removed

**Examples:**
```cel
[1, 2, 2, 3, 3, 3].distinct()      // [1, 2, 3]
["b", "b", "c", "a", "c"].distinct() // ["b", "c", "a"]
[1, "b", 2, "b"].distinct()         // [1, "b", 2]
[true, false, true].distinct()      // [true, false]
[].distinct()                       // []
```

**Behavior:**
- Preserves the order of first occurrences
- Works with any comparable type (strings, numbers, booleans)
- Empty lists return empty lists
- Mixed types are supported

## 5. List Reversal - `reverse()`

**Introduced in version:** 2

Returns the elements of a list in reverse order.

**Syntax:** `<list(T)>.reverse() -> <list(T)>`

**Parameters:**
- `list`: Input list of any type

**Return Type:** List with elements in reverse order

**Examples:**
```cel
[5, 3, 1, 2].reverse()              // [2, 1, 3, 5]
["a", "b", "c"].reverse()           // ["c", "b", "a"]
[].reverse()                        // []
[42].reverse()                      // [42]
[[1, 2], [3, 4]].reverse()          // [[3, 4], [1, 2]]
```

## 6. List Sorting

**Introduced in version:** 2

### 6.1 sort()

Sorts a list with comparable elements. If the element type is not comparable or the element types are not the same, the function will produce an error.

**Syntax:** `list.sort()`

**Parameters:**
- `list`: Input list of comparable elements

**Return Type:** Sorted list

**Supported Types:** int, uint, double, bool, duration, timestamp, string, bytes

**Examples:**
```cel
[3, 2, 1].sort()                    // [1, 2, 3]
["b", "c", "a"].sort()              // ["a", "b", "c"]
[true, false, true].sort()          // [false, true, true]
[].sort()                           // []
[1, "b"].sort()                     // error (mixed types)
[[1, 2, 3]].sort()                  // error (non-comparable type)
```

### 6.2 sortBy()

Sorts a list by a key value, i.e., the order is determined by the result of an expression applied to each element of the list. The output of the key expression must be a comparable type.

**Syntax:** `list.sortBy(var, key_expr)`

**Parameters:**
- `list`: Input list
- `var`: Variable name for each element
- `key_expr`: Expression to compute sort key (must return comparable type)

**Return Type:** Sorted list

**Examples:**
```cel
[
  {"name": "foo", "score": 0},
  {"name": "bar", "score": -10},
  {"name": "baz", "score": 1000}
].sortBy(e, e.score).map(e, e.name)
// ["bar", "foo", "baz"]

["apple", "pie", "banana"].sortBy(s, s.size())
// ["pie", "apple", "banana"]

[3, -1, 4, -2].sortBy(n, math.abs(n))
// [-1, -2, 3, 4]
```

## 7. Number Ranges - `range()`

**Introduced in version:** 2

Returns a list of integers from 0 to n-1.

**Syntax:** `lists.range(n)`

**Parameters:**
- `n`: Upper bound (exclusive)

**Return Type:** List of integers

**Examples:**
```cel
lists.range(5)                      // [0, 1, 2, 3, 4]
lists.range(0)                      // []
lists.range(1)                      // [0]
lists.range(3)                      // [0, 1, 2]
```

**Behavior:**
- Creates a list from 0 to n-1 (inclusive)
- If n <= 0, returns empty list
- Only supports non-negative integers

## 8. Usage Examples

### Data Deduplication and Cleaning
```cel
// Remove duplicates and sort user IDs
cel.bind(user_ids, [1, 3, 2, 1, 4, 2, 5],
  user_ids.distinct().sort()
)
// Result: [1, 2, 3, 4, 5]
```

### Nested Data Processing
```cel
// Flatten and process nested categories
cel.bind(categories, [["tech", "ai"], ["health", "fitness"], ["tech", "web"]],
  categories.flatten().distinct().sort()
)
// Result: ["ai", "fitness", "health", "tech", "web"]
```

### Pagination and Slicing
```cel
// Implement pagination
cel.bind(items, lists.range(101),     // [0, 1, 2, ..., 100]
  cel.bind(page, 2,
    cel.bind(page_size, 10,
      cel.bind(start, (page - 1) * page_size,
        items.slice(start, start + page_size)
      )
    )
  )
)
// Result: [10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
```

### Complex Data Transformation
```cel
// Process and sort player data
cel.bind(players, [
  {"name": "Alice", "scores": [85, 92, 78]},
  {"name": "Bob", "scores": [91, 87, 95]},
  {"name": "Charlie", "scores": [76, 88, 82]}
],
  players.map(player, {
    "name": player.name,
    "avg_score": player.scores.sum() / player.scores.size(),
    "best_score": player.scores.sort().reverse()[0]
  }).sortBy(p, p.avg_score).reverse()
)
// Result: Sorted by average score (descending)
```

### Multi-level Flattening
```cel
// Flatten nested structures with controlled depth
cel.bind(nested, [1, [2, [3, [4, 5]]], [6, [7, 8]]],
  {
    "level_1": nested.flatten(1),     // [1, 2, [3, [4, 5]], 6, [7, 8]]
    "level_2": nested.flatten(2),     // [1, 2, 3, [4, 5], 6, 7, 8]
    "complete": nested.flatten()      // [1, 2, 3, [4, 5], 6, 7, 8]
  }
)
```

### Range-based Operations
```cel
// Generate and process number sequences
cel.bind(range_10, lists.range(10),
  {
    "numbers": range_10,
    "evens": range_10.filter(n, n % 2 == 0),
    "odds": range_10.filter(n, n % 2 == 1),
    "squares": range_10.map(n, n * n),
    "reversed": range_10.reverse()
  }
)
// Result: Various transformations of [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```

### Statistical Analysis
```cel
// Analyze list statistics with sorting
cel.bind(numbers, [1, 5, 3, 9, 2, 7, 4, 8, 6],
  cel.bind(sorted, numbers.sort(),
    cel.bind(unique_sorted, numbers.distinct().sort(),
      {
        "original": numbers,
        "sorted": sorted,
        "unique": unique_sorted,
        "median": sorted[sorted.size() / 2],
        "range": sorted.reverse()[0] - sorted[0],
        "duplicates_removed": numbers.size() - unique_sorted.size()
      }
    )
  )
)
```

The lists extension provides comprehensive list manipulation capabilities, enabling sophisticated data processing and analysis within CEL expressions while maintaining immutability and type safety. 