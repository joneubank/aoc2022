use crate::inputs::read_lines;

#[allow(dead_code)]
pub fn run() {
	println!("Day X");
	let data = read_data();
	q1(&data);
	q2(&data);
}

/* ***** Parse Input Data ***** */

fn read_data() -> Vec<String> {
	let mut output: Vec<String> = Vec::new();
	let input_text = read_lines("./src/solutions/template/input.txt");
	if let Ok(lines) = input_text {
		lines.for_each(|line| {
			let text = line.unwrap();
			output.push(text);
		})
	}
	return output;
}

/* ***** Solve Problem ***** */

fn q1(data: &Vec<String>) {
	println!("\nQuestion 1");
}

fn q2(data: &Vec<String>) {
	println!("\nQuestion 2");
}
