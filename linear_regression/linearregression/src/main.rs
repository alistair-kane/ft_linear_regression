use std::{error::Error, io, process};

fn example() -> Result<(), Box<dyn Error>> {
    let mut file = csv::Reader::from_reader(io::stdin());
    for record in file.records() {
        let line = record?;
        println!("Line {:?}", line);
    }
    Ok(())
}

fn estimatePrice(mileage: f32, theta0: f32, theta1: f32) -> f32 {
    return theta0 + (theta1 * mileage)
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
