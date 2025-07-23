# Bindings 扩展

Bindings 扩展提供变量绑定功能，允许您在 CEL 表达式中定义局部变量绑定，以提高可读性和性能。

## 目录

- [Bindings 扩展](#bindings-扩展)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. 变量绑定 - `cel.bind()`](#2-变量绑定---celbind)
    - [cel.bind()](#celbind)
  - [3. 使用示例](#3-使用示例)
    - [性能优化](#性能优化)
    - [嵌套绑定](#嵌套绑定)
    - [复杂条件逻辑](#复杂条件逻辑)
    - [数据转换管道](#数据转换管道)
    - [避免重复计算](#避免重复计算)
    - [字符串处理](#字符串处理)
    - [带重用的列表处理](#带重用的列表处理)
    - [配置处理](#配置处理)
    - [数学计算](#数学计算)
    - [API 响应处理](#api-响应处理)
    - [带共享值的条件处理](#带共享值的条件处理)

## 1. 概述

Bindings 扩展引入了 `cel.bind()` 宏，允许您将简单标识符绑定到初始化表达式，这些表达式可以在后续结果表达式中使用。这对于复杂表达式特别有用，您希望避免多次重新计算相同的值或提高可读性。

**关键特性：**
- 表达式内的局部变量绑定
- 支持嵌套绑定
- 通过值重用进行性能优化
- 改善表达式可读性

**启用扩展**:
```rust
let env = Env::builder()
    .with_ext_bindings(true)
    .build()?;
```

## 2. 变量绑定 - `cel.bind()`

### cel.bind()

将简单标识符绑定到初始化表达式，该表达式可以在后续结果表达式中使用。绑定也可以相互嵌套。

**语法:** `cel.bind(varName, initExpr, resultExpr)`

**参数:**
- `varName`: 变量名（标识符）
- `initExpr`: 要绑定到变量的初始化表达式
- `resultExpr`: 可以使用绑定变量的结果表达式

**返回类型:** 结果表达式的结果

**示例:**
```cel
cel.bind(a, 'hello',
cel.bind(b, 'world', a + b + b + a)) // "helloworldworldhello"

cel.bind(x, 5, x * x)                        // 25
cel.bind(name, "Alice", "Hello, " + name)    // "Hello, Alice"
cel.bind(list, [1, 2, 3], list.size() > 2)  // true
```

**行为:**
- 变量仅在结果表达式作用域内可用
- 变量可以在嵌套绑定中使用
- 初始化表达式在创建绑定时计算一次
- 局部绑定不保证在使用前求值（惰性求值）
- 支持任何 CEL 类型（基本类型、列表、映射等）

## 3. 使用示例

### 性能优化
```cel
// 避免在 exists 理解中进行列表分配
cel.bind(valid_values, [a, b, c],
  [d, e, f].exists(elem, elem in valid_values))
```

### 嵌套绑定
```cel
// 链接多个绑定
cel.bind(a, 'hello',
  cel.bind(b, 'world', 
    cel.bind(c, a + ' ' + b,
      c + '!')))
// 结果: "hello world!"
```

### 复杂条件逻辑
```cel
// 使用中间变量简化复杂条件
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

### 数据转换管道
```cel
// 使用中间结果链接数据转换
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

### 避免重复计算
```cel
// 计算昂贵操作一次
cel.bind(expensive_calc, computeComplexValue(input),
  cel.bind(threshold, 100,
    expensive_calc > threshold ? 
      expensive_calc * 2 : 
      expensive_calc / 2
  )
)
```

### 字符串处理
```cel
// 使用中间结果进行复杂字符串操作
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

### 带重用的列表处理
```cel
// 对相同数据进行多种操作处理列表
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

### 配置处理
```cel
// 带验证的配置处理
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

### 数学计算
```cel
// 带中间值的复杂数学表达式
cel.bind(a, 3,
  cel.bind(b, 4,
    cel.bind(c, 5,
      cel.bind(s, (a + b + c) / 2.0,  // 半周长
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

### API 响应处理
```cel
// 使用多种转换处理 API 响应
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

### 带共享值的条件处理
```cel
// 带共享计算的复杂条件逻辑
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

**重要注意事项：**
- 绑定创建局部作用域，不影响全局变量
- 嵌套绑定可以引用外部作用域的变量
- 相同的变量名可以在不同绑定作用域中重用
- 绑定对于避免重复昂贵计算特别有用
- 局部绑定不保证在使用前求值（惰性求值）

Bindings 扩展对于编写可维护、高效的 CEL 表达式至关重要，它允许您将复杂逻辑分解为可管理、可重用的组件，同时避免冗余计算。 