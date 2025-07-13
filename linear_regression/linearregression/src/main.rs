mod env_conversion;
mod estimate_price;
mod file_io;
mod train;

fn main() {
    let thetas: file_io::Thetas = match file_io::read_or_create_file() {
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    env_conversion::set_env(0, thetas.theta0);
    env_conversion::set_env(1, thetas.theta1);
    train::train_for_epochs(24);
    let mileage: f32 = 300000.0;
    let price = estimate_price::estimate_price(mileage);
    println!("Estimated price for {} mileage: {}", mileage, price);
}
