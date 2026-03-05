//! Advanced protobuf example: nested messages, repeated fields, maps, WKTs, and more
//!
//! This example builds on the basic protobuf example to demonstrate:
//! - Deeply nested messages (Order -> Person -> Address)
//! - Repeated message fields (Order.items)
//! - Map fields (Order.labels)
//! - Enums (Person.status)
//! - Well-Known Types: Duration, Timestamp, wrappers (Int64Value, StringValue, BoolValue)
//! - `google.protobuf.Struct` as a dynamic map
//! - Field presence checks with `has()`
//! - Type checks with `type()`
//! - CEL comprehensions (`.filter()`, `.map()`, `.exists()`, `.all()`)
//! - Message construction with `Type{field: value}` syntax
//! - Rust-side field extraction from nested results
//!
//! Run with: `cargo run --example protobuf-advanced`

use cel_cxx::*;

fn compile_descriptors() -> Vec<u8> {
    use prost::Message;
    protox::compile(["tests/fixtures/test.proto"], ["tests/fixtures/"])
        .expect("failed to compile test.proto")
        .encode_to_vec()
}

// =============================================================================
// Protobuf encoding helpers (using prost's low-level encoding)
//
// In a real application you would use generated prost structs or protobuf
// message types.  Here we encode by hand so the example has no codegen step.
// =============================================================================

mod proto {
    use prost::encoding::*;

    // -- primitive field encoders -------------------------------------------

    pub fn string(buf: &mut Vec<u8>, field: u32, v: &str) {
        if !v.is_empty() {
            encode_key(field, WireType::LengthDelimited, buf);
            encode_varint(v.len() as u64, buf);
            buf.extend_from_slice(v.as_bytes());
        }
    }

    pub fn int32(buf: &mut Vec<u8>, field: u32, v: i32) {
        if v != 0 {
            encode_key(field, WireType::Varint, buf);
            encode_varint(v as i64 as u64, buf);
        }
    }

    pub fn int64(buf: &mut Vec<u8>, field: u32, v: i64) {
        if v != 0 {
            encode_key(field, WireType::Varint, buf);
            #[allow(clippy::cast_sign_loss)]
            encode_varint(v as u64, buf);
        }
    }

    pub fn bool_(buf: &mut Vec<u8>, field: u32, v: bool) {
        if v {
            encode_key(field, WireType::Varint, buf);
            encode_varint(1, buf);
        }
    }

    pub fn nested(buf: &mut Vec<u8>, field: u32, msg: &[u8]) {
        encode_key(field, WireType::LengthDelimited, buf);
        encode_varint(msg.len() as u64, buf);
        buf.extend_from_slice(msg);
    }

    pub fn repeated_nested(buf: &mut Vec<u8>, field: u32, items: &[Vec<u8>]) {
        for item in items {
            nested(buf, field, item);
        }
    }

    pub fn map_string_string(buf: &mut Vec<u8>, field: u32, entries: &[(&str, &str)]) {
        for (k, v) in entries {
            let mut entry = Vec::new();
            string(&mut entry, 1, k);
            string(&mut entry, 2, v);
            nested(buf, field, &entry);
        }
    }

    // -- message encoders ---------------------------------------------------

    /// test.Address { street, city, zip }
    pub fn address(street: &str, city: &str, zip: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        string(&mut buf, 1, street);
        string(&mut buf, 2, city);
        string(&mut buf, 3, zip);
        buf
    }

    /// test.Person { name, age, address, status, tags, active }
    pub fn person(
        name: &str,
        age: i32,
        addr: Option<&[u8]>,
        status: i32,
        tags: &[&str],
        active: bool,
    ) -> Vec<u8> {
        let mut buf = Vec::new();
        string(&mut buf, 1, name);
        int32(&mut buf, 2, age);
        if let Some(a) = addr {
            nested(&mut buf, 3, a);
        }
        int32(&mut buf, 4, status);
        for t in tags {
            string(&mut buf, 5, t);
        }
        bool_(&mut buf, 6, active);
        buf
    }

    /// test.Item { name, price }
    pub fn item(name: &str, price: i32) -> Vec<u8> {
        let mut buf = Vec::new();
        string(&mut buf, 1, name);
        int32(&mut buf, 2, price);
        buf
    }

    /// test.Order { buyer, items, labels }
    pub fn order(buyer: &[u8], items: &[Vec<u8>], labels: &[(&str, &str)]) -> Vec<u8> {
        let mut buf = Vec::new();
        nested(&mut buf, 1, buyer);
        repeated_nested(&mut buf, 2, items);
        map_string_string(&mut buf, 3, labels);
        buf
    }

