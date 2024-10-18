//! Day 2: Cube Conundrum

use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Default)]
struct RevealedSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct MaxCubeConstraint {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    revealed_sets: Vec<RevealedSet>,
}

impl Game {
    fn is_possible(&self, max_cube_constraint: &MaxCubeConstraint) -> bool {
        self.revealed_sets.iter().all(|revealed_set| {
            revealed_set.red <= max_cube_constraint.red
                && revealed_set.green <= max_cube_constraint.green
                && revealed_set.blue <= max_cube_constraint.blue
        })
    }

    fn max_color(&self, color: &str) -> u32 {
        self.revealed_sets
            .iter()
            .map(|revealed_set| match color {
                "red" => revealed_set.red,
                "green" => revealed_set.green,
                "blue" => revealed_set.blue,
                _ => 0,
            })
            .max()
            .unwrap_or(0)
    }

    fn calc_power(&self) -> u32 {
        self.max_color("red") * self.max_color("green") * self.max_color("blue")
    }
}

fn parse_game_id(line: &str) -> u32 {
    line.split(": ")
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_revealed_sets(line: &str) -> Vec<RevealedSet> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split("; ")
        .map(parse_revealed_set)
        .collect()
}

fn parse_revealed_set(revealed_set: &str) -> RevealedSet {
    let mut set = RevealedSet::default();
    for part in revealed_set.split(", ") {
        let (count, color) = parse_color_count(part);
        match color {
            "red" => set.red = count,
            "green" => set.green = count,
            "blue" => set.blue = count,
            _ => panic!("Unknown color: {color}"),
        }
    }
    set
}

fn parse_color_count(part: &str) -> (u32, &str) {
    let mut iter = part.split_whitespace();
    let count = iter.next().unwrap().parse().unwrap();
    let color = iter.next().unwrap();
    (count, color)
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let id = parse_game_id(line);
        let revealed_sets = parse_revealed_sets(line);
        Ok(Game { id, revealed_sets })
    }
}

fn solution1(input: &[&str], max_cube_constraint: &MaxCubeConstraint) -> u32 {
    input
        .iter()
        .map(|line| line.parse::<Game>().expect("expect valid game"))
        .filter(|game| game.is_possible(max_cube_constraint))
        .map(|game| game.id)
        .sum()
}

fn solution2(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| line.parse::<Game>().expect("expect valid game"))
        .map(|game| game.calc_power())
        .sum()
}

pub fn main() {
    println!("Day 2: Cube Conundrum");
    let input = include_str!("../../input/day2.txt")
        .lines()
        .collect::<Vec<&str>>();

    let solution_sum_1 = solution1(
        &input,
        &MaxCubeConstraint {
            red: 12,
            green: 13,
            blue: 14,
        },
    );
    println!("solution 1: {solution_sum_1}");

    let solution_sum_2 = solution2(&input);
    println!("solution 2: {solution_sum_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let max_cube_constraint = MaxCubeConstraint {
            red: 12,
            green: 13,
            blue: 14,
        };

        let solution = solution1(&input, &max_cube_constraint);

        assert_eq!(solution, 8);
    }

    #[test]
    fn test_solution_1() {
        let input = include_str!("../../input/day2.txt");
        let input = input.lines().collect::<Vec<&str>>();

        let max_cube_constraint = MaxCubeConstraint {
            red: 12,
            green: 13,
            blue: 14,
        };

        let solution = solution1(&input, &max_cube_constraint);

        assert_eq!(solution, 2176);
    }

    #[test]
    fn test_solution_2() {
        let input = include_str!("../../input/day2.txt");
        let input = input.lines().collect::<Vec<&str>>();

        let solution = solution2(&input);

        assert_eq!(solution, 63700);
    }
}
