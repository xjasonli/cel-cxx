#![allow(unused)]

use cel_cxx::*;
use std::convert::Infallible;

type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    exercise1()?;
    exercise2()?;
    exercise3()?;
    exercise4()?;
    exercise5()?;
    exercise6()?;
    exercise9()?;

    Ok(())
}


fn exercise1() -> Result<()> {
    println!("exercise1 - testing simple program");
    let env = Env::builder().build()?;
    let program = env.compile("1+1")?;
    let result = program.evaluate(Activation::new())?;
    println!("exercise1, result: {}", result);

    Ok(())
}

// variable binding
fn exercise2() -> Result<()> {
    println!("exercise2 - testing variable binding");
    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;
    let program = env.compile("a + b")?;

    let activation = Activation::new()
        .bind_variable("a", 1)?
        .bind_variable("b", 1)?;
    let result = program.evaluate(activation)?;
    println!("exercise2, result: {}", result);
    Ok(())
}

// variable provider binding
fn exercise3() -> Result<()> {
    println!("exercise3 - testing variable provider binding");
    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;
    let program = env.compile("a + b")?;


    let activation = Activation::new()
        .bind_variable("a", 1)?
        .bind_variable_provider("b", || -> Result<i64, Error> {
            Ok(1)
        })?;
    let result = program.evaluate(activation)?;
    println!("exercise3, result: {}", result);
    Ok(())
}

// declare global function
fn exercise4() -> Result<()> {
    println!("exercise4 - testing global function");
    let env = Env::builder()
        .declare_global_function::<fn() -> i64>("get_const")
        .unwrap()
        .build()
        .unwrap();
    let program = env.compile("get_const()")?;

    let activation = Activation::new()
        .bind_global_function("get_const", || -> Result<i64, Error> {
            Ok(1)
        })?;
    let result = program.evaluate(activation)?;
    println!("exercise4, result: {}", result);
    
    Ok(())
}

// register global function
fn exercise5() -> Result<()> {
    println!("exercise5 - testing string function");
    let env = Env::builder()
        .register_global_function("get_const", || -> Result<i64, Error> {
            Ok(1)
        })?
        .build()?;
    let program = env.compile("get_const()")?;

    let activation = Activation::new();
    let result = program.evaluate(activation)?;
    println!("exercise5, result: {}", result);
    
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

// register sync method function
fn exercise6() -> Result<(), Error> {
    println!("exercise6 - testing method function");
    let env = Env::builder()
        .declare_variable::<Student>("student")?
        .register_member_function("get_name", |student: Student| -> Result<String, Infallible> {
            Ok(student.name.clone())
        })?
        .register_member_function("get_age", |student: Student| -> Result<i32, Infallible> {
            Ok(student.age)
        })?
        .build()?;

    let activation = Activation::new()
        .bind_variable("student", Student { name: "John".to_string(), age: 18 })?;

    let program = env.compile("student.get_name()")?;
    let result = program.evaluate(&activation)?;
    println!("exercise6, get_name result: {}", result);

    let program = env.compile("student.get_age()")?;
    let result = program.evaluate(&activation)?;
    println!("exercise6, get_age result: {}", result);

    Ok(())
}


fn exercise9() -> Result<()> {
    println!("exercise9 - testing program return type");
    let env = Env::builder()
        .declare_variable::<i64>("a")?
        .declare_variable::<i64>("b")?
        .build()?;
    let program = env.compile("a + b")?;

    let typ = program.return_type();
    println!("exercise9, program return type: {}", typ);

    let env = Env::builder()
        .declare_variable::<String>("a")?
        .declare_variable::<String>("b")?
        .build()?;
    let program = env.compile("a + b")?;

    let typ = program.return_type();
    println!("exercise9, program return type: {}", typ);
    let activation = Activation::new()
        .bind_variable("a", "1".to_string())?
        .bind_variable("b", "2".to_string())?;
    let result = program.evaluate(&activation)?;
    println!("exercise9, program result: {}", result);

    Ok(())
}
