# 正则表达式扩展

正则表达式扩展为模式匹配和文本提取提供了高级的正则表达式操作。

## 目录

- [正则表达式扩展](#正则表达式扩展)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. 模式提取](#2-模式提取)
    - [2.1 regex.extract()](#21-regexextract)
    - [2.2 regex.extractAll()](#22-regexextractall)
  - [3. 模式替换 - `regex.replace()`](#3-模式替换---regexreplace)
    - [regex.replace()](#regexreplace)
  - [4. 使用示例](#4-使用示例)
    - [邮箱验证和解析](#邮箱验证和解析)
    - [电话号码提取](#电话号码提取)
    - [数据清理和转换](#数据清理和转换)
    - [URL 处理](#url-处理)
    - [带捕获组的文本处理](#带捕获组的文本处理)
    - [日志解析](#日志解析)
    - [数据验证](#数据验证)
    - [带限制的文本替换](#带限制的文本替换)
    - [带可选结果的模式匹配](#带可选结果的模式匹配)

## 1. 概述

正则表达式扩展使用正则表达式实现强大的模式匹配和文本提取。所有函数都使用标准正则表达式语法并提供安全、确定性的操作。

**注意**: 此库依赖于 CEL 可选类型。在使用正则表达式扩展时，请确保启用了 `cel.OptionalTypes()`。

**正则表达式语法**: 所有正则表达式函数都使用 RE2 语法。详细语法参考请见：[RE2 语法文档](https://github.com/google/re2/wiki/Syntax)

**启用扩展**:
```rust
let env = Env::builder()
    .with_ext_regex(true)
    .build()?;
```

## 2. 模式提取

### 2.1 regex.extract()

返回字符串中正则表达式模式的第一个匹配。如果未找到匹配，返回可选的 none 值。对于无效正则表达式或多个捕获组会抛出错误。

**语法:** `regex.extract(target, pattern)`

**参数:**
- `target`: 要搜索的输入字符串
- `pattern`: 正则表达式模式（建议使用原始字符串）

**返回类型:** `optional<string>`（第一个匹配，如果没有匹配则为 optional.none）

**示例:**
```cel
regex.extract('hello world', 'hello(.*)') == optional.of(' world')
regex.extract('item-A, item-B', 'item-(\\w+)') == optional.of('A')
regex.extract('HELLO', 'hello') == optional.none
regex.extract('testuser@testdomain', '(.*)@([^.]*)') // 运行时错误：多个捕获组
```

**行为:**
- 如果有捕获组，返回第一个捕获组
- 如果没有捕获组，返回整个匹配
- 如果可选组为空，返回 optional.none
- 最多支持一个捕获组（多个组会导致错误）

### 2.2 regex.extractAll()

返回目标字符串中正则表达式模式的所有匹配列表。如果未找到匹配，返回空列表。对于无效正则表达式或多个捕获组会抛出错误。

**语法:** `regex.extractAll(target, pattern)`

**参数:**
- `target`: 要搜索的输入字符串
- `pattern`: 正则表达式模式（建议使用原始字符串）

**返回类型:** `list<string>`（所有匹配）

**示例:**
```cel
regex.extractAll('id:123, id:456', 'id:\\d+') == ['id:123', 'id:456']
regex.extractAll('id:123, id:456', 'assa') == []
regex.extractAll('testuser@testdomain', '(.*)@([^.]*)') // 运行时错误：多个捕获组
```

**行为:**
- 如果有一个捕获组，返回所有捕获组
- 如果没有捕获组，返回所有整个匹配
- 空捕获组被排除在结果之外
- 最多支持一个捕获组（多个组会导致错误）

## 3. 模式替换 - `regex.replace()`

### regex.replace()

用替换字符串替换字符串中正则表达式模式的所有出现。

**语法:** `regex.replace(target, pattern, replacement)`

**参数:**
- `target`: 要搜索的输入字符串
- `pattern`: 正则表达式模式（建议使用原始字符串）
- `replacement`: 用于替换匹配的字符串

**返回类型:** `string`

**示例:**
```cel
regex.replace('hello world hello', 'hello', 'hi') == 'hi world hi'
regex.replace('banana', 'a', 'x', 0) == 'banana'
regex.replace('banana', 'a', 'x', 1) == 'bxnana'
regex.replace('banana', 'a', 'x', 2) == 'bxnxna'
regex.replace('banana', 'a', 'x', -12) == 'bxnxnx'
regex.replace('foo bar', '(fo)o (ba)r', r'\2 \1') == 'ba fo'
```

**捕获组引用:**
- 在替换字符串中使用 `\1`、`\2` 等引用捕获组
- `\0` 引用整个匹配
- 使用 `\\` 包含字面反斜杠
- 无效引用（例如，只有 2 个组时使用 `\9`）会导致错误

**错误处理:**
```cel
regex.replace('test', '(.)', r'\2') // 运行时错误：无效替换字符串
regex.replace('foo bar', '(', '$2 $1') // 运行时错误：无效正则表达式字符串
regex.replace('id=123', r'id=(?P<value>\d+)', r'value: \values') // 运行时错误：无效替换字符串
```

**行为:**
- 当计数为 0 时，返回原始字符串不变
- 当计数为负数时，替换所有出现
- 当计数超过可用匹配时，替换所有可用匹配
- 仅支持数字捕获组引用（`\1` 到 `\9`）

## 4. 使用示例

### 邮箱验证和解析
```cel
// 从邮箱提取用户名
cel.bind(email, "john.doe@company.com",
  cel.bind(username, regex.extract(email, r"([^@]+)@"),
    username.hasValue() ? {
      "valid": true,
      "username": username.value(),
      "full_email": email
    } : {"valid": false}
  )
)
// 结果: {"valid": true, "username": "john.doe", "full_email": "john.doe@company.com"}
```

### 电话号码提取
```cel
// 从文本中提取所有电话号码
cel.bind(text, "Call me at 555-123-4567 or 555-987-6543",
  cel.bind(phones, regex.extractAll(text, r"\d{3}-\d{3}-\d{4}"),
    {
      "found_count": phones.size(),
      "phone_numbers": phones
    }
  )
)
// 结果: {"found_count": 2, "phone_numbers": ["555-123-4567", "555-987-6543"]}
```

### 数据清理和转换
```cel
// 清理和转换用户输入
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
// 结果: {"original": "  Hello    World!!!  ", "cleaned": "Hello World"}
```

### URL 处理
```cel
// 从 URL 中提取域名
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
// 结果: ["www.example.com", "subdomain.test.org", "another-site.net"]
```

### 带捕获组的文本处理
```cel
// 使用捕获组转换日期格式
cel.bind(date_text, "Today is 2023-12-25 and tomorrow is 2023-12-26",
  regex.replace(date_text, r"(\d{4})-(\d{2})-(\d{2})", r"\3/\2/\1")
)
// 结果: "Today is 25/12/2023 and tomorrow is 26/12/2023"
```

### 日志解析
```cel
// 解析日志条目
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
// 结果: {"timestamp": "2023-12-25 14:30:15", "level": "ERROR", "message": "Database connection failed"}
```

### 数据验证
```cel
// 验证各种数据格式
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
// 结果: {"email_valid": true, "phone_valid": true, "zip_valid": true}
```

### 带限制的文本替换
```cel
// 使用不同计数限制进行替换
cel.bind(text, "foo foo foo foo",
  {
    "replace_all": regex.replace(text, "foo", "bar"),
    "replace_first": regex.replace(text, "foo", "bar", 1),
    "replace_two": regex.replace(text, "foo", "bar", 2),
    "replace_none": regex.replace(text, "foo", "bar", 0)
  }
)
// 结果: {
//   "replace_all": "bar bar bar bar",
//   "replace_first": "bar foo foo foo",
//   "replace_two": "bar bar foo foo",
//   "replace_none": "foo foo foo foo"
// }
```

### 带可选结果的模式匹配
```cel
// 处理可选提取结果
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
// 结果: {"text": "No numbers here", "has_number": false, "number": "none found", "message": "No numbers detected"}
```

正则表达式扩展提供了强大的模式匹配功能，对于 CEL 表达式中的文本处理、数据验证和信息提取任务至关重要。与 CEL 可选类型的集成使得处理模式可能匹配或不匹配的情况既安全又方便。 