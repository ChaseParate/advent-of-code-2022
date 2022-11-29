use std::error::Error;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(1);

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;
    let lines = puzzle_input.lines().collect::<Vec<&str>>();
    println!("{:?}", &lines);

    Ok(())
}