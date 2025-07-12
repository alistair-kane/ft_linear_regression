mod env_conversion;

pub fn estimate_price(mileage: f32) { 
    let theta0 = env_conversion::read_env(0);
    let theta1 = env_conversion::read_env(1);
    
    // return match theta0 + (theta1 * mileage){
    //     Ok(val) => Ok(val),
    //     Err(e) => panic!("Problem opening the file: {error:?}");
    // };
}
