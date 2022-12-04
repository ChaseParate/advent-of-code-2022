use std::error::Error;
use std::ops::RangeInclusive;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(4);

fn part_one(
    range_pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)],
) -> Result<u32, Box<dyn Error>> {
    let sum = range_pairs
        .iter()
        .filter(|(left, right)| {
            let left_bigger = left.size_hint() > right.size_hint();

            let smaller_range = if left_bigger { right } else { left };
            let larger_range = if left_bigger { left } else { right };

            larger_range.start() <= smaller_range.start()
                && larger_range.end() >= smaller_range.end()
        })
        .count() as u32;

    Ok(sum)
}

fn part_two(
    range_pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)],
) -> Result<u32, Box<dyn Error>> {
    let sum = range_pairs
        .iter()
        .filter(|(left, right)| left.start() <= right.end() && right.start() <= left.end())
        .count() as u32;

    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;

    let range_pairs: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = puzzle_input
        .lines()
        .map(|line| {
            let (first_range, second_range) = line.split_once(',').unwrap();
            let first_range = first_range.split_once('-').unwrap();
            let second_range = second_range.split_once('-').unwrap();

            let first_left_bound: u32 = first_range.0.parse().unwrap();
            let first_right_bound: u32 = first_range.1.parse().unwrap();
            let second_left_bound: u32 = second_range.0.parse().unwrap();
            let second_right_bound: u32 = second_range.1.parse().unwrap();

            (
                (first_left_bound..=first_right_bound),
                (second_left_bound..=second_right_bound),
            )
        })
        .collect();

    println!("Part 1: {}", part_one(&range_pairs)?);
    println!("Part 2: {}", part_two(&range_pairs)?);

    Ok(())
}
