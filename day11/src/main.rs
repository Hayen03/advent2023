use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::io;
use std::io::BufRead;
use colored::Colorize;
use rand::prelude::*;
use colored::CustomColor;

static INPUT_PATH: &str = "./rsrc/input.txt";

type C = (usize, usize);
fn ca(a: C, b: C) -> C {
	(a.0+b.0, a.1+b.1)
}

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");
	let input_iter = io::BufReader::new(f);

	let mut galaxies: Vec<C> = Vec::new();
	for (y, r) in input_iter.lines().enumerate() {
		match r {
			Ok(l) => {
				let line = l.trim();
				for (x, c) in line.chars().enumerate() {
					if c == '#' {
						galaxies.push((x, y));
					}
				}
			},
			Err(e) => panic!("Erreur en lisant le fichier"),
		}
	}
	let (width, height) = {
		let mut minx = usize::MAX;
		let mut miny = usize::MAX;
		let mut maxx = 0;
		let mut maxy = 0;
		for (x, y) in &galaxies {
			if x < &minx {
				minx = *x;
			}
			if y < &miny {
				miny = *y;
			}
			if x > &maxx {
				maxx = *x;
			}
			if y > &maxy {
				maxy = *y;
			}
		}
		// repasse pour ajuster les positions 
		for i in 0..galaxies.len() {
			let g = unsafe {galaxies.get_mut(i).unwrap_unchecked()};
			*g = (g.0 - minx, g.1 - miny);
		}
		if (galaxies.len() == 0) {
			(0, 0)
		} else {
			(maxx-minx+1, maxy-miny+1)
		}
	};

	let mut doubled_lines: HashSet<usize> = (0..width).collect();
	let mut doubled_column: HashSet<usize> = (0..width).collect();
	for (x, y) in &galaxies {
		doubled_lines.remove(y);
		doubled_column.remove(x);
	}

	let normal_space_color = CustomColor::new(5, 13, 99);
	let wide_space_color = CustomColor::new(6, 200, 80);
	let very_wide_space_color = CustomColor::new(150, 176, 9);
	let red = CustomColor::new(200, 0, 0);

	println!("{}", "■".repeat(width+2));
	for y in 0..height {
		print!("■");
		for x in 0..width {
			if galaxies.contains(&(x, y)) {
				print!("⋆");
			} else {
				// print space
				let dx = if doubled_column.contains(&y) {2} else {1};
				let dy = if doubled_lines.contains(&x) {2} else {1};
				let color = {
					let fac = dx*dy;
					if fac == 1{
						normal_space_color
					} else if fac == 2 {
						wide_space_color
					} else if fac == 4 {
						very_wide_space_color
					} else {
						red
					}
				};
				let symb = ".";
				print!("{}", symb.custom_color(color));
			}
		}
		println!("■");
	}
	println!("{}", "■".repeat(width+2));

}

#[derive(Clone, Copy, Debug)]
struct Space {
	cost: usize,
	delta: C,
	previous: Option<C>
}
struct Context<'a> {
	spaces: Vec<Vec<Option<Space>>>,
	width: usize,
	height: usize,
	exline: &'a HashSet<usize>,
	excol: &'a HashSet<usize>,
}
impl<'a> Context<'a> {
	fn get(&self, c: C) -> &Option<Space> {
		&self.spaces[c.1][c.0]
	}
	fn get_mut(&mut self, c: C) -> &mut Option<Space> {
		&mut self.spaces[c.1][c.0]
	}
	fn new(width: usize, height: usize, exline: &'a HashSet<usize>, excol: &'a HashSet<usize>) -> Self {
		let mut spaces = Vec::new();
		for _ in 0..height {
			spaces.push(vec![None; width]);
		}
		Self{spaces, width, height, exline, excol}
	}
	fn get_delta(&self, c: C) -> C {
		let dx = if self.excol.contains(&c.0) {2} else {1};
		let dy = if self.exline.contains(&c.1) {2} else {1};
		(dx, dy)
	}
	fn set(&mut self, c: C, cost: usize, delta:C, previous: Option<C>) {
		*self.get_mut(c) = Some(Space {cost, delta, previous});
	}
}

struct Candidats {
	at: C,
	cost: usize,
}
impl Candidats {
	fn new(at: C, cost: usize) -> Candidats {
		Self {at, cost}
	}
}
impl PartialEq for Candidats {
	fn eq(&self, other: &Self) -> bool {
		self.cost.eq(&other.cost)
	}
}
impl Eq for Candidats {}
impl PartialOrd for Candidats {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.cost.partial_cmp(&other.cost)
	}
}
impl Ord for Candidats {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.cost.cmp(&other.cost)
	}
}

fn get_distance_map(start: C, targets: &[C], width: usize, height: usize, double_lines: &HashSet<usize>, double_columns: &HashSet<usize>, results: &HashMap<(C, C), usize>) {
	let mut context = Context::new(width, height, double_lines, double_columns);
	// set l'espace courant
	context.set(start, 0, (1, 1), None);
	//let mut 
}

fn get_nexts_possible_candidate(from: C, delta: C, context: &Context) -> Vec<(C, usize)> {
	let mut candidats: Vec<(C, usize)> = Vec::new();
	// 4 directions à chercher, on va le faire hardcoded
	// Nord
	if from.1 > 0 && context.get((from.0, from.1-1)).is_none() {
		candidats.push(((from.0, from.1-1), delta.1));
	}
	// sud
	if from.1 < context.height-1 && context.get((from.0, from.1+1)).is_none() {
		candidats.push(((from.0, from.1+1), delta.1));
	}
	// ouest
	if from.0 > 0 && context.get((from.0-1, from.1)).is_none() {
		candidats.push(((from.0-1, from.1), delta.0));
	}
	// est
	if from.0 < context.width-1 && context.get((from.0+1, from.1)).is_none() {
		candidats.push(((from.0+1, from.1), delta.0));
	}
	candidats
}