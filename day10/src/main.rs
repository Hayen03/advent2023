use core::fmt;
use std::default;
use std::fs;
use std::io;
use std::io::BufRead;
use std::str::Chars;

use colored::Colorize;

static INPUT_PATH: &str = "./rsrc/input.txt";

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");
	let mut input_iter = InputIterator::new(f);
	let mut tiles: Vec<Vec<Tile>> = Vec::new();
	let mut line: Vec<Tile> = Vec::new();
	let mut start_pos:Option<(usize, usize)> = None;
	for i in input_iter {
		match i {
			Input::Tile(t) => {
				if let Tile::StartPos = &t {
					start_pos = Some((line.len(), tiles.len()));
				}
				line.push(t);
			},
			Input::EndLine => {
				tiles.push(line);
				line = Vec::new();
			},
		}
	}
	if line.len() > 0 {
		tiles.push(line);
	}
	// padd
	let width = tiles.iter().map(Vec::len).max().unwrap_or(0);
	let height = tiles.len();
	for ln in &mut tiles {
		if ln.len() < width {
			for _ in 0..(width-ln.len()) {
				ln.push(Tile::Unkwown);
			}
		}
	}
	let mut ts: Vec<Tile> = Vec::new();
	//println!("{}", "■".repeat(width+2));
	for mut ln in tiles {
		//println!("■{}■", ln.iter().map(Tile::to_string).collect::<Vec<String>>().join(""));
		ts.append(&mut ln);
	}
	//println!("{}", "■".repeat(width+2));

	let mut tiles = Context::new(ts, width, height);
	let mut path = Vec::new();
	let mut success = false;
	if let Some((x, y)) = start_pos {
		if let Some(p) = gess_pipe_at(&tiles, x, y) {
			let t = tiles.get_mut(x, y);
			*t = Tile::Pipe(p);
		} else {
			println!("{}", "N'a pas pu changer la position de départ".red().bold());
		}
		path = match follow(&tiles, x, y) {
			Ok(p) => {success = true; p},
			Err(p) => p,
		};
	}

	println!("{}", "■".repeat(tiles.width+2));
	for i in 0..tiles.height {
		let ln = tiles.line_at(i);
		print!("■");
		for j in 0..tiles.width {
			if path.contains(&(j, i)) {
				if success {
					print!("{}", ln[j].to_string().green());
				} else {
					print!("{}", ln[j].to_string().red());
				}
			} else {
				print!("{}", ln[j].to_string());
			}
		}
		println!("■");
	}
	println!("{}", "■".repeat(width+2)); 
	println!("SIZE: {}", path.len());
	println!("HALF: {}", path.len()/2);

}

fn follow(context: &Context, start_x: usize, start_y: usize) -> Result<Vec<(usize, usize)>, Vec<(usize, usize)>> {
	let mut path: Vec<(usize, usize)> = vec![(start_x, start_y)];

	let (mut x, mut y) = (start_x, start_y);
	let mut d = None;
	while {
		if let Some((dir, nx, ny)) = follow_next(context, x, y, d) {
			if (nx, ny) == (start_x, start_y) {
				false
			} else {
				path.push((nx, ny));
				d = Some(dir);
				(x, y) = (nx, ny);
				true
			}
		} else {
			return Err(path);
		}
	} {}

	Ok(path)
}
fn follow_next(context: &Context, x: usize, y: usize, from: Option<Dir>) -> Option<(Dir, usize, usize)> {
	if let Tile::Pipe(p) = *context.get(x, y) {
		if let Some(d) = from {
			let out = if p.0 == d {p.1} else {p.0};
			//println!("[DBG] pipe {} venant de {} direction {}", p, d, out);
			let delta = out.delta();
			let (nx, ny) = (x as i128+delta.0, y as i128+delta.1);
			if context.in_bounds(nx, ny) {
				let (nx, ny) = (nx as usize, ny as usize);
				if let Tile::Pipe(o) = *context.get(nx, ny) {
					if o.connect_to(out.opposite()) {
						Some((out.opposite(), nx, ny))
					} else {
						//println!("[ERR] ne connecte pas (pipe {} dir {})", o, out.opposite());
						None
					}
				} else {
					//println!("[ERR] Pas une pipe au {}", d.as_str());
					None
				}
			} else {
				//println!("[ERR] out of bounds");
				None
			}
		} else {
			follow_next(context, x, y, Some(p.0))
		}
	} else {
		//println!("[ERR] Pas une pipe");
		None
	}
}

