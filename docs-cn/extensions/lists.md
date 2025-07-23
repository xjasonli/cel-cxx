# Lists 扩展

Lists 扩展提供了高级列表处理函数，扩展了 CEL 的内置列表功能。

## 目录

- [Lists 扩展](#lists-扩展)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. 列表切片 - `slice()`](#2-列表切片---slice)
  - [3. 列表扁平化 - `flatten()`](#3-列表扁平化---flatten)
  - [4. 列表去重 - `distinct()`](#4-列表去重---distinct)
  - [5. 列表反转 - `reverse()`](#5-列表反转---reverse)
  - [6. 列表排序](#6-列表排序)
    - [6.1 sort()](#61-sort)
    - [6.2 sortBy()](#62-sortby)
  - [7. 数字范围 - `range()`](#7-数字范围---range)
  - [8. 使用示例](#8-使用示例)
    - [数据去重和清理](#数据去重和清理)
    - [嵌套数据处理](#嵌套数据处理)
    - [分页和切片](#分页和切片)
    - [复杂数据转换](#复杂数据转换)
    - [多级扁平化](#多级扁平化)
    - [基于范围的操作](#基于范围的操作)
    - [统计分析](#统计分析)

## 1. 概述

Lists 扩展通过强大的列表操作功能增强了 CEL。所有函数都保持 CEL 的不可变性保证，返回新列表而不是修改现有列表。所有索引都从零开始。

**启用扩展**：
```rust
let env = Env::builder()
    .with_ext_lists(true)
    .build()?;
```

## 2. 列表切片 - `slice()`

**引入版本：** 0

使用提供的索引返回新的子列表。

**语法：** `list.slice(start, end)`

**参数：**
- `list`：输入列表
- `start`：起始索引（包含）
- `end`：结束索引（不包含）

**返回类型：** `list`

**示例：**
```cel
[1, 2, 3, 4, 5].slice(1, 4)     // [2, 3, 4]
["a", "b", "c", "d"].slice(0, 2) // ["a", "b"]
[1, 2, 3].slice(1, 1)           // []（空列表）
```

## 3. 列表扁平化 - `flatten()`

**语法：** `list.flatten()`

**描述：** 将嵌套列表扁平化为单一层级的列表。

**参数：**
- `list`：包含嵌套列表的输入列表

**返回类型：** `list`

**示例：**
```cel
[[1, 2], [3, 4], [5]].flatten()     // [1, 2, 3, 4, 5]
[["a", "b"], ["c"]].flatten()       // ["a", "b", "c"]
[[], [1, 2], []].flatten()          // [1, 2]
```

## 4. 列表去重 - `distinct()`

**语法：** `list.distinct()`

**描述：** 返回包含唯一元素的新列表，保持第一次出现的顺序。

**参数：**
- `list`：输入列表

**返回类型：** `list`

**示例：**
```cel
[1, 2, 2, 3, 1, 4].distinct()      // [1, 2, 3, 4]
["a", "b", "a", "c"].distinct()    // ["a", "b", "c"]
[].distinct()                      // []
```

## 5. 列表反转 - `reverse()`

**语法：** `list.reverse()`

**描述：** 返回元素顺序相反的新列表。

**参数：**
- `list`：输入列表

**返回类型：** `list`

**示例：**
```cel
[1, 2, 3, 4].reverse()             // [4, 3, 2, 1]
["hello", "world"].reverse()       // ["world", "hello"]
[42].reverse()                     // [42]
```

## 6. 列表排序

### 6.1 sort()

**语法：** `list.sort()`

**描述：** 返回按升序排序的新列表。

**参数：**
- `list`：包含可比较元素的输入列表

**返回类型：** `list`

**示例：**
```cel
[3, 1, 4, 1, 5].sort()             // [1, 1, 3, 4, 5]
["c", "a", "b"].sort()             // ["a", "b", "c"]
[3.14, 2.71, 1.41].sort()          // [1.41, 2.71, 3.14]
```

### 6.2 sortBy()

**语法：** `list.sortBy(keyFunction)`

**描述：** 使用键函数返回排序的新列表。

**参数：**
- `list`：输入列表
- `keyFunction`：为每个元素生成排序键的函数

**返回类型：** `list`

**示例：**
```cel
["apple", "pie", "cherry"].sortBy(x, size(x))  // ["pie", "apple", "cherry"]
[{"name": "Bob", "age": 25}, {"name": "Alice", "age": 30}].sortBy(x, x.age)
// [{"name": "Bob", "age": 25}, {"name": "Alice", "age": 30}]
```

## 7. 数字范围 - `range()`

**语法：** `lists.range(end)` 或 `lists.range(start, end)` 或 `lists.range(start, end, step)`

**描述：** 生成数字范围的列表。

**参数：**
- `end`：结束值（不包含）
- `start`：起始值（包含，默认为 0）
- `step`：步长（默认为 1）

**返回类型：** `list<int>`

**示例：**
```cel
lists.range(5)                     // [0, 1, 2, 3, 4]
lists.range(2, 6)                  // [2, 3, 4, 5]
lists.range(0, 10, 2)              // [0, 2, 4, 6, 8]
lists.range(10, 0, -2)             // [10, 8, 6, 4, 2]
```

## 8. 使用示例

### 数据去重和清理
```cel
// 清理和去重用户输入
cel.bind(userIds, [1, 2, 2, 3, 1, 4, null, 5],
  userIds.filter(id, id != null).distinct().sort()
)
// 结果：[1, 2, 3, 4, 5]
```

### 嵌套数据处理
```cel
// 处理嵌套的标签列表
cel.bind(posts, [
  {"tags": ["tech", "ai"]},
  {"tags": ["web", "js"]},
  {"tags": ["tech", "web"]}
],
  posts.map(p, p.tags).flatten().distinct().sort()
)
// 结果：["ai", "js", "tech", "web"]
```

### 分页和切片
```cel
// 实现分页
cel.bind(items, lists.range(100),  // 0-99 的列表
  cel.bind(page, 2,
    cel.bind(pageSize, 10,
      cel.bind(start, page * pageSize,
        items.slice(start, start + pageSize)
      )
    )
  )
)
// 结果：[20, 21, 22, 23, 24, 25, 26, 27, 28, 29]
```

### 复杂数据转换
```cel
// 处理和转换数据
cel.bind(data, [
  {"name": "Alice", "scores": [85, 92, 78]},
  {"name": "Bob", "scores": [90, 88, 95]},
  {"name": "Charlie", "scores": [75, 80, 85]}
],
  data.map(student, {
    "name": student.name,
    "average": student.scores.sum() / student.scores.size(),
    "best": student.scores.sort().reverse()[0]
  }).sortBy(s, s.average).reverse()
)
```

### 多级扁平化
```cel
// 扁平化多层嵌套结构
cel.bind(nested, [[[1, 2]], [[3, 4], [5]], [[6]]],
  nested.flatten().flatten()
)
// 结果：[1, 2, 3, 4, 5, 6]
```

### 基于范围的操作
```cel
// 生成和处理数字序列
cel.bind(fibonacci_like, lists.range(10).map(i, 
  i < 2 ? i : (i * 2 - 1)  // 简化的类斐波那契序列
),
  fibonacci_like.filter(n, n % 2 == 0)  // 只保留偶数
)
// 结果：[0, 2, 6, 10, 14]
```

### 统计分析
```cel
// 计算分位数和统计信息
cel.bind(values, [23, 45, 12, 67, 34, 89, 56, 78, 91, 43],
  cel.bind(sorted, values.sort(),
    {
      "min": sorted[0],
      "max": sorted[sorted.size() - 1],
      "median": sorted[sorted.size() / 2],
      "q1": sorted[sorted.size() / 4],
      "q3": sorted[sorted.size() * 3 / 4],
      "range": sorted[sorted.size() - 1] - sorted[0]
    }
  )
)
```

Lists 扩展提供了强大的列表操作功能，使您能够高效地处理和转换复杂的数据结构。所有操作都保持不可变性，确保数据安全和可预测的行为。 