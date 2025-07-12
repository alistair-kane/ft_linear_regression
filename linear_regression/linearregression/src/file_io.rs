use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use serde::Deserialize;
use std::{error::Error, io, process};

#[derive(serde::Deserialize)]
pub struct Thetas {
    pub theta0: f32,
    pub theta1: f32,
}

pub fn read_or_create_file() -> Result<(Thetas), Box<dyn Error>> {
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
			// println!("IN FILE:{}", thetas.theta0);
			// println!("IN FILE:{}", thetas.theta1);
			break;
		}
	}
	else {
		println!("Writing File {file_path}");
		let mut file = OpenOptions::new()
			.create_new(true)
			.write(true)
			.append(true)
			.open(file_path)
			.unwrap();
		if let Err(e) = writeln!(file, "{}", "theta0,theta1\n0.0,0.0") {
			eprintln!("Couldn't write to file: {}", e);
		}
	}
	Ok(thetas)
}


