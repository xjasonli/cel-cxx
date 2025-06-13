use cel_cxx::*;
use std::convert::Infallible;

type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    exercise7().await?;
    exercise8().await?;
    Ok(())
}

#[derive(Opaque, Debug, Clone, PartialEq)]
#[cel_cxx(type = "helloworld.Student")]
struct Student {
    name: String,
    age: i32,
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Student {{ name: {}, age: {} }}", self.name, self.age)
    }
}

async fn exercise7() -> Result<(), Error> {
    println!("exercise7 - testing async method function");
    let env = Env::builder()
        .declare_variable::<Student>("student")?
        .register_member_function("get_name", |student: Student| -> Result<String, Error> {
            Ok(student.name.clone())
        })?
        .register_member_function("get_age", async |student: Student| -> Result<i32, Error> {
            Ok(student.age)
        })?
        .use_tokio()
        .build()?;

    let activation = Activation::new()
        .bind_variable("student", Student { name: "John".to_string(), age: 18 })?;

    let program = env.compile("student.get_name()")?;
    let result = program.evaluate(&activation).await?;
    println!("exercise7, get_name result: {}", result);

    let program = env.compile("student.get_age()")?;
    let result = program.evaluate(&activation).await?;
    println!("exercise7, get_age result: {}", result);

    Ok(())
}

// register async global function and async variable provider
async fn exercise8() -> Result<()> {
    println!("exercise8 - testing async global function and async variable provider");
    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .register_global_function("get_const", async move || -> Result<i64, Infallible> {
            Ok(1)
        })?
        .use_tokio()
        .build()?;
    let program = env.compile("a + b + get_const()")?;


    let activation = Activation::new()
        .bind_variable("a", 1)?
        .bind_variable_provider("b", async move || -> Result<i64, Infallible> {
            Ok(1)
        })?;
    let result = program.evaluate(&activation).await?;
    println!("exercise8, result: {}", result);
    Ok(())
}
