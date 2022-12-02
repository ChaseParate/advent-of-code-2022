use std::error::Error;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(2);

fn part_one(rounds: &Vec<(char, char)>) -> Result<u32, Box<dyn Error>> {
    let score = rounds
        .iter()
        .map(|(opponent_move, my_move)| {
            let mut round_score = match (opponent_move, my_move) {
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                _ => todo!(),
            };

            round_score += match my_move {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => todo!(),
            };

            round_score
        })
        .sum();

    Ok(score)
}

fn part_two(rounds: &Vec<(char, char)>) -> Result<u32, Box<dyn Error>> {
    let score = rounds
        .iter()
        .map(|(opponent_move, result)| {
            let my_move = match (opponent_move, result) {
                ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 'X',
                ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 'Y',
                ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 'Z',
                _ => todo!(),
            };

            let mut round_score = match my_move {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => todo!(),
            };

            round_score += match (opponent_move, my_move) {
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                _ => todo!(),
            };

            round_score
        })
        .sum();

    Ok(score)
}

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;
    let rounds: Vec<(char, char)> = puzzle_input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(' ').unwrap();
            (x.chars().next().unwrap(), y.chars().next().unwrap())
        })
        .collect();

    println!("Part 1: {}", part_one(&rounds)?);
    println!("Part 2: {}", part_two(&rounds)?);

    Ok(())
}
