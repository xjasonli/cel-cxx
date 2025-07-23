# Protocol Buffers 扩展

Protocol Buffers 扩展为在 CEL 表达式中处理 Protocol Buffer 消息提供增强支持，特别是用于访问 proto2 语法消息中的扩展字段。

## 目录

- [Protocol Buffers 扩展](#protocol-buffers-扩展)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. 扩展字段访问 - `proto.getExt()`](#2-扩展字段访问---protogetext)
    - [proto.getExt()](#protogetext)
  - [3. 扩展字段存在性测试 - `proto.hasExt()`](#3-扩展字段存在性测试---protohasext)
    - [proto.hasExt()](#protohasext)
  - [4. 使用示例](#4-使用示例)
    - [基本扩展字段访问](#基本扩展字段访问)
    - [基于扩展的条件处理](#基于扩展的条件处理)
    - [扩展字段验证](#扩展字段验证)
    - [多个扩展字段](#多个扩展字段)
    - [基于扩展的消息分类](#基于扩展的消息分类)
    - [嵌套扩展访问](#嵌套扩展访问)
    - [扩展字段聚合](#扩展字段聚合)
    - [基于扩展的路由逻辑](#基于扩展的路由逻辑)

## 1. 概述

Protocol Buffers 扩展通过处理 proto2 扩展字段的附加功能增强了 CEL 的内置 protobuf 支持。此扩展提供了使访问和测试 Protocol Buffer 消息上的扩展字段更容易的宏。

**注意**: 所有宏都使用 'proto' 命名空间；但是，在宏展开时，命名空间看起来就像任何其他标识符一样。如果您当前使用名为 'proto' 的变量，宏可能会按预期工作；但是，存在一些冲突的可能性。

**启用扩展**:
```rust
let env = Env::builder()
    .with_ext_proto(true)
    .build()?;
```

## 2. 扩展字段访问 - `proto.getExt()`

### proto.getExt()

生成选择表达式的宏，该表达式从输入的 proto2 语法消息中检索扩展字段。如果字段未设置，根据安全遍历语义返回扩展字段的默认值。

**语法:** `proto.getExt(msg, fully.qualified.extension.name)`

**参数:**
- `msg`: 包含扩展字段的 proto2 消息
- `fully.qualified.extension.name`: 扩展字段的完全限定名

**返回类型:** 扩展字段的类型

**示例:**
```cel
proto.getExt(msg, google.expr.proto2.test.int32_ext)     // 返回 int 值
proto.getExt(msg, google.expr.proto2.test.string_ext)    // 返回 string 值
proto.getExt(msg, google.expr.proto2.test.nested_ext)    // 返回嵌套消息
```

**行为:**
- 如果设置了扩展字段，返回其值
- 如果未设置扩展字段，返回默认值
- 使用安全遍历语义（缺失字段不会出错）
- 仅适用于支持扩展的 proto2 语法消息

## 3. 扩展字段存在性测试 - `proto.hasExt()`

### proto.hasExt()

生成仅测试选择表达式的宏，该表达式确定是否在 proto2 语法消息上设置了扩展字段。

**语法:** `proto.hasExt(msg, fully.qualified.extension.name)`

**参数:**
- `msg`: 要测试的 proto2 消息
- `fully.qualified.extension.name`: 扩展字段的完全限定名

**返回类型:** bool

**示例:**
```cel
proto.hasExt(msg, google.expr.proto2.test.int32_ext)     // 返回 true || false
proto.hasExt(msg, google.expr.proto2.test.string_ext)    // 返回 true || false
proto.hasExt(msg, google.expr.proto2.test.nested_ext)    // 返回 true || false
```

**行为:**
- 如果扩展字段被显式设置，返回 `true`
- 如果扩展字段未设置，返回 `false`
- 不触发默认值计算
- 仅适用于支持扩展的 proto2 语法消息

## 4. 使用示例

### 基本扩展字段访问
```cel
// 使用默认回退访问扩展字段
cel.bind(msg, getProtoMessage(),
  cel.bind(priority, proto.getExt(msg, com.example.priority_ext),
    cel.bind(has_priority, proto.hasExt(msg, com.example.priority_ext),
      {
        "priority": priority,
        "has_explicit_priority": has_priority,
        "effective_priority": has_priority ? priority : 0
      }
    )
  )
)
```

### 基于扩展的条件处理
```cel
// 根据扩展存在性以不同方式处理消息
cel.bind(msg, request.message,
  proto.hasExt(msg, com.example.metadata_ext) ?
    processWithMetadata(msg, proto.getExt(msg, com.example.metadata_ext)) :
    processDefault(msg)
)
```

### 扩展字段验证
```cel
// 验证扩展字段值
cel.bind(msg, input.message,
  cel.bind(config_ext, proto.getExt(msg, com.example.config_ext),
    cel.bind(has_config, proto.hasExt(msg, com.example.config_ext),
      {
        "valid": !has_config || (config_ext.timeout > 0 && config_ext.retries >= 0),
        "config": has_config ? config_ext : null,
        "timeout": has_config ? config_ext.timeout : 30,
        "retries": has_config ? config_ext.retries : 3
      }
    )
  )
)
```

### 多个扩展字段
```cel
// 处理多个扩展字段
cel.bind(msg, getRequestMessage(),
  {
    "has_auth": proto.hasExt(msg, com.example.auth_ext),
    "has_routing": proto.hasExt(msg, com.example.routing_ext),
    "has_metadata": proto.hasExt(msg, com.example.metadata_ext),
    "auth_info": proto.hasExt(msg, com.example.auth_ext) ? 
      proto.getExt(msg, com.example.auth_ext) : null,
    "routing_info": proto.hasExt(msg, com.example.routing_ext) ? 
      proto.getExt(msg, com.example.routing_ext) : null
  }
)
```

### 基于扩展的消息分类
```cel
// 根据扩展字段对消息进行分类
cel.bind(msg, input.message,
  cel.bind(msg_type, proto.getExt(msg, com.example.message_type_ext),
    cel.bind(has_type, proto.hasExt(msg, com.example.message_type_ext),
      {
        "classification": has_type ? msg_type : "unknown",
        "is_priority": has_type && msg_type == "priority",
        "is_bulk": has_type && msg_type == "bulk",
        "needs_special_handling": has_type && msg_type in ["priority", "urgent", "critical"]
      }
    )
  )
)
```

### 嵌套扩展访问
```cel
// 访问嵌套扩展字段
cel.bind(msg, getComplexMessage(),
  cel.bind(nested_ext, proto.getExt(msg, com.example.nested_ext),
    cel.bind(has_nested, proto.hasExt(msg, com.example.nested_ext),
      {
        "has_nested_extension": has_nested,
        "nested_value": has_nested ? nested_ext.inner_field : "",
        "nested_count": has_nested ? nested_ext.items.size() : 0,
        "is_valid_nested": has_nested && nested_ext.inner_field != "" && nested_ext.items.size() > 0
      }
    )
  )
)
```

### 扩展字段聚合
```cel
// 从带扩展的多个消息聚合数据
cel.bind(messages, input.message_list,
  cel.bind(with_priority, messages.filter(msg, proto.hasExt(msg, com.example.priority_ext)),
    cel.bind(priorities, with_priority.map(msg, proto.getExt(msg, com.example.priority_ext)),
      {
        "total_messages": messages.size(),
        "messages_with_priority": with_priority.size(),
        "max_priority": priorities.size() > 0 ? math.max(priorities) : 0,
        "avg_priority": priorities.size() > 0 ? 
          priorities.sum() / double(priorities.size()) : 0.0
      }
    )
  )
)
```

### 基于扩展的路由逻辑
```cel
// 根据扩展字段路由消息
cel.bind(msg, request.message,
  cel.bind(routing_ext, proto.getExt(msg, com.example.routing_ext),
    cel.bind(has_routing, proto.hasExt(msg, com.example.routing_ext),
      {
        "destination": has_routing ? routing_ext.destination : "default",
        "priority": has_routing ? routing_ext.priority : 1,
        "route_config": {
          "use_fast_path": has_routing && routing_ext.priority > 5,
          "enable_caching": has_routing && routing_ext.cache_enabled,
          "timeout_ms": has_routing ? routing_ext.timeout_ms : 5000
        }
      }
    )
  )
)
```

**重要注意事项：**
- 扩展字段仅在 proto2 语法消息中可用
- 扩展字段名必须完全限定（例如，`com.example.my_extension`）
- 扩展字段必须在 proto 定义中正确定义并注册
- 宏生成在编译时优化的高效选择表达式
- 应用安全遍历语义：访问不存在的扩展返回默认值而不是错误

Protocol Buffers 扩展为在 CEL 表达式中处理 proto2 扩展字段提供了强大的功能，支持灵活的消息处理和验证模式。 