    // -- WKT encoders -------------------------------------------------------

    /// google.protobuf.Duration { seconds, nanos }
    pub fn duration(seconds: i64, nanos: i32) -> Vec<u8> {
        let mut buf = Vec::new();
        int64(&mut buf, 1, seconds);
        int32(&mut buf, 2, nanos);
        buf
    }

    /// google.protobuf.Timestamp { seconds, nanos }
    pub fn timestamp(seconds: i64, nanos: i32) -> Vec<u8> {
        let mut buf = Vec::new();
        int64(&mut buf, 1, seconds);
        int32(&mut buf, 2, nanos);
        buf
    }

    /// google.protobuf.Int64Value { value }
    pub fn int64_value(v: i64) -> Vec<u8> {
        let mut buf = Vec::new();
        int64(&mut buf, 1, v);
        buf
    }

    /// google.protobuf.StringValue { value }
    pub fn string_value(v: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        string(&mut buf, 1, v);
        buf
    }

    /// google.protobuf.BoolValue { value }
    pub fn bool_value(v: bool) -> Vec<u8> {
        let mut buf = Vec::new();
        bool_(&mut buf, 1, v);
        buf
    }

    /// google.protobuf.Struct with string-typed values
    pub fn struct_with_strings(entries: &[(&str, &str)]) -> Vec<u8> {
        let mut buf = Vec::new();
        for (key, val) in entries {
            // Value message: field 3 = string_value
            let mut value_msg = Vec::new();
            string(&mut value_msg, 3, val);
            // MapEntry: { 1: key, 2: Value }
            let mut entry = Vec::new();
            string(&mut entry, 1, key);
            nested(&mut entry, 2, &value_msg);
            // Struct.fields is field 1 (map<string, Value>)
            nested(&mut buf, 1, &entry);
        }
        buf
    }

