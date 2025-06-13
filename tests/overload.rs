use std::collections::HashMap;
use cel_cxx::*;

const MY_NAMESPACE: &str = "testing";

#[test]
fn test_opaque_function_overload() -> Result<(), Error> {
    println!("test opaque function overload");

    #[derive(Opaque)]
    #[derive(Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, std::any::type_name::<Self>()))]
    struct MyOpaqueGeneric<T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static>(T);
    impl<T> std::fmt::Display for MyOpaqueGeneric<T>
    where T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }
    fn sum(a: MyOpaqueGeneric<i64>, b: i64) -> Result<i64,Error> { Ok(a.0 + b) }
    fn sum1(a: MyOpaqueGeneric<u64>, b: i64) -> Result<u64,Error> { Ok(a.0 + b as u64 * 2) }

    let env = Env::builder()
        .register_member_function("sum", sum)?
        .register_member_function("sum1", sum1)?
        .declare_variable::<i64>("a")?
        .declare_variable::<u64>("b")?
        .declare_variable::<MyOpaqueGeneric<u64>>("mu")?
        .declare_variable::<MyOpaqueGeneric<i64>>("mi")?
        .declare_variable::<MyOpaqueGeneric<u32>>("mu32")?
        .declare_variable::<Vec<i64>>("il")?
        .build()?;

    {
        let expr = "il.map(m, mu.sum(m))";
        let program = env.compile(expr)?;

        let mu: MyOpaqueGeneric<u64> = MyOpaqueGeneric(16);
        let il: Vec<i64> = vec![84,85,86,87,88];

        let activation = Activation::new()
            .bind_variable("mu", mu)?
            .bind_variable("il", il)?
            ;

        let res = program.evaluate(&activation)?;
        println!("res: {:?}", res);
    }

    {
        let expr = "il.map(m, mi.sum(m))";
        let program = env.compile(expr)?;
        assert!(true, "mi test pass");
        let mi: MyOpaqueGeneric<i64> = MyOpaqueGeneric(16);
        let il: Vec<i64> = vec![84,85,86,87,88];

        let activation = Activation::new()
            .bind_variable("mi", mi)?
            .bind_variable("il", il)?
            ;

        let res = program.evaluate(&activation)?;
        println!("res: {:?}", res);
    }

    {
        let expr = "il.map(m, mu32.sum(m))";
        let program = env.compile(expr)?;
        let mu32: MyOpaqueGeneric<u32> = MyOpaqueGeneric(16);
        let il: Vec<i64> = vec![84,85,86,87,88];

        let activation = Activation::new()
            .bind_variable("mu32", mu32)?
            .bind_variable("il", il)?
            ;

        let res = program.evaluate(&activation)?;
        println!("res: {:?}", res);
    }

    Ok(())
}

fn test_list_i64_overload(env: &Env) -> Result<(), Error> {
    let expr = "li64.num()";
    let program = env.compile(expr)?;
    assert!(true, "mu test pass");
    let li64: Vec<i64> = vec![84, 85, 86, 87, 88];

    let activation = Activation::new()
        .bind_variable("li64", li64)?
        ;

    let res = program.evaluate(&activation)?;
    println!("li64 num: {:?}", res);
    Ok(())
}

fn test_list_i32_overload(env: &Env) -> Result<(), Error>{
    let expr = "li32.num()";
    let program = env.compile(expr)?;
    assert!(true, "li32 test pass");
    let li32: Vec<i32> = vec![84, 85, 86, 87, 88];

    let activation = Activation::new()
        .bind_variable("li32", li32)?
        ;

    let res = program.evaluate(&activation)?;
    println!("li32 num: {:?}", res);
    Ok(())
}

fn test_list_bool_overload(env: &Env) -> Result<(), Error> {
    let expr = "lb.num()";
    let program = env.compile(expr)?;
    assert!(false, "unpected ok");
    let lb: Vec<bool> = vec![false, true, true, false, true];

    let activation = Activation::new()
        .bind_variable("lb", lb)?
        ;

    let res = program.evaluate(&activation)?;
    println!("lb num: {:?}", res);
    Ok(())
}

fn test_list_string_overload(env: &Env) -> Result<(), Error> {
    let expr = "ls.num()";
    let program = env.compile(expr)?;
    assert!(true, "ok");
    let ls: Vec<String> = vec![];

    let activation = Activation::new()
        .bind_variable("ls", ls)?
        ;

    let res = program.evaluate(&activation)?;
    println!("ls num: {:?}", res);

    Ok(())
}

