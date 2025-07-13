use std::error::Error;

use crate::env_conversion::read_env;
use crate::env_conversion::set_env;
use crate::estimate_price::estimate_price;
use crate::file_io::update_file;

#[derive(serde::Deserialize)]
struct Row {
    milage: f32,
    price: f32,
}

fn create_vector() -> Result<Vec<(f32, f32)>, Box<dyn Error>> {
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
    Ok(normalised_vector)
}

fn train(vector: &Vec<(f32, f32)>) {
    let learning_rate: f32 = 0.1;
    let mut theta0: f32 = read_env(0);
    let mut theta1: f32 = read_env(1);
    let mut sum_error: f32 = 0.0;
    let mut sum_error_price: f32 = 0.0;
    let m: usize = vector.len();
    for i in 0..m {
        let error = estimate_price(vector[i].0) - vector[i].1;
        sum_error += error;
        sum_error_price += error * vector[i].0;
    }
    let gradient0 = (1.0 / m as f32) * sum_error;
    let gradient1 = (1.0 / m as f32) * sum_error_price;
    theta0 = theta0 - learning_rate * gradient0;
    theta1 = theta1 - learning_rate * gradient1;
    set_env(0, theta0);
    set_env(1, theta1);
}

pub fn train_for_epochs(epochs: u32) {
    let vector = match create_vector() {
        Ok(v) => v,
        Err(_) => return,
    };
    for _ in 0..epochs {
        train(&vector);
        // update thetas file
        update_file();
    }
}
