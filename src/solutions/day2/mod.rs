/*

*/

use std::marker::Copy;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug)]
enum Outcome {
	Win,
	Loss,
	Draw,
}

#[derive(Debug)]
enum Throw {
	Rock,
	Paper,
	Scissors,
}

#[derive(Copy, Clone, Debug, EnumString)]
enum OpponentCode {
	A,
	B,
	C,
}

#[derive(Copy, Clone, Debug, EnumString)]
enum SelfCode {
	X,
	Y,
	Z,
}

#[derive(Debug)]
struct Game {
	opponent: Throw,
	me: Throw,
}

#[allow(dead_code)]
pub fn run() {
	println!("Day 2\n");
	let data = read_data();
	q1(&data);
	q2(&data);
}

fn convert_opponent(value: &str) -> OpponentCode {
	OpponentCode::from_str(value)
		.unwrap_or_else(|error| panic!("Unknown value for opponent move: {} - {}", value, error))
}
fn convert_self(value: &str) -> SelfCode {
	SelfCode::from_str(value)
		.unwrap_or_else(|error| panic!("Unknown value for self move: {} - {}", value, error))
}

fn map_opponent_to_throw(code: OpponentCode) -> Throw {
	match code {
		OpponentCode::A => Throw::Rock,
		OpponentCode::B => Throw::Paper,
		OpponentCode::C => Throw::Scissors,
	}
}
fn map_self_to_throw(code: SelfCode) -> Throw {
	match code {
		SelfCode::X => Throw::Rock,
		SelfCode::Y => Throw::Paper,
		SelfCode::Z => Throw::Scissors,
	}
}

fn map_self_to_outcome(code: SelfCode) -> Outcome {
	match code {
		SelfCode::X => Outcome::Loss,
		SelfCode::Y => Outcome::Draw,
		SelfCode::Z => Outcome::Win,
	}
}

fn map_result_to_throw(result: Outcome, opponent: Throw) -> Throw {
	match (result, opponent) {
		(Outcome::Win, Throw::Rock) => Throw::Paper,
		(Outcome::Win, Throw::Paper) => Throw::Scissors,
		(Outcome::Win, Throw::Scissors) => Throw::Rock,
		(Outcome::Draw, Throw::Rock) => Throw::Rock,
		(Outcome::Draw, Throw::Paper) => Throw::Paper,
		(Outcome::Draw, Throw::Scissors) => Throw::Scissors,
		(Outcome::Loss, Throw::Rock) => Throw::Scissors,
		(Outcome::Loss, Throw::Paper) => Throw::Rock,
		(Outcome::Loss, Throw::Scissors) => Throw::Paper,
	}
}

fn split_line<'a>(value: &'a str) -> (&'a str, &'a str) {
	let space_index = value.find(' ');
	space_index
		.map(|i| (&value[0..i], &value[i + 1..]))
		.unwrap_or_else(|| {
			panic!(
				"Unexpected formatting of input line: No whitespace to split on: '{}'",
				value
			)
		})
}

fn parse_line(value: &str) -> (OpponentCode, SelfCode) {
	let split = split_line(value);
	(convert_opponent(split.0), convert_self(split.1))
}

fn read_data() -> Vec<(OpponentCode, SelfCode)> {
	let mut output: Vec<(OpponentCode, SelfCode)> = vec![];
	if let Ok(lines) = crate::inputs::read_lines("./src/solutions/day2/input.txt") {
		for line in lines {
			let text = &*line.unwrap_or_else(|error| panic!("Unable to read line: {}", error));
			let result = parse_line(text);
			output.push(result);
		}
	}

	return output;
}

#[derive(Debug)]
struct PlayerStats {
	rock: i32,
	paper: i32,
	scissors: i32,
	win: i32,
	loss: i32,
	draw: i32,
}

#[derive(Debug)]
struct GameStats {
	rock_rock: i32,
	rock_paper: i32,
	rock_scissors: i32,
	paper_rock: i32,
	paper_paper: i32,
	paper_scissors: i32,
	scissors_rock: i32,
	scissors_paper: i32,
	scissors_scissors: i32,
}

fn calculate_score(stats: PlayerStats) -> i32 {
	stats.rock + stats.paper * 2 + stats.scissors * 3 + stats.draw * 3 + stats.win * 6
}

