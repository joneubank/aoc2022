use array_tool::vec::Uniq;

use crate::inputs::read_to_string;

#[allow(dead_code)]
pub fn run() {
	println!("Day 6");
	let data = read_data();
	q1(&data);
	q2(&data);
}

/* ***** Parse Input Data ***** */

fn read_data() -> String {
	read_to_string("./src/solutions/day6/input.txt").expect("Input must exist")
}

/* ***** Solve Problem ***** */

fn first_unique_seq(data: &str, length: usize) -> usize {
	let mut i = 0;
	let mut buffer = Vec::from(&data[0..length]);
	for c in data.chars() {
		buffer[i % length] = c as u8;

		if buffer.is_unique() {
			// the less than length check returns the value of length if the first sequence of characters is all unique
			return if i < length { length } else { i + 1 };
		}

		i += 1;
	}
	panic!("No unique strings!");
}

fn q1(data: &String) {
	println!("\nQuestion 1");
	let solution = first_unique_seq(data, 4);
	println!("Start Signal at: {}", solution);
}

fn q2(data: &String) {
	println!("\nQuestion 2");
	let solution = first_unique_seq(data, 14);
	println!("Start Signal at: {}", solution);
}
