//! Part 1
//! ---
//! Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.
//!
//! However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).
//!
//! For example, consider the following list of section assignment pairs:
//!
//! ```txt
//! 2-4,6-8
//! 2-3,4-5
//! 5-7,7-9
//! 2-8,3-7
//! 6-6,4-6
//! 2-6,4-8
//! ```
//! For the first few pairs, this list means:
//!
//! Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
//! The Elves in the second pair were each assigned two sections.
//! The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
//! This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:
//!
//! ```txt
//! .234.....  2-4
//! .....678.  6-8
//!
//! .23......  2-3
//! ...45....  4-5
//!
//! ....567..  5-7
//! ......789  7-9
//!
//! .2345678.  2-8
//! ..34567..  3-7
//!
//! .....6...  6-6
//! ...456...  4-6
//!
//! .23456...  2-6
//! ...45678.  4-8
//! ```
//! Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.
//!
//! In how many assignment pairs does one range fully contain the other?
//!
//! Part 2
//! ---
//! It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that overlap at all.
//!
//! In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:
//!
//! 5-7,7-9 overlaps in a single section, 7.
//! 2-8,3-7 overlaps all of the sections 3 through 7.
//! 6-6,4-6 overlaps in a single section, 6.
//! 2-6,4-8 overlaps in sections 4, 5, and 6.
//! So, in this example, the number of overlapping assignment pairs is 4.
//!
//! In how many assignment pairs do the ranges overlap?

use advent_of_code::input::Input;

type Range = (u32, u32);

fn range_is_subset_of_range(range1: Range, range2: Range) -> bool {
    range1.0 >= range2.0 && range1.1 <= range2.1 || range1.0 <= range2.0 && range1.1 >= range2.1
}

fn split_lines_into_range_pairs(lines: &[String]) -> Vec<(Range, Range)> {
    let mut range_pairs = Vec::new();
    for line in lines {
        // Split each line into two using ',', then split each half using '-'.
        let (left, right) = line
            .split_once(',')
            .expect("expect each line to have a ','");
        let (l1, l2) = left
            .split_once('-')
            .expect("expect each left half to have a '-'");
        let (r1, r2) = right
            .split_once('-')
            .expect("expect each right half to have a '-'");
        let first_range = (
            l1.parse::<u32>().expect("expect l1 to be a number"),
            l2.parse::<u32>().expect("expect l2 to be a number"),
        );
        let second_range = (
            r1.parse::<u32>().expect("expect r1 to be a number"),
            r2.parse::<u32>().expect("expect r2 to be a number"),
        );
        range_pairs.push((first_range, second_range));
    }
    range_pairs
}

fn count_subset_pairs(range_pairs: &Vec<(Range, Range)>) -> u32 {
    let mut count = 0;
    for range_pair in range_pairs {
        if range_is_subset_of_range(range_pair.0, range_pair.1) {
            count += 1;
        }
    }
    count
}

fn ranges_overlap(range1: Range, range2: Range) -> bool {
    range1.0 <= range2.1 && range1.1 >= range2.0
}

fn count_overlapping_pairs(range_pairs: &Vec<(Range, Range)>) -> u32 {
    let mut count = 0;
    for range_pair in range_pairs {
        if ranges_overlap(range_pair.0, range_pair.1) {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = Input::new(4);
    let range_pairs = split_lines_into_range_pairs(input.lines());
    // Count the number of range_pairs which are subsets.
    let subset_count = count_subset_pairs(&range_pairs);
    println!("{subset_count}");

    let overlapping_count = count_overlapping_pairs(&range_pairs);
    println!("{overlapping_count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subset_count_test() {
        let lines = vec![
            String::from("2-4,6-8"),
            String::from("2-3,4-5"),
            String::from("5-7,7-9"),
            String::from("2-8,3-7"),
            String::from("6-6,4-6"),
            String::from("2-6,4-8"),
        ];
        let range_pairs = split_lines_into_range_pairs(&lines);
        let subset_count = count_subset_pairs(&range_pairs);
        assert_eq!(subset_count, 2);
    }

    #[test]

    fn overlap_count_test() {
        let lines = vec![
            String::from("2-4,6-8"),
            String::from("2-3,4-5"),
            String::from("5-7,7-9"),
            String::from("2-8,3-7"),
            String::from("6-6,4-6"),
            String::from("2-6,4-8"),
        ];
        let range_pairs = split_lines_into_range_pairs(&lines);
        let overlapping_count = count_overlapping_pairs(&range_pairs);
        assert_eq!(overlapping_count, 4);
    }
}