#[test]
fn test_list_function_overload() -> Result<(), Error> {
    println!("test list function overload");

    fn num(a: Vec<i64>) -> Result<usize, Error> { Ok(a.len() + 1) }
    fn num1(a: Vec<u64>) -> Result<usize, Error> { Ok(a.len() + 2) }
    fn num2(a: Vec<String>) -> Result<usize, Error> { Ok(a.len() + 3) }
    fn num3(a: Vec<Option<i32>>) -> Result<usize, Error> { Ok(a.len() + 4) }

    let env = Env::builder()
        .register_member_function("num", num)?
        .register_member_function("num", num1)?
        .register_member_function("num", num2)?
        .register_member_function("num", num3)?
        .declare_variable::<Vec<i32>>("li32")?
        .declare_variable::<Vec<i64>>("li64")?
        .declare_variable::<Vec<bool>>("lb")?
        .declare_variable::<Vec<String>>("ls")?
        .declare_variable::<Vec<Option<i32>>>("loi32")?
        .build()?;

    test_list_i32_overload(&env)?;
    test_list_i64_overload(&env)?;
    test_list_bool_overload(&env)?;
    test_list_string_overload(&env)?;

    Ok(())
}

fn test_list_option_havenone_overload(env: &Env) -> Result<(), Error> {
    let expr = "loi64.name()";
    let program = env.compile(expr)?;
    assert!(true, "loi64 test pass");
    let loi64: Vec<Option<i64>> = vec![None, None, None, Some(88), None, None];
    // let loi32: Vec<i32> = vec![84, 85, 86, 87, 88];
    let activation = Activation::new()
        .bind_variable("loi64", loi64)?
        ;

    if let Value::String(res) = program.evaluate(&activation)? {
        println!("{} name: {:?}", expr, res);
        assert!(res.contains("list of option i64"))
    } else {
        assert!(false, "unpected error value")
    }

    Ok(())
}

fn test_list_option_overload(env: &Env) -> Result<(), Error> {
    let expr = "loi64.name()";
    let program = env.compile(expr)?;

    let loi64: Vec<Option<i64>> = vec![Some(81), Some(88)];
    // let loi32: Vec<i32> = vec![84, 85, 86, 87, 88];
    let activation = Activation::new()
        .bind_variable("loi64", loi64)?
        ;

    if let Value::String(res) = program.evaluate(&activation)? {
        println!("{} name: {:?}", expr, res);
        assert!(res.contains("list of option i64"))
    }else {
        assert!(false, "unpected error value")
    }

    Ok(())
}

fn test_list_empty_overload(env: &Env) -> Result<(), Error> {
    let expr = "loi64.name()";
    let program = env.compile(expr)?;
    assert!(true, "loi64 test pass");
    let loi64: Vec<Option<i64>> = vec![];
    // let loi32: Vec<i32> = vec![84, 85, 86, 87, 88];
    let activation = Activation::new()
        .bind_variable("loi64", loi64)?
        ;

    if let Value::String(res) = program.evaluate(&activation)? {
        println!("{} name: {:?}", expr, res);
        assert!(res.contains("list of option i64"))
    }else {
        assert!(false, "unpected error value")
    }
    Ok(())
}

#[test]
fn test_list_option_function_overload() -> Result<(), Error> {
    println!("test list function overload");

    #[derive(Opaque, Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, std::any::type_name::<Self>()))]
    struct MyOpaqueGeneric<T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static>(T);

    impl<T> std::fmt::Display for MyOpaqueGeneric<T>
    where
        T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    fn name1(_a: Vec<i64>) -> Result<String, Error> { Ok("list of i64".to_string()) }
    fn name2(_a: Vec<Option<i64>>) -> Result<String, Error> { Ok("list of option i64".to_string()) }
    fn name3(_a: Vec<MyOpaqueGeneric<i64>>) -> Result<String, Error> { Ok("list of MyOpaqueGeneric i64".to_string()) }
    // fn name4(a: Vec<Option<MyOpaqueGeneric<i64>>>) -> Result<String, Error> { Ok("list of option MyOpaqueGeneric i64".to_string()) }

    let env = Env::builder()
        .register_member_function("name", name1)?
        .register_member_function("name", name2)?
        .register_member_function("name", name3)?
        // .declare_variable::<Vec<i32>,_>("li32")?
        // .declare_variable::<Vec<i64>,_>("li64")?
        // .declare_variable::<Vec<u64>,_>("lu64")?
        // .declare_variable::<Vec<bool>,_>("lb")?
        // .declare_variable::<Vec<String>,_>("ls")?
        .declare_variable::<Vec<Option<i64>>>("loi64")?
        .build()?;

    // test_list_i32_overload(&env)?;
    // test_list_i64_overload(&env)?;
    // test_list_bool_overload(&env)?;
    // test_list_string_overload(&env)?;
    for _ in 0..100 {
        test_list_option_havenone_overload(&env)?;
        test_list_option_overload(&env)?;
        test_list_empty_overload(&env)?;
    }
    Ok(())
}

