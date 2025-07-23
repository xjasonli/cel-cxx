# Encoders 扩展

Encoders 扩展为字符串、字节和对象编码提供编码和解码函数。目前，它专注于 Base64 编码操作。

## 目录

- [Encoders 扩展](#encoders-扩展)
  - [目录](#目录)
  - [1. 概述](#1-概述)
  - [2. Base64 编码](#2-base64-编码)
    - [2.1 base64.decode()](#21-base64decode)
    - [2.2 base64.encode()](#22-base64encode)
  - [3. 使用示例](#3-使用示例)
    - [基本数据编码](#基本数据编码)
    - [二进制数据处理](#二进制数据处理)
    - [配置存储](#配置存储)
    - [错误处理](#错误处理)
    - [数据验证](#数据验证)
    - [填充处理](#填充处理)

## 1. 概述

Encoders 扩展为常见数据交换格式提供安全、确定性的编码和解码操作。目前，它支持 Base64 编码和解码操作，所有函数都优雅地处理边缘情况并维护 CEL 的安全保证。

**启用扩展**:
```rust
let env = Env::builder()
    .with_ext_encoders(true)
    .build()?;
```

## 2. Base64 编码

### 2.1 base64.decode()

将 base64 编码的字符串解码为字节。

**语法:** `base64.decode(string)`

**参数:**
- `string`: Base64 编码的字符串

**返回类型:** bytes（解码的二进制数据）

**示例:**
```cel
base64.decode('aGVsbG8=')                  // b"hello"
base64.decode('aGVsbG8')                   // b"hello" (处理缺失填充)
base64.decode('')                          // b""
```

**错误处理:**
- 如果字符串输入不是 base64 编码的，返回错误
- 支持标准和原始（无填充）Base64 编码
- 通过尝试替代编码自动处理缺失填充

### 2.2 base64.encode()

将字节编码为 base64 编码的字符串。

**语法:** `base64.encode(bytes)`

**参数:**
- `bytes`: 要编码的二进制数据

**返回类型:** string（Base64 编码）

**示例:**
```cel
base64.encode(b'hello')                    // "aGVsbG8="
base64.encode(b'world')                    // "d29ybGQ="
base64.encode(b'')                         // ""
```

**行为:**
- 使用带填充的标准 Base64 编码
- 总是产生有效的 Base64 输出
- 空字节输入产生空字符串输出

## 3. 使用示例

### 基本数据编码
```cel
// 编码和解码文本数据
cel.bind(text, "Hello, World!",
  cel.bind(encoded, base64.encode(bytes(text)),
    cel.bind(decoded, string(base64.decode(encoded)),
      {
        "original": text,
        "encoded": encoded,
        "decoded": decoded,
        "roundtrip_success": text == decoded
      }
    )
  )
)
// 结果: {"original": "Hello, World!", "encoded": "SGVsbG8sIFdvcmxkIQ==", "decoded": "Hello, World!", "roundtrip_success": true}
```

### 二进制数据处理
```cel
// 处理二进制数据
cel.bind(binary_data, b"\x00\x01\x02\x03\xFF",
  cel.bind(encoded, base64.encode(binary_data),
    cel.bind(decoded, base64.decode(encoded),
      {
        "original_size": binary_data.size(),
        "encoded": encoded,
        "decoded_size": decoded.size(),
        "data_preserved": binary_data == decoded
      }
    )
  )
)
```

### 配置存储
```cel
// 将配置存储为 Base64
cel.bind(config, {"version": "1.0", "enabled": true},
  cel.bind(json_config, json.encode(config),
    cel.bind(encoded_config, base64.encode(bytes(json_config)),
      {
        "config": config,
        "storage_format": encoded_config,
        "retrieval_test": json.decode(string(base64.decode(encoded_config)))
      }
    )
  )
)
```

### 错误处理
```cel
// 处理无效的 Base64 输入
cel.bind(invalid_b64, "not-valid-base64!@#",
  cel.bind(decode_result, base64.decode(invalid_b64),
    {
      "input": invalid_b64,
      "decode_success": !types.isError(decode_result),
      "result": decode_result
    }
  )
)
// 结果: 由于无效 Base64 输入导致错误
```

### 数据验证
```cel
// 验证 Base64 编码数据
cel.bind(data, "SGVsbG8gV29ybGQ=",
  cel.bind(decoded, base64.decode(data),
    {
      "is_valid_base64": !types.isError(decoded),
      "decoded_text": types.isError(decoded) ? "" : string(decoded),
      "original_data": data
    }
  )
)
// 结果: {"is_valid_base64": true, "decoded_text": "Hello World", "original_data": "SGVsbG8gV29ybGQ="}
```

### 填充处理
```cel
// 演示自动填充处理
cel.bind(padded, "SGVsbG8=",
  cel.bind(unpadded, "SGVsbG8",
    {
      "padded_decode": string(base64.decode(padded)),
      "unpadded_decode": string(base64.decode(unpadded)),
      "both_equal": base64.decode(padded) == base64.decode(unpadded)
    }
  )
)
// 结果: {"padded_decode": "Hello", "unpadded_decode": "Hello", "both_equal": true}
```

**注意**: CEL-Go 中 encoders 扩展的当前实现专门专注于 Base64 操作。虽然扩展设计为可扩展到其他编码格式（URL 编码、JSON 编码等），但这些在核心库中尚未实现。URL 和 JSON 编码的文档代表潜在的未来功能或自定义实现。

Encoders 扩展为在 CEL 表达式中处理 Web API、数据存储和系统间通信提供了基本的数据转换功能。 