//! Responsible for helping us read puzzle inputs.

use std::{fmt::Display, fs};

pub enum FirstOrSecond {
    First,
    Second,
}

impl Display for FirstOrSecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirstOrSecond::First => write!(f, "first"),
            FirstOrSecond::Second => write!(f, "second"),
        }
    }
}

pub struct Input {
    lines: Vec<String>,
}

impl Input {
    pub fn new(day: u8, first_or_second: FirstOrSecond) -> Self {
        let path = format!("inputs/day{day}/input_{first_or_second}.txt",);
        let contents = fs::read_to_string(path).expect("expect input.txt file to exist at path");

        let lines_string = contents
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        // Drop the terminating newline.
        let lines = lines_string[..lines_string.len() - 1].to_vec();

        Input { lines }
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }
}
