use std::error::Error;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(3);

fn get_item_priority(item: &char) -> u32 {
    let code_point = *item as u32;
    match item {
        ('a'..='z') => code_point - 96,
        ('A'..='Z') => (code_point - 64) + 26,
        _ => 0,
    }
}

fn part_one(rucksacks: &[String]) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;

    for sack in rucksacks {
        let (first_half, second_half) = sack.split_at(sack.len() / 2);

        let c1: Vec<char> = first_half.chars().collect();
        let c2: Vec<char> = second_half.chars().collect();

        for char in c1 {
            if c2.contains(&char) {
                sum += get_item_priority(&char);
                break;
            }
        }
    }

    Ok(sum)
}

fn part_two(rucksacks: &[String]) -> Result<u32, Box<dyn Error>> {
    let mut iter = rucksacks.iter();

    let mut sum = 0;

    while let (Some(s1), Some(s2), Some(s3)) = (iter.next(), iter.next(), iter.next()) {
        for char in s1.chars() {
            if s2.chars().any(|x| x == char) && s3.chars().any(|x| x == char) {
                sum += get_item_priority(&char);
                break;
            }
        }
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;
    let rucksacks: Vec<String> = puzzle_input
        .lines()
        .map(|line| String::from(line.trim()))
        .collect();

    println!("Part 1: {}", part_one(&rucksacks)?);
    println!("Part 2: {}", part_two(&rucksacks)?);

    Ok(())
}
