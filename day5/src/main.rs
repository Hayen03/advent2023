use colored::Colorize;
use colored::CustomColor;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::Map;
use std::str::FromStr;

static INPUT_PATH: &str = "./rsrc/input.txt";

lazy_static! {
	static ref MAP_DECL_REG: Regex =
		Regex::new(r"^\s*(?P<src>[a-zA-Z]+)-to-(?P<dest>[a-zA-Z]+)\s+map\s*:\s*$").unwrap();
	static ref MAP_ENTRY_REG: Regex =
		Regex::new(r"^\s*(?P<dest>\d+)\s+(?P<src>\d+)\s+(?P<len>\d+)\s*$").unwrap();
	static ref RSRC_ENTRY_REG: Regex =
		Regex::new(r"^\s*(?P<type>[a-zA-Z]+)\s*:(?P<nums>(?:\s*\d+)*)\s*$").unwrap();
}

fn mainp() {
	let t = Transform {
		source_type: TransformType::Seed,
		dest_type: TransformType::Soil,
		source_start: 100,
		dest_start: 600,
		len: 20,
	};
	let r1 = Rng {
		start: Seed(100),
		len: 20,
	};
	let r2 = Rng {
		start: Seed(70),
		len: 20,
	};
	let r3 = Rng {
		start: Seed(90),
		len: 20,
	};
	let r4 = Rng {
		start: Seed(110),
		len: 5,
	};
	let r5 = Rng {
		start: Seed(110),
		len: 20,
	};
	let r6 = Rng {
		start: Seed(130),
		len: 20,
	};
	let r7 = Rng {
		start: Seed(90),
		len: 40,
	};
	println!("{:?}", t.apply_range::<Seed, Soil>(r1));
	println!("{:?}", t.apply_range::<Seed, Soil>(r2));
	println!("{:?}", t.apply_range::<Seed, Soil>(r3));
	println!("{:?}", t.apply_range::<Seed, Soil>(r4));
	println!("{:?}", t.apply_range::<Seed, Soil>(r5));
	println!("{:?}", t.apply_range::<Seed, Soil>(r6));
	println!("{:?}", t.apply_range::<Seed, Soil>(r7));

	let v = vec![r1, r2, r3, r4, r5, r6, r7];
	let ts = vec![t];
	let res: Vec<Rng<Soil>> = apply_maps_rng(&ts, &v);
	println!("RES: {:?}", res);

	let min = res
		.iter()
		.min_by(|r, s| r.start.0.cmp(&s.start.0))
		.unwrap()
		.start
		.0;
	println!("MIN: {}", min);
}

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");

	// loop sur les lignes
	let reader = io::BufReader::new(f);
	let mut src_t: Option<TransformType> = None;
	let mut dest_t: Option<TransformType> = None;
	let mut map_reg: HashMap<(TransformType, TransformType), Vec<Transform>> = HashMap::new();

	let mut seeds: Vec<Seed> = Vec::new();
	let mut seeds_r: Vec<Rng<Seed>> = Vec::new();

	for line in reader.lines() {
		let line = line.unwrap_or(String::from(""));
		if let Some(cap) = MAP_DECL_REG.captures(&line) {
			src_t = cap.name("src").unwrap().as_str().parse().ok();
			dest_t = cap.name("dest").unwrap().as_str().parse().ok();
			if src_t.is_none() {
				println!("{}", "Source Type invalide".red().bold());
			}
			if dest_t.is_none() {
				println!("{}", "Dest Type invalide".red().bold());
			}
		} else if let Some(cap) = MAP_ENTRY_REG.captures(&line) {
			if let (Some(st), Some(dt)) = (src_t, dest_t) {
				let ss: u64 = cap.name("src").unwrap().as_str().parse().unwrap();
				let ds: u64 = cap.name("dest").unwrap().as_str().parse().unwrap();
				let len: u64 = cap.name("len").unwrap().as_str().parse().unwrap();
				let trans = Transform {
					source_start: ss,
					source_type: st,
					dest_start: ds,
					dest_type: dt,
					len,
				};
				let k = (st, dt);
				let ls = match map_reg.get_mut(&k) {
					Some(l) => l,
					None => {
						map_reg.insert(k, Vec::new());
						map_reg.get_mut(&k).unwrap()
					}
				};
				ls.push(trans);
			}
		} else if let Some(cap) = RSRC_ENTRY_REG.captures(&line) {
			let t: Option<TransformType> = cap.name("type").unwrap().as_str().parse().ok();
			println!("{:?}", t);
			let nums: Vec<u64> = cap
				.name("nums")
				.unwrap()
				.as_str()
				.split_whitespace()
				.map(|s| str::parse::<u64>(s).unwrap())
				.collect();

			let mut nums_r: Vec<(u64, u64)> = Vec::new();
			for i in (0..nums.len()).step_by(2) {
				if nums[i + 1] != 0 {
					nums_r.push((nums[i], nums[i + 1]));
				}
			}

			#[allow(clippy::single_match)]
			match t {
				Some(TransformType::Seed) => {
					seeds.extend(nums.into_iter().map(Seed));
					seeds_r.extend(nums_r.into_iter().map(|r| Rng {
						start: r.0.into(),
						len: r.1,
					}));
				}
				_ => (),
			}
		}
	}

	for ls in map_reg.values() {
		for m in ls {
			println!("{}", m);
		}
		println!();
	}

	print!("RESSOURCES:\n\tSeeds:");
	for s in &seeds_r {
		print!(" {}", s);
	}
	print!("\n\t");
	for s in &seeds {
		print!(" {}", s);
	}
	println!();

	let k = (TransformType::Seed, TransformType::Soil);
	let soils: Vec<Soil> = apply_maps(map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()), &seeds);
	print!("\tSoils:");
	let soils_r: Vec<Rng<Soil>> = apply_maps_rng(
		map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()),
		&seeds_r,
	);
	for s in &soils_r {
		print!(" {}", s);
	}
	print!("\n\t");
	for s in &soils {
		print!(" {}", s);
	}
	println!();

	let k = (TransformType::Soil, TransformType::Fertilizer);
	let fertilizers: Vec<Fertilizer> =
		apply_maps(map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()), &soils);
	let fertilizers_r: Vec<Rng<Fertilizer>> = apply_maps_rng(
		map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()),
		&soils_r,
	);
	print!("\tFertilizers:");
	for s in &fertilizers_r {
		print!(" {}", s);
	}
	print!("\n\t");
	for s in &fertilizers {
		print!(" {}", s);
	}
	println!();

	let k = (TransformType::Fertilizer, TransformType::Water);
	let waters: Vec<Water> = apply_maps(
		map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()),
		&fertilizers,
	);
	let waters_r: Vec<Rng<Water>> = apply_maps_rng(
		map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()),
		&fertilizers_r,
	);
	print!("\tWaters:");
	for s in &waters_r {
		print!(" {}", s);
	}
	print!("\n\t");
	for s in &waters {
		print!(" {}", s);
	}
	println!();

	let k = (TransformType::Water, TransformType::Light);
	let lights: Vec<Light> =
		apply_maps(map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()), &waters);
	let lights_r: Vec<Rng<Light>> = apply_maps_rng(
		map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()),
		&waters_r,
	);
	print!("\tLights:");
	for s in &lights_r {
		print!(" {}", s);
	}
	print!("\n\t");
	for s in &lights {
		print!(" {}", s);
	}
	println!();

	let k = (TransformType::Light, TransformType::Temperature);
	let temperatures: Vec<Temperature> =
		apply_maps(map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()), &lights);
	let temperatures_r: Vec<Rng<Temperature>> = apply_maps_rng(
		map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()),
		&lights_r,
	);
	print!("\tTemperatures:");
	for s in &temperatures_r {
		print!(" {}", s);
	}
	print!("\n\t");
	for s in &temperatures {
		print!(" {}", s);
	}
	println!();

	let k = (TransformType::Temperature, TransformType::Humidity);
	let humidities: Vec<Humidity> = apply_maps(
		map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()),
		&temperatures,
	);
	let humidities_r: Vec<Rng<Humidity>> = apply_maps_rng(
		map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()),
		&temperatures_r,
	);
	print!("\tHumidities:");
	for s in &humidities_r {
		print!(" {}", s);
	}
	print!("\n\t");
	for s in &humidities {
		print!(" {}", s);
	}
	println!();

	let k = (TransformType::Humidity, TransformType::Location);
	let locations: Vec<Location> = apply_maps(
		map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()),
		&humidities,
	);
	let locations_r: Vec<Rng<Location>> = apply_maps_rng(
		map_reg.get(&k).unwrap_or(&Vec::<Transform>::new()),
		&humidities_r,
	);
	print!("\tLocations:");
	for s in &locations_r {
		print!(" {}", s);
	}
	print!("\n\t");
	for s in &locations {
		print!(" {}", s);
	}
	println!();

	println!("MIN LOCATION: {}", locations.iter().min().unwrap());
	let min_loc_r = locations_r
		.iter()
		.min_by(|r, s| r.start.0.cmp(&s.start.0))
		.unwrap()
		.start
		.0;
	println!("MIN LOCATION_R: {}", min_loc_r);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Seed(u64);
