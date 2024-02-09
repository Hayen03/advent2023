use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;
use std::io::Write;

static INPUT_PATH: &str = "./rsrc/input.txt";

lazy_static! {
	static ref INPUT_REG: Regex = Regex::new(r"^\s*(?i)(?:(?P<seq>[LR]+)|(?P<node>(?P<id>[a-zA-Z]+)\s*=\s*\((?P<left>[a-zA-Z]+),\s*(?P<right>[a-zA-Z]+)\)))\s*$").unwrap();
}

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");
	let mut input_iter = InputIterator::new(f);

	let mut node_reg: HashMap<u32, Node> = HashMap::new();
	let mut seq: Vec<Dir> = Vec::new();
	let mut starting_nodes: HashSet<u32> = HashSet::new();
	let mut ending_nodes: HashSet<u32> = HashSet::new();
	for input in input_iter {
		match input {
			Input::Seq(s) => seq.extend(s),
			Input::Node(n) => {
				if n.is_initial() {
					starting_nodes.insert(n.id);
				} else if n.is_final() {
					ending_nodes.insert(n.id);
				}

				node_reg.insert(n.id, n);
			}
		}
	}

	let start_node = *node_reg
		.get(&Node::compute_id("AAA"))
		.expect("Le noeud de départ n'existe pas");
	let end_node = *node_reg
		.get(&Node::compute_id("ZZZ"))
		.expect("Le noeud d'arrivé n'existe pas");

	println!(
		"INITIAL: {}",
		starting_nodes
			.iter()
			.map(|nid| Node::reverse_id(*nid))
			.collect::<Vec<_>>()
			.join("\t")
	);
	println!(
		"FINAL: {}",
		ending_nodes
			.iter()
			.map(|nid| Node::reverse_id(*nid))
			.collect::<Vec<_>>()
			.join("\t")
	);

	let result = compute_paths(&starting_nodes, &ending_nodes, &node_reg, &seq);
	println!("{}", result.unwrap_or(0));

	/* let mut count = 0;
	for node in PathIterator::new(&seq, &node_reg, start_node) {
		/* if count != 0 {
			print!(" > ");
		} */
		count += 1;
		//print!("{}", Node::reverse_id(node));
		if node == end_node.id {
			break;
		}
	}
	println!();
	println!("Count: {}", count - 1); */

	/* let mut paths_iter = PathsIterator::new(
		&seq,
		&node_reg,
		starting_nodes
			.into_iter()
			.map(|nid| node_reg.get(&nid).unwrap().to_owned())
			.collect(),
	);
	println!("\nPART 2\n#############");
	let mut count = 0;
	for step in paths_iter {
		count += 1;
		/* let s = step.iter().map(|nid| {
			let s = Node::reverse_id(*nid);
			if is_initial(*nid) {
				s.blue()
			} else if is_final(*nid) {
				s.green()
			} else {
				s.white()
			}
		});
		for ss in s {
			print!("{ss}\t");
		}
		println!(); */
		let mut cont = true;
		for nid in step.iter() {
			cont = cont && is_final(*nid);
		}
		if cont {
			break;
		}
	}
	println!();
	println!("Count: {}", count - 1); */
	let aaaa: u128 = 43 * 59 * 61 * 67 * 73 * 79 * 263;
	println!("{aaaa}");
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Node {
	id: u32,
	left: u32,
	right: u32,
}
impl Node {
	fn get(ids: &str, lefts: &str, rights: &str) -> Node {
		let id = Self::compute_id(ids);
		let left = Self::compute_id(lefts);
		let right = Self::compute_id(rights);
		Node { id, left, right }
	}
	fn new(id: u32, left: u32, right: u32) -> Node {
		Node { id, left, right }
	}
	fn compute_id(id: &str) -> u32 {
		let mut idn = 0;
		for c in id.chars() {
			let c = c.to_ascii_uppercase() as u32;
			idn = idn * 26 + (c - 64);
		}
		idn
	}
	fn reverse_id(mut id: u32) -> String {
		let mut chars: Vec<u8> = Vec::new();
		while id > 0 {
			let c = match (id % 26) as u8 {
				0 => 26,
				a => a,
			} + 64;

			chars.insert(0, c);
			id = (id - 1) / 26;
		}
		unsafe { String::from_utf8_unchecked(chars) }
	}
	fn is_initial(&self) -> bool {
		is_initial(self.id)
	}
	fn is_final(&self) -> bool {
		is_final(self.id)
	}
}
fn is_final(id: u32) -> bool {
	id != 0 && id % 26 == 0
}
fn is_initial(id: u32) -> bool {
	id % 26 == 1
}
impl std::fmt::Display for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} = ({}, {})",
			Node::reverse_id(self.id),
			Node::reverse_id(self.left),
			Node::reverse_id(self.right)
		)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
