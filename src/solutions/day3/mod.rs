use crate::inputs::read_lines;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn run() {
	println!("Day 3");
	let data = read_data();
	q1(&data);
	q2(&data);
}

/* ***** Parse Input Data ***** */

/// Split one string into 2 at the halfway point
fn half_strings<'a>(input: &'a str) -> (&'a str, &'a str) {
	let half = input.len() / 2;
	return (&input[0..half], &input[half..]);
}

fn read_data() -> Vec<String> {
	let mut output: Vec<String> = Vec::new();
	let input_text = read_lines("./src/solutions/day3/input.txt");
	if let Ok(lines) = input_text {
		lines.for_each(|line| {
			let text = line.unwrap_or_else(|error| panic!("{error}"));
			output.push(text)
		})
	}
	return output;
}

/* ***** Utilities ***** */

fn find_shared_char(a: &str, b: &str) -> Option<char> {
	let mut set_a = HashSet::new();
	a.chars().for_each(|letter| {
		set_a.insert(letter);
	});

	// find the element of b that is in the set from a
	return b.chars().find(|letter| set_a.contains(letter));
}

fn find_triple_shared_char(a: &str, b: &str, c: &str) -> Option<char> {
	let set_a: HashSet<char> = HashSet::from_iter(a.chars());

	let set_ab: HashSet<char> =
		HashSet::from_iter(b.chars().filter(|letter| set_a.contains(letter)));

	return c.chars().find(|letter| set_ab.contains(letter));
}

fn convert_char_to_value(input: char) -> u8 {
	(input as u8) % 32 + if input.is_uppercase() { 26 } else { 0 }
}

/* ***** Solve Problem ***** */

fn q1(data: &Vec<String>) {
	println!("\nQuestion 1");

	let mut sum: u32 = 0;

	data.iter().for_each(|rucksack| {
		let compartments = half_strings(&*rucksack);
		let shared_char = find_shared_char(&compartments.0, &compartments.1).unwrap_or_else(|| {
			panic!("No shared character in rucksack '{}'", rucksack);
		});
		sum += convert_char_to_value(shared_char) as u32;
	});
	println!("Sum: {sum}");
}

fn q2(data: &Vec<String>) {
	println!("\nQuestion 2");
	let mut sum: u32 = 0;
	data.chunks(3).for_each(|chunk| {
		let a = chunk
			.get(0)
			.unwrap_or_else(|| panic!("Chunk missing element 0"));
		let b = chunk
			.get(1)
			.unwrap_or_else(|| panic!("Chunk missing element 1"));
		let c = chunk
			.get(2)
			.unwrap_or_else(|| panic!("Chunk missing element 2"));
		let shared_char = find_triple_shared_char(a, b, c).unwrap_or_else(|| {
			panic!("No shared character in group with '{a}', '{b}', and '{c}");
		});
		// println!("{a} {b} {c} {shared_char}");
		sum += convert_char_to_value(shared_char) as u32;
	});
	println!("Sum: {sum}");
}
