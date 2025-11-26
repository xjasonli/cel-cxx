# Sets 扩展

Sets 扩展为 CEL 提供了集合关系函数。虽然 CEL 中没有专用的集合类型，但此扩展提供了将列表视为集合并执行集合操作的功能。

## 目录

- [Sets 扩展](#sets-扩展)
  - [目录](#目录)
  - [概述](#概述)
  - [启用扩展](#启用扩展)
  - [函数](#函数)
    - [sets.contains()](#setscontains)
    - [sets.equivalent()](#setsequivalent)
    - [sets.intersects()](#setsintersects)
  - [使用示例](#使用示例)
    - [权限检查](#权限检查)
    - [数据验证](#数据验证)
    - [集合操作](#集合操作)

## 概述

Sets 扩展通过在列表上操作的集合关系函数增强了 CEL。由于 CEL 没有原生的集合类型，这些函数将列表视为集合，并提供确定集合包含、等价和交集的基本功能。

**关键特性：**
- 集合包含检查
- 集合等价测试
- 集合交集检测
- 适用于任何可比较类型
- 使用标准 CEL 等式进行比较

## 启用扩展

要在 CEL 环境中启用 sets 扩展：

```rust
let env = Env::builder()
    .with_ext_sets(true)
    .build()?;
```

## 函数

### sets.contains()

返回第一个列表参数是否包含第二个列表参数中的所有元素。

**语法**: `sets.contains(list(T), list(T)) -> bool`

**参数**:
- `list1` (list): 容器列表
- `list2` (list): 要检查包含性的元素

**返回**: `bool` - 如果 `list2` 中的所有元素都存在于 `list1` 中则返回 `true`

**行为**:
- 使用标准 CEL 等式来确定元素存在性
- 如果第二个列表为空，总是返回 `true`
- 元素可以是任何类型
- 支持类型强制转换（例如，`1`、`1.0`、`1u` 被认为是相等的）

**示例**:
```cel
sets.contains([], [])                    // true
sets.contains([], [1])                   // false
sets.contains([1, 2, 3, 4], [2, 3])      // true
sets.contains([1, 2.0, 3u], [1.0, 2u, 3]) // true
```

### sets.equivalent()

返回第一个和第二个列表是否集合等价。

**语法**: `sets.equivalent(list(T), list(T)) -> bool`

**参数**:
- `list1` (list): 要比较的第一个列表
- `list2` (list): 要比较的第二个列表

**返回**: `bool` - 如果列表集合等价则返回 `true`

**行为**:
- 如果第一个列表中的每个项目在第二个列表中都有相等的元素，则列表集合等价
- 列表可能大小不同（重复项不影响等价性）
- 使用标准 CEL 等式进行比较
- 支持类型强制转换

**示例**:
```cel
sets.equivalent([], [])                  // true
sets.equivalent([1], [1, 1])             // true
sets.equivalent([1], [1u, 1.0])          // true
sets.equivalent([1, 2, 3], [3u, 2.0, 1]) // true
```

### sets.intersects()

返回第一个列表是否至少有一个元素的值等于第二个列表中的元素。

**语法**: `sets.intersects(list(T), list(T)) -> bool`

**参数**:
- `list1` (list): 要比较的第一个列表
- `list2` (list): 要比较的第二个列表

**返回**: `bool` - 如果列表至少有一个公共元素则返回 `true`

**行为**:
- 如果任一列表为空，返回 `false`
- 使用标准 CEL 等式进行比较
- 支持类型强制转换
- 只需要一个匹配元素就返回 `true`

**示例**:
```cel
sets.intersects([1], [])                 // false
sets.intersects([1], [1, 2])             // true
sets.intersects([[1], [2, 3]], [[1, 2], [2, 3.0]]) // true
```

## 使用示例

### 权限检查

```cel
// 检查用户是否有所需权限
cel.bind(user_permissions, ["read", "write", "execute"],
  cel.bind(required_permissions, ["read", "write"],
    sets.contains(user_permissions, required_permissions)
  )
)
// 结果: true
```

### 数据验证

```cel
// 验证提交的类别是否来自允许列表
cel.bind(allowed_categories, ["tech", "health", "finance", "education"],
  cel.bind(submitted_categories, ["tech", "health"],
    sets.contains(allowed_categories, submitted_categories)
  )
)
// 结果: true
```

### 集合操作

```cel
// 检查两个用户组是否有等价权限
cel.bind(group_a_permissions, ["read", "write", "delete"],
  cel.bind(group_b_permissions, ["delete", "read", "write"],
    sets.equivalent(group_a_permissions, group_b_permissions)
  )
)
// 结果: true

// 检查用户是否有任何管理员权限
cel.bind(user_permissions, ["read", "write"],
  cel.bind(admin_permissions, ["delete", "admin", "sudo"],
    sets.intersects(user_permissions, admin_permissions)
  )
)
// 结果: false
```

Sets 扩展为 CEL 表达式提供了基本的集合操作，支持强大的数据验证和权限检查场景，同时保持 CEL 的类型安全和不可变性保证。 