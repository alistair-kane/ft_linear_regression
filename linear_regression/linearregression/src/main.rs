#![allow(unused)]

mod estimate_price;
mod train;
mod file_io;
use serde::Deserialize;
use std::{error::Error, io, process};
mod env_conversion;

#[derive(serde::Deserialize)]
struct Row {
    km: f32,
    price: f32,
}

fn create_vector(vector: &mut Vec<(f32, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: String = "data/data.csv".to_string();
    let mut rdr = csv::Reader::from_path(file_path)?;
    for record in rdr.records() {		
        let line = record?;
        let row: Row = line.deserialize(None)?;
        // println!("Line {:?}\t{:?}", row.km, row.price);
        vector.push((row.km, row.price));
    }
    Ok(())
}

fn main() {
    let thetas : file_io::Thetas = match file_io::read_or_create_file(){
        Ok(val) => val,
        Err(err) => {
            println!("couldn't interpret {}:", err);
            return panic!("Problem with thetas?")
        },
    };
    env_conversion::set_env(0, thetas.theta0);
    env_conversion::set_env(1, thetas.theta1);

    let mut vector: Vec<(f32, f32)> = vec![];
    let _ = create_vector(&mut vector);
    let m = vector.len();
    let float_m: f32 = m as f32;
    let learning_rate = 0.000001;
    for row in &vector {
        // println!("Vec Line {:?}\t{:?}", row.0, row.1);
        let result = train::train(&vector, m as i32, learning_rate);

        let mut tmp_theta0 = learning_rate * (1.0 / float_m) * result.0;
        let mut tmp_theta1 = learning_rate * (1.0 / float_m) * result.1;
        env_conversion::set_env(0, tmp_theta0);
        env_conversion::set_env(1, tmp_theta1);
    }

}
