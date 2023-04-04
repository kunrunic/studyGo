/*
https://users.rust-lang.org/t/how-to-get-function-name/68993/8
fn learn_rust() {
    println!("I am learning rust...");
}
fn get_function_name<F>(_: F) -> &'static str
where
    F: Fn(),
{
    std::any::type_name::<F>()
}
println!("{}", get_function_name(learn_rust));
*/
fn get_fname<F>(f: F) -> Result<(), String>
where
    F: Fn() -> Result<(), String>,
{
    println!("{}", std::any::type_name::<F>());
    f()
}

fn split_case() -> Result<(), String> {
    let s = "test;test1;test2;test3";

    let sp: Vec<&str> = s.split(";").collect();
    sp.iter().for_each(|v| println!("{}", v));

    Ok(())
}

fn str_match_case() -> Result<(), String> {
    let m = match "http" {
        "https" => 1,
        "http2" => 2,
        "http" => 3,
        &_ => -1,
    };
    println!("{}", m);
    Ok(())
}

fn test_replace() {
    let mut a = String::from("IMSI-450090000001234");
    let mut b = String::from("45012341234");
    println!("a : {}", a);
    a = a.to_lowercase().replace("imsi-", "");
    println!("a : {} (found replace case)", a);
    println!("b : {}", b);
    b = b.replace("imsi-", "");
    println!("b : {} (not found replace case)", b);
}

fn test_eq() {
    let a = String::from("None");
    if a.eq(&"None") {
        println!("Eq True")
    } else {
        println!("Eq False")
    }
}

fn test_loop() {
    let mut c: usize = 100;

    for _ in 0..100 {
        c = c + 1;
        println!("{}", c);
    }
}

fn main() -> Result<(), String> {
    let _r = get_fname(split_case);
    let _r = get_fname(str_match_case);
    test_replace();
    test_eq();
    test_loop();

    Ok(())
}