/// Summarizes all games into counts of all results and throws, one for each player
/// returns a tuple with (GameStats, Opponent PlayerStats, Self PlayerStats)
fn summarize_games(games: &Vec<Game>) -> (GameStats, PlayerStats, PlayerStats) {
	let mut self_stats = PlayerStats {
		rock: 0,
		paper: 0,
		scissors: 0,
		win: 0,
		loss: 0,
		draw: 0,
	};
	let mut opponent_stats = PlayerStats {
		rock: 0,
		paper: 0,
		scissors: 0,
		win: 0,
		loss: 0,
		draw: 0,
	};
	let mut game_stats = GameStats {
		rock_rock: 0,
		rock_paper: 0,
		rock_scissors: 0,
		paper_rock: 0,
		paper_paper: 0,
		paper_scissors: 0,
		scissors_rock: 0,
		scissors_paper: 0,
		scissors_scissors: 0,
	};

	games.iter().for_each(|game| match game {
		Game {
			opponent: Throw::Rock,
			me: Throw::Rock,
		} => {
			game_stats.rock_rock += 1;

			opponent_stats.rock += 1;
			opponent_stats.draw += 1;

			self_stats.rock += 1;
			self_stats.draw += 1;
		}
		Game {
			opponent: Throw::Rock,
			me: Throw::Paper,
		} => {
			game_stats.rock_paper += 1;

			opponent_stats.rock += 1;
			opponent_stats.loss += 1;

			self_stats.paper += 1;
			self_stats.win += 1;
		}
		Game {
			opponent: Throw::Rock,
			me: Throw::Scissors,
		} => {
			game_stats.rock_scissors += 1;

			opponent_stats.rock += 1;
			opponent_stats.win += 1;

			self_stats.scissors += 1;
			self_stats.loss += 1;
		}
		Game {
			opponent: Throw::Paper,
			me: Throw::Rock,
		} => {
			game_stats.paper_rock += 1;

			opponent_stats.paper += 1;
			opponent_stats.win += 1;

			self_stats.rock += 1;
			self_stats.loss += 1;
		}
		Game {
			opponent: Throw::Paper,
			me: Throw::Paper,
		} => {
			game_stats.paper_paper += 1;

			opponent_stats.paper += 1;
			opponent_stats.draw += 1;

			self_stats.paper += 1;
			self_stats.draw += 1;
		}
		Game {
			opponent: Throw::Paper,
			me: Throw::Scissors,
		} => {
			game_stats.paper_scissors += 1;

			opponent_stats.paper += 1;
			opponent_stats.loss += 1;

			self_stats.scissors += 1;
			self_stats.win += 1;
		}
		Game {
			opponent: Throw::Scissors,
			me: Throw::Rock,
		} => {
			game_stats.scissors_rock += 1;

			opponent_stats.scissors += 1;
			opponent_stats.loss += 1;

			self_stats.rock += 1;
			self_stats.win += 1;
		}
		Game {
			opponent: Throw::Scissors,
			me: Throw::Paper,
		} => {
			game_stats.scissors_paper += 1;

			opponent_stats.scissors += 1;
			opponent_stats.win += 1;

			self_stats.paper += 1;
			self_stats.loss += 1;
		}
		Game {
			opponent: Throw::Scissors,
			me: Throw::Scissors,
		} => {
			game_stats.scissors_scissors += 1;

			opponent_stats.scissors += 1;
			opponent_stats.draw += 1;

			self_stats.scissors += 1;
			self_stats.draw += 1;
		}
	});
	return (game_stats, opponent_stats, self_stats);
}

fn q1(data: &Vec<(OpponentCode, SelfCode)>) {
	println!("Question 1");

	let games = data
		.iter()
		.map(|line| Game {
			opponent: map_opponent_to_throw(line.0),
			me: map_self_to_throw(line.1),
		})
		.collect::<Vec<_>>();

	let summary = summarize_games(&games);
	let game_stats = summary.0;
	let opponent_stats = summary.1;
	let self_stats = summary.2;
	println!("Game Results: {game_stats:?}");
	println!("Opponent Throws: {opponent_stats:?}");
	println!("Opponent Score: {}", calculate_score(opponent_stats));
	println!("Self Throws: {self_stats:?}");
	println!("Self Score: {}", calculate_score(self_stats));
}

fn q2(data: &Vec<(OpponentCode, SelfCode)>) {
	println!("\nQuestion 2");

	let games = data
		.iter()
		.map(|line| Game {
			opponent: map_opponent_to_throw(line.0),
			me: map_result_to_throw(map_self_to_outcome(line.1), map_opponent_to_throw(line.0)),
		})
		.collect::<Vec<_>>();

	let summary = summarize_games(&games);
	let game_stats = summary.0;
	let opponent_stats = summary.1;
	let self_stats = summary.2;
	println!("Game Results: {game_stats:?}");
	println!("Opponent Throws: {opponent_stats:?}");
	println!("Opponent Score: {}", calculate_score(opponent_stats));
	println!("Self Throws: {self_stats:?}");
	println!("Self Score: {}", calculate_score(self_stats));
}
