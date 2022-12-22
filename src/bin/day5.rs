//! The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.
//!
//! The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.
//!
//! The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
//!
//! They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:
//!
//! ```txt
//!     [D]    
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! ```
//!
//! move 1 from 2 to 1
//! move 3 from 1 to 3
//! move 2 from 2 to 1
//! move 1 from 1 to 2
//! In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.
//!
//! Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
//!
//! ```txt
//! [D]        
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! ```
//!
//! In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:
//!
//! ```txt
//!         [Z]
//!         [N]
//!     [C] [D]
//!     [M] [P]
//!  1   2   3
//! ```
//! Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:
//!
//! ```txt
//!         [Z]
//!         [N]
//! [M]     [D]
//! [C]     [P]
//!  1   2   3
//! ```
//! Finally, one crate is moved from stack 1 to stack 2:
//!
//! ```txt
//!         [Z]
//!         [N]
//!         [D]
//! [C] [M] [P]
//!  1   2   3
//! ```
//! The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.
//!
//! --- Part Two ---
//! As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.
//!
//! Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.
//!
//! The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.
//!
//! Again considering the example above, the crates begin in the same configuration:
//!
//!     [D]    
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! Moving a single crate from stack 2 to stack 1 behaves the same as before:
//!
//! [D]        
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:
//!
//!         [D]
//!         [N]
//!     [C] [Z]
//!     [M] [P]
//!  1   2   3
//! Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:
//!
//!         [D]
//!         [N]
//! [C]     [Z]
//! [M]     [P]
//!  1   2   3
//! Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:
//!
//!         [D]
//!         [N]
//!         [Z]
//! [M] [C] [P]
//!  1   2   3
//! In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

/// Transpose a 2D vector.
fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed = vec![vec![' '; matrix.len()]; matrix[0].len()];

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            transposed[j][i] = *c;
        }
    }

    transposed
}

type Instruction = (u32, u32, u32);

fn parse_stacks(text: &str) -> Vec<Vec<char>> {
    // Split the text using newlines.
    let mut lines = text.split('\n').collect::<Vec<&str>>();

    // Drop the last line.
    lines.pop();

    // Split each line into a matrix of chars.
    let mut matrix = vec![];
    for line in lines {
        matrix.push(line.chars().collect::<Vec<char>>());
    }

    // Transpose the matrix into stacks.
    let mut stacks = transpose(matrix);

    // Drop the first item.
    stacks.remove(0);

    // Keep every fourth vector.
    stacks = stacks
        .iter()
        .step_by(4)
        .cloned()
        .collect::<Vec<Vec<char>>>();

    // Filter empty spaces.
    stacks = stacks
        .iter()
        .map(|stack| {
            stack
                .iter()
                .filter(|&c| *c != ' ')
                .cloned()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    // Reverse each stack so that the top is at the end. Without cloning.
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    stacks
}

fn parse_instructions(text: &str) -> Vec<Instruction> {
    // Parse the instructions. Split on empty space. The second part is the count. The fourth the
    // stack to move from, the sixth the stack to move to.
    text.lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();
            (
                parts[1].parse::<u32>().unwrap(),
                parts[3].parse::<u32>().unwrap(),
                parts[5].parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<Instruction>>()
}

fn execute_instructions_method_one(stacks: &mut [Vec<char>], instructions: &Vec<Instruction>) {
    // Move the crates according to the instructions.
    // The first number is the count to move.
    // The second is the stack to take from.
    // The third is the stack to move to.
    // The stacks are 0-indexed.
    for (count, from, to) in instructions {
        for _ in 0..*count {
            let c = stacks[*from as usize - 1].pop().unwrap();
            stacks[*to as usize - 1].push(c);
        }
    }
}

fn execute_instructions_method_two(stacks: &mut [Vec<char>], instructions: &Vec<Instruction>) {
    // Move the crates on the stacks according to the instructions.
    // The first number is the count to move.
    // The second is the stack to take from.
    // The third is the stack to move to.
    // The stacks are 0-indexed.
    // The crates move in groups.
    for (count, from, to) in instructions {
        let mut group = vec![];
        for _ in 0..*count {
            let c = stacks[*from as usize - 1].pop().unwrap();
            group.push(c);
        }
        group.reverse();
        stacks[*to as usize - 1].append(&mut group);
    }
}

fn main() {
    let input = include_str!("../../input/day5.txt");

    // Split the input into the stacks and the instructions using an empty newline.
    let (stacks_text, instructions_text) = input
        .split_once("\n\n")
        .expect("input to have two sections");

    let stacks = parse_stacks(stacks_text);
    let instructions = parse_instructions(instructions_text);

    // Clone the stacks so that we can execute the instructions on a copy.
    let mut stacks_one = stacks.clone();

    // Execute the instructions.
    execute_instructions_method_one(&mut stacks_one, &instructions);

    // Print the top crates.
    for stack in stacks_one {
        print!("{}", stack[stack.len() - 1]);
    }

    println!();

    // Execute the instructions according to method two.
    #[allow(clippy::redundant_clone)]
    let mut stacks_two = stacks.clone();
    execute_instructions_method_two(&mut stacks_two, &instructions);

    // Print the top crates.
    for stack in stacks_two {
        print!("{}", stack[stack.len() - 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_test() {
        let input = include_str!("../../input/day5-example.txt");

        // Split the input into the stacks and the instructions using an empty newline.
        let (stacks_text, instructions_text) = input
            .split_once("\n\n")
            .expect("input to have two sections");

        let mut stacks = parse_stacks(stacks_text);
        let instructions = parse_instructions(instructions_text);

        // Execute the instructions.
        execute_instructions_method_one(&mut stacks, &instructions);

        // Print the top crates.
        let mut result = String::new();
        for stack in stacks {
            result.push(stack[stack.len() - 1]);
        }

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn example_two_test() {
        let input = include_str!("../../input/day5-example.txt");

        // Split the input into the stacks and the instructions using an empty newline.
        let (stacks_text, instructions_text) = input
            .split_once("\n\n")
            .expect("input to have two sections");

        let mut stacks = parse_stacks(stacks_text);
        let instructions = parse_instructions(instructions_text);

        // Execute the instructions.
        execute_instructions_method_two(&mut stacks, &instructions);

        // Print the top crates.
        let mut result = String::new();
        for stack in stacks {
            result.push(stack[stack.len() - 1]);
        }

        assert_eq!(result, "MCD");
    }
}