enum Dir {
	L,
	R,
}
impl Dir {
	fn as_str(&self) -> &str {
		match self {
			Dir::L => "L",
			Dir::R => "R",
		}
	}
}
impl TryFrom<char> for Dir {
	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'l' | 'L' => Ok(Dir::L),
			'r' | 'R' => Ok(Dir::R),
			_ => Err(format!("Direction invalide ({})", value)),
		}
	}

	type Error = String;
}
impl Borrow<str> for Dir {
	fn borrow(&self) -> &str {
		self.as_str()
	}
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Input {
	Seq(Vec<Dir>),
	Node(Node),
}
impl Input {}
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
			ret = parse_line(ln.trim());
			ret.is_none()
		} {}
		ret
	}
}

fn parse_line(ln: &str) -> Option<Input> {
	if let Some(cap) = INPUT_REG.captures(ln) {
		if let Some(seq) = cap.name("seq") {
			//println!("{}: {}", "Reading sequence".red(), seq.as_str());
			let seq = seq.as_str();
			let mut v: Vec<Dir> = Vec::with_capacity(seq.len());
			for c in seq.chars() {
				v.push(Dir::try_from(c).expect("Direction invalide"));
			}
			Some(Input::Seq(v))
		} else {
			//println!("{}: {}", "Reading node".red(), cap.get(0).unwrap().as_str());
			Some(Input::Node(Node::get(
				cap.name("id").unwrap().as_str(),
				cap.name("left").unwrap().as_str(),
				cap.name("right").unwrap().as_str(),
			)))
		}
	} else {
		//println!("{}", "Found nothing".red());
		None
	}
}

struct PathIterator<'a> {
	seq: &'a [Dir],
	at: usize,
	nodes: &'a HashMap<u32, Node>,
	curr: Option<Node>,
}
impl<'a> PathIterator<'a> {
	fn new(seq: &'a [Dir], nodes: &'a HashMap<u32, Node>, start: Node) -> PathIterator<'a> {
		Self {
			seq,
			nodes,
			at: 0,
			curr: Some(start),
		}
	}
}
impl<'a> Iterator for PathIterator<'a> {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(n) = self.curr {
			let res = n.id;
			// compute next node
			let nxid = match self.seq[self.at % (self.seq.len())] {
				Dir::L => n.left,
				Dir::R => n.right,
			};
			self.at += 1;
			self.curr = self.nodes.get(&nxid).copied();
			Some(res)
		} else {
			None
		}
	}
}
struct PathsIterator<'a> {
	seq: &'a [Dir],
	at: usize,
	nodes: &'a HashMap<u32, Node>,
	currs: Option<Box<[Node]>>,
}
impl<'a> PathsIterator<'a> {
	fn new(seq: &'a [Dir], nodes: &'a HashMap<u32, Node>, start: Box<[Node]>) -> PathsIterator<'a> {
		Self {
			seq,
			nodes,
			at: 0,
			currs: Some(start),
		}
	}
}
impl<'a> Iterator for PathsIterator<'a> {
	type Item = Box<[u32]>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(ref mut ns) = self.currs {
			let res: Box<[u32]> = ns.iter().map(|n| n.id).collect();
			// compute next node
			let dir = self.seq[self.at % (self.seq.len())];
			for i in 0..ns.len() {
				let potentiel = match dir {
					Dir::L => self.nodes.get(&ns[i].left),
					Dir::R => self.nodes.get(&ns[i].right),
				};
				if let Some(n) = potentiel {
					ns[i] = *n;
				} else {
					self.currs = None;
					break;
				}
			}
			self.at += 1;
			//self.curr = self.nodes.get(&nxid).copied();
			Some(res)
		} else {
			None
		}
	}
}

type PathReg = HashMap<(u32, usize), PathStatus>;
enum PathStatus {
	EnCours,
	Trouve(u32, usize),
}

