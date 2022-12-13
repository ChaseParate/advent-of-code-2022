use std::error::Error;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(6);

fn part_one(stream: &[char]) -> Result<u32, Box<dyn Error>> {
    let window_size = 4;

    for (index, window) in stream.windows(window_size).enumerate() {
        let mut all_unique = true;
        for (i, char1) in window.iter().enumerate() {
            for char2 in &window[i + 1..] {
                if char1 == char2 {
                    all_unique = false;
                }
            }
        }

        if all_unique {
            return Ok((index + window_size) as u32);
        }
    }

    Ok(0)
}

fn part_two(stream: &[char]) -> Result<u32, Box<dyn Error>> {
    let window_size = 14;

    for (index, window) in stream.windows(window_size).enumerate() {
        let mut all_unique = true;
        for (i, char1) in window.iter().enumerate() {
            for char2 in &window[i + 1..] {
                if char1 == char2 {
                    all_unique = false;
                }
            }
        }

        if all_unique {
            return Ok((index + window_size) as u32);
        }
    }

    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;

    let stream: Vec<char> = puzzle_input.chars().collect();

    println!("Part 1: {}", part_one(&stream)?);
    println!("Part 2: {}", part_two(&stream)?);

    Ok(())
}
