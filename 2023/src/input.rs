//! Responsible for helping us read puzzle inputs.

use std::fs;

pub struct Input {
    lines: Vec<String>,
}

impl Input {
    pub fn new(day: u8) -> Self {
        let path = format!("input/day{day}.txt",);
        let contents = fs::read_to_string(path).expect("expect input.txt file to exist at path");

        let lines_string = contents
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        // Drop the terminating newline.
        let lines = lines_string[..lines_string.len() - 1].to_vec();

        Input { lines }
    }

    pub fn new_example(day: u8, example: u8) -> Self {
        let path = format!("input/day{day}_example{example}.txt",);
        let contents = fs::read_to_string(path).expect("expect input.txt file to exist at path");

        let lines_string = contents
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        // Drop the terminating newline.
        let lines = lines_string[..lines_string.len() - 1].to_vec();

        Input { lines }
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn raw_string(&self) -> String {
        self.lines.join("\n")
    }
}
