use colored::Colorize;
use core::fmt;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::ops::Index;
use std::ops::IndexMut;
use std::path::Display;
use std::rc::Rc;

static INPUT_PATH: &str = "./rsrc/input.txt";
lazy_static! {
	static ref REG: Regex = Regex::new(r"^\s*(?i)(?P<a>[KQJAT2-9]{5})\s+(?P<bid>\d+)\s*$").unwrap();
}

fn main() {
	println!("Hello, world!");
	let f = fs::File::open(INPUT_PATH).expect("Le fichier n'existe pas");

	// loop sur les lignes
	let mut input_iter = InputIterator::new(f);
	//println!("{}", input_iter.count());
	let mut inputs: Vec<Input> = Vec::from_iter(input_iter);
	inputs.sort_by(|i, j| cmp_hand5(&i.hand, &j.hand));
	let mut prev: Option<Hand<5>> = None;
	let mut rank = 0;
	let res: i32 = inputs
		.iter()
		.map(|i| {
			if prev.is_none() || prev.unwrap() != i.hand {
				rank += 1;
			}
			prev = Some(i.hand);
			rank * i.bid
		})
		.sum();
	println!("TOTAL: {}", res);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
enum Card {
	A = 14,
	K = 13,
	Q = 12,
	V = 11,
	Ten = 10,
	Nine = 9,
	Eight = 8,
	Seven = 7,
	Six = 6,
	Five = 5,
	Four = 4,
	Three = 3,
	Two = 2,
	J = 1,
}
impl std::fmt::Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}
impl TryFrom<char> for Card {
	type Error = String;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'A' | 'a' => Ok(Card::A),
			'K' | 'k' => Ok(Card::K),
			'Q' | 'q' => Ok(Card::Q),
			'V' | 'v' => Ok(Card::V),
			'T' | 't' => Ok(Card::Ten),
			'9' => Ok(Card::Nine),
			'8' => Ok(Card::Eight),
			'7' => Ok(Card::Seven),
			'6' => Ok(Card::Six),
			'5' => Ok(Card::Five),
			'4' => Ok(Card::Four),
			'3' => Ok(Card::Three),
			'2' => Ok(Card::Two),
			'J' | 'j' => Ok(Card::J),
			c => Err(format!("Carte invalide: {}", c)),
		}
	}
}
impl Card {
	fn as_str(&self) -> &str {
		match self {
			Card::A => "A",
			Card::K => "K",
			Card::Q => "Q",
			Card::J => "J",
			Card::Ten => "T",
			Card::Nine => "9",
			Card::Eight => "8",
			Card::Seven => "7",
			Card::Six => "6",
			Card::Five => "5",
			Card::Four => "4",
			Card::Three => "3",
			Card::Two => "2",
			Card::V => "V",
		}
	}
}
impl From<Card> for char {
	fn from(value: Card) -> Self {
		match value {
			Card::A => 'A',
			Card::K => 'K',
			Card::Q => 'Q',
			Card::J => 'J',
			Card::Ten => 'T',
			Card::Nine => '9',
			Card::Eight => '8',
			Card::Seven => '7',
			Card::Six => '6',
			Card::Five => '5',
			Card::Four => '4',
			Card::Three => '3',
			Card::Two => '2',
			Card::V => 'V',
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Hand<const N: usize>([Card; N]);
impl<const N: usize> Hand<N> {
	fn new(cards: [Card; N]) -> Self {
		//cards.sort_unstable_by(Card::inverse_cmp);
		Self(cards)
	}
}
impl<const N: usize> TryFrom<&str> for Hand<N> {
	type Error = String;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		if N == 0 {
			return Ok(Self([Card::A; N]));
		}
		let mut v: Vec<Card> = Vec::new();
		for ch in value.chars() {
			match Card::try_from(ch) {
				Ok(card) => v.push(card),
				Err(_) => {
					let s = format!("Carte invalide: {}", ch);
					//println!("{}", s.red());
					return Err(s);
				}
			}
			if v.len() >= N {
				break;
			}
		}
		match v.as_slice().try_into() {
			Ok(hands) => Ok(Hand::new(hands)),
			Err(_) => Err(String::from("Pas assez de carte...")),
		}
	}
}
impl<const N: usize> fmt::Display for Hand<N> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"[{}]",
			self.0
				.iter()
				.map(Card::as_str)
				.collect::<Vec<&str>>()
				.join("")
		)
	}
}
impl<const N: usize> AsRef<[Card]> for Hand<N> {
	fn as_ref(&self) -> &[Card] {
		&self.0
	}
}
impl<const N: usize> std::borrow::Borrow<[Card]> for Hand<N> {
	fn borrow(&self) -> &[Card] {
		&self.0
	}
}
impl<const N: usize> AsRef<[Card; N]> for Hand<N> {
	fn as_ref(&self) -> &[Card; N] {
		&self.0
	}
}
impl<const N: usize> std::borrow::Borrow<[Card; N]> for Hand<N> {
	fn borrow(&self) -> &[Card; N] {
		&self.0
	}
}
impl<const N: usize> AsMut<[Card; N]> for Hand<N> {
	fn as_mut(&mut self) -> &mut [Card; N] {
		&mut self.0
	}
}
impl<const N: usize> AsMut<[Card]> for Hand<N> {
	fn as_mut(&mut self) -> &mut [Card] {
		&mut self.0
	}
}
impl<const N: usize> BorrowMut<[Card; N]> for Hand<N> {
	fn borrow_mut(&mut self) -> &mut [Card; N] {
		&mut self.0
	}
}
impl<const N: usize> BorrowMut<[Card]> for Hand<N> {
	fn borrow_mut(&mut self) -> &mut [Card] {
		&mut self.0
	}
}
impl<const N: usize> Index<usize> for Hand<N> {
	type Output = Card;
	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}
