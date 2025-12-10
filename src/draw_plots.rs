use plotters::prelude::*;

use crate::env_conversion::get_env;
use crate::train::create_vector;

pub fn draw_scatterplot_and_regression(real_mileage: f32, norm_price: f32, min_milage: f32, max_milage: f32, min_price: f32, max_price: f32) {
    let norm_vector = match create_vector() {
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
    let final_theta0 = get_env(0);
    let final_theta1 = get_env(1);
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
        .caption("Mileage vs Price", ("sans-serif", 40).into_font())
        .margin(30)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_milage..max_milage, min_price..max_price)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Mileage")
        .y_desc("Price")
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
            line_mileages
                .iter()
                .zip(line_prices.iter())
                .map(|(&x, &y)| (x, y)),
                &BLUE,
        ))
        .unwrap()
        .label("Regression Line")
        .legend(|(x, y)| Circle::new((x, y), 5, ShapeStyle::from(&BLUE).filled()));

    chart.configure_series_labels().draw().unwrap();

    // Save the plot as an image file
    let _ = root.present();
}

pub fn draw_mse_plot(mse_vec: &Vec<f32>, epochs: u32) {
    // Create a plot using plotters
    let root = BitMapBackend::new("mse_plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Mean Squared Error over Epochs", ("sans-serif", 40))
        .margin(30)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..epochs as f32, 0f32..mse_vec.iter().cloned().fold(0./0., f32::max))
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Epochs")
        .y_desc("MSE")
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..epochs).map(|x| (x as f32, mse_vec[x as usize])),
            &RED,
        ))
        .unwrap()
        .label("MSE")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).draw().unwrap();
}
