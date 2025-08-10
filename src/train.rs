use std::error::Error;
use plotters::prelude::*;

use crate::env_conversion::get_env;
use crate::env_conversion::set_env;
use crate::estimate_price::estimate_price;
use crate::file_io::update_file_min_max;

#[derive(serde::Deserialize)]
struct Row {
    milage: f32,
    price: f32,
}

pub fn create_vector() -> Result<Vec<(f32, f32)>, Box<dyn Error>> {
    let mut min_milage = f32::MAX;
    let mut max_milage = f32::MIN;
    let mut min_price = f32::MAX;
    let mut max_price = f32::MIN;
    let mut raw_data: Vec<(f32, f32)> = vec![];
    let file_path: String = "data/data.csv".to_string();
    let mut rdr = csv::Reader::from_path(file_path)?;
    for record in rdr.records() {
        let line = record?;
        let row: Row = line.deserialize(None)?;
        if row.milage < min_milage {
            min_milage = row.milage;
        }
        if row.milage > max_milage {
            max_milage = row.milage;
        }
        if row.price < min_price {
            min_price = row.price;
        }
        if row.price > max_price {
            max_price = row.price;
        }
        raw_data.push((row.milage, row.price));
    }
    let mut normalised_vector: Vec<(f32, f32)> = vec![];
    for (milage, price) in raw_data {
        let norm_milage = if max_milage > min_milage {
            (milage - min_milage) / (max_milage - min_milage)
        } else {
            0.0
        };
        let norm_price = if max_price > min_price {
            (price - min_price) / (max_price - min_price)
        } else {
            0.0
        };
        normalised_vector.push((norm_milage, norm_price));
    }
    let _ = update_file_min_max(min_milage, max_milage, min_price, max_price);
    Ok(normalised_vector)
}

fn train(vector: &Vec<(f32, f32)>) -> f32 {
    let learning_rate: f32 = 0.1;
    let mut theta0: f32 = get_env(0);
    let mut theta1: f32 = get_env(1);
    let mut sum_error: f32 = 0.0;
    let mut sum_error_price: f32 = 0.0;
    let mut mse: f32 = 0.0;
    let m: usize = vector.len();
    for i in 0..m {
        let error = estimate_price(vector[i].0) - vector[i].1;
        sum_error += error;
        sum_error_price += error * vector[i].0;
        mse += error * error;
    }
    mse /= m as f32;
    let gradient0: f32 = (1.0 / m as f32) * sum_error;
    let gradient1: f32 = (1.0 / m as f32) * sum_error_price;
    theta0 = theta0 - learning_rate * gradient0;
    theta1 = theta1 - learning_rate * gradient1;
    set_env(0, theta0);
    set_env(1, theta1);
    return mse;
}

fn draw_mse_plot(mse_vec: &Vec<f32>, epochs: u32) {
    // Create a plot using plotters
    let root = BitMapBackend::new("mse_plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Mean Squared Error over Epochs", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..epochs as f32, 0f32..mse_vec.iter().cloned().fold(0./0., f32::max))
        .unwrap();

    chart.configure_mesh().draw().unwrap();

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

pub fn train_for_epochs(epochs: u32) {
    let vector = match create_vector() {
        Ok(v) => v,
        Err(_) => return,
    };

    let mut mse_vec: Vec<f32> = Vec::new();
    for _ in 0..epochs {
        let mse = train(&vector);
        mse_vec.push(mse);
    }
    draw_mse_plot(&mse_vec, epochs);
}