impl<const N: usize> IndexMut<usize> for Hand<N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}
impl<const N: usize> PartialOrd for Hand<N> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl<const N: usize> Ord for Hand<N> {
	fn cmp(&self, other: &Self) -> Ordering {
		for i in 0..N {
			match self[i].cmp(&other[i]) {
				Ordering::Equal => continue,
				o => return o,
			}
		}
		Ordering::Equal
	}
}

fn cmp_hand5(first: &Hand<5>, second: &Hand<5>) -> Ordering {
	let s1 = Strength::from(*first);
	let s2 = Strength::from(*second);
	match s1.cmp(&s2) {
		Ordering::Equal => first.cmp(second),
		o => o,
	}
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
enum Strength {
	FiveOfAKind = 6,  // where all five cards have the same label: AAAAA
	FourOfAKind = 5,  //where four cards have the same label and one card has a different label: AA8AA
	FullHouse = 4, //where three cards have the same label, and the remaining two cards share a different label: 23332
	ThreeOfAKind = 3, //where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
	TwoPair = 2, //where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
	OnePair = 1, //where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
	HighCard = 0, //where all cards' labels are distinct: 23456
}
impl From<Hand<5>> for Strength {
	fn from(value: Hand<5>) -> Self {
		let mut reg: HashMap<Card, usize> = HashMap::new();
		let mut jokers = 0;
		for card in value.0 {
			if card == Card::J {
				jokers += 1;
			} else {
				let n = reg.get(&card).unwrap_or(&0) + 1;
				reg.insert(card, n);
			}
		}
		let mut res: Vec<usize> = reg.into_values().collect();
		res.sort_by(usize::inverse_cmp);
		if res.first().unwrap_or(&0) + jokers >= 5 {
			Strength::FiveOfAKind
		} else if res[0] + jokers >= 4 {
			Strength::FourOfAKind
		} else if res[0] + jokers >= 3 {
			if res[1] == 2 {
				Strength::FullHouse
			} else {
				Strength::ThreeOfAKind
			}
		} else if res[0] + jokers >= 2 {
			if res[1] == 2 {
				Strength::TwoPair
			} else {
				Strength::OnePair
			}
		} else {
			Strength::HighCard
		}
	}
}
impl Strength {
	fn as_str(&self) -> &str {
		match self {
			Strength::FiveOfAKind => "Five of a kind",
			Strength::FourOfAKind => "Four of a kind",
			Strength::FullHouse => "Full house",
			Strength::ThreeOfAKind => "Three of a kind",
			Strength::TwoPair => "Two pair",
			Strength::OnePair => "One pair",
			Strength::HighCard => "High card",
		}
	}
	fn rank(&self) -> i32 {
		*self as i32 + 1
	}
}
impl fmt::Display for Strength {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Input {
	hand: Hand<5>,
	bid: i32,
}
impl Input {
	fn new(hand: Hand<5>, bid: i32) -> Input {
		Self { hand, bid }
	}
}
impl std::fmt::Display for Input {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({} {})", self.hand, self.bid)
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
	if let Some(cap) = REG.captures(ln) {
		let a = Hand::try_from(cap.name("a").unwrap().as_str());
		let bid: i32 = cap.name("bid").unwrap().as_str().parse().unwrap();
		if let Ok(hand) = a {
			let i = Input::new(hand, bid);
			println!("FOUND {} {}", i, Strength::from(i.hand));
			return Some(i);
		} //else {
		 //println!("{}", a.unwrap_err().red());
		 //}
	}
	println!("Invalid format: {}", ln);
	None
}

trait InverseOrd: Ord {
	fn inverse_cmp(&self, other: &Self) -> Ordering;
}
impl<T> InverseOrd for T
where
	T: Ord,
{
	fn inverse_cmp(&self, other: &Self) -> Ordering {
		flip_cmp(self.cmp(other))
	}
}
fn flip_cmp(cmp: Ordering) -> Ordering {
	match cmp {
		Ordering::Greater => Ordering::Less,
		Ordering::Less => Ordering::Greater,
		Ordering::Equal => Ordering::Equal,
	}
}
