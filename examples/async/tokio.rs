//! Run synchronous and asynchronous CEL callbacks with tokio.
//!
//! Demonstrates member functions, lazy variable providers, owned captures,
//! concurrent reuse of a compiled program, and propagation of callback errors.
//!
//! Run with: `cargo run --example tokio --features tokio`

use cel_cxx::*;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    member_functions().await?;
    functions_and_providers().await?;
    concurrent_requests().await?;
    Ok(())
}

#[derive(Opaque, Debug, Clone, PartialEq)]
#[cel_cxx(type = "example.Student")]
#[cel_cxx(display = write!(fmt, "Student {{ name: {}, age: {} }}", self.name, self.age))]
struct Student {
    name: String,
    age: i32,
}

impl Student {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    async fn get_age(&self) -> i32 {
        // Stand in for asynchronous I/O without a long demonstration delay.
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        self.age
    }
}

async fn member_functions() -> Result<(), Error> {
    println!("Member functions: synchronous name and asynchronous age");
    let env = Env::builder()
        .declare_variable::<Student>("student")?
        .register_member_function("get_name", Student::get_name)?
        .register_member_function("get_age", Student::get_age)?
        .use_tokio()
        .build()?;

    let activation = Activation::new().bind_variable(
        "student",
        Student {
            name: "John".to_string(),
            age: 18,
        },
    )?;

    // The asynchronous environment also accepts synchronous callbacks.
    let program = env.compile("student.get_name()")?;
    let result = program.evaluate(&activation).await?;
    assert_eq!(result, Value::String("John".into()));
    println!("student.get_name() => {result}");

    let program = env.compile("student.get_age()")?;
    let result = program.evaluate(&activation).await?;
    assert_eq!(result, Value::Int(18));
    println!("student.get_age() => {result}");

    Ok(())
}

async fn functions_and_providers() -> Result<(), Error> {
    println!("Asynchronous global function and lazy variable provider");
    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .register_global_function("get_const", || async {
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            1i64
        })?
        .use_tokio()
        .build()?;
    let program = env.compile("a + b + get_const()")?;

    // Providers run only when their variable is needed during evaluation.
    let activation = Activation::new()
        .bind_variable("a", 1i64)?
        .bind_variable_provider("b", || async { 1i64 })?;
    let result = program.evaluate(&activation).await?;
    assert_eq!(result, Value::Int(3));
    println!("a + b + get_const() => {result}");
    Ok(())
}

async fn concurrent_requests() -> Result<(), Error> {
    println!("Concurrent requests with an owned lookup table");
    let scores = Arc::new(HashMap::from([
        ("Alice".to_owned(), 42i64),
        ("Bob".to_owned(), 17i64),
    ]));
    let env = Env::builder()
        .declare_variable::<String>("name")?
        // Own the captured state. Clone the Arc for each returned future so
        // callbacks can run concurrently without borrowing a local variable.
        .register_global_function("lookup_score", move |name: String| {
            let scores = Arc::clone(&scores);
            async move {
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                scores
                    .get(&name)
                    .copied()
                    .ok_or_else(|| Error::not_found(format!("no score registered for {name}")))
            }
        })?
        .use_tokio()
        .build()?;
    let program = env.compile("lookup_score(name)")?;
    let alice = Activation::new().bind_variable("name", "Alice")?;
    let bob = Activation::new().bind_variable("name", "Bob")?;

    // Separate activations supply request data to the same compiled program.
    let (alice_score, bob_score) =
        futures::future::try_join(program.evaluate(&alice), program.evaluate(&bob)).await?;
    assert_eq!(alice_score, Value::Int(42));
    assert_eq!(bob_score, Value::Int(17));
    assert_eq!(program.evaluate(&alice).await?, Value::Int(42));
    println!("Alice: {alice_score}; Bob: {bob_score}");

    let missing = Activation::new().bind_variable("name", "Unknown")?;
    let error = program.evaluate(&missing).await.unwrap_err();
    assert_eq!(error.code(), Code::NotFound);
    assert!(error.message().contains("no score registered for Unknown"));
    println!("Callback error preserves its code: {error}");
    Ok(())
}
