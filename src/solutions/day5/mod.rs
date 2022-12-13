use regex::Regex;
use std::{
	fs::File,
	io::{BufReader, Lines},
};

use crate::inputs::read_lines;

#[allow(dead_code)]
pub fn run() {
	println!("Day 5");
	let data = read_data();
	q1(&data.0, &data.1);
	q2(&data.0, &data.1);
}

/* ***** Data ***** */

struct Move {
	count: usize,
	start: usize,
	finish: usize,
}

#[derive(Clone)]
struct Crates {
	stacks: Vec<Vec<char>>,
}

impl Crates {
	fn display(&self, label: &str) {
		println!("\n{label}:");
		let underline = label.chars().map(|_c| "-").collect::<String>() + "-";
		println!("{underline}");
		self.stacks.iter().for_each(|stack| {
			stack.iter().for_each(|c| print!("{c}"));
			print!("\n");
		});
	}

	fn tops(&self) -> String {
		self.stacks
			.iter()
			.map(|stack| stack.last().unwrap())
			.collect::<String>()
	}
	fn move_one(&mut self, start: usize, finish: usize) {
		let item = self.stacks[start].pop().unwrap();
		self.stacks[finish].push(item);
	}
	fn move_one_by_one(&mut self, mv: &Move) {
		for _ in 0..mv.count {
			self.move_one(mv.start, mv.finish);
		}
	}

	/// This isn't how I would prefer to write this, but for now I am struggling with ownership when moving array slices around
	fn move_stack(&mut self, mv: &Move) {
		let mut stack = Vec::new();
		for _ in 0..mv.count {
			let item = self.stacks[mv.start].pop().unwrap();
			stack.push(item);
		}
		stack.reverse();
		stack
			.iter()
			.for_each(|item| self.stacks[mv.finish].push(*item));
	}
}

/* ***** Parse Input Data ***** */

fn parse_crates(lines: Lines<BufReader<File>>) -> Crates {
	let stacks: Vec<Vec<char>> = lines
		.map(|line| {
			let text = line.unwrap();
			return text.chars().collect();
		})
		.collect();
	return Crates { stacks };
}

fn parse_moves(lines: Lines<BufReader<File>>) -> Vec<Move> {
	let moves_regex =
		Regex::new(r"move (?P<count>\d+) from (?P<start>\d+) to (?P<finish>\d+)").unwrap();

	return lines
		.map(|line| {
			let text = line.unwrap();
			let caps = moves_regex.captures(&text).unwrap();
			let count = caps["count"].parse::<usize>().unwrap();
			let start = caps["start"].parse::<usize>().unwrap() - 1;
			let finish = caps["finish"].parse::<usize>().unwrap() - 1;
			return Move {
				count: count,
				start: start,
				finish: finish,
			};
		})
		.collect();
}

fn read_data() -> (Crates, Vec<Move>) {
	let crates_input = read_lines("./src/solutions/day5/input_crates.txt");
	let crates = parse_crates(crates_input.unwrap());

	let moves_input = read_lines("./src/solutions/day5/input_moves.txt");
	let moves = parse_moves(moves_input.unwrap());
	return (crates, moves);
}

/* ***** Utilities ***** */

/* ***** Solve Problem ***** */

fn q1(crates: &Crates, moves: &Vec<Move>) {
	println!("\nQuestion 1");
	let mut mut_crates: Crates = crates.clone();
	mut_crates.display("Starting");
	// println!("{}", mut_crates.tops());
	moves.iter().for_each(|mv| mut_crates.move_one_by_one(mv));
	mut_crates.display("Ending");
	println!("{}", mut_crates.tops());
}

fn q2(crates: &Crates, moves: &Vec<Move>) {
	println!("\nQuestion 2");
	let mut mut_crates: Crates = crates.clone();
	mut_crates.display("Starting");
	// println!("{}", mut_crates.tops());
	moves.iter().for_each(|mv| mut_crates.move_stack(mv));
	mut_crates.display("Ending");
	println!("{}", mut_crates.tops());
}
