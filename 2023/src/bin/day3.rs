use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

/// Find all part numbers in a line.
/// Example line: `467..114..`
///               `...*......`
/// Only numbers with a symbol that is not a dot, to its left, top, right, or bottom are part numbers.
/// Part numbers are separated by dots. Here we would only have one part number `467`.
///
/// To avoid doing the same work over and over we break the problem into two parts. First we parse
/// every number, and record the coordinate of their digits.
/// Second we walk through our list of numbers and their coordinates, determine the coordinates of
/// each char around our number and check if any of them are a non-dot symbol.
fn find_part_numbers(input: &[&str]) -> Vec<u32> {
    #[derive(Debug)]
    struct Number {
        value: String,
        row: usize,
        column: usize,
    }

    let mut numbers: Vec<Number> = Vec::new();

    // Convert input to a grid of chars
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    // First part: Parse every number and record the coordinates of their digits
    for (i, line) in grid.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            if line[j].is_ascii_digit() {
                // Start collecting digits of the number
                let mut value = String::new();
                while j < line.len() && line[j].is_ascii_digit() {
                    value.push(line[j]);
                    j += 1;
                }
                numbers.push(Number {
                    row: i,
                    // After parsing the full number, we want to find the column of the first digit,
                    // but j will now be set to the starting index + the length of the number. That is,
                    // the column directly to the right of our number. We backtrack one step to get
                    // the correct column.
                    column: j - value.len(),
                    value,
                });
            } else {
                j += 1;
            }
        }
    }

    let mut part_numbers = Vec::new();

    // Second part: Check surrounding characters of each number's digits
    for number in numbers {
        let mut is_part_number = false;

        // Adjust start and end indices to avoid out-of-bounds access
        let start_row = if number.row > 0 {
            number.row - 1
        } else {
            number.row
        };
        let start_column = if number.column > 0 {
            number.column - 1
        } else {
            number.column
        };
        let end_row = (number.row + 1).min(grid.len() - 1);
        let end_column = (number.column + number.value.len()).min(grid[number.row].len() - 1);

        #[allow(clippy::needless_range_loop)]
        for i in start_row..=end_row {
            for j in start_column..=end_column {
                // Skip the digits of the number itself
                let is_digit_row = i == number.row;
                let is_within_digit_columns =
                    j >= number.column && j < number.column + number.value.len();
                if is_digit_row && is_within_digit_columns {
                    continue;
                }

                let current_char = grid[i][j];
                if current_char != '.' && !current_char.is_ascii_digit() {
                    is_part_number = true;
                }
            }
        }

        if is_part_number {
            part_numbers.push(u32::from_str(&number.value).unwrap());
        }
    }

    part_numbers
}

fn find_part_number_sum(input: &[&str]) -> u32 {
    find_part_numbers(input).into_iter().sum()
}

/// Find all numbers and their positions.
fn find_numbers_with_positions(input: &[&str]) -> (Vec<String>, HashMap<(usize, usize), usize>) {
    let mut numbers: Vec<String> = Vec::new();
    let mut position_to_number: HashMap<(usize, usize), usize> = HashMap::new();

    // Convert input to a grid of chars
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    // Parse every number and record the coordinates of their digits
    for (i, line) in grid.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            if line[j].is_ascii_digit() {
                // Start collecting digits of the number
                let mut value = String::new();
                let mut positions = Vec::new();
                while j < line.len() && line[j].is_ascii_digit() {
                    value.push(line[j]);
                    positions.push((i, j));
                    j += 1;
                }
                let index = numbers.len();
                for &pos in &positions {
                    position_to_number.insert(pos, index);
                }
                numbers.push(value);
            } else {
                j += 1;
            }
        }
    }

    (numbers, position_to_number)
}

/// Find the sum of the products of all gear ratios.
/// Gear ratios are two numbers that share a `*` symbol.
fn find_gear_ratio_sum(input: &[&str]) -> u32 {
    let (numbers, position_to_number) = find_numbers_with_positions(input);

    // Get all gear positions
    let gear_positions: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, c)| if c == '*' { Some((i, j)) } else { None })
        })
        .collect();

    let mut total = 0;

    for (i, j) in gear_positions {
        // For each gear, find all adjacent positions (including diagonals)
        let directions = [
            (-1, 0),  // up
            (1, 0),   // down
            (0, -1),  // left
            (0, 1),   // right
            (-1, -1), // up-left
            (-1, 1),  // up-right
            (1, -1),  // down-left
            (1, 1),   // down-right
        ];

        let mut adjacent_numbers = HashSet::new();

        for &(di, dj) in &directions {
            let adj_i = i as isize + di;
            let adj_j = j as isize + dj;
            if adj_i >= 0
                && adj_i < input.len() as isize
                && adj_j >= 0
                && adj_j < input[adj_i as usize].len() as isize
            {
                let adj_i = adj_i as usize;
                let adj_j = adj_j as usize;
                if let Some(&number_index) = position_to_number.get(&(adj_i, adj_j)) {
                    adjacent_numbers.insert(number_index);
                }
            }
        }

        if adjacent_numbers.len() == 2 {
            let indices: Vec<_> = adjacent_numbers.into_iter().collect();
            let val1 = numbers[indices[0]].parse::<u32>().unwrap();
            let val2 = numbers[indices[1]].parse::<u32>().unwrap();
            total += val1 * val2;
        }
    }

    total
}

fn main() {
    let input = include_str!("../../input/day3.txt");
    let lines: Vec<&str> = input.lines().collect();
    let solution = find_part_number_sum(&lines);
    println!("Solution 1: {}", solution);

    let solution = find_gear_ratio_sum(&lines);
    println!("Solution 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn test_example1() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let lines: Vec<&str> = input.lines().collect();
        let solution = find_part_number_sum(&lines);
        assert_eq!(solution, 4361);
    }

    #[test]
    fn test_with_diagonals() {
        let input = indoc! {"
           ..1..
           .#...
           .....
        "};
        let lines: Vec<&str> = input.lines().collect();
        let solution = find_part_number_sum(&lines);
        assert_eq!(solution, 1);
    }

    #[test]
    fn test_partial_input_1() {
        let input = indoc! {"
        ...............................930...................................283................
        ....=.........370...........................48..456......424...-.341*.....554...*807.571
        ..159.........../..........539*.....73......-...*.......+....954.........*.....7.......*
        "};
        let lines: Vec<&str> = input.lines().collect();
        let solution = find_part_number_sum(&lines);
        assert_eq!(solution, 5513);
    }

    #[test]
    fn test_solution_1() {
        let input = include_str!("../../input/day3.txt");
        let lines: Vec<&str> = input.lines().collect();
        let solution = find_part_number_sum(&lines);
        assert_eq!(solution, 536576);
    }

    #[test]
    fn test_example2() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let lines: Vec<&str> = input.lines().collect();
        let solution = find_gear_ratio_sum(&lines);
        assert_eq!(solution, 467835);
    }

    #[test]
    fn test_solution_2() {
        let input = include_str!("../../input/day3.txt");
        let lines: Vec<&str> = input.lines().collect();
        let solution = find_gear_ratio_sum(&lines);
        assert_eq!(solution, 75741499);
    }

    #[test]
    fn test_multi_cell_adjacent() {
        let input = indoc! {"
            12.
            *12
        "};
        let lines: Vec<&str> = input.lines().collect();
        let solution = find_gear_ratio_sum(&lines);
        assert_eq!(solution, 144);
    }
}
