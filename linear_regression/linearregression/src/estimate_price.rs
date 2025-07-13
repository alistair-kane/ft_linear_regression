use crate::env_conversion::get_env;

pub fn estimate_price(mileage: f32) -> f32 {
    // Read theta env variable
    let theta0: f32 = get_env(0);
    let theta1: f32 = get_env(1);
    let result = theta0 + (theta1 * mileage);
    return result;
}
