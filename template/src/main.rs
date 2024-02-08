use std::fs;
use std::io;
use std::io::BufRead;

static INPUT_PATH: &str = "./rsrc/input.txt";

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");
	let mut input_iter = InputIterator::new(f);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Input {}
impl Input {
	fn new() -> Input {
		todo!()
	}
}
impl std::fmt::Display for Input {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

struct InputIterator<T>
where
	T: io::Read,
{
	reader: io::BufReader<T>,
}
impl<T> InputIterator<T>
where
	T: io::Read,
{
	fn new(input: T) -> Self {
		Self {
			reader: io::BufReader::new(input),
		}
	}
}
impl<T: io::Read> Iterator for InputIterator<T> {
	type Item = Input;

	fn next(&mut self) -> Option<Self::Item> {
		let mut ret = None;
		while {
			let mut ln: String = String::new();
			let read_res = self.reader.read_line(&mut ln);
			if read_res.unwrap_or(0) == 0 {
				return None;
			}
			ret = parse_line(&ln);
			ret.is_none()
		} {}
		ret
	}
}

fn parse_line(ln: &str) -> Option<Input> {
	todo!()
}
