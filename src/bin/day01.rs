use std::error::Error;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(1);

fn part_one(inventories: &Vec<Vec<u32>>) -> Result<u32, Box<dyn Error>> {
    let mut max = 0;
    for inventory in inventories {
        let mut total = 0;
        for number in inventory {
            total += number;
        }

        if total > max {
            max = total;
        }
    }

    Ok(max)
}

fn part_two(inventories: &Vec<Vec<u32>>) -> Result<u32, Box<dyn Error>> {
    let mut totals: Vec<u32> = inventories
        .iter()
        .map(|inventory| {
            let mut total = 0;
            for number in inventory {
                total += number;
            }
            total
        })
        .collect();

    totals.sort();
    totals.reverse();

    let top_three_totals = totals.get(0).unwrap() + totals.get(1).unwrap() + totals.get(2).unwrap();
    Ok(top_three_totals)
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
