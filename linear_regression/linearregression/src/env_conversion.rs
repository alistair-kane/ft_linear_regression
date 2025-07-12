use std::env;

pub fn env_conversion()
{
    for (n,v) in env::vars() {
        println!("{}: {}", n,v);
    }
}

fn main() {
    let key = "THETA1";
    env::set_var(key, "1.1");
    println!("Tutaj {}", key);
    env_conversion();
}