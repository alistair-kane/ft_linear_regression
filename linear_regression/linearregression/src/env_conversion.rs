use std::env;

pub fn read_env(index: u32) -> Result<(f32), Box<dyn std::error::Error>>{
    let env_var = format!("THETA{}", index);
    let env = match env::var(&env_var) {
        Ok(value) => value,
        Err(e) => 
        {
            println!("couldn't interpret {}: {}", env_var, e);
            return panic!("Problem opening the index: {index:?}")
        },
    };
    println!("Hell yeah your read env is {}", env);
    return Ok(env.parse().unwrap());
}

// pub fn set_env(key: String, value: f32)
// {
//     env::set_var(key, value.to_string());
// }


// pub fn env_conversion()
// {
//     for (n,v) in env::vars() {
//         println!("{}: {}", n,v);
//     }
// }

// fn main() {
//     let key = "THETA1";
//     env::set_var(key, "1.1");
//     println!("Tutaj {}", key);
//     env_conversion();
// }