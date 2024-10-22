//! # Day 4: Scratchcards
//! The Elf leads you over to the pile of colorful cards. There, you discover dozens of
//! scratchcards, all with their opaque covering already scratched off. Picking one up, it looks
//! like each card has two lists of numbers separated by a vertical bar (|): a list of winning
//! numbers and then a list of numbers you have. You organize the information into a table (your
//! puzzle input).
//!
//! As far as the Elf has been able to figure out, you have to figure out which of the numbers you
//! have appear in the list of winning numbers. The first match makes the card worth one point and
//! each match after the first doubles the point value of that card.
//! --- Part Two ---
//! Just as you're about to report your findings to the Elf, one of you realizes that the rules
//! have actually been printed on the back of every card this whole time.
//!
//! There's no such thing as "points". Instead, scratchcards only cause you to win more
//! scratchcards equal to the number of winning numbers you have.
//!
//! Specifically, you win copies of the scratchcards below the winning card equal to the number of
//! matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards
//! 11, 12, 13, 14, and 15.
//!
//! Copies of scratchcards are scored like normal scratchcards and have the same card number as the
//! card they copied. So, if you win a copy of card 10 and it has 5 matching numbers, it would then
//! win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This
//! process repeats until none of the copies cause you to win any more cards. (Cards will never
//! make you copy a card past the end of the table.)

use std::collections::{HashMap, HashSet};

fn calc_winning_cards_score(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let _card_title = line.split(':').next().unwrap();
        let card_contents = line.split(':').last().unwrap().trim_start();
        let mut winning_numbers = card_contents.split('|');
        let winning_numbers = winning_numbers.next().unwrap();
        let winning_numbers: HashSet<u8> = winning_numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let candidate_numbers = card_contents.split('|').last().unwrap();
        let candidate_numbers: Vec<u8> = candidate_numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let winning_count = candidate_numbers.iter().fold(0, |acc, number| {
            if winning_numbers.contains(number) {
                acc + 1
            } else {
                acc
            }
        });

        let score = if winning_count > 0 {
            2_u32.pow(winning_count - 1)
        } else {
            0
        };

        sum += score;
    }

    sum
}

fn calc_final_card_count(input: &str) -> u32 {
    let mut card_counts = HashMap::new();

    for line in input.lines() {
        let card_title = line.split(':').next().unwrap();
        let card_number: u32 = card_title
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let card_contents = line.split(':').last().unwrap().trim_start();
        let mut winning_numbers = card_contents.split('|');
        let winning_numbers = winning_numbers.next().unwrap();
        let winning_numbers: HashSet<u8> = winning_numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let candidate_numbers = card_contents.split('|').last().unwrap();
        let candidate_numbers: Vec<u8> = candidate_numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let winning_count = candidate_numbers.iter().fold(0, |acc, number| {
            if winning_numbers.contains(number) {
                acc + 1
            } else {
                acc
            }
        });

        // Every original counts as a copy. Add it to our list of counts.
        card_counts
            .entry(card_number)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        if winning_count == 0 {
            continue;
        }

        // Number of copies we won depends on how many copies we have of the card number we
        // matched from.
        let count_won = *card_counts.get(&card_number).unwrap_or(&0);

        for i in 1..=winning_count {
            // Calculate the card number we're winning copies of.
            let won_card_number = card_number + i;

            // Increment the card count for the copy.
            card_counts
                .entry(won_card_number)
                .and_modify(|count| *count += count_won)
                .or_insert(count_won);
        }
    }

    card_counts.values().sum()
}

pub fn main() {
    println!("Day 4: Scratchcards");

    let input = include_str!("../../input/day4.txt");

    let solution = calc_winning_cards_score(input);
    println!("The total score of the winning cards is: {solution}");

    let solution = calc_final_card_count(input);
    println!("The total number of cards won is: {solution}");
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_1: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_example_1() {
        let solution = calc_winning_cards_score(EXAMPLE_1);

        assert_eq!(solution, 13);
    }

    #[test]
    fn test_solution_1() {
        let input = include_str!("../../input/day4.txt");

        let solution = calc_winning_cards_score(input);

        assert_eq!(solution, 25231);
    }

    #[test]
    fn test_example_2() {
        let solution = calc_final_card_count(EXAMPLE_1);

        assert_eq!(solution, 30);
    }

    #[test]
    fn test_solution_2() {
        let input = include_str!("../../input/day4.txt");

        let solution = calc_final_card_count(input);

        assert_eq!(solution, 9721255);
    }
}
