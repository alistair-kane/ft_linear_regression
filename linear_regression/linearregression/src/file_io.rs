use std::fs::OpenOptions;
use std::io::prelude::*;
use std::{error::Error, process};

use crate::env_conversion::get_env;

#[derive(serde::Deserialize)]
pub struct Thetas {
    pub theta0: f32,
    pub theta1: f32,
}

fn write_to_file(thetas: &Thetas) -> Result<(), Box<dyn Error>> {
    let file_path: String = "data/saved_theta.csv".to_string();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)?;
    if let Err(e) = writeln!(file, "theta0,theta1\n{},{}", thetas.theta0, thetas.theta1) {
        eprintln!("Couldn't write to file: {}", e);
    }
    Ok(())
}

pub fn read_or_create_file() -> Result<Thetas, Box<dyn Error>> {
    let file_path: String = "data/saved_theta.csv".to_string();
    let mut thetas = Thetas {
        theta0: 0.0,
        theta1: 0.0,
    };
    if std::path::Path::new(&file_path).exists() {
        println!("In file {file_path}");
        let mut rdr = csv::Reader::from_path(file_path)?;
        for record in rdr.records() {
            let line = record?;
            thetas = line.deserialize(None)?;
            println!("Read from file: Theta0: {}, Theta1: {}", thetas.theta0, thetas.theta1);
            break;
        }
    } else {
        println!("Writing File {file_path}");
        write_to_file(&thetas)?;
    }
    Ok(thetas)
}

pub fn update_file_thetas() {
    let theta0: f32 = get_env(0);
    let theta1: f32 = get_env(1);
    let thetas = Thetas { theta0, theta1 };
    if let Err(e) = write_to_file(&thetas) {
        eprintln!("Couldn't write to file: {}", e);
        process::exit(1);
    }
}

pub fn update_file_min_max(min_milage: f32, max_milage: f32, min_price: f32, max_price: f32) -> Result<(), Box<dyn Error>> {
    let file_path: String = "data/saved_min_max.csv".to_string();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)?;
    if let Err(e) = writeln!(file, "min_milage,max_milage,min_price,max_price\n{},{},{},{}", min_milage, max_milage, min_price, max_price) {
        eprintln!("Couldn't write to file: {}", e);
        return Err(Box::new(e));
    }
    Ok(())
}

pub fn read_file_min_max() -> Result<(f32, f32, f32, f32), Box<dyn Error>> {
    let file_path: String = "data/saved_min_max.csv".to_string();
    if !std::path::Path::new(&file_path).exists() {
        return Err("Min/Max file does not exist".into());
    }
    let mut rdr = csv::Reader::from_path(file_path)?;
    for record in rdr.records() {
        let line = record?;
        let (min_milage, max_milage, min_price, max_price): (f32, f32, f32, f32) = line.deserialize(None)?;
        return Ok((min_milage, max_milage, min_price, max_price));
    }
    Err("No records found in min/max file".into())
}