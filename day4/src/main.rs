use colored::ColoredString;
use colored::Colorize;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::ops::AddAssign;

static INPUT_PATH: &str = "./rsrc/input.txt";

static LINE_REG: &str =
	r"(?i)Card\s+(?P<id>\d+)\s*:\s*(?P<win>\d+(?:\s+\d+)*)\s*\|\s*(?P<hand>\d+(?:\s+\d+)*)";

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");

	let line_reg = regex::Regex::new(LINE_REG).unwrap();

	// loop sur les lignes
	let reader = io::BufReader::new(f);
	let mut point_sum = 0;
	let mut card_registry: HashMap<usize, u32> = HashMap::new();
	let mut first = usize::MAX;
	let mut last = usize::MIN;
	for line in reader.lines() {
		let line = line.expect("Pas du UTF-8 valide...");

		let cap = line_reg
			.captures(&line)
			.expect("Pas le bon format de ligne");
		let id: usize = cap.name("id").unwrap().as_str().parse().unwrap();
		if id < first {
			first = id;
		}
		if id > last {
			last = id;
		}
		let win: HashSet<u32> = cap
			.name("win")
			.unwrap()
			.as_str()
			.split_whitespace()
			.map(|s| s.parse().expect("wtf"))
			.collect();
		let hand: HashSet<u32> = cap
			.name("hand")
			.unwrap()
			.as_str()
			.split_whitespace()
			.map(|s| s.parse().expect("wtf"))
			.collect();
		let points = get_points_for(&win, &hand);
		point_sum += points;
		let hs = hand
			.iter()
			.map(|n| {
				let s = n.to_string();
				if win.contains(n) {
					s.green().to_string()
				} else {
					s.red().to_string()
				}
			})
			.collect::<Vec<String>>()
			.join(" ");
		let ws = win
			.iter()
			.map(u32::to_string)
			.collect::<Vec<String>>()
			.join(" ");
		println!(
			"Game {}:\n\tHand: {}\n\tWin: {}\n\tPoints: {}",
			id, hs, ws, points
		);
		let mtch = hand.intersection(&win).count() as u32;
		card_registry.insert(id, mtch);
	}
	println!("Total: {}", point_sum);

	let mut card_result: HashMap<usize, u32> = card_registry.keys().map(|k| (*k, 1)).collect();
	let mut candidats: Vec<usize> = card_registry.keys().map(usize::to_owned).collect();
	let mut card_sum = 0;
	candidats.sort_unstable();
	println!("CARD COUNT: ");
	for c in &candidats {
		let mul = *card_result.get(c).unwrap();
		let nxts = get_next_cards(&candidats, *c, *card_registry.get(c).unwrap() as usize);
		println!("\t{} -> {} {:?}", c, mul, nxts);
		for nxt in nxts {
			if let Some(n) = card_result.get_mut(nxt) {
				n.add_assign(mul)
			}
		}
		card_sum += mul;
	}
	println!("TOTAL: {}", card_sum);
}

fn get_points_for(win: &HashSet<u32>, hand: &HashSet<u32>) -> u32 {
	let mut point = 0;
	for n in hand {
		if win.contains(n) {
			point = if point == 0 { 1 } else { point * 2 };
		}
	}
	point
}

fn get_next_cards(reg: &[usize], from: usize, n: usize) -> &[usize] {
	let idx = reg.binary_search(&from);
	if let Ok(start) = idx {
		//let start = max(0, idx);
		let start = start + 1;
		let end = min(reg.len(), start + n);
		&reg[start..end]
	} else {
		&reg[0..0]
	}
}
