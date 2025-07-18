# Sets Extension

The sets extension provides set operations on lists, treating them as mathematical sets for comparison and analysis.

## Table of Contents

- [Sets Extension](#sets-extension)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Set Containment - `contains()`](#2-set-containment---contains)
    - [contains()](#contains)
  - [3. Set Equivalence - `equivalent()`](#3-set-equivalence---equivalent)
    - [equivalent()](#equivalent)
  - [4. Set Intersection - `intersects()`](#4-set-intersection---intersects)
    - [intersects()](#intersects)
  - [5. Usage Examples](#5-usage-examples)
    - [Permission Checking](#permission-checking)
    - [Data Validation](#data-validation)
    - [Feature Compatibility](#feature-compatibility)
    - [Tag Matching](#tag-matching)
    - [Role-Based Access Control](#role-based-access-control)
    - [Configuration Validation](#configuration-validation)
    - [Membership Testing](#membership-testing)

## 1. Overview

The sets extension allows you to perform mathematical set operations on CEL lists. Lists are treated as sets, meaning duplicate elements are ignored for comparison purposes.

**Enabling the Extension**:
```rust
let env = Env::builder()
    .with_ext_sets(true)
    .build()?;
```

## 2. Set Containment - `contains()`

### contains()

Checks if one set contains all elements of another set.

**Syntax:** `list1.contains(list2)`

**Parameters:**
- `list1`: The container set (list)
- `list2`: The subset to check (list)

**Return Type:** bool

**Examples:**
```cel
[1, 2, 3, 4].contains([2, 3])         // true
[1, 2, 3].contains([3, 4])            // false
["a", "b", "c"].contains(["a", "c"])  // true
[1, 2, 3].contains([])                // true (empty set is subset of any set)
[].contains([1])                      // false
[1, 2, 2, 3].contains([2, 3])         // true (duplicates ignored)
```

**Behavior:**
- Returns `true` if all elements of `list2` are present in `list1`
- Duplicates are ignored (treats lists as sets)
- Empty list is considered a subset of any list
- Order doesn't matter

## 3. Set Equivalence - `equivalent()`

### equivalent()

Checks if two sets contain exactly the same elements.

**Syntax:** `list1.equivalent(list2)`

**Parameters:**
- `list1`: First set (list)
- `list2`: Second set (list)

**Return Type:** bool

**Examples:**
```cel
[1, 2, 3].equivalent([3, 2, 1])       // true
[1, 2, 2, 3].equivalent([1, 2, 3])    // true (duplicates ignored)
[1, 2, 3].equivalent([1, 2, 4])       // false
["a", "b"].equivalent(["b", "a"])     // true
[].equivalent([])                     // true
[1].equivalent([1, 1, 1])             // true
```

**Behavior:**
- Returns `true` if both sets contain exactly the same unique elements
- Order doesn't matter
- Duplicates are ignored
- Two empty lists are equivalent

## 4. Set Intersection - `intersects()`

### intersects()

Checks if two sets have any elements in common.

**Syntax:** `list1.intersects(list2)`

**Parameters:**
- `list1`: First set (list)
- `list2`: Second set (list)

**Return Type:** bool

**Examples:**
```cel
[1, 2, 3].intersects([3, 4, 5])       // true (common element: 3)
[1, 2, 3].intersects([4, 5, 6])       // false (no common elements)
["a", "b"].intersects(["b", "c"])     // true (common element: "b")
[].intersects([1, 2, 3])              // false (empty set intersects with nothing)
[1, 2].intersects([])                 // false
[1, 1, 2].intersects([2, 2, 3])       // true (common element: 2)
```

**Behavior:**
- Returns `true` if at least one element exists in both sets
- Duplicates are ignored
- Empty sets don't intersect with anything
- Order doesn't matter

## 5. Usage Examples

### Permission Checking
```cel
// Check if user has required permissions
cel.bind(user_permissions, ["read", "write", "delete"],
  cel.bind(required_permissions, ["read", "write"],
    user_permissions.contains(required_permissions)
  )
)
// Result: true
```

### Data Validation
```cel
// Validate that submitted data contains only allowed fields
cel.bind(submitted_fields, ["name", "email", "age"],
  cel.bind(allowed_fields, ["name", "email", "age", "phone"],
    allowed_fields.contains(submitted_fields)
  )
)
// Result: true
```

### Feature Compatibility
```cel
// Check if client supports all required features
cel.bind(client_features, ["websockets", "push", "offline", "sync"],
  cel.bind(required_features, ["websockets", "push"],
    cel.bind(optional_features, ["offline", "camera"],
      {
        "has_required": client_features.contains(required_features),
        "has_optional": client_features.intersects(optional_features),
        "fully_compatible": client_features.equivalent(required_features.concat(optional_features))
      }
    )
  )
)
// Result: {"has_required": true, "has_optional": true, "fully_compatible": false}
```

### Tag Matching
```cel
// Match articles by tags
cel.bind(article_tags, ["tech", "ai", "programming"],
  cel.bind(user_interests, ["ai", "machine-learning", "tech"],
    {
      "has_common_interests": article_tags.intersects(user_interests),
      "exact_match": article_tags.equivalent(user_interests),
      "covers_all_interests": article_tags.contains(user_interests)
    }
  )
)
// Result: {"has_common_interests": true, "exact_match": false, "covers_all_interests": false}
```

### Role-Based Access Control
```cel
// Complex RBAC logic
cel.bind(user_roles, ["editor", "reviewer"],
  cel.bind(admin_roles, ["admin", "super-admin"],
    cel.bind(editor_roles, ["editor", "author"],
      cel.bind(required_roles, ["editor"],
        {
          "is_admin": user_roles.intersects(admin_roles),
          "can_edit": user_roles.contains(required_roles),
          "has_editor_access": user_roles.intersects(editor_roles),
          "role_summary": {
            "admin": user_roles.intersects(admin_roles),
            "editor": user_roles.intersects(editor_roles),
            "has_all_required": user_roles.contains(required_roles)
          }
        }
      )
    )
  )
)
```

### Configuration Validation
```cel
// Validate configuration compatibility
cel.bind(current_config, ["feature-a", "feature-b", "feature-c"],
  cel.bind(new_config, ["feature-a", "feature-c", "feature-d"],
    cel.bind(breaking_changes, ["feature-x", "feature-y"],
      {
        "is_compatible": !new_config.intersects(breaking_changes),
        "has_changes": !current_config.equivalent(new_config),
        "maintains_features": new_config.contains(current_config.filter(f, f != "deprecated")),
        "change_summary": {
          "added": new_config.filter(f, !current_config.contains([f])),
          "removed": current_config.filter(f, !new_config.contains([f])),
          "unchanged": current_config.filter(f, new_config.contains([f]))
        }
      }
    )
  )
)
```

### Membership Testing
```cel
// Test various membership scenarios
cel.bind(group_a, ["alice", "bob", "charlie"],
  cel.bind(group_b, ["bob", "charlie", "david"],
    cel.bind(group_c, ["alice", "bob"],
      {
        "a_contains_c": group_a.contains(group_c),        // true
        "b_contains_c": group_b.contains(group_c),        // false (missing alice)
        "a_intersects_b": group_a.intersects(group_b),    // true (bob, charlie)
        "c_equivalent_subset": group_c.equivalent(["bob", "alice"]), // true
        "all_groups_intersect": group_a.intersects(group_b) && 
                               group_b.intersects(group_c) && 
                               group_a.intersects(group_c)   // true
      }
    )
  )
)
```

The sets extension provides essential set-theoretic operations that are particularly useful for permission systems, data validation, and logical comparisons where the order of elements doesn't matter. 