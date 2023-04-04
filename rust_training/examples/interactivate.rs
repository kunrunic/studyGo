use std::{io::Write, process::exit};

fn input(command: &str) -> String {
    print!("{}", command);
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(n) => {
            return buf.trim().to_string();
        }
        Err(err) => {
            println!("err = {}", err);
            exit(1);
        }
    }
}

fn main() {
    println!("input : [{}]", input("test input >> "));
}
