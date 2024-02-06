use colored::Colorize;
use integer_sqrt::IntegerSquareRoot;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::io::BufRead;

static INPUT_PATH: &str = "./rsrc/input.txt";

lazy_static! {
	static ref TIME_REG: Regex = Regex::new(r"^(?i)time\s*:(?P<nums>(?:\s*\d+)*)\s*$").unwrap();
	static ref DIST_REG: Regex = Regex::new(r"^(?i)distance\s*:(?P<nums>(?:\s*\d+)*)\s*$").unwrap();
}
static START_SPEED: i64 = 0; // mm/ms
static ACCEL: i64 = 1; // ms/ms^2

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");

	let mut times: Vec<i64> = Vec::new();
	let mut distances: Vec<i64> = Vec::new();
	let mut full_time: i64 = 0;
	let mut full_distance: i64 = 0;

	// loop sur les lignes
	let reader = io::BufReader::new(f);
	for line in reader.lines() {
		let line = line.unwrap_or(String::from(""));
		if let Some(cap) = TIME_REG.captures(&line) {
			times.extend(
				cap.name("nums")
					.unwrap()
					.as_str()
					.split_whitespace()
					.map(str::parse::<i64>)
					.map(Result::unwrap),
			);
			full_time = cap
				.name("nums")
				.unwrap()
				.as_str()
				.split_whitespace()
				.collect::<Vec<&str>>()
				.join("")
				.parse()
				.unwrap();
		} else if let Some(cap) = DIST_REG.captures(&line) {
			distances.extend(
				cap.name("nums")
					.unwrap()
					.as_str()
					.split_whitespace()
					.map(str::parse::<i64>)
					.map(Result::unwrap),
			);
			full_distance = cap
				.name("nums")
				.unwrap()
				.as_str()
				.split_whitespace()
				.collect::<Vec<&str>>()
				.join("")
				.parse()
				.unwrap();
		}
	}

	println!(
		"{}: {}",
		"Times".bold(),
		times
			.iter()
			.map(i64::to_string)
			.collect::<Vec<String>>()
			.join(" ")
	);
	println!(
		"{}: {}",
		"Distances".bold(),
		distances
			.iter()
			.map(i64::to_string)
			.collect::<Vec<String>>()
			.join(" ")
	);

	let results: Vec<Option<(i64, i64)>> = times
		.iter()
		.zip(distances.iter())
		/* .map(|td| {
			if let Some((mn, mx)) = find(*td.0, *td.1) {
				mx - mn - 1
			} else {
				0
			}
		}) */
		.map(|td| find(*td.0, *td.1))
		.collect();

	let mut total = 1;
	for (i, ((mt, trgt), res)) in times
		.iter()
		.zip(distances.iter())
		.zip(results.iter())
		.enumerate()
	{
		let delta = match res {
			Some((mn, mx)) => mx - mn + 1,
			None => 0,
		};
		print!("{} {} {}[{}]: ", "Course".bold(), i + 1, mt, trgt);
		if let Some((mn, mx)) = res {
			print!("{}..={} ", mn, mx);
		} else {
			print!("{} ", "None".red());
		}
		total *= delta;
		println!("({}) ({})", delta, total);
	}
	println!("{}: {}", "TOTAL".bold(), total);

	println!();

	println!("{}", "FULL".bold());
	println!("\t{}: {}", "time".bold(), full_time);
	println!("\t{}: {}", "target".bold(), full_distance);
	let res = find(full_time, full_distance);
	if let Some((mn, mx)) = res {
		let delta = mx - mn + 1;
		println!("\t{}: {}..={} ({})", "result".bold(), mn, mx, delta);
	} else {
		println!("\t{}: {}", "result".bold(), "None".red());
	}
}

fn dist_travelled(max_time: i64, time_accel: i64) -> i64 {
	let dt = max_time - time_accel;
	if dt > 0 {
		dt * (time_accel * ACCEL + START_SPEED)
	} else {
		0
	}
}

fn find(max_time: i64, target: i64) -> Option<(i64, i64)> {
	let a = -ACCEL;
	//println!("a = {}", a);
	let b = (max_time * ACCEL) - START_SPEED;
	//println!("b = {}", b);
	let c = (max_time * START_SPEED) - target;
	//println!("c = {}", c);
	let disc = b * b - 4 * a * c;
	//println!("d = {}", disc);
	if disc < 0 {
		None
	} else {
		let discsqrt = disc.integer_sqrt();
		//println!("√d = {}", discsqrt);
		let r1 = (-b - discsqrt) / (2 * a);
		let r2 = (-b + discsqrt) / (2 * a);
		let (mut r1, mut r2) = (min(r1, r2), max(r1, r2));
		if dist_travelled(max_time, r1) < target {
			r1 += 1;
		}
		if dist_travelled(max_time, r2) < target {
			r2 -= 1;
		}
		//println!("r1 = {}", r1);
		//println!("r2 = {}", r2);
		Some((r1, r2))
	}
}
