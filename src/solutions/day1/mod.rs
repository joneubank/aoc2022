#[allow(dead_code)]
pub fn run() {
	println!("Day 1");
	let data = read_data();
	q1(&data);
	q2(&data);
}

fn read_data() -> Vec<i32> {
	let mut output: Vec<i32> = vec![];
	if let Ok(lines) = crate::inputs::read_lines("./src/solutions/day1/input.txt") {
		let mut sum = 0;
		for line in lines {
			if let Ok(text) = line {
				if text.is_empty() {
					output.push(sum);
					sum = 0;
				} else {
					// convert string to number, add to current
					if let Ok(num) = text.parse::<i32>() {
						sum += num;
					}
				}
			}
		}
	}
	output.sort();
	output.reverse();
	return output;
}

fn q1(data: &[i32]) {
	println!("Question 1");

	let max = data[0];
	println!("Max elf calories: {}", max);
}

fn q2(data: &[i32]) {
	println!("\nQuestion 2");
	let sum = data[0] + data[1] + data[2];

	println!("Calories from top 3 elves: {}", sum);
}
