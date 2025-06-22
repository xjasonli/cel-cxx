use cel_cxx::*;
use std::convert::Infallible;

type Result<T, E = Error> = std::result::Result<T, E>;

#[async_std::main]
async fn main() -> Result<()> {
    exercise1().await?;
    exercise2().await?;
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

impl Student {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    
    async fn get_age(&self) -> i32 {
        self.age
    }
}

async fn exercise1() -> Result<(), Error> {
    println!("exercise7 - testing async method function");
    let env = Env::builder()
        .declare_variable::<Student>("student")?
        // ✨ Register struct methods directly using RustType::method_name syntax
        .register_member_function("get_name", Student::get_name)?
        .register_member_function("get_age", Student::get_age)?
        .use_async_std()
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
async fn exercise2() -> Result<()> {
    println!("exercise8 - testing async global function and async variable provider");
    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .register_global_function("get_const", async move || -> Result<i64, Infallible> {
            Ok(1)
        })?
        .use_async_std()
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
