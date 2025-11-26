# Protocol Buffers Extension

The Protocol Buffers extension provides enhanced support for working with Protocol Buffer messages in CEL expressions, particularly for accessing extension fields in proto2 syntax messages.

## Table of Contents

- [Protocol Buffers Extension](#protocol-buffers-extension)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Extension Field Access - `proto.getExt()`](#2-extension-field-access---protogetext)
    - [proto.getExt()](#protogetext)
  - [3. Extension Field Presence Testing - `proto.hasExt()`](#3-extension-field-presence-testing---protohasext)
    - [proto.hasExt()](#protohasext)
  - [4. Usage Examples](#4-usage-examples)
    - [Basic Extension Field Access](#basic-extension-field-access)
    - [Conditional Processing Based on Extensions](#conditional-processing-based-on-extensions)
    - [Extension Field Validation](#extension-field-validation)
    - [Multiple Extension Fields](#multiple-extension-fields)
    - [Extension-based Message Classification](#extension-based-message-classification)
    - [Nested Extension Access](#nested-extension-access)
    - [Extension Field Aggregation](#extension-field-aggregation)
    - [Extension-based Routing Logic](#extension-based-routing-logic)

## 1. Overview

The Protocol Buffers extension enhances CEL's built-in protobuf support with additional functionality for working with proto2 extension fields. This extension provides macros that make it easier to access and test extension fields on Protocol Buffer messages.

**Note**: All macros use the 'proto' namespace; however, at the time of macro expansion the namespace looks just like any other identifier. If you are currently using a variable named 'proto', the macro will likely work just as intended; however, there is some chance for collision.

**Enabling the Extension**:
```rust
let env = Env::builder()
    .with_ext_proto(true)
    .build()?;
```

## 2. Extension Field Access - `proto.getExt()`

### proto.getExt()

Macro which generates a select expression that retrieves an extension field from the input proto2 syntax message. If the field is not set, the default value for the extension field is returned according to safe-traversal semantics.

**Syntax:** `proto.getExt(msg, fully.qualified.extension.name)`

**Parameters:**
- `msg`: The proto2 message containing the extension field
- `fully.qualified.extension.name`: The fully qualified name of the extension field

**Return Type:** The type of the extension field

**Examples:**
```cel
proto.getExt(msg, google.expr.proto2.test.int32_ext)     // returns int value
proto.getExt(msg, google.expr.proto2.test.string_ext)    // returns string value
proto.getExt(msg, google.expr.proto2.test.nested_ext)    // returns nested message
```

**Behavior:**
- Returns the value of the extension field if set
- Returns the default value for the extension field if not set
- Uses safe-traversal semantics (no errors on missing fields)
- Only works with proto2 syntax messages that support extensions

## 3. Extension Field Presence Testing - `proto.hasExt()`

### proto.hasExt()

Macro which generates a test-only select expression that determines whether an extension field is set on a proto2 syntax message.

**Syntax:** `proto.hasExt(msg, fully.qualified.extension.name)`

**Parameters:**
- `msg`: The proto2 message to test
- `fully.qualified.extension.name`: The fully qualified name of the extension field

**Return Type:** bool

**Examples:**
```cel
proto.hasExt(msg, google.expr.proto2.test.int32_ext)     // returns true || false
proto.hasExt(msg, google.expr.proto2.test.string_ext)    // returns true || false
proto.hasExt(msg, google.expr.proto2.test.nested_ext)    // returns true || false
```

**Behavior:**
- Returns `true` if the extension field is explicitly set
- Returns `false` if the extension field is not set
- Does not trigger default value computation
- Only works with proto2 syntax messages that support extensions

## 4. Usage Examples

### Basic Extension Field Access
```cel
// Access extension field with default fallback
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

### Conditional Processing Based on Extensions
```cel
// Process message differently based on extension presence
cel.bind(msg, request.message,
  proto.hasExt(msg, com.example.metadata_ext) ?
    processWithMetadata(msg, proto.getExt(msg, com.example.metadata_ext)) :
    processDefault(msg)
)
```

### Extension Field Validation
```cel
// Validate extension field values
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

### Multiple Extension Fields
```cel
// Work with multiple extension fields
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

### Extension-based Message Classification
```cel
// Classify messages based on extension fields
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

### Nested Extension Access
```cel
// Access nested extension fields
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

### Extension Field Aggregation
```cel
// Aggregate data from multiple messages with extensions
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

### Extension-based Routing Logic
```cel
// Route messages based on extension fields
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

**Important Notes:**
- Extension fields are only available in proto2 syntax messages
- The extension field names must be fully qualified (e.g., `com.example.my_extension`)
- Extension fields must be properly defined in the proto definition and registered
- The macros generate efficient select expressions that are optimized at compile time
- Safe-traversal semantics apply: accessing non-existent extensions returns default values rather than errors

The Protocol Buffers extension provides powerful capabilities for working with proto2 extension fields in CEL expressions, enabling flexible message processing and validation patterns. 