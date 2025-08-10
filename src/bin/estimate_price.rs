use std::env;
use std::process;

use linearregression::env_conversion;
use linearregression::estimate_price;
use linearregression::file_io;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: {} <mileage>", args[0]);
        println!("Example: {} 300000", args[0]);
        process::exit(1);
    }

    let mileage: f32 = match args[1].parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Error: Mileage must be a valid number");
            process::exit(1);
        }
    };

    // Read thetas from file
    let thetas: file_io::Thetas = match file_io::read_or_create_file() {
        Ok(val) => val,
        Err(e) => {
            println!("Error reading thetas file: {}", e);
            process::exit(1);
        }
    };

    // Set environment variables for the estimate_price function
    env_conversion::set_env(0, thetas.theta0);
    env_conversion::set_env(1, thetas.theta1);

    // Read min/max values for normalization
    let (min_mileage, max_mileage, min_price, max_price) = match file_io::read_file_min_max() {
        Ok(values) => values,
        Err(e) => {
            println!("Error reading data file: {}", e);
            process::exit(1);
        }
    };

    // Normalize the input mileage
    let norm_mileage: f32 = (mileage - min_mileage) / (max_mileage - min_mileage);
    
    // Estimate price using normalized values
    let norm_price = estimate_price::estimate_price(norm_mileage);
    
    // Denormalize the result
    let real_price = norm_price * (max_price - min_price) + min_price;

    println!("Estimated price for {} mileage: {:.2}", mileage, real_price);
}
