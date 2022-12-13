use std::collections::LinkedList;
use std::error::Error;

use regex::Regex;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(5);
const NUM_STACKS: usize = 9;

#[derive(Debug)]
struct Instruction {
    amount: u32,
    source: u32,
    destination: u32,
}

fn parse_input(input: &str) -> (Vec<LinkedList<char>>, Vec<Instruction>) {
    let (stack_lines, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<LinkedList<char>> = vec![LinkedList::new(); NUM_STACKS];

    let stack_lines: Vec<&str> = stack_lines.lines().collect();
    stack_lines[..stack_lines.len() - 1]
        .iter()
        .map(|line| line.chars().collect())
        .for_each(|line: Vec<char>| {
            for stack_index in 0..NUM_STACKS {
                let stack = stacks.get_mut(stack_index).unwrap();
                let char_index = 4 * stack_index + 1;
                if let Some(char) = line.get(char_index) {
                    if !char.is_whitespace() {
                        stack.push_front(*char);
                    }
                }
            }
        });

    let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            Instruction {
                amount: captures.get(1).unwrap().as_str().parse().unwrap(),
                source: captures.get(2).unwrap().as_str().parse().unwrap(),
                destination: captures.get(3).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect();

    (stacks, instructions)
}

fn part_one(
    stacks: &mut [LinkedList<char>],
    instructions: &[Instruction],
) -> Result<String, Box<dyn Error>> {
    for instruction in instructions {
        let mut items_to_move = Vec::with_capacity(instruction.amount as usize);

        let source_stack = stacks.get_mut((instruction.source - 1) as usize).unwrap();
        for _ in 0..instruction.amount {
            items_to_move.push(source_stack.pop_back().unwrap());
        }

        let destination_stack = stacks
            .get_mut((instruction.destination - 1) as usize)
            .unwrap();
        for item in items_to_move {
            destination_stack.push_back(item);
        }
    }

    let mut output = String::new();
    for stack in stacks {
        output.push(*stack.back().unwrap());
    }

    Ok(output)
}

fn part_two(
    stacks: &mut [LinkedList<char>],
    instructions: &[Instruction],
) -> Result<String, Box<dyn Error>> {
    for instruction in instructions {
        let mut items_to_move = Vec::new();

        let source_stack = stacks.get_mut((instruction.source - 1) as usize).unwrap();
        for _ in 0..instruction.amount {
            items_to_move.push(source_stack.pop_back().unwrap());
        }

        items_to_move.reverse();

        let destination_stack = stacks
            .get_mut((instruction.destination - 1) as usize)
            .unwrap();
        for item in items_to_move {
            destination_stack.push_back(item);
        }
    }

    let mut output = String::new();
    for stack in stacks {
        output.push(*stack.back().unwrap());
    }

    Ok(output)
}

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;

    let (mut stacks, instructions) = parse_input(&puzzle_input);

    println!("Part 1: {}", part_one(&mut stacks.clone(), &instructions)?);
    println!("Part 2: {}", part_two(&mut stacks, &instructions)?);

    Ok(())
}
