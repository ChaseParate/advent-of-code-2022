use std::error::Error;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(8);

fn part_one(data: &[Vec<u32>]) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            let tree = data[y][x];

            let mut visible_up = true;
            for y_scan in 0..y {
                if tree <= data[y_scan][x] {
                    visible_up = false;
                    break;
                }
            }

            let mut visible_down = true;
            for y_scan in y + 1..data.len() {
                if tree <= data[y_scan][x] {
                    visible_down = false;
                    break;
                }
            }

            let mut visible_left = true;
            for x_scan in 0..x {
                if tree <= data[y][x_scan] {
                    visible_left = false;
                    break;
                }
            }

            let mut visible_right = true;
            for x_scan in x + 1..data[y].len() {
                if tree <= data[y][x_scan] {
                    visible_right = false;
                    break;
                }
            }

            let visible = visible_up | visible_down | visible_left | visible_right;
            if visible {
                sum += 1;
            }
        }
    }
    Ok(sum)
}

fn part_two(data: &[Vec<u32>]) -> Result<u32, Box<dyn Error>> {
    let mut max = 0;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            let tree = data[y][x];

            let mut distance_up = 0;
            for y_scan in (0..y).rev() {
                distance_up += 1;
                if tree <= data[y_scan][x] {
                    break;
                }
            }

            let mut distance_down = 0;
            for y_scan in y + 1..data.len() {
                distance_down += 1;
                if tree <= data[y_scan][x] {
                    break;
                }
            }

            let mut distance_left = 0;
            for x_scan in (0..x).rev() {
                distance_left += 1;
                if tree <= data[y][x_scan] {
                    break;
                }
            }

            let mut distance_right = 0;
            for x_scan in x + 1..data[y].len() {
                distance_right += 1;
                if tree <= data[y][x_scan] {
                    break;
                }
            }

            let value = distance_up * distance_down * distance_left * distance_right;
            if value > max {
                max = value;
            }
        }
    }
    Ok(max)
}

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;

    let data: Vec<Vec<u32>> = puzzle_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    println!("Part 1: {}", part_one(&data)?);
    println!("Part 2: {}", part_two(&data)?);

    Ok(())
}