    /// test.Event { name, created_at, ttl, optional_count, optional_label,
    ///              optional_flag, metadata, payload }
    pub fn event(
        name: &str,
        created_at: Option<&[u8]>,
        ttl: Option<&[u8]>,
        optional_count: Option<&[u8]>,
        optional_label: Option<&[u8]>,
        optional_flag: Option<&[u8]>,
        metadata: Option<&[u8]>,
        payload: Option<&[u8]>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();
        string(&mut buf, 1, name);
        if let Some(v) = created_at {
            nested(&mut buf, 2, v);
        }
        if let Some(v) = ttl {
            nested(&mut buf, 3, v);
        }
        if let Some(v) = optional_count {
            nested(&mut buf, 4, v);
        }
        if let Some(v) = optional_label {
            nested(&mut buf, 5, v);
        }
        if let Some(v) = optional_flag {
            nested(&mut buf, 6, v);
        }
        if let Some(v) = metadata {
            nested(&mut buf, 7, v);
        }
        if let Some(v) = payload {
            nested(&mut buf, 8, v);
        }
        buf
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Compile, evaluate, and print.  Returns the result for further inspection.
fn eval(env: &Env, activation: &Activation, expr: &str) -> Result<Value, Error> {
    let program = env.compile(expr)?;
    let result = program.evaluate(activation)?;
    println!("  {expr}\n    => {result}");
    Ok(result)
}

// =============================================================================
// Main
// =============================================================================

fn main() -> Result<(), Error> {
    // =========================================================================
    // 1. Nested messages, enums, repeated fields, maps
    // =========================================================================
    println!("=== 1. Complex message: Order with nested Person, Items, and labels ===\n");

    let env = Env::builder()
        .with_file_descriptor_set(&compile_descriptors())
        .declare_variable_with_type("order", ValueType::Struct(StructType::new("test.Order")))?
        // CEL has no built-in reduce/fold, so register helpers for aggregation
        .register_global_function("sum_ints", |v: Vec<i64>| -> i64 { v.iter().sum() })?
        .build()?;

    // Build a realistic Order
    let addr = proto::address("123 Main St", "Springfield", "62704");
    let buyer = proto::person(
        "Alice",
        30,
        Some(&addr),
        1, // STATUS_ACTIVE
        &["vip", "early-adopter"],
        true,
    );
    let items = vec![
        proto::item("Widget", 1299),    // $12.99 in cents
        proto::item("Gadget", 4599),     // $45.99
        proto::item("Doohickey", 299),   // $2.99
        proto::item("Thingamajig", 999), // $9.99
    ];
    let labels = [("priority", "high"), ("region", "us-west"), ("channel", "web")];

    let order_bytes = proto::order(&buyer, &items, &labels);
    let activation = Activation::new()
        .bind_variable_dynamic("order", StructValue::from_bytes("test.Order", order_bytes))?;

    // Deep field access
    eval(&env, &activation, "order.buyer.name")?;
    eval(&env, &activation, "order.buyer.address.city")?;
    eval(&env, &activation, "order.buyer.address.zip")?;
    eval(&env, &activation, "order.buyer.active")?;

    // Enum
    eval(&env, &activation, "order.buyer.status == 1")?;
    eval(
        &env,
        &activation,
        "order.buyer.status == test.Status.STATUS_ACTIVE",
    )?;

    // Repeated string field
    eval(&env, &activation, "order.buyer.tags")?;
    eval(&env, &activation, "'vip' in order.buyer.tags")?;

    // Repeated message field
    eval(&env, &activation, "order.items.size()")?;
    eval(&env, &activation, "order.items[0].name")?;
    eval(&env, &activation, "order.items[1].price")?;

    // Map field
    eval(&env, &activation, "order.labels")?;
    eval(&env, &activation, "order.labels['priority']")?;
    eval(&env, &activation, "'region' in order.labels")?;

    // =========================================================================
    // 2. CEL comprehensions on repeated message fields
    // =========================================================================
    println!("\n=== 2. Comprehensions: filter, map, exists, all ===\n");

    // Filter items by price
    eval(
        &env,
        &activation,
        "order.items.filter(i, i.price > 1000)",
    )?;
    eval(
        &env,
        &activation,
        "order.items.filter(i, i.price > 1000).size()",
    )?;

    // Map: extract names of expensive items
    eval(
        &env,
        &activation,
        "order.items.filter(i, i.price >= 4000).map(i, i.name)",
    )?;

    // Exists: is there an item over $40?
    eval(
        &env,
        &activation,
        "order.items.exists(i, i.price > 4000)",
    )?;

    // All: are all items under $100?
    eval(
        &env,
        &activation,
        "order.items.all(i, i.price < 10000)",
    )?;

    // CEL has no built-in reduce/fold — use a custom Rust function to aggregate
    eval(
        &env,
        &activation,
        "sum_ints(order.items.map(i, i.price))",
    )?;

    // =========================================================================
    // 3. has() — field presence checks
    // =========================================================================
    println!("\n=== 3. has() — field presence checks ===\n");

    eval(&env, &activation, "has(order.buyer)")?;
    eval(&env, &activation, "has(order.buyer.address)")?;
    eval(&env, &activation, "has(order.buyer.address.city)")?;

    // Build a minimal order with no address
    let buyer_no_addr = proto::person("Bob", 25, None, 0, &[], false);
    let minimal_order = proto::order(&buyer_no_addr, &[], &[]);
    let activation2 = Activation::new()
        .bind_variable_dynamic("order", StructValue::from_bytes("test.Order", minimal_order))?;

    eval(&env, &activation2, "has(order.buyer.address)")?;
    eval(&env, &activation2, "has(order.buyer.address) ? order.buyer.address.city : 'N/A'")?;

    // =========================================================================
    // 4. type() — runtime type inspection
    // =========================================================================
    println!("\n=== 4. type() — runtime type inspection ===\n");

    eval(&env, &activation, "type(order)")?;
    eval(&env, &activation, "type(order.buyer)")?;
    eval(&env, &activation, "type(order.buyer.name)")?;
    eval(&env, &activation, "type(order.items)")?;
    eval(&env, &activation, "type(order.buyer.active)")?;

    // =========================================================================
    // 5. Well-Known Types: Duration, Timestamp, wrappers, Struct
    // =========================================================================
    println!("\n=== 5. Well-Known Types ===\n");

    let env_event = Env::builder()
        .with_file_descriptor_set(&compile_descriptors())
        .declare_variable_with_type("event", ValueType::Struct(StructType::new("test.Event")))?
        .build()?;

    let event_bytes = proto::event(
        "deploy-v2.3",
        Some(&proto::timestamp(1700000000, 0)), // 2023-11-14T22:13:20Z
        Some(&proto::duration(3600, 0)),         // 1 hour TTL
        Some(&proto::int64_value(42)),           // optional_count = 42
        Some(&proto::string_value("production")),// optional_label = "production"
        Some(&proto::bool_value(true)),          // optional_flag = true
        Some(&proto::struct_with_strings(&[     // metadata
            ("env", "prod"),
            ("version", "2.3.0"),
            ("deployer", "ci-bot"),
        ])),
        None, // no Any payload
    );

    let act_event = Activation::new()
        .bind_variable_dynamic("event", StructValue::from_bytes("test.Event", event_bytes))?;

    // Timestamp — auto-converts to CEL timestamp type
    eval(&env_event, &act_event, "event.name")?;
    eval(&env_event, &act_event, "event.created_at")?;
    eval(
        &env_event,
        &act_event,
        "event.created_at > timestamp('2023-01-01T00:00:00Z')",
    )?;
    eval(
        &env_event,
        &act_event,
        "event.created_at.getFullYear()",
    )?;

    // Duration — auto-converts to CEL duration type
    eval(&env_event, &act_event, "event.ttl")?;
    eval(
        &env_event,
        &act_event,
        "event.ttl > duration('30m')",
    )?;
    eval(
        &env_event,
        &act_event,
        "event.ttl == duration('1h')",
    )?;

    // Wrapper types — auto-unbox to CEL primitives
    eval(&env_event, &act_event, "event.optional_count")?;
    eval(&env_event, &act_event, "event.optional_count > 10")?;
    eval(&env_event, &act_event, "event.optional_label")?;
    eval(
        &env_event,
        &act_event,
        "event.optional_label == 'production'",
    )?;
    eval(&env_event, &act_event, "event.optional_flag")?;

    // Struct — behaves as a dynamic map
    eval(&env_event, &act_event, "event.metadata.env")?;
    eval(&env_event, &act_event, "event.metadata.version")?;
    eval(&env_event, &act_event, "has(event.metadata.deployer)")?;

    // has() on wrapper fields — true when set
    eval(&env_event, &act_event, "has(event.optional_count)")?;

    // Event without optional fields
    let sparse_event = proto::event("bare-event", None, None, None, None, None, None, None);
    let act_sparse = Activation::new()
        .bind_variable_dynamic("event", StructValue::from_bytes("test.Event", sparse_event))?;
    eval(&env_event, &act_sparse, "has(event.optional_count)")?;
    eval(&env_event, &act_sparse, "has(event.ttl)")?;

    // =========================================================================
    // 6. Message construction in CEL
    // =========================================================================
    println!("\n=== 6. Message construction in CEL ===\n");

    let env_construct = Env::builder()
        .with_file_descriptor_set(&compile_descriptors())
        .build()?;
    let empty = Activation::new();

    // Simple message
    eval(
        &env_construct,
        &empty,
        "test.SimpleMessage{name: 'Eve', id: 7}",
    )?;

    // Nested: Person with inline Address
    eval(
        &env_construct,
        &empty,
        "test.Person{name: 'Frank', age: 40, address: test.Address{street: '456 Oak Ave', city: 'Portland', zip: '97201'}, active: true}",
    )?;

    // Order with repeated items
    eval(
        &env_construct,
        &empty,
        "test.Order{buyer: test.Person{name: 'Grace', age: 28}, items: [test.Item{name: 'Book', price: 1599}, test.Item{name: 'Pen', price: 199}]}",
    )?;

    // Inline field access on a constructed message
    eval(
        &env_construct,
        &empty,
        "test.Order{buyer: test.Person{name: 'Grace'}, items: [test.Item{name: 'Book', price: 1599}]}.items[0].name",
    )?;

    // =========================================================================
    // 7. Rust-side field extraction from complex results
    // =========================================================================
    println!("\n=== 7. Rust-side field extraction ===\n");

    // Evaluate an expression that returns a message
    let program = env.compile("order.buyer")?;
    let buyer_result = program.evaluate(&activation)?;
    let buyer_sv = StructValue::from_value(&buyer_result)?;
    println!("  Extracted buyer: type={}, {} bytes", buyer_sv.type_name(), buyer_sv.to_bytes().len());

    // Read individual fields from the extracted message
    let name = env.get_struct_field(&buyer_sv, "name")?;
    let age = env.get_struct_field(&buyer_sv, "age")?;
    let has_addr = env.has_struct_field(&buyer_sv, "address")?;
    println!("  buyer.name  = {name}");
    println!("  buyer.age   = {age}");
    println!("  has(buyer.address) = {has_addr}");

    // Extract the nested address
    let addr_result = env.get_struct_field(&buyer_sv, "address")?;
    let addr_sv = StructValue::from_value(&addr_result)?;
    let city = env.get_struct_field(&addr_sv, "city")?;
    let zip = env.get_struct_field(&addr_sv, "zip")?;
    println!("  buyer.address.city = {city}");
    println!("  buyer.address.zip  = {zip}");

    // Extract a repeated field via CEL comprehension
    let program = env.compile("order.items.map(i, i.name)")?;
    let names_result = program.evaluate(&activation)?;
    println!("  item names = {names_result}");

    println!("\nDone.");
    Ok(())
}
