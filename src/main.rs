use plotters::prelude::*;

mod env_conversion;
mod estimate_price;
mod file_io;
mod train;

fn draw_scatterplot_and_regression(real_mileage: f32, norm_price: f32, min_milage: f32, max_milage: f32, min_price: f32, max_price: f32) {
        // Here
    let norm_vector = match train::create_vector() {
        Ok(v) => v,
        Err(_) => return,
    };

    let mileages: Vec<f32> = norm_vector
        .iter()
        .map(|(m, _)| m * (max_milage - min_milage) + min_milage)
        .collect();
    let prices: Vec<f32> = norm_vector
        .iter()
        .map(|(_, p)| p * (max_price - min_price) + min_price)
        .collect();
    let real_price = norm_price * (max_price - min_price) + min_price;
    println!("Denormalised Price: {}", real_price);
    println!("Estimated price for {} mileage: {}", real_mileage, real_price);
    let final_theta0 = env_conversion::get_env(0);
    let final_theta1 = env_conversion::get_env(1);
    // Generate two points for the regression line
    let line_mileages: Vec<f32> = vec![min_milage, max_milage];
    let line_prices: Vec<f32> = line_mileages
        .iter()
        .map(|m| {
            let norm_m = (m - min_milage) / (max_milage - min_milage);
            let norm_p = final_theta0 + final_theta1 * norm_m;
            norm_p * (max_price - min_price) + min_price
        })
        .collect();

    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Mileage vs Price", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_milage..max_milage, min_price..max_price)
        .unwrap();

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()
        .unwrap();

    chart
        .draw_series(
            mileages
                .iter()
                .zip(prices.iter())
                .map(|(&x, &y)| Circle::new((x, y), 5, ShapeStyle::from(&RED).filled())),
        )
        .unwrap()
        .label("Data Points")
        .legend(|(x, y)| Circle::new((x, y), 5, ShapeStyle::from(&RED).filled()));

    chart
        .draw_series(LineSeries::new(
            line_mileages.iter().zip(line_prices.iter()).map(|(&x, &y)| (x, y)),
            &BLUE,
        ))
        .unwrap()
        .label("Regression Line")
        .legend(|(x, y)| Circle::new((x, y), 5, ShapeStyle::from(&BLUE).filled()));

    chart.configure_series_labels().draw().unwrap();

    // Save the plot as an image file
    let _ = root.present();
}

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
    
    draw_scatterplot_and_regression(
        real_mileage,
        norm_price,
        min_milage,
        max_milage,
        min_price,
        max_price
    );
}
