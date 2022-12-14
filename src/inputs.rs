use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
	P: AsRef<Path>,
{
	let file_result = File::open(filename);
	let file = file_result.unwrap();
	return Ok(io::BufReader::new(file).lines());
}

pub fn read_to_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
	let data = fs::read_to_string(filepath)?;
	Ok(data)
}
