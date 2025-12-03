mod env_conversion;
mod estimate_price;
mod file_io;
mod train;
mod draw_plots;

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
    train::train_for_epochs(100);
    // update thetas file
    file_io::update_file_thetas();

    // Estimate a price for a given mileage
    let (min_milage, max_milage, min_price, max_price) = match file_io::read_file_min_max() {
        Ok(values) => values,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let real_mileage: f32 = 300000.0;
    let norm_milage: f32 = (real_mileage - min_milage) / (max_milage - min_milage);
    println!("Normalised Mileage: {}", norm_milage);
    let norm_price = estimate_price::estimate_price(norm_milage);
    
    draw_plots::draw_scatterplot_and_regression(
        real_mileage,
        norm_price,
        min_milage,
        max_milage,
        min_price,
        max_price
    );
}
