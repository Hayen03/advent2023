use regex::Match;
use regex::Regex;
use std::fs;
use std::io;
use std::io::BufRead;

static INPUT_PATH: &str = "./rsrc/input.txt";

static MATCH_PATTERN: &str =
	"(?i)0|zero|1|one|2|two|3|three|4|four|5|five|6|six|7|seven|8|eight|9|nine";
fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");
	let reader = io::BufReader::new(f);
	let reg = Regex::new(MATCH_PATTERN).unwrap();
	let res: i32 = reader
		.lines()
		.map(|l| read_input_line(l.expect("ligne invalide").as_str(), &reg).unwrap())
		.sum();
	println!("{}", res);
}

/*fn read_input_line(line: &str) -> Result<i32, &'static str> {
	let line = line.trim();
	let matches: Vec<&str> = line.matches(char::is_numeric).collect();
	if matches.is_empty() {
		Err("Aucun chiffre dans la ligne")
	} else {
		let first: i32 = matches.first().unwrap().parse().unwrap();
		let second: i32 = matches.last().unwrap().parse().unwrap();
		let res = first * 10 + second;
		println!("\"{}\" -> {}", line, res);
		Ok(res)
	}
}*/

fn read_input_line(line: &str, reg: &Regex) -> Result<i32, &'static str> {
	//let mut matches = reg.find_iter(line);
	let (first, last) = find_matches(line, reg);
	#[allow(unused_assignments)]
	let mut firsts = "";
	#[allow(unused_assignments)]
	let mut lasts = "";
	let first: i32 = match first {
		None => return Err("Aucun chiffre dans la ligne"),
		Some(m) => {
			firsts = m.as_str();
			lasts = firsts;
			match parse_written_digit(firsts) {
				Ok(n) => n,
				Err(e) => return Err(e),
			}
		}
	};
	let last: i32 = match last {
		None => first,
		Some(m) => {
			lasts = m.as_str();
			match parse_written_digit(lasts) {
				Ok(n) => n,
				Err(e) => return Err(e),
			}
		}
	};
	let res = first * 10 + last;
	println!("\"{}\" -> ({}, {}) {}", line, firsts, lasts, res);
	Ok(res)
}

fn parse_written_digit(src: &str) -> Result<i32, &'static str> {
	match src.trim().to_lowercase().as_str() {
		"zero" => Ok(0),
		"one" => Ok(1),
		"two" => Ok(2),
		"three" => Ok(3),
		"four" => Ok(4),
		"five" => Ok(5),
		"six" => Ok(6),
		"seven" => Ok(7),
		"eight" => Ok(8),
		"nine" => Ok(9),
		s => {
			if let Ok(t) = s.parse() {
				Ok(t)
			} else {
				Err("N'a pas pu convertir en nombre...")
			}
		}
	}
}

fn find_matches<'a>(haystack: &'a str, reg: &Regex) -> (Option<Match<'a>>, Option<Match<'a>>) {
	let first = reg.find(haystack);
	let last = first.map(|f| {
		let mut prev = f;
		let mut at = prev.start() + 1;
		loop {
			let m = reg.find_at(haystack, at);
			if let Some(m) = m {
				prev = m;
				at = prev.start() + 1;
			} else {
				return prev;
			}
		}
	});
	(first, last)
}
