use std::error::Error;
use std::str::FromStr;

use advent_of_code_2022::Puzzle;

const PUZZLE: Puzzle = Puzzle::new(2);

#[derive(PartialEq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
impl FromStr for Move {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, <Self as FromStr>::Err> {
        match string {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(String::from("Invalid character given")),
        }
    }
}
impl Move {
    fn play_round(&self, other: &Move) -> RoundOutcome {
        use Move::*;
        use RoundOutcome::*;

        if self == other {
            return Draw;
        }

        let wins_against = match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        };

        if other == &wins_against {
            Win
        } else {
            Lose
        }
    }
}

#[derive(Debug)]
enum RoundOutcome {
    Win,
    Draw,
    Lose,
}
impl FromStr for RoundOutcome {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, <Self as FromStr>::Err> {
        match string {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(String::from("Invalid character given")),
        }
    }
}

fn part_one(rounds: &[(Move, char)]) -> Result<u32, Box<dyn Error>> {
    use Move::*;
    use RoundOutcome::*;

    let score = rounds
        .iter()
        .map(|(opponent_move, my_move)| {
            let my_move: Move = String::from(*my_move).parse().unwrap();

            let mut round_score = match my_move {
                Rock => 1,
                Paper => 2,
                Scissors => 3,
            };
            round_score += match my_move.play_round(opponent_move) {
                Lose => 0,
                Draw => 3,
                Win => 6,
            };

            round_score
        })
        .sum();

    Ok(score)
}

fn part_two(rounds: &[(Move, char)]) -> Result<u32, Box<dyn Error>> {
    use Move::*;
    use RoundOutcome::*;

    let score = rounds
        .iter()
        .map(|(opponent_move, result)| {
            let result: RoundOutcome = String::from(*result).parse().unwrap();
            let my_move = match (opponent_move, result) {
                (Rock, Draw) | (Paper, Lose) | (Scissors, Win) => Rock,
                (Rock, Win) | (Paper, Draw) | (Scissors, Lose) => Paper,
                (Rock, Lose) | (Paper, Win) | (Scissors, Draw) => Scissors,
            };

            let mut round_score = match my_move {
                Rock => 1,
                Paper => 2,
                Scissors => 3,
            };
            round_score += match my_move.play_round(opponent_move) {
                Lose => 0,
                Draw => 3,
                Win => 6,
            };

            round_score
        })
        .sum();

    Ok(score)
}

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = PUZZLE.get_input()?;
    let rounds: Vec<(Move, char)> = puzzle_input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(' ').unwrap();
            (String::from(x).parse().unwrap(), y.chars().next().unwrap())
        })
        .collect();

    println!("Part 1: {}", part_one(&rounds)?);
    println!("Part 2: {}", part_two(&rounds)?);

    Ok(())
}
