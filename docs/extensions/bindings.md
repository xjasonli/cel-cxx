# Bindings Extension

The bindings extension provides variable binding capabilities that allow you to define local variable bindings within CEL expressions for better readability and performance.

## Table of Contents

- [Bindings Extension](#bindings-extension)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Variable Binding - `cel.bind()`](#2-variable-binding---celbind)
    - [cel.bind()](#celbind)
  - [3. Usage Examples](#3-usage-examples)
    - [Performance Optimization](#performance-optimization)
    - [Nested Bindings](#nested-bindings)
    - [Complex Conditional Logic](#complex-conditional-logic)
    - [Data Transformation Pipeline](#data-transformation-pipeline)
    - [Avoiding Repeated Calculations](#avoiding-repeated-calculations)
    - [String Processing](#string-processing)
    - [List Processing with Reuse](#list-processing-with-reuse)
    - [Configuration Processing](#configuration-processing)
    - [Mathematical Calculations](#mathematical-calculations)
    - [API Response Processing](#api-response-processing)
    - [Conditional Processing with Shared Values](#conditional-processing-with-shared-values)

## 1. Overview

The bindings extension introduces the `cel.bind()` macro that allows you to bind simple identifiers to initialization expressions which may be used in subsequent result expressions. This is particularly useful for complex expressions where you want to avoid recalculating the same value multiple times or improve readability.

**Key Features:**
- Local variable bindings within expressions
- Nested bindings support
- Performance optimization through value reuse
- Improved expression readability

**Enabling the Extension**:
```rust
let env = Env::builder()
    .with_ext_bindings(true)
    .build()?;
```

## 2. Variable Binding - `cel.bind()`

### cel.bind()

Binds a simple identifier to an initialization expression which may be used in a subsequent result expression. Bindings may also be nested within each other.

**Syntax:** `cel.bind(varName, initExpr, resultExpr)`

**Parameters:**
- `varName`: Variable name (identifier)
- `initExpr`: Initialization expression to bind to the variable
- `resultExpr`: Result expression that can use the bound variable

**Return Type:** Result of the result expression

**Examples:**
```cel
cel.bind(a, 'hello',
cel.bind(b, 'world', a + b + b + a)) // "helloworldworldhello"

cel.bind(x, 5, x * x)                        // 25
cel.bind(name, "Alice", "Hello, " + name)    // "Hello, Alice"
cel.bind(list, [1, 2, 3], list.size() > 2)  // true
```

**Behavior:**
- The variable is only available within the result expression scope
- Variables can be used in nested bindings
- The initialization expression is evaluated once when the binding is created
- Local bindings are not guaranteed to be evaluated before use (lazy evaluation)
- Supports any CEL type (primitives, lists, maps, etc.)

## 3. Usage Examples

### Performance Optimization
```cel
// Avoid list allocation within exists comprehension
cel.bind(valid_values, [a, b, c],
  [d, e, f].exists(elem, elem in valid_values))
```

### Nested Bindings
```cel
// Chain multiple bindings
cel.bind(a, 'hello',
  cel.bind(b, 'world', 
    cel.bind(c, a + ' ' + b,
      c + '!')))
// Result: "hello world!"
```

### Complex Conditional Logic
```cel
// Simplify complex conditions with intermediate variables
cel.bind(user, request.user,
  cel.bind(permissions, user.permissions,
    cel.bind(required_permission, "admin",
      cel.bind(has_permission, permissions.contains([required_permission]),
        has_permission && user.active && user.verified
      )
    )
  )
)
```

### Data Transformation Pipeline
```cel
// Chain data transformations with intermediate results
cel.bind(raw_data, input.data,
  cel.bind(cleaned_data, raw_data.filter(item, item.valid),
    cel.bind(processed_data, cleaned_data.map(item, item.value * 2),
      cel.bind(result, processed_data.sort(),
        {
          "count": result.size(),
          "sum": result.sum(),
          "average": result.size() > 0 ? result.sum() / double(result.size()) : 0.0
        }
      )
    )
  )
)
```

### Avoiding Repeated Calculations
```cel
// Calculate expensive operations once
cel.bind(expensive_calc, computeComplexValue(input),
  cel.bind(threshold, 100,
    expensive_calc > threshold ? 
      expensive_calc * 2 : 
      expensive_calc / 2
  )
)
```

### String Processing
```cel
// Complex string manipulation with intermediate results
cel.bind(input, "  Hello, World!  ",
  cel.bind(trimmed, input.trim(),
    cel.bind(lower, trimmed.lowerAscii(),
      cel.bind(words, lower.split(" "),
        cel.bind(processed, words.map(word, 
          word.charAt(0).upperAscii() + word.substring(1)
        ),
          {
            "original": input,
            "processed": processed.join(" "),
            "word_count": words.size(),
            "char_count": trimmed.size()
          }
        )
      )
    )
  )
)
```

### List Processing with Reuse
```cel
// Process lists with multiple operations on same data
cel.bind(numbers, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
  cel.bind(evens, numbers.filter(n, n % 2 == 0),
    cel.bind(odds, numbers.filter(n, n % 2 == 1),
      {
        "all_numbers": numbers,
        "even_numbers": evens,
        "odd_numbers": odds,
        "even_sum": evens.sum(),
        "odd_sum": odds.sum(),
        "total_sum": numbers.sum(),
        "even_count": evens.size(),
        "odd_count": odds.size()
      }
    )
  )
)
```

### Configuration Processing
```cel
// Process configuration with validation
cel.bind(config, input.config,
  cel.bind(db_config, config.database,
    cel.bind(is_valid_db, 
      has(db_config.host) && has(db_config.port) && db_config.port > 0,
      cel.bind(auth_config, config.auth,
        cel.bind(is_valid_auth, has(auth_config.enabled) && auth_config.enabled,
          {
            "database_valid": is_valid_db,
            "auth_valid": is_valid_auth,
            "overall_valid": is_valid_db && is_valid_auth,
            "db_host": is_valid_db ? db_config.host : "localhost",
            "db_port": is_valid_db ? db_config.port : 5432,
            "auth_enabled": is_valid_auth
          }
        )
      )
    )
  )
)
```

### Mathematical Calculations
```cel
// Complex mathematical expressions with intermediate values
cel.bind(a, 3,
  cel.bind(b, 4,
    cel.bind(c, 5,
      cel.bind(s, (a + b + c) / 2.0,  // semi-perimeter
        cel.bind(area_squared, s * (s - a) * (s - b) * (s - c),
          {
            "sides": [a, b, c],
            "perimeter": a + b + c,
            "semi_perimeter": s,
            "area": area_squared > 0 ? math.sqrt(area_squared) : 0.0,
            "is_valid_triangle": area_squared > 0
          }
        )
      )
    )
  )
)
```

### API Response Processing
```cel
// Process API response with multiple transformations
cel.bind(response, api.getData(),
  cel.bind(items, response.items,
    cel.bind(valid_items, items.filter(item, item.status == "active"),
      cel.bind(enriched_items, valid_items.map(item, 
        item + {"display_name": item.first_name + " " + item.last_name}
      ),
        cel.bind(sorted_items, enriched_items.sortBy(item, item.display_name),
          {
            "total_items": items.size(),
            "valid_items": valid_items.size(),
            "processed_items": sorted_items,
            "success_rate": items.size() > 0 ? 
              double(valid_items.size()) / double(items.size()) : 0.0
          }
        )
      )
    )
  )
)
```

### Conditional Processing with Shared Values
```cel
// Complex conditional logic with shared computations
cel.bind(user_type, user.role,
  cel.bind(base_permissions, 
    user_type == "admin" ? ["read", "write", "delete", "admin"] :
    user_type == "editor" ? ["read", "write"] :
    user_type == "viewer" ? ["read"] : [],
    cel.bind(requested_action, request.action,
      cel.bind(has_permission, base_permissions.contains([requested_action]),
        cel.bind(rate_limit_ok, user.requests_today < user.daily_limit,
          {
            "user": user.name,
            "user_type": user_type,
            "action": requested_action,
            "authorized": has_permission && rate_limit_ok,
            "reason": !has_permission ? "insufficient_permissions" :
                     !rate_limit_ok ? "rate_limit_exceeded" : "authorized",
            "available_permissions": base_permissions
          }
        )
      )
    )
  )
)
```

**Important Notes:**
- Bindings create local scopes and do not affect global variables
- Nested bindings can reference variables from outer scopes
- The same variable name can be reused in different binding scopes
- Bindings are particularly useful for avoiding repeated expensive computations
- Local bindings are not guaranteed to be evaluated before use (lazy evaluation)

The bindings extension is essential for writing maintainable, efficient CEL expressions by allowing you to break down complex logic into manageable, reusable components while avoiding redundant calculations. 