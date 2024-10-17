//! Part 1
//! Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
//!
//! The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).
//!
//! The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.
//!
//! For example, suppose you have the following list of contents from six rucksacks:
//!
//! ```txt
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ```
//! The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
//! The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
//! The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
//! The fourth rucksack's compartments only share item type v.
//! The fifth rucksack's compartments only share item type t.
//! The sixth rucksack's compartments only share item type s.
//! To help prioritize item rearrangement, every item type can be converted to a priority:
//!
//! Lowercase item types a through z have priorities 1 through 26.
//! Uppercase item types A through Z have priorities 27 through 52.
//! In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
//!
//! Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
//!
//! Part 2
//! The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.
//!
//! Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:
//!
//! ```txt
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! ```
//! And the second group's rucksacks are the next three lines:
//!
//! ```txt
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ```
//! In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.

use std::collections::HashSet;

use advent_of_code::input::Input;

#[derive(Debug)]
struct AlphabetOrdinal(u8);

impl From<&char> for AlphabetOrdinal {
    fn from(c: &char) -> Self {
        if c.is_lowercase() {
            // Encoded as utf8, in decimal 'a' is encoded 97.
            let pos = *c as u8 - 96;
            AlphabetOrdinal(pos)
        } else {
            // encoded as utf8, in decimal 'A' is encoded 65.
            let pos = *c as u8 - 65 + 27;
            AlphabetOrdinal(pos)
        }
    }
}

impl From<AlphabetOrdinal> for u32 {
    fn from(alphabet_ordinal: AlphabetOrdinal) -> Self {
        alphabet_ordinal.0 as u32
    }
}

fn get_sum_method_one(lines: &[String]) -> u32 {
    let mut errors = vec![];

    // Split each line into two halves based on the length of the vector.
    for line in lines {
        let compartment_one_chars = &line[0..line.len() / 2];
        let compartment_two_chars = &line[line.len() / 2..].chars().collect::<HashSet<_>>();
        for char in compartment_one_chars.chars() {
            if compartment_two_chars.contains(&char) {
                errors.push(char);
                break;
            }
        }
    }

    // Convert the errors to their ordinal value, cast them to u32 and sum them.
    errors
        .iter()
        .map(|c| u32::from(AlphabetOrdinal::from(c)))
        .sum()
}

fn get_sum_method_two(lines: &[String]) -> u32 {
    // Store matches in a vec.
    let mut matches = vec![];

    // Split the lines into groups of three.
    let groups = lines.chunks(3);
    for group in groups {
        // Find the characters that are common to all three lines.
        // Turn the first two lines into HashSet's.
        let first_line_chars = group[0].chars().collect::<HashSet<_>>();
        let second_line_chars = group[1].chars().collect::<HashSet<_>>();

        // Now check each char in the third line for matches in both sets.
        for char in group[2].chars() {
            if first_line_chars.contains(&char) && second_line_chars.contains(&char) {
                matches.push(char);
                break;
            }
        }
    }

    // Turn the matches into their ordinal value, cast them to u32 and sum them.
    matches
        .iter()
        .map(|c| u32::from(AlphabetOrdinal::from(c)))
        .sum()
}

fn main() {
    let input = Input::new(3);

    let sum_one = get_sum_method_one(input.lines());
    println!("answer1: {sum_one}");

    let sum_two = get_sum_method_two(input.lines());
    println!("answer2: {sum_two}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_method_one_test() {
        let input = Input::new(3);

        let sum_one = get_sum_method_one(input.lines());

        assert_eq!(8085, sum_one);
    }

    #[test]
    fn sum_method_two_test() {
        let input = Input::new(3);

        let sum_two = get_sum_method_two(input.lines());

        assert_eq!(2515, sum_two);
    }
}
