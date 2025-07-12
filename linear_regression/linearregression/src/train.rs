use env;
use crate::env_conversion::read_env;
use crate::env_conversion::set_env;
use crate::estimate_price::estimate_price;

pub fn train(vector: &Vec<(f32, f32)>, m: i32, learning_rate: f32) -> (f32, f32){

    // let theta0: f32 = read_env(0);
    // let theta1: f32 = read_env(1);
    let mut sum_error: f32 = 0.0;
    let mut sum_error_price: f32 = 0.0;
    for i in 0..m as usize
    {
		let error = estimate_price(vector[i].0) - vector[i].1;
        sum_error += error;
        sum_error_price += error * vector[i].0;
		// println!("price estimate {}", estimate_price(vector[i].1));
    }
	

    // let float_m: f32 = m as f32;
    // let mut tmp_theta0 = learning_rate * (1.0 / float_m) * sum_error;
    // let mut tmp_theta1 = learning_rate * (1.0 / float_m) * sum_error_price;


    // set_env(0, tmp_theta0);
    // set_env(1, tmp_theta1);
	return (sum_error, sum_error_price)
} 
