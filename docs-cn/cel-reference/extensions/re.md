# RE 扩展

RE 扩展提供了基于 RE2 库构建的 C++ 特定正则表达式函数。此扩展为字符串模式匹配、提取和捕获操作提供了附加功能。

## 目录

- [RE 扩展](#re-扩展)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. 模式提取 - `re.extract()`](#2-模式提取---reextract)
  - [3. 组捕获 - `re.capture()` 和 `re.captureN()`](#3-组捕获---recapture-和-recapturen)
    - [re.capture()](#recapture)
    - [re.captureN()](#recapturen)
  - [4. 使用示例](#4-使用示例)
    - [日期格式转换](#日期格式转换)
    - [电子邮件信息提取](#电子邮件信息提取)
    - [电话号码格式化](#电话号码格式化)
    - [URL 解析](#url-解析)
    - [日志解析](#日志解析)
    - [文本清理和格式化](#文本清理和格式化)
    - [信用卡号码掩码](#信用卡号码掩码)
    - [版本号解析](#版本号解析)
    - [代码提取](#代码提取)

## 1. 概述

RE 扩展基于高性能的 RE2 正则表达式库，提供了高级的模式匹配和文本处理功能。它专为性能和安全性而设计，避免了正则表达式的灾难性回溯问题。

**主要功能：**
- 模式提取和重写（`re.extract`）
- 组捕获（`re.capture`、`re.captureN`）
- 直接错误处理
- RE2 库性能和安全性

**正则表达式语法**: 所有正则表达式函数都使用 RE2 语法。详细语法参考请见：[RE2 语法文档](https://github.com/google/re2/wiki/Syntax)

**启用扩展**：
```rust
let env = Env::builder()
    .with_ext_re(true)
    .build()?;
```

## 2. 模式提取 - `re.extract()`

**语法：** `re.extract(string, pattern, replacement)`

**描述：** 使用正则表达式模式从字符串中提取并重写内容。

**参数：**
- `string`：输入字符串
- `pattern`：正则表达式模式（支持捕获组）
- `replacement`：替换模板（可使用 \1, \2 等引用捕获组）

**返回类型：** `string`

**示例：**
```cel
re.extract("Hello World", r"(\w+) (\w+)", r"\2, \1")  // "World, Hello"
re.extract("user@example.com", r"(\w+)@(\w+\.\w+)", r"User: \1, Domain: \2")  // "User: user, Domain: example.com"
re.extract("2024-01-15", r"(\d{4})-(\d{2})-(\d{2})", r"\3/\2/\1")  // "15/01/2024"
```

## 3. 组捕获 - `re.capture()` 和 `re.captureN()`

### re.capture()

**语法：** `re.capture(string, pattern)`

**描述：** 捕获正则表达式中第一个捕获组的内容。

**参数：**
- `string`：输入字符串
- `pattern`：包含捕获组的正则表达式模式

**返回类型：** `string`

**示例：**
```cel
re.capture("user@example.com", r"(\w+)@[\w.]+")       // "user"
re.capture("Price: $123.45", r"Price: \$(\d+\.\d+)")  // "123.45"
re.capture("Hello World", r"Hello (\w+)")            // "World"
```

### re.captureN()

**语法：** `re.captureN(string, pattern)`

**描述：** 捕获正则表达式中所有命名捕获组，返回映射。

**参数：**
- `string`：输入字符串
- `pattern`：包含命名捕获组的正则表达式模式

**返回类型：** `map<string, string>`

**示例：**
```cel
re.captureN("John Doe", r"(?P<first>\w+) (?P<last>\w+)")  
// {"first": "John", "last": "Doe"}

re.captureN("2024-01-15", r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})")
// {"year": "2024", "month": "01", "day": "15"}

re.captureN("user@example.com", r"(?P<username>\w+)@(?P<domain>[\w.]+)")
// {"username": "user", "domain": "example.com"}
```

## 4. 使用示例

### 日期格式转换
```cel
// 将日期从 YYYY-MM-DD 转换为 DD/MM/YYYY
cel.bind(date, "2024-01-15",
  re.extract(date, r"(\d{4})-(\d{2})-(\d{2})", r"\3/\2/\1")
)
// 结果："15/01/2024"
```

### 电子邮件信息提取
```cel
// 从电子邮件地址提取用户名和域名
cel.bind(email, "john.doe@company.com",
  re.captureN(email, r"(?P<username>[\w.]+)@(?P<domain>[\w.]+)")
)
// 结果：{"username": "john.doe", "domain": "company.com"}
```

### 电话号码格式化
```cel
// 格式化电话号码
cel.bind(phone, "1234567890",
  re.extract(phone, r"(\d{3})(\d{3})(\d{4})", r"(\1) \2-\3")
)
// 结果："(123) 456-7890"
```

### URL 解析
```cel
// 解析 URL 组件
cel.bind(url, "https://www.example.com:8080/path/to/resource?param=value",
  re.captureN(url, r"(?P<protocol>https?)://(?P<host>[\w.]+)(:(?P<port>\d+))?(?P<path>/[^?]*)?(\?(?P<query>.*))?")
)
// 结果：{"protocol": "https", "host": "www.example.com", "port": "8080", "path": "/path/to/resource", "query": "param=value"}
```

### 日志解析
```cel
// 解析日志条目
cel.bind(log, "2024-01-15 10:30:45 [ERROR] Connection timeout in module auth",
  re.captureN(log, r"(?P<date>\d{4}-\d{2}-\d{2}) (?P<time>\d{2}:\d{2}:\d{2}) \[(?P<level>\w+)\] (?P<message>.*)")
)
// 结果：{"date": "2024-01-15", "time": "10:30:45", "level": "ERROR", "message": "Connection timeout in module auth"}
```

### 文本清理和格式化
```cel
// 清理和重新格式化文本
cel.bind(text, "  Hello,    World!   ",
  cel.bind(cleaned, re.extract(text, r"\s*(.+?)\s*", r"\1"),
    re.extract(cleaned, r"(\w+),\s+(\w+)!", r"\1 \2")
  )
)
// 结果："Hello World"
```

### 信用卡号码掩码
```cel
// 掩码信用卡号码，只显示最后四位
cel.bind(card, "1234567890123456",
  re.extract(card, r"(\d{4})(\d{4})(\d{4})(\d{4})", r"****-****-****-\4")
)
// 结果："****-****-****-3456"
```

### 版本号解析
```cel
// 解析语义版本号
cel.bind(version, "v1.2.3-beta.1",
  re.captureN(version, r"v(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(-(?P<prerelease>[\w.]+))?")
)
// 结果：{"major": "1", "minor": "2", "patch": "3", "prerelease": "beta.1"}
```

### 代码提取
```cel
// 从文本中提取代码块
cel.bind(text, "The function `calculateSum(a, b)` returns the sum.",
  re.capture(text, r"`([^`]+)`")
)
// 结果："calculateSum(a, b)"
```

RE 扩展提供了强大的正则表达式功能，基于高性能的 RE2 库，为复杂的文本处理和数据提取任务提供了安全可靠的解决方案。 