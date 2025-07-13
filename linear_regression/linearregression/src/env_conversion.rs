use std::env;
use std::process;

pub fn get_env(index: u32) -> f32 {
    let env_var = format!("THETA{}", index);
    let env = match env::var(&env_var) {
        Ok(value) => value,
        Err(e) => {
            println!("couldn't interpret {}: {}", env_var, e);
            process::exit(1);
        }
    };
    let result: f32 = env.parse().unwrap();
    return result;
}

pub fn set_env(index: u32, value: f32) {
    let env_var = format!("THETA{}", index);
    println!("Updating {} to {}", env_var, value);
    unsafe {
        env::set_var(env_var, value.to_string());
    }
}