fn gess_pipe_at(context: &Context, x: usize, y: usize) -> Option<Pipe> {
	if let Tile::Pipe(p) = context.get(x, y) {
		Some(*p)
	} else {
		let mut dirs: Vec<Dir> = Vec::new();
		for d in DIRS {
			let (dx, dy) = d.delta();
			let (nx, ny) = (x as i128 + dx, y as i128 + dy);
			if context.in_bounds(nx, ny) {
				if let Tile::Pipe(p) = *context.get(nx as usize, ny as usize) {
					//println!("Checking {} for {} at ({}, {})", d, p, nx, ny);
					//print!("Trying to connect to {}... ", d.opposite().as_str());
					if p.connect_to(d.opposite()) {
						//println!("Success!");
						dirs.push(d);
					} else {
						//println!("Failure...");
					}
				}
			}
		}
		if dirs.len() >= 2 {
			Some(Pipe(dirs[0], dirs[1]))
		} else {
			None
		}
	}
}

struct Context {
	tiles: Vec<Tile>,
	width: usize,
	height: usize,
}
impl Context {
	fn new(tiles: Vec<Tile>, width: usize, height: usize) -> Self {
		assert!(tiles.len() == width*height, "waht");
		Self {tiles, width, height}
	}
	fn get(&self, x:usize, y:usize) -> &Tile {
		assert!(x < self.width && y < self.height, "Index out of bounds");
		&self.tiles[y*self.width + x]
	}
	fn get_mut(&mut self, x:usize, y:usize) -> &mut Tile {
		assert!(x < self.width && y < self.height, "Index out of bounds");
		&mut self.tiles[y*self.width + x]
	}
	fn in_bounds(&self, x: i128, y: i128) -> bool{
		0 <= x && x < self.width as i128 && 0 <= y && y < self.height as i128
	}
	fn line_at(&self, y: usize) -> &[Tile] {
		let at = y*self.width;
		&self.tiles[at..(at+self.width)]
	}
}

static DIRS: [Dir;4] = [Dir::North, Dir::East, Dir::South, Dir::West];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
	North = 0,
	East,
	South,
	West,
}
impl Dir {
	fn opposite(self) -> Self {
		match self {
			Dir::North => Dir::South,
			Dir::East => Dir::West,
			Dir::South => Dir::North,
			Dir::West => Dir::East,
		}
	}
	fn left(self) -> Self {
		match self {
			Dir::North => Dir::West,
			Dir::East => Dir::North,
			Dir::South => Dir::East,
			Dir::West => Dir::South,
		}
	}
	fn right(self) -> Self {
		match self {
			Dir::North => Dir::East,
			Dir::East => Dir::South,
			Dir::South => Dir::West,
			Dir::West => Dir::North,
		}
	}
	fn as_short_str(self) -> &'static str {
		match self {
			Dir::North => "N",
			Dir::East => "E",
			Dir::South => "S",
			Dir::West => "W",
		}
	}
	fn as_str(self) -> &'static str {
		match self {
			Dir::North => "North",
			Dir::East => "East",
			Dir::South => "South",
			Dir::West => "West",
		}
	}
	fn delta(self) -> (i128, i128) {
		match self {
			Dir::North => (0,-1),
			Dir::East => (1,0),
			Dir::South => (0,1),
			Dir::West => (-1, 0),
		}
	}
}
impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
enum Tile {
	Ground,
	Pipe(Pipe),
	#[default]
	Unkwown,
	StartPos,
}
static GROUND_COLOR: colored::CustomColor = colored::CustomColor{r:139,g:69, b:19};
impl fmt::Display for Tile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Pipe(p) => write!(f, "{}", p.as_str()),
			Self::Ground => write!(f, "{}", "■".custom_color(GROUND_COLOR)),
			Self::Unkwown => write!(f, "{}", "?".red()),
			Self::StartPos => write!(f, "{}", "S".yellow()),
		}
	}
}

