use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn create_file(file_path: String) {
	if std::path::Path::new(&file_path).exists() {
		println!("In file {file_path}");
		let contents = fs::read_to_string(&file_path)
			.expect("Should have been able to read the file");
		
		println!("With text:\n{contents}");
	}	
	println!("Writing File {file_path}");
	
	fs::File::create("test_file.txt").unwrap();
	let mut file = OpenOptions::new()
		.create_new(true)
		.write(true)
		.append(true)
		.open(file)
		.unwrap();

	if let Err(e) = writeln!(file, "{}", "HMMMMMM") {
		eprintln!("Couldn't write to file: {}", e);
	}
	
}

pub fn read_or_create_file() {
	let file_path: String = "saved_theta.csv".to_string();
	create_file(file_path)
}
