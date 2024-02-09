use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::fs;
use std::io;
use std::io::BufRead;

use colored::Colorize;

static INPUT_PATH: &str = "./rsrc/input.txt";

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");
	let mut input_iter = InputIterator::new(f);
	let mut sum_nx = 0;
	let mut sum_prev = 0;
	for (i, input) in input_iter.enumerate() {
		let res = double_extrapolate(input.as_ref());
		println!("{i}: {input} => {}", match res {
			Some((prev, nx)) => {
				sum_nx += nx;
				sum_prev += prev;
				format!("({}; {})", prev, nx).green()
			},
			None => "None".red()
		});
	}
	println!("RESULT (Previous): {}", sum_prev.to_string().green());
	println!("RESULT (Next): {}", sum_nx.to_string().green());

	/* let table = [0, 3, 6, 9, 12, 15, 18];
	let mut traces: Vec<String> = Vec::new();
	let res = extrapolate_wth_trace(&table, &mut traces).expect("sequence invalide");
	for (i, s) in traces.iter().enumerate() {
		println!("{}{s}", " ".repeat(i));
	}
	println!("{}", res.to_string().green()); */

}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Input(Box<[i32]>);
impl Input {
	fn new() -> Input {
		Input(Vec::new().into_boxed_slice())
	}
}
impl std::fmt::Display for Input {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}]", self.0.iter().map(i32::to_string).collect::<Vec<String>>().join(", "))
	}
}
impl AsRef<[i32]> for Input {
	fn as_ref(&self) -> &[i32] {
		&self.0
	}
}
impl AsMut<[i32]> for Input {
	fn as_mut(&mut self) -> &mut [i32] {
		&mut self.0
	}
}
impl Borrow<[i32]> for Input {
	fn borrow(&self) -> &[i32] {
		&self.0
	}
}
impl BorrowMut<[i32]> for Input {
	fn borrow_mut(&mut self) -> &mut [i32] {
		&mut self.0
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
			let b = ret.is_none();
			if b {
				println!("{}", format!("Ligne invalide! \"{}\"", ln).red());
			}
			b
		} {}
		ret
	}
}

fn parse_line(ln: &str) -> Option<Input> {
	let mut v: Vec<i32> = Vec::new();
	for n in ln.split_whitespace() {
		match n.parse() {
			Ok(n) => v.push(n),
			Err(_) => return None,
		}
	}
	Some(Input(v.into_boxed_slice()))
}

fn extrapolate(ls: &[i32]) -> Option<i32> {
	if ls.len() <= 1 {
		return None;
	}
	//let mut trace = ls.iter().map(i32::to_string).collect::<Vec<String>>();
	let mut ls2: Vec<i32> = Vec::with_capacity(ls.len()-1);
	let mut all_zeros = true;
	for i in 0..ls.len()-1 {
		let v = ls[i+1] - ls[i];
		ls2.push(v);
		all_zeros = all_zeros && v==0;
		//println!("Testing {}-{}={v} ({} : {all_zeros})", ls[i+1], ls[i], v==0);
	}
	let ls2_nx = if all_zeros {
		0
	} else {
		match extrapolate(&ls2) {
			Some(n) => n,
			None => return None,
		}
	};
	let ls_nx = ls.last().unwrap() + ls2_nx;
	//let mut nv = ls2.iter().map(i32::to_string).collect::<Vec<String>>();
	//trace.push((res+ls.last().unwrap()).to_string());
	//traces.insert(0, trace.join(" "));
	Some(ls_nx)
}
fn extrapolate_wth_trace(ls: &[i32], traces: &mut Vec<String>) -> Option<i32> {
	if ls.len() <= 1 {
		return None;
	}
	let mut trace = ls.iter().map(i32::to_string).collect::<Vec<String>>();
	let mut ls2: Vec<i32> = Vec::with_capacity(ls.len()-1);
	let mut all_zeros = true;
	for i in 0..ls.len()-1 {
		let v = ls[i+1] - ls[i];
		ls2.push(v);
		all_zeros = all_zeros && v==0;
	}
	let ls2_nx = if all_zeros {
		traces.push(vec![0; ls.len()].iter().map(i32::to_string).collect::<Vec<String>>().join(", "));
		0
	} else {
		match extrapolate_wth_trace(&ls2, traces) {
			Some(n) => n,
			None => return None,
		}
	};
	let ls_nx = ls.last().unwrap() + ls2_nx;
	trace.push(ls_nx.to_string());
	traces.insert(0, trace.join(", "));
	Some(ls_nx)
}

fn double_extrapolate(ls: &[i32]) -> Option<(i32, i32)> {
	if ls.len() <= 1 {
		return None;
	}
	//let mut trace = ls.iter().map(i32::to_string).collect::<Vec<String>>();
	let mut ls2: Vec<i32> = Vec::with_capacity(ls.len()-1);
	let mut all_zeros = true;
	for i in 0..ls.len()-1 {
		let v = ls[i+1] - ls[i];
		ls2.push(v);
		all_zeros = all_zeros && v==0;
		//println!("Testing {}-{}={v} ({} : {all_zeros})", ls[i+1], ls[i], v==0);
	}
	let (ls2_prev, ls2_nx) = if all_zeros {
		(0, 0)
	} else {
		match double_extrapolate(&ls2) {
			Some(n) => n,
			None => return None,
		}
	};
	let (ls_prev, ls_nx) = (ls.first().unwrap() - ls2_prev, ls.last().unwrap() + ls2_nx);
	//let mut nv = ls2.iter().map(i32::to_string).collect::<Vec<String>>();
	//trace.push((res+ls.last().unwrap()).to_string());
	//traces.insert(0, trace.join(" "));
	Some((ls_prev, ls_nx))
}