#[derive(Clone, Copy, Debug, Hash)]
struct Pipe(Dir, Dir);
impl PartialEq for Pipe {
	fn eq(&self, other: &Self) -> bool {
		(self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
	}
}
impl Eq for Pipe {}
impl fmt::Display for Pipe {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}", self.0.as_short_str(), self.1.as_short_str())
	}
}
static PNS: Pipe = Pipe(Dir::North, Dir::South);
static PEW: Pipe = Pipe(Dir::East, Dir::West);
static PNW: Pipe = Pipe(Dir::North, Dir::West);
static PSW: Pipe = Pipe(Dir::South, Dir::West);
static PNE: Pipe = Pipe(Dir::North, Dir::East);
static PSE: Pipe = Pipe(Dir::South, Dir::East);
impl TryFrom<char> for Pipe {
	type Error = ();
	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'|' => Ok(PNS),
			'-' => Ok(PEW),
			'L' => Ok(PNE),
			'J' => Ok(PNW),
			'7' => Ok(PSW),
			'F' => Ok(PSE),
			_ => Err(()),
		}
	}
}
impl Pipe {
	fn as_str(self) -> &'static str {
		match self {
			Pipe(Dir::North, Dir::South) | Pipe(Dir::South, Dir::North) => "║",
			Pipe(Dir::East, Dir::West) | Pipe(Dir::West, Dir::East) => "═",
			Pipe(Dir::North, Dir::West) | Pipe(Dir::West, Dir::North) => "╝",
			Pipe(Dir::South, Dir::West) | Pipe(Dir::West, Dir::South) => "╗",
			Pipe(Dir::North, Dir::East) | Pipe(Dir::East, Dir::North) => "╚",
			Pipe(Dir::South, Dir::East) | Pipe(Dir::East, Dir::South) => "╔",
			Pipe(Dir::North, Dir::North) => "╨",
			Pipe(Dir::East, Dir::East) => "╞",
			Pipe(Dir::South, Dir::South) => "╥",
			Pipe(Dir::West, Dir::West) => "╡",
		}
	}
	fn connect_to(&self, dir: Dir) -> bool {
		self.0 == dir || self.1 == dir
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Input {
	Tile(Tile),
	EndLine,
}
impl Input {
	fn new() -> Input {
		todo!()
	}
}
impl std::fmt::Display for Input {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Tile(p) => p.fmt(f),
			Self::EndLine => write!(f, "\n"),
		}
	}
}

struct InputIterator<T>
where
	T: io::Read,
{
	reader: io::BufReader<T>,
	buffer: Option<String>,
	chars_at: usize,
}
impl<T> InputIterator<T>
where
	T: io::Read,
{
	fn new(input: T) -> Self {
		Self {
			reader: io::BufReader::new(input),
			buffer: None,
			chars_at: 0,
		}
	}
}
impl<T: io::Read> Iterator for InputIterator<T> {
	type Item = Input;

	fn next(&mut self) -> Option<Self::Item> {
		if self.buffer.is_none() {
			let mut buf = String::new();
			let res = self.reader.read_line(&mut buf);
			match res {
				Ok(n) => {
					if n == 0 {return None;
					} else {
						self.buffer = Some(buf);
					}
				},
				Err(_) => return None,
			}
			self.chars_at = 0;
		}
		if let Some(buf) = &self.buffer {
			if let Some(c) = buf.chars().nth(self.chars_at) {
				self.chars_at += 1;
				if c == 'S' {
					return Some(Input::Tile(Tile::StartPos));
				}
				if c == '.' {
					return Some(Input::Tile(Tile::Ground));
				}
				if c == '\n' {
					self.buffer = None;
					return Some(Input::EndLine);
				}
				if let Ok(t) = Pipe::try_from(c) {
					return Some(Input::Tile(Tile::Pipe(t)));
				}
				//println!("{}", format!("Tuile inconnu ({c})").red());
				return Some(Input::Tile(Tile::Unkwown));
			} else {
				self.buffer = None;
				return Some(Input::EndLine);
			}
		}
		None
	}
}
