use std::{fmt::Display, str::FromStr};

use advent_of_code::input;

use Outcome::*;
use Shape::*;

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Draw,
    Lose,
    Win,
}

impl Outcome {
    fn get_outcome_score(&self) -> u32 {
        match self {
            Self::Draw => 3,
            Self::Lose => 0,
            Self::Win => 6,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOutcomeError;

impl FromStr for Outcome {
    type Err = ParseOutcomeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(ParseOutcomeError),
        }
    }
}

impl Shape {
    fn shape_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play_against(&self, opponent_shape: Shape) -> Outcome {
        use Outcome::*;
        use Shape::*;

        match (self, opponent_shape) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Paper) => Draw,
            (Paper, Rock) => Win,
            (Paper, Scissors) => Lose,
            (Scissors, Scissors) => Draw,
            (Scissors, Paper) => Win,
            (Scissors, Rock) => Lose,
        }
    }

    fn desired_shape(&self, desired_outcome: &Outcome) -> Self {
        match (self, desired_outcome) {
            (Rock, Draw) => Rock,
            (Rock, Lose) => Scissors,
            (Rock, Win) => Paper,
            (Paper, Draw) => Paper,
            (Paper, Lose) => Rock,
            (Paper, Win) => Scissors,
            (Scissors, Draw) => Scissors,
            (Scissors, Lose) => Paper,
            (Scissors, Win) => Rock,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseShapeError;

impl FromStr for Shape {
    type Err = ParseShapeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(ParseShapeError),
        }
    }
}

impl Display for ParseShapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "expect shape string to be one of 'A', 'B', 'C', 'X', 'Y', 'Z'"
        )
    }
}

/// Uses the interpretation where both columns represent shapes played.
fn get_score_method_one() -> u32 {
    let lines = input::get_input_lines(&2);

    let mut score = 0;
    for line in lines {
        let splits = line.split(' ').collect::<Vec<&str>>();
        let opponent_shape: Shape = splits[0].parse().unwrap();
        let our_shape: Shape = (splits[1]).parse().unwrap();

        let shape_score = our_shape.shape_score();
        let outcome_score = our_shape.play_against(opponent_shape).get_outcome_score();

        score = score + shape_score + outcome_score;
    }

    score
}

/// Uses the interpretation where the first column is the shape played but the second is the
/// outcome desired.
fn get_score_method_two() -> u32 {
    let lines = input::get_input_lines(&2);

    let mut score = 0;
    for line in lines {
        let splits = line.split(' ').collect::<Vec<&str>>();
        let opponent_shape: Shape = splits[0].parse().unwrap();
        let desired_outcome: Outcome = splits[1].parse().unwrap();
        let our_shape = opponent_shape.desired_shape(&desired_outcome);

        let shape_score = our_shape.shape_score();
        let outcome_score = our_shape.play_against(opponent_shape).get_outcome_score();

        score = score + shape_score + outcome_score;
    }

    score
}

fn main() {
    let score_one = get_score_method_one();
    println!("answer1: {}", score_one);

    let score_two = get_score_method_two();
    println!("answer2: {}", score_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_score_one_test() {
        let score_one = get_score_method_one();

        assert_eq!(11449, score_one);
    }

    #[test]
    fn get_score_two_test() {
        let score = get_score_method_two();

        assert_eq!(13187, score);
    }
}
