use crate::inputs::read_lines;

#[allow(dead_code)]
pub fn run() {
	println!("Day 4");
	let data = read_data();
	q1(&data);
	q2(&data);
}

/* ***** Parse Input Data ***** */

struct Range {
	min: u32,
	max: u32,
}

impl Range {
	#[allow(dead_code)] // Unused in this problem after refactor
	fn contains_value(&self, value: u32) -> bool {
		value >= self.min && value <= self.max
	}

	fn contains_range(&self, other: &Range) -> bool {
		self.min <= other.min && self.max >= other.max
	}

	fn overlaps(&self, other: &Range) -> bool {
		self.min <= other.max && self.max >= other.min
	}
}

fn unsafe_parse(value: &str) -> u32 {
	return value
		.parse::<u32>()
		.unwrap_or_else(|error| panic!("Failure parsing int from {value} - {}", error));
}

fn text_to_range(text: &str) -> Range {
	let split = text
		.split_once('-')
		.unwrap_or_else(|| panic!("Cannot split {text} on '-'"));
	return Range {
		min: unsafe_parse(split.0),
		max: unsafe_parse(split.1),
	};
}

fn line_to_ranges(line: &str) -> (Range, Range) {
	let split = line
		.split_once(',')
		.unwrap_or_else(|| panic!("Cannot split {line} on ','"));
	return (text_to_range(split.0), text_to_range(split.1));
}

fn read_data() -> Vec<(Range, Range)> {
	let mut output: Vec<String> = Vec::new();
	let input_text = read_lines("./src/solutions/day4/input.txt");
	if let Ok(lines) = input_text {
		lines.for_each(|line| {
			let text = line.unwrap();
			output.push(text);
		})
	}
	return output.iter().map(|line| line_to_ranges(line)).collect();
}

/* ***** Solve Problem ***** */

fn q1(data: &Vec<(Range, Range)>) {
	println!("\nQuestion 1");

	let count = data
		.iter()
		.filter(|pair| pair.0.contains_range(&pair.1) || pair.1.contains_range(&pair.0))
		.count();

	println!("Count: {count}");
}

fn q2(data: &Vec<(Range, Range)>) {
	println!("\nQuestion 2");

	let count = data.iter().filter(|pair| pair.0.overlaps(&pair.1)).count();

	println!("Count: {count}");
}