fn test_empty_map_string_overload(env: &Env) -> Result<(), Error> {
    //test empty list
    let expr = "mso.name()";
    let program = env.compile(expr)?;
    //empty map
    let mso: HashMap<String, Option<i64>> = HashMap::new();

    let activation = Activation::new()
        .bind_variable("mso", mso)?
        ;

    if let Value::String(res) = program.evaluate(&activation)? {
        println!("{}: {:?}", expr, res);
        assert!(res.contains("map of str->optioni64"))
    } else {
        assert!(false)
    }

    Ok(())
}

fn test_map_string_overload(env: &Env) -> Result<(), Error> {
    //test empty list
    let expr = "mso.name()";
    let program = env.compile(expr)?;

    let mut mso: HashMap<String, Option<i64>> = HashMap::new();
    mso.insert("key1".to_string(), Some(10)); //None won't infer type
    mso.insert("key2".to_string(), Some(20));
    mso.insert("key3".to_string(), Some(30));

    let activation = Activation::new()
        .bind_variable("mso", mso)?
        ;

    if let Value::String(res) = program.evaluate(&activation)? {
        // println!("{}: {:?}", expr, res);
        assert!(res.contains("map of str->optioni64"))
    } else {
        assert!(false)
    }

    Ok(())
}

fn test_map_option_havenone_overload(env: &Env) -> Result<(), Error> {
    //test empty list
    let expr = "mso.name()";
    let program = env.compile(expr)?;

    assert!(true, "ok");
    let mut mso: HashMap<String, Option<i64>> = HashMap::new();
    mso.insert("key1".to_string(), None); //None won't infer type
    mso.insert("key2".to_string(), None);
    mso.insert("key3".to_string(), Some(30));

    let activation = Activation::new()
        .bind_variable("mso", mso)?
        ;

    if let Value::String(res) = program.evaluate(&activation)? {
        // println!("{}: {:?}", expr, res);
        assert!(res.contains("map of str->optioni64"))
    } else {
        assert!(false)
    }

    Ok(())
}

fn test_map_typeerror_overload(env: &Env) -> Result<(), Error> {

    //test empty list
    let expr = "msi.name()";
    let program = env.compile(expr)?;

    assert!(true, "ok");
    let mut msi: HashMap<String, Option<i64>> = HashMap::new();
    msi.insert("key1".to_string(), Some(10)); //None won't infer type
    msi.insert("key2".to_string(), Some(20));
    msi.insert("key3".to_string(), Some(30));

    let activation = Activation::new()
        .bind_variable("msi", msi)?
        ;

    let _value = program.evaluate(&activation)?;
    Ok(())
}

#[test]
fn test_map_function_overload() -> Result<(), Error> {
    println!("test list function overload");

    #[derive(Opaque, Debug, Clone, PartialEq)]
    #[cel_cxx(type = format!("{}.{}", MY_NAMESPACE, std::any::type_name::<Self>()))]
    struct MyOpaqueGeneric<T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static>(T);

    impl<T> std::fmt::Display for MyOpaqueGeneric<T>
    where
        T: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    fn name1(_: HashMap<String,i64>) -> Result<String, Error> { Ok("map of str->i64".to_string()) }
    fn name2(_: HashMap<String,Option<i64>>) -> Result<String, Error> { Ok("map of str->optioni64".to_string()) }
    fn name3(_: HashMap<String,MyOpaqueGeneric<i64>>) -> Result<String, Error> { Ok("map of str->MyOpaqueGeneric_i64".to_string()) }
    // fn name4(a: Vec<Option<MyOpaqueGeneric<i64>>>) -> Result<String, Error> { Ok("list of option MyOpaqueGeneric i64".to_string()) }
    fn name5(_ :HashMap<String,Option<u32>>) -> Result<String, Error> { Ok("map of str->optionu32".to_string()) }

    let env = Env::builder()
        .register_member_function("name", name1)?
        .register_member_function("name", name2)?
        .register_member_function("name", name3)?
        // .register_member_function("name", name4)
        .register_member_function("name", name5)?
        // .declare_variable::<Vec<i32>,_>("li32")?
        // .declare_variable::<Vec<i64>,_>("li64")?
        // .declare_variable::<Vec<u64>,_>("lu64")?
        // .declare_variable::<Vec<bool>,_>("lb")?
        .declare_variable::<HashMap<String, Option<i64>>>("mso")?
        .declare_variable::<HashMap<String, i64>>("msi")?
        .declare_variable::<Vec<Option<i64>>>("loi64")?
        .build()?;

    for _i in 0..1000 {
        test_map_string_overload(&env)?;
        test_map_option_havenone_overload(&env)?;
        test_empty_map_string_overload(&env)?;
        test_map_typeerror_overload(&env)?;
        // test_map_option_opaque_overload(&env)?;
    }

    Ok(())
}
