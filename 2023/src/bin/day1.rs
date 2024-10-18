//! advent of code day 1
//!
//! part 1:
//! Given a list of strings, find the first and last integer in each string, concatenate them, and sum them all up.
//!
//! part 2:
//! Given a list of strings, find the first and last integer in each string, concatenate them, and sum them all up.
//! This time, integers may be spelled out, e.g. "one" instead of "1".

use advent_of_code::Input;

const DIGITS_AND_SPELLED_OUT_INTS: [(&str, u32); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

// walks through the string, looking for integers, and recording them in an vec that
fn find_first_and_last_digit(s: &str) -> (u32, u32) {
    let first = s
        .chars()
        .find(char::is_ascii_digit)
        .expect("expect at least one digit in str")
        .to_digit(10)
        .unwrap();

    let last = s
        .chars()
        .rev()
        .find(char::is_ascii_digit)
        .expect("expect at least one digit in str")
        .to_digit(10)
        .unwrap();

    (first, last)
}

// looks for all substring matches of spelled out integers and integers in the string and records
// them in order as their integer values.
fn find_ints_and_spelled_out_ints(s: &str) -> (u32, u32) {
    let first = DIGITS_AND_SPELLED_OUT_INTS
        .iter()
        .filter_map(|(pattern, value)| s.find(pattern).map(|index| (index, *value)))
        .min_by_key(|(index, _)| *index)
        .map(|(_, value)| value)
        .expect("expect at least one digit in str");

    let last = DIGITS_AND_SPELLED_OUT_INTS
        .iter()
        .filter_map(|(pattern, value)| s.rfind(pattern).map(|index| (index, *value)))
        .max_by_key(|(index, _)| *index)
        .map(|(_, value)| value)
        .expect("expect at least one digit in str");

    (first, last)
}

// Concatenates the first and last integer in a pair.
// For example, if the first integer is 1 and the last integer is 2, the result is 12.
fn concat_int_pair((first, last): (u32, u32)) -> u32 {
    first * 10 + last
}

fn main() {
    let input = Input::new(1);
    let solution1 = input
        .lines()
        .iter()
        .map(String::as_str)
        .map(find_first_and_last_digit)
        .map(concat_int_pair)
        .sum::<u32>();
    println!("solution 1: {solution1}");

    let solution2 = input
        .lines()
        .iter()
        .map(String::as_str)
        .map(find_ints_and_spelled_out_ints)
        .map(concat_int_pair)
        .sum::<u32>();
    println!("solution 2: {solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let sum = input
            .into_iter()
            .map(find_first_and_last_digit)
            .map(concat_int_pair)
            .sum::<u32>();
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_example_2() {
        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let sum = input
            .into_iter()
            .map(find_ints_and_spelled_out_ints)
            .map(concat_int_pair)
            .sum::<u32>();
        assert_eq!(sum, 281);
    }

    #[test]
    fn test_solution_1() {
        let input = Input::new(1);
        let solution = input
            .lines()
            .iter()
            .map(|string| string.as_str())
            .map(find_first_and_last_digit)
            .map(concat_int_pair)
            .sum::<u32>();
        assert_eq!(solution, 53080);
    }

    #[test]
    fn test_solution_2() {
        let input = Input::new(1);
        let solution = input
            .lines()
            .iter()
            .map(|string| string.as_str())
            .map(find_ints_and_spelled_out_ints)
            .map(concat_int_pair)
            .sum::<u32>();
        assert_eq!(solution, 53268);
    }

    #[test]
    fn test_eighthree() {
        let pair = find_ints_and_spelled_out_ints("eighthree");
        let value = concat_int_pair(pair);
        assert_eq!(value, 83);
    }

    #[test]
    fn test_sevenine() {
        let pair = find_ints_and_spelled_out_ints("sevenine");
        let value = concat_int_pair(pair);
        assert_eq!(value, 79);
    }

    #[test]
    fn test_nineight() {
        let pair = find_ints_and_spelled_out_ints("nineight");
        let value = concat_int_pair(pair);
        assert_eq!(value, 98);
    }

    #[test]
    fn test_one_spelled_out() {
        let pair = find_ints_and_spelled_out_ints("one");
        let value = concat_int_pair(pair);
        assert_eq!(value, 11);
    }

    #[test]
    fn test_two_spelled_out_with_middle() {
        let pair = find_ints_and_spelled_out_ints("two3two");
        let value = concat_int_pair(pair);
        assert_eq!(value, 22);
    }
}
