use lazy_static::lazy_static;
use regex::*;
use std::env; // 1.4.0

/*
    From: https://users.rust-lang.org/t/expand-win-env-var-in-string/50320/3
*/
pub fn expand_env_vars(s: &str) -> std::io::Result<String> {
    lazy_static! {
        static ref ENV_VAR: Regex = Regex::new("\\$").expect("Invalid Regex");
    }

    let result: String = ENV_VAR
        .replace_all(s, |c: &Captures| match &c[1] {
            "" => String::from(""),
            varname => {
                println!("TTT {}", varname);
                env::var(varname).expect("Bad Var Name")
            } // varname => env::var(varname).expect("Bad Var Name"),
        })
        .into();

    Ok(result)
}

fn main() -> Result<(), String> {
    let path = String::from("$HOME/log/test.dat");
    let res = expand_env_vars(path.as_str());
    println!("{:?}", res);
    Ok(())
}
