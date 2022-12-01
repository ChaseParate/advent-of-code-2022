use std::error::Error;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(1);

fn part_one(inventories: &Vec<Vec<u32>>) -> Result<u32, Box<dyn Error>> {
    let mut totals: Vec<u32> = inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect();
    totals.sort();

    Ok(*totals.last().unwrap())
}

fn part_two(inventories: &Vec<Vec<u32>>) -> Result<u32, Box<dyn Error>> {
    let mut totals: Vec<u32> = inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect();
    totals.sort();
    totals.reverse();

    Ok(totals.iter().take(3).sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;
    let inventories: Vec<Vec<u32>> = puzzle_input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    println!("Part 1: {}", part_one(&inventories)?);
    println!("Part 2: {}", part_two(&inventories)?);

    Ok(())
}