fn compute_paths(
	initials: &HashSet<u32>,
	finals: &HashSet<u32>,
	nodes: &HashMap<u32, Node>,
	seq: &[Dir],
) -> Option<usize> {
	let mut paths: PathReg = HashMap::new();
	let mut curr_size = 0;
	let mut currs: Box<[(Node, usize)]> =
		Vec::from_iter(initials.iter().map(|nid| (*nodes.get(nid).unwrap(), 0))).into_boxed_slice();

	let mut it_count = 0u16;
	let mut loops: Box<[Option<(usize, usize)>]> = currs.iter().map(|_a| None).collect();

	#[allow(clippy::never_loop)]
	while let Some(i) = select_next(&currs, finals) {
		//print_paths(&currs, finals);
		//let _ = stdout().flush();
		//let _ = stdin().read_line(&mut String::new());
		it_count = it_count.checked_add(1).unwrap_or(0);
		if it_count == 0 {
			//print_paths(&currs, finals);
			//println!();
			print!(".");
		}

		let (n, dist) = currs[i];
		let at = dist % seq.len();

		let l = &mut loops[i];
		if let Some((_, d)) = l {
			currs[i].1 += *d;
			continue;
		}

		//println!("at: {at}");
		//print!("{}{} > ", "Exploring node ".green(), Node::reverse_id(n.id));
		let res = count_path(n, dist, initials, finals, nodes, seq, at, &mut paths, l);
		if let Some((nid, d)) = res {
			//println!("Found ({}, {})", Node::reverse_id(nid), d);
			let n2 = *nodes.get(&nid).unwrap();
			curr_size = dist + d;
			//print!("{} {} {} {} {:?} ", dist, d, curr_size, at, currs[i]);
			currs[i] = (n2, curr_size);
		//println!("{:?}", currs[i]);
		//return None;
		} else {
			//println!("{}", "waht..".red());
			return None;
		}
	}
	println!("{}", "end".green());
	print_paths(&currs, finals);
	println!();
	Some(curr_size)
}

fn count_path(
	start: Node,
	startdist: usize,
	initials: &HashSet<u32>,
	finals: &HashSet<u32>,
	nodes: &HashMap<u32, Node>,
	seq: &[Dir],
	mut at: usize,
	paths: &mut PathReg,
	loops: &mut Option<(usize, usize)>,
) -> Option<(u32, usize)> {
	//print!("Searching from: ({}, {}) ", Node::reverse_id(start.id), at);
	/* if finals.contains(&start.id) {
		println!("Found None (Final)");
		return None;
	} */
	let test = paths.get(&(start.id, at));
	match test {
		Some(PathStatus::EnCours) => {
			//println!("{}", "Found None (En cours)".red());
			return None;
		}
		Some(PathStatus::Trouve(a, b)) => {
			//println!("Found ({}, {}) (déjà trouvé)", Node::reverse_id(*a), b);
			return Some((*a, *b));
		}
		_ => {
			paths.insert((start.id, at), PathStatus::EnCours);
		}
	}
	let mut curr = start;
	let iat = at;
	let mut dist: usize = 0;

	loop {
		let dir = seq[at % seq.len()];
		let nx_node = nodes.get(match dir {
			Dir::L => &curr.left,
			Dir::R => &curr.right,
		});
		if let Some(nx_node) = nx_node.copied() {
			dist += 1;
			at = (at + 1) % seq.len();
			if nx_node.id == start.id && at == iat && loops.is_none() {
				// looping
				*loops = Some((startdist, dist));
			}
			if finals.contains(&nx_node.id) {
				paths.insert((start.id, iat), PathStatus::Trouve(nx_node.id, dist));
				//println!("Found ({}, {})", Node::reverse_id(nx_node.id), dist);
				return Some((nx_node.id, dist));
			} else if initials.contains(&nx_node.id) {
				//print!("recurs ");
				let ret = count_path(
					nx_node,
					startdist + dist,
					initials,
					finals,
					nodes,
					seq,
					at,
					paths,
					loops,
				);
				match ret {
					None => return None,
					Some((nid, d)) => {
						let total_dist = dist + d;
						paths.insert((start.id, iat), PathStatus::Trouve(nid, total_dist));
						//println!("Found ({}, {})", Node::reverse_id(nx_node.id), total_dist);
						return Some((nid, total_dist));
					}
				}
			}
			curr = nx_node;
		} else {
			//println!("Found None (No path)");
			return None;
		}
	}
}

fn select_next(ls: &[(Node, usize)], finals: &HashSet<u32>) -> Option<usize> {
	let mut curr: Option<(usize, usize)> = None;
	let mut f = true;
	for (i, (_, at)) in ls.iter().enumerate() {
		f = f && finals.contains(&ls[i].0.id);
		match curr {
			Some((_, cd)) => {
				if cd > *at {
					curr = Some((i, *at));
				}
				f = f && ls[i].1 == cd;
			}
			None => {
				curr = Some((i, *at));
			}
		}
	}
	if f {
		//println!("{}", "oupe".red());
		None
	} else {
		//println!("{}", "oupe".green());
		curr.map(|t| t.0)
	}
}

fn print_paths(paths: &[(Node, usize)], finals: &HashSet<u32>) {
	let max_dist = paths.iter().map(|n| n.1).max().unwrap();
	for (n, dist) in paths {
		let s = format!("({}, {})", Node::reverse_id(n.id), dist);
		if finals.contains(&n.id) && dist >= &max_dist {
			print!("{} ", s.green());
		} else {
			print!("{}", s);
		}
	}
}
