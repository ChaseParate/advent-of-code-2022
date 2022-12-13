use std::{collections::HashMap, error::Error};

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(7);

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    ListFiles(Vec<File>),
}

#[derive(Debug)]
enum File {
    File(u32),
    Directory(String),
}

fn get_directory_size_map(commands: &[Command]) -> HashMap<Vec<&String>, u32> {
    let mut current_directory = Vec::new();
    let mut filesystem = HashMap::new();

    for command in commands {
        match command {
            Command::ChangeDirectory(directory) => {
                if directory == ".." {
                    current_directory.pop();
                } else {
                    current_directory.push(directory);
                }
            }
            Command::ListFiles(files) => {
                filesystem.insert(current_directory.clone(), files);
            }
        }
    }

    let mut directory_size_map = HashMap::new();
    for directory in filesystem.keys() {
        directory_size_map.insert(
            directory.clone(),
            get_directory_size(&filesystem, directory),
        );
    }

    directory_size_map
}

fn get_directory_size(
    filesystem: &HashMap<Vec<&String>, &Vec<File>>,
    directory: &Vec<&String>,
) -> u32 {
    let files = filesystem.get(directory).unwrap();

    let mut sum = 0;
    for file in *files {
        sum += match file {
            File::File(size) => *size,
            File::Directory(name) => {
                let mut new_dir = directory.clone();
                new_dir.push(name);
                get_directory_size(filesystem, &new_dir)
            }
        }
    }

    sum
}

fn part_one(commands: &[Command]) -> Result<u32, Box<dyn Error>> {
    let directory_size_map = get_directory_size_map(commands);

    let sum = directory_size_map
        .values()
        .filter(|size| **size <= 100_000)
        .sum();

    Ok(sum)
}

fn part_two(commands: &[Command]) -> Result<u32, Box<dyn Error>> {
    let directory_size_map = get_directory_size_map(commands);

    let root = String::from('/');
    let root_path = vec![&root];

    let total_space = 70000000;
    let total_used = directory_size_map.get(&root_path).unwrap();
    let total_unused = total_space - total_used;

    let space_needed = 30000000 - total_unused;

    let mut all_sizes: Vec<&u32> = directory_size_map
        .values()
        .filter(|size| **size >= space_needed)
        .collect();
    all_sizes.sort();

    let smallest_viable = all_sizes.first().unwrap();

    Ok(**smallest_viable)
}

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;

    let commands: Vec<&str> = puzzle_input.split('$').collect();
    let commands: Vec<Command> = commands[1..]
        .iter()
        .map(|command_string| {
            let (command_name, extra_stuff) = command_string.trim().split_at(2);
            match command_name {
                "cd" => Command::ChangeDirectory(extra_stuff.trim().to_owned()),
                "ls" => Command::ListFiles(
                    extra_stuff
                        .trim()
                        .lines()
                        .map(|line| {
                            let (kind, name) = line.split_once(' ').unwrap();
                            match kind {
                                "dir" => File::Directory(name.to_owned()),
                                size => File::File(size.parse().unwrap()),
                            }
                        })
                        .collect(),
                ),
                _ => unimplemented!(),
            }
        })
        .collect();

    println!("Part 1: {}", part_one(&commands)?);
    println!("Part 2: {}", part_two(&commands)?);

    Ok(())
}
