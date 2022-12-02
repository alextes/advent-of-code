//! Responsible for helping us read puzzle inputs.

use std::fs;

fn get_input(day: &u32) -> String {
    let path = format!("input/input{day}.txt");
    fs::read_to_string(path).unwrap()
}

/// Turns the requested input file into a set of lines.
///
/// The files with have the usual terminating newline, which for our purposes is not input, and
/// therefore dropped.
pub fn get_input_lines(day: &u32) -> Vec<String> {
    let lines: Vec<_> = get_input(day).split('\n').map(String::from).collect();
    let (_last, rest) = lines.split_last().unwrap();
    rest.to_vec()
}
