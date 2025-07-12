use crate::env_conversion::read_env;

pub fn estimate_price(mileage: f32) -> f32 {
	// Read theta env variable
    let theta0: f32 = read_env(0);
    let theta1: f32 = read_env(1);
    // println!("{} : {} ",theta0, theta1);
    let result = theta0 + (theta1 * mileage);
    return result;
}
