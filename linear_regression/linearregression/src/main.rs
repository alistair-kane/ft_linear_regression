use std::{error::Error, io, process};

fn example() -> Result<(), Box<dyn Error>> {
    let mut file = csv::Reader::from_reader(io::stdin());
    for record in file.records()
    {
        let check = record?;
        println!("Line {:?}", check);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}