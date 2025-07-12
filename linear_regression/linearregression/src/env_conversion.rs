use std::env;
use std::process;

pub fn read_env(index: u32) -> f32 {
    let env_var = format!("THETA{}", index);
    let env = match env::var(&env_var) {
        Ok(value) => value,
        Err(e) => 
        {
            println!("couldn't interpret {}: {}", env_var, e);
            process::exit(1);
        },
    };
    
    // println!("Read env theta: {} is {}", env_var, env);
    let result: f32 = env.parse().unwrap();
    // println!("RESULT OF UNWRAP: {}", result);
    return result
}

pub fn set_env(index: u32, value: f32)
{
    let env_var = format!("THETA{}", index);
    println!("updating env theta: {} is {}", env_var, value);
    unsafe {
        env::set_var(env_var, value.to_string());
    }
}

// fn main() {
//     let key = "THETA1";
//     env::set_var(key, "1.1");
//     println!("Tutaj {}", key);
//     env_conversion();
// }