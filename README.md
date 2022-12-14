# Advent of Code 2022

[Official Site](https://adventofcode.com/2022)

> Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.
> 
> To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.
> 
> Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Progress Summary

**Total Score:** 12

| Day | Puzzle 1 | Puzzle 2 | Notes |
|:---:|:--------:|:--------:|:----- |
| [1](src/solutions/day1/README.md) | X | X | Sum numbers, use vector sorting to find max 1 and max 3 |
| [2](src/solutions/day2/README.md) | X | X | Split lines on whitespace, convert values to Rock/Paper/Scissor throws and outcomes then count points |
| [3](src/solutions/day3/README.md) | X | X | Use sets to find characters repeated in multiple strings |
| [4](src/solutions/day4/README.md) | X | X | String parsing into ranges, checking overlaps |
| [5](src/solutions/day5/README.md) | X | X | Vector manipulation, popping and pushing |
| [6](src/solutions/day6/README.md) | X | X | Find first unique characters sequence of length N in string |
| 7 |  |  |  |
| 8 |  |  |  |
| 9 |  |  |  |
| 10 |  |  |  |
| 11 |  |  |  |
| 12 |  |  |  |
| 13 |  |  |  |
| 14 |  |  |  |
| 15 |  |  |  |
| 16 |  |  |  |
| 17 |  |  |  |
| 18 |  |  |  |
| 19 |  |  |  |
| 20 |  |  |  |
| 21 |  |  |  |
| 22 |  |  |  |
| 23 |  |  |  |
| 24 |  |  |  |
| 25 |  |  |  |

## Running the Solutions

`cargo run`

The application runs as a standard Rust crate. Usually the [`main.rs`](src/main.rs) file is setup to only run the solution currently being worked on. To run a solution for any given day, update the code in the main file to call `run` on the day in question. Example for running day 2:

```
fn main() {
	solutions::day2::run();
}
```

For convenience, I will probably leave statements to run each given day in the main function, commented out.

### Note about Dead Code

Because I am not referencing the solutions for most days, the `run` functions for each day will be annotated with `#[allow(dead_code)]` to hide all the compile warnings.


## Solution Template

A solution template folder is created at `./src/solutions/template` . This contains a `mod.rs` file with stubbed functions for running the solution, reading the input data, and calculating solutions to both parts of the puzzle.

To prepare a new solution:

1. copy the template folder into the `./src/solutions` dir
1. rename the new folder for your new day ex. `./src/solutions/day100`
2. add the new module declaration to the [`./src/solutions/mod.rs`](./src/solutions/mod.rs) file. ex. `pub mod day100`

You can now run the new solution by calling its run function from the main function in [`./src/main.rs`](./src/main.rs):

```
fn main() {
	solutions::day100::run();
}
```