impl From<Seed> for u64 {
	fn from(value: Seed) -> Self {
		value.0
	}
}
impl AsRef<u64> for Seed {
	fn as_ref(&self) -> &u64 {
		&self.0
	}
}
impl From<u64> for Seed {
	fn from(value: u64) -> Self {
		Seed(value)
	}
}
impl std::fmt::Display for Seed {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_string().green())
	}
}

static SOIL_COLOR: CustomColor = CustomColor {
	r: 204,
	g: 51,
	b: 0,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Soil(u64);
impl From<Soil> for u64 {
	fn from(value: Soil) -> Self {
		value.0
	}
}
impl AsRef<u64> for Soil {
	fn as_ref(&self) -> &u64 {
		&self.0
	}
}
impl std::fmt::Display for Soil {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_string().custom_color(SOIL_COLOR))
	}
}
impl From<u64> for Soil {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

static FERTILIZER_COLOR: CustomColor = CustomColor { r: 0, g: 153, b: 0 };
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Fertilizer(u64);
impl From<Fertilizer> for u64 {
	fn from(value: Fertilizer) -> Self {
		value.0
	}
}
impl AsRef<u64> for Fertilizer {
	fn as_ref(&self) -> &u64 {
		&self.0
	}
}
impl std::fmt::Display for Fertilizer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_string().custom_color(FERTILIZER_COLOR))
	}
}
impl From<u64> for Fertilizer {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Water(u64);
impl From<Water> for u64 {
	fn from(value: Water) -> Self {
		value.0
	}
}
impl AsRef<u64> for Water {
	fn as_ref(&self) -> &u64 {
		&self.0
	}
}
impl std::fmt::Display for Water {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_string().blue())
	}
}
impl From<u64> for Water {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Light(u64);
impl From<Light> for u64 {
	fn from(value: Light) -> Self {
		value.0
	}
}
impl AsRef<u64> for Light {
	fn as_ref(&self) -> &u64 {
		&self.0
	}
}
impl std::fmt::Display for Light {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_string().yellow())
	}
}
impl From<u64> for Light {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Temperature(u64);
impl From<Temperature> for u64 {
	fn from(value: Temperature) -> Self {
		value.0
	}
}
impl AsRef<u64> for Temperature {
	fn as_ref(&self) -> &u64 {
		&self.0
	}
}
impl std::fmt::Display for Temperature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_string().red())
	}
}
impl From<u64> for Temperature {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Humidity(u64);
impl From<Humidity> for u64 {
	fn from(value: Humidity) -> Self {
		value.0
	}
}
impl AsRef<u64> for Humidity {
	fn as_ref(&self) -> &u64 {
		&self.0
	}
}
impl std::fmt::Display for Humidity {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_string().bright_blue())
	}
}
impl From<u64> for Humidity {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

static LOCATION_COLOR: CustomColor = CustomColor {
	r: 180,
	g: 180,
	b: 180,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location(u64);
impl From<Location> for u64 {
	fn from(value: Location) -> Self {
		value.0
	}
}
impl AsRef<u64> for Location {
	fn as_ref(&self) -> &u64 {
		&self.0
	}
}
impl std::fmt::Display for Location {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_string().custom_color(LOCATION_COLOR))
	}
}
impl From<u64> for Location {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

lazy_static! {
	static ref SEED_REG: Regex = Regex::new(r"(?i)seed").unwrap();
	static ref SOIL_REG: Regex = Regex::new(r"(?i)soil").unwrap();
	static ref FERTILIZER_REG: Regex = Regex::new(r"(?i)fertilizer").unwrap();
	static ref WATER_REG: Regex = Regex::new(r"(?i)water").unwrap();
	static ref LIGHT_REG: Regex = Regex::new(r"(?i)light").unwrap();
	static ref TEMPERATURE_REG: Regex = Regex::new(r"(?i)temperature").unwrap();
	static ref HUMIDITY_REG: Regex = Regex::new(r"(?i)humidity").unwrap();
	static ref LOCATION_REG: Regex = Regex::new(r"(?i)location").unwrap();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TransformType {
	Seed,
	Soil,
	Fertilizer,
	Water,
	Light,
	Temperature,
	Humidity,
	Location,
}
impl std::fmt::Display for TransformType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match *self {
				Self::Seed => "Seed",
				Self::Soil => "Soil",
				Self::Fertilizer => "Fertilizer",
				Self::Water => "Water",
				Self::Light => "Light",
				Self::Humidity => "Humidity",
				Self::Location => "Location",
				Self::Temperature => "Temperature",
			}
		)
	}
}
impl FromStr for TransformType {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		print!("RECEIVED {} ", s.green());
		let res = if SEED_REG.is_match(s) {
			Ok(Self::Seed)
		} else if SOIL_REG.is_match(s) {
			Ok(Self::Soil)
		} else if FERTILIZER_REG.is_match(s) {
			Ok(Self::Fertilizer)
		} else if WATER_REG.is_match(s) {
			Ok(Self::Water)
		} else if LIGHT_REG.is_match(s) {
			Ok(Self::Light)
		} else if HUMIDITY_REG.is_match(s) {
			Ok(Self::Humidity)
		} else if LOCATION_REG.is_match(s) {
			Ok(Self::Location)
		} else if TEMPERATURE_REG.is_match(s) {
			Ok(Self::Temperature)
		} else {
			Err("Type Invalide")
		};

