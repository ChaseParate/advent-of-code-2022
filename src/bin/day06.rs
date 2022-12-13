use std::error::Error;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(6);

fn get_index_of_unique_window(stream: &[char], window_size: usize) -> Option<usize> {
    'main_loop: for (index, window) in stream.windows(window_size).enumerate() {
        for (i, char1) in window.iter().enumerate() {
            for char2 in &window[i + 1..] {
                if char1 == char2 {
                    continue 'main_loop;
                }
            }
        }

        return Some(index + window_size);
    }

    None
}

fn part_one(stream: &[char]) -> Result<u32, Box<dyn Error>> {
    get_index_of_unique_window(stream, 4)
        .map(|index| index as u32)
        .ok_or_else(|| "Could not find index".into())
}

fn part_two(stream: &[char]) -> Result<u32, Box<dyn Error>> {
    get_index_of_unique_window(stream, 14)
        .map(|index| index as u32)
        .ok_or_else(|| "Could not find index".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;

    let stream: Vec<char> = puzzle_input.chars().collect();

    println!("Part 1: {}", part_one(&stream)?);
    println!("Part 2: {}", part_two(&stream)?);

    Ok(())
}
