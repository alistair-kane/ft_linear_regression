#![allow(unused)]

mod estimate_price;
mod train;
mod file_io;
use serde::Deserialize;
use std::{error::Error, io, process};

#[derive(serde::Deserialize)]
struct Row {
    km: u64,
    price: u64,
}

fn create_vector(vector: &mut Vec<(u64, u64)>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = csv::Reader::from_reader(io::stdin());
    for record in file.records() {
        let line = record?;
        let row: Row = line.deserialize(None)?;
        // println!("Line {:?}\t{:?}", row.km, row.price);
        vector.push((row.km, row.price));
    }
    Ok(())
}

fn main() {
    // let mut vector: Vec<(u64, u64)> = vec![];
    // let learning_rate = 0.1;
    // let _ = create_vector(&mut vector);
    // for row in vector {
    //     println!("Vec Line {:?}\t{:?}", row.0, row.1);
    //     train::train(learning_rate, row.0, row.1);
    // }
    file_io::read_or_create_file();
}
