use colored::Colorize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::io;
use std::io::BufRead;
use std::ptr;
use std::rc::Rc;

static INPUT_PATH: &str = "./rsrc/input.txt";

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum PartType {
	Part,
	Gear,
}

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");

	let mut parts: HashMap<Coord, PartType> = HashMap::new(); // coordonées des parts
	let mut nums: HashMap<Coord, (u32, Rc<u32>)> = HashMap::new(); // nombres

	// loop sur les lignes
	let reader = io::BufReader::new(f);
	let mut w = 0;
	let mut h = 0;
	#[allow(clippy::explicit_counter_loop)]
	for (line_count, line) in reader.lines().enumerate() {
		h += 1;
		let line = line.expect("Pas du UTF-8 valide...");
		let mut curr_num: Option<(u32, u32, u32)> = None;
		for (char_count, c) in line.chars().enumerate() {
			if char_count > w {
				w = char_count;
			}
			if c == '.' {
				// empty
				push_number(&mut nums, &mut curr_num, line_count as u32);
			} else if c.is_numeric() {
				// num
				// si on a un nombre en ce moment, on l'agrandi
				if let Some((start, _, val)) = curr_num {
					let v: u32 = char::to_digit(c, 10).unwrap();
					curr_num = Some((start, char_count as u32, val * 10 + v));
				} else {
					let v: u32 = char::to_digit(c, 10).unwrap();
					curr_num = Some((char_count as u32, char_count as u32, v));
				}
			} else {
				// part
				// on rajoute la partie à la liste
				parts.insert(
					Coord::new(line_count as u32, char_count as u32),
					if c == '*' {
						PartType::Gear
					} else {
						PartType::Part
					},
				);
				push_number(&mut nums, &mut curr_num, line_count as u32);
			}
		}
		push_number(&mut nums, &mut curr_num, line_count as u32);
	}

	let mut part_nums: HashSet<P> = HashSet::new();
	for (p, _) in &parts {
		part_nums.extend(get_part_nums_for(*p, &nums));
	}

	let gears = get_gears(&parts, &nums);

	//println!("PARTS:");
	//for part in &parts {
	//	println!("\t{}", part);
	//}
	//println!("NUMS:");
	//for (c, n) in &nums {
	//	println!("\t{} -> {} -> {}", c, Rc::as_ptr(n) as u32, *n);
	//}

	print_schematics(w as u32, h, &parts, &nums, &part_nums, &gears);

	let sum: u32 = part_nums.into_iter().map(|n| *n.0).sum();
	println!("Sum: {}", sum);

	let ratio: u32 = gears.into_iter().map(|g| g.1).sum();
	println!("Ratio: {}", ratio);
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Coord {
	x: u32,
	y: u32,
}
impl From<(u32, u32)> for Coord {
	fn from(value: (u32, u32)) -> Self {
		Coord {
			x: value.0,
			y: value.1,
		}
	}
}
impl From<Coord> for (u32, u32) {
	fn from(value: Coord) -> Self {
		(value.x, value.y)
	}
}
impl Coord {
	fn new(x: u32, y: u32) -> Self {
		Coord { x, y }
	}
}
impl std::fmt::Display for Coord {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

fn push_number(
	nums: &mut HashMap<Coord, (u32, Rc<u32>)>,
	num: &mut Option<(u32, u32, u32)>,
	line: u32,
) {
	if let Some((start, end, val)) = *num {
		let n = Rc::new(val);
		let len = end - start + 1;
		for i in 0..len {
			nums.insert(Coord::new(line, start + i), (len - i - 1, n.clone()));
		}
		*num = None;
	}
}

// wrapper pour pouvoir hash avec l'adresse
#[derive(Debug, Eq)]
struct P(Rc<u32>);
impl Hash for P {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		ptr::hash(self.0.as_ref(), state);
	}
}
impl PartialEq for P {
	fn eq(&self, other: &Self) -> bool {
		ptr::eq(Rc::as_ptr(&self.0), Rc::as_ptr(&other.0))
	}
}

fn get_part_nums_for(part: Coord, nums: &HashMap<Coord, (u32, Rc<u32>)>) -> HashSet<P> {
	let mut set: HashSet<P> = HashSet::new();
	for i in 0..9 {
		let y = match i % 3 {
			0 => part.y.checked_sub(1),
			1 => Some(part.y),
			2 => part.y.checked_add(1),
			_ => None,
		};
		let x = match i / 3 {
			0 => part.x.checked_sub(1),
			1 => Some(part.x),
			2 => part.x.checked_add(1),
			_ => None,
		};
		if let (Some(x), Some(y)) = (x, y) {
			if x != part.x || y != part.y {
				let c = Coord::from((x, y));
				let r = nums.get(&c).cloned();
				if let Some((_, r)) = r {
					set.insert(P(r));
				}
			}
		}
	}
	set
}

fn print_schematics(
	width: u32,
	height: u32,
	parts: &HashMap<Coord, PartType>,
	nums: &HashMap<Coord, (u32, Rc<u32>)>,
	part_nums: &HashSet<P>,
	gears: &HashMap<Coord, u32>,
) {
	for x in 0..height {
		for y in 0..width {
			let c = Coord::new(x, y);
			if parts.contains_key(&c) {
				// imprime un symbole
				if gears.contains_key(&c) {
					print!("{}", "*".bright_blue());
				} else {
					print!("{}", "#".yellow());
				}
			} else if let Some((at, rf)) = nums.get(&c) {
				let pt = P(rf.clone());
				let digit = (**rf / 10u32.pow(*at)) % 10;
				let digit = String::from(char::from_digit(digit, 10).unwrap());
				if part_nums.contains(&pt) {
					print!("{}", digit.green());
				} else {
					print!("{}", digit.red());
				}
			} else {
				print!(".");
			}
		}
		println!();
	}
}

fn get_gears(
	parts: &HashMap<Coord, PartType>,
	nums: &HashMap<Coord, (u32, Rc<u32>)>,
) -> HashMap<Coord, u32> {
	let mut gears: HashMap<Coord, u32> = HashMap::new();
	for p in parts {
		if let (c, &PartType::Gear) = p {
			let partnums = get_part_nums_for(*c, nums);
			if partnums.len() == 2 {
				let mut ratio = 1u32;
				for pn in partnums {
					ratio *= *pn.0;
				}
				gears.insert(*c, ratio);
			}
		}
	}
	gears
}
