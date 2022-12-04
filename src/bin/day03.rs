use std::collections::HashSet;
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

        let h1: HashSet<char> = first_half.chars().collect();
        let h2: HashSet<char> = second_half.chars().collect();

        let intersection_char = h1.intersection(&h2).next().unwrap();
        sum += get_item_priority(intersection_char);
    }

    Ok(sum)
}

fn part_two(rucksacks: &[String]) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;

    let mut iter = rucksacks.iter();
    while let (Some(s1), Some(s2), Some(s3)) = (iter.next(), iter.next(), iter.next()) {
        let h1: HashSet<char> = s1.chars().collect();
        let h2: HashSet<char> = s2.chars().collect();
        let h3: HashSet<char> = s3.chars().collect();

        let intersection: HashSet<char> = h1.intersection(&h2).copied().collect();
        let intersection_char = intersection.intersection(&h3).next().unwrap();
        sum += get_item_priority(intersection_char);
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