		println!("{}", if res.is_ok() { '👌' } else { '👎' });

		res
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Transform {
	source_type: TransformType,
	dest_type: TransformType,
	source_start: u64,
	dest_start: u64,
	len: u64,
}

trait Transformable:
	From<u64>
	+ Into<u64>
	+ Clone
	+ Copy
	+ std::fmt::Debug
	+ PartialEq
	+ Eq
	+ PartialOrd
	+ Ord
	+ std::hash::Hash
	+ std::fmt::Display
{
	fn trans_type() -> TransformType;
}
impl Transformable for Seed {
	fn trans_type() -> TransformType {
		TransformType::Seed
	}
}
impl Transformable for Soil {
	fn trans_type() -> TransformType {
		TransformType::Soil
	}
}
impl Transformable for Fertilizer {
	fn trans_type() -> TransformType {
		TransformType::Fertilizer
	}
}
impl Transformable for Water {
	fn trans_type() -> TransformType {
		TransformType::Water
	}
}
impl Transformable for Light {
	fn trans_type() -> TransformType {
		TransformType::Light
	}
}
impl Transformable for Temperature {
	fn trans_type() -> TransformType {
		TransformType::Temperature
	}
}
impl Transformable for Humidity {
	fn trans_type() -> TransformType {
		TransformType::Humidity
	}
}
impl Transformable for Location {
	fn trans_type() -> TransformType {
		TransformType::Location
	}
}

impl Transform {
	fn apply<S: Transformable, D: Transformable>(&self, src: S) -> Result<D, &'static str> {
		if S::trans_type() != self.source_type || D::trans_type() != self.dest_type {
			return Err("Type Incompatible");
		}
		let delta: Option<u64> = u64::checked_sub(src.into(), self.source_start);
		if let Some(delta) = delta {
			if delta < self.len {
				return Ok(D::from(self.dest_start + delta));
			}
		}
		Err("Pas dans la portée")
	}
	fn accept<S: Transformable, D: Transformable>(&self, src: S) -> bool {
		if S::trans_type() != self.source_type || D::trans_type() != self.dest_type {
			return false;
		}
		let delta: Option<u64> = u64::checked_sub(src.into(), self.source_start);
		if let Some(delta) = delta {
			if delta < self.len {
				return true;
			}
		}
		false
	}
	fn apply_range<S: Transformable, D: Transformable>(
		&self,
		src: Rng<S>,
	) -> (Option<Rng<D>>, Vec<Rng<S>>) {
		let mut left: Vec<Rng<S>> = Vec::new();
		if src.len == 0 {
			return (None, left);
		}

		let start: u64 = max(self.source_start, src.start.into());
		let end: u64 = min(self.source_start + self.len, src.start.into() + src.len);
		let len = u64::checked_sub(end, start);
		if len == Some(0) {
			return (None, left);
		}
		let transformed = len.map(|l| {
			let delta = start - self.source_start;
			Rng {
				start: D::from(self.dest_start + delta),
				len: l,
			}
		});
		if start > src.start.into() {
			if (start <= src.start.into() + src.len) {
				left.push(Rng {
					start: src.start,
					len: start - src.start.into(),
				});
			} else {
				left.push(src);
			}
		}
		if end < src.start.into() + src.len {
			if end >= src.start.into() {
				left.push(Rng {
					start: end.into(),
					len: src.start.into() + src.len - end,
				});
			} else {
				left.push(src);
			}
		}

		(transformed, left)
	}
	fn key(&self) -> (TransformType, TransformType) {
		(self.source_type, self.dest_type)
	}
	fn convert<S: Transformable, D: Transformable>(s: S) -> D {
		D::from(s.into())
	}
	fn convert_rng<S: Transformable, D: Transformable>(s: Rng<S>) -> Rng<D> {
		Rng {
			start: Transform::convert(s.start),
			len: s.len,
		}
	}
}
impl std::fmt::Display for Transform {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}({}) -[{}]> {}({})",
			self.source_type, self.source_start, self.len, self.dest_type, self.dest_start
		)
	}
}

fn apply_maps<S: Transformable, D: Transformable>(maps: &[Transform], src: &[S]) -> Vec<D> {
	src.iter()
		.map(|s| {
			for m in maps {
				if let Ok(d) = m.apply(*s) {
					return d;
				}
			}
			D::from((*s).into())
		})
		.collect()
}
fn apply_maps_rng<S: Transformable, D: Transformable>(
	maps: &[Transform],
	src: &[Rng<S>],
) -> Vec<Rng<D>> {
	let mut left: Vec<Rng<S>> = Vec::from(src);
	let mut trans: Vec<Rng<D>> = Vec::new();

	'top: while let Some(rng) = left.pop() {
		if rng.len == 0 {
			continue;
		}
		for m in maps {
			let (rt, rl) = m.apply_range::<S, D>(rng);
			if let Some(rt) = rt {
				//println!(" * {} -- {} --> {} {:?}", rng, m, rt, rl);
				trans.push(rt);
				left.extend(rl);
				continue 'top;
			}
		}
		trans.push(Transform::convert_rng(rng));
	}

	trans
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rng<T: Transformable> {
	start: T,
	len: u64,
}
impl<T: Transformable> std::fmt::Display for Rng<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}:{})", self.start, self.len)
	}
}
