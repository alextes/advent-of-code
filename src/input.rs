use std::fs;

fn get_input(day: &i32) -> String {
    let path = format!("input/input{day}.txt");
    fs::read_to_string(path).unwrap()
}

pub fn get_input_lines(day: &i32) -> Vec<String> {
    get_input(day).split('\n').map(String::from).collect()
}
