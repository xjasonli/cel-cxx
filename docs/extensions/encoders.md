# Encoders Extension

The encoders extension provides encoding and decoding functions for string, byte, and object encodings. Currently, it focuses on Base64 encoding operations.

## Table of Contents

- [Encoders Extension](#encoders-extension)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Base64 Encoding](#2-base64-encoding)
    - [base64.decode()](#base64decode)
    - [base64.encode()](#base64encode)
  - [3. Usage Examples](#3-usage-examples)
    - [Basic Data Encoding](#basic-data-encoding)
    - [Binary Data Handling](#binary-data-handling)
    - [Configuration Storage](#configuration-storage)
    - [Error Handling](#error-handling)
    - [Data Validation](#data-validation)
    - [Padding Handling](#padding-handling)

## 1. Overview

The encoders extension provides safe, deterministic encoding and decoding operations for common data interchange formats. Currently, it supports Base64 encoding and decoding operations, with all functions handling edge cases gracefully and maintaining CEL's safety guarantees.

**Enabling the Extension**:
```rust
let env = Env::builder()
    .with_ext_encoders(true)
    .build()?;
```

## 2. Base64 Encoding

### base64.decode()

Decodes base64-encoded string to bytes.

**Syntax:** `base64.decode(string)`

**Parameters:**
- `string`: Base64 encoded string

**Return Type:** bytes (decoded binary data)

**Examples:**
```cel
base64.decode('aGVsbG8=')                  // b"hello"
base64.decode('aGVsbG8')                   // b"hello" (handles missing padding)
base64.decode('')                          // b""
```

**Error Handling:**
- Returns an error if the string input is not base64-encoded
- Supports both standard and raw (unpadded) Base64 encoding
- Automatically handles missing padding by trying alternative encoding

### base64.encode()

Encodes bytes to a base64-encoded string.

**Syntax:** `base64.encode(bytes)`

**Parameters:**
- `bytes`: Binary data to encode

**Return Type:** string (Base64 encoded)

**Examples:**
```cel
base64.encode(b'hello')                    // "aGVsbG8="
base64.encode(b'world')                    // "d29ybGQ="
base64.encode(b'')                         // ""
```

**Behavior:**
- Uses standard Base64 encoding with padding
- Always produces valid Base64 output
- Empty bytes input produces empty string output

## 3. Usage Examples

### Basic Data Encoding
```cel
// Encode and decode text data
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
// Result: {"original": "Hello, World!", "encoded": "SGVsbG8sIFdvcmxkIQ==", "decoded": "Hello, World!", "roundtrip_success": true}
```

### Binary Data Handling
```cel
// Work with binary data
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

### Configuration Storage
```cel
// Store configuration as Base64
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

### Error Handling
```cel
// Handle invalid Base64 input
cel.bind(invalid_b64, "not-valid-base64!@#",
  cel.bind(decode_result, base64.decode(invalid_b64),
    {
      "input": invalid_b64,
      "decode_success": !types.isError(decode_result),
      "result": decode_result
    }
  )
)
// Result: Error due to invalid Base64 input
```

### Data Validation
```cel
// Validate Base64 encoded data
cel.bind(data, "SGVsbG8gV29ybGQ=",
  cel.bind(decoded, base64.decode(data),
    {
      "is_valid_base64": !types.isError(decoded),
      "decoded_text": types.isError(decoded) ? "" : string(decoded),
      "original_data": data
    }
  )
)
// Result: {"is_valid_base64": true, "decoded_text": "Hello World", "original_data": "SGVsbG8gV29ybGQ="}
```

### Padding Handling
```cel
// Demonstrate automatic padding handling
cel.bind(padded, "SGVsbG8=",
  cel.bind(unpadded, "SGVsbG8",
    {
      "padded_decode": string(base64.decode(padded)),
      "unpadded_decode": string(base64.decode(unpadded)),
      "both_equal": base64.decode(padded) == base64.decode(unpadded)
    }
  )
)
// Result: {"padded_decode": "Hello", "unpadded_decode": "Hello", "both_equal": true}
```

**Note**: The current implementation of the encoders extension in CEL-Go focuses specifically on Base64 operations. While the extension is designed to be extensible for other encoding formats (URL encoding, JSON encoding, etc.), these are not yet implemented in the core library. The documentation for URL and JSON encoding represents potential future functionality or custom implementations. 
        {
          "original": nested_data,
          "json_size": json_str.size(),
          "url_encoded_size": url_encoded.size(),
          "base64_size": base64_encoded.size(),
          "final_encoded": base64_encoded,
          "roundtrip_valid": json.decode(url.decode(string(base64.decode(base64_encoded)))) == nested_data
        }
      )
    )
  )
)
```

The encoders extension provides essential data transformation capabilities for working with web APIs, data storage, and inter-system communication in CEL expressions. 