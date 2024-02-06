use std::fs;
use std::io;
use std::io::BufRead;

static INPUT_PATH: &str = "./rsrc/input.txt";

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");

	// loop sur les lignes
	let reader = io::BufReader::new(f);
	for line in reader.lines() {
		let line = line.unwrap_or(String::from(""));
	}
}
