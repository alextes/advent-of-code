//! Advent of Code Day 6
//! To be able to communicate with the Elves, the device needs to lock on to their signal. The signal is a series of seemingly-random characters that the device receives one at a time.
//!
//! To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.
//!
//! The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.
//!
//! For example, suppose you receive the following datastream buffer:
//!
//! mjqjpqmgbljsphdztnvjfqwrcgsmlb
//! After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj. Because j is repeated, this isn't a marker.
//!
//! The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm, which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been processed.
//!
//! Here are a few more examples:
//!
//! bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
//! nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
//! nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
//! zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
//!
//! A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.
//!
//! Here are the first positions of start-of-message markers for all of the above examples:
//!
//! mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
//! bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
//! nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
//! nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
//! zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26

use std::collections::HashSet;

fn find_marker(input: &str, size: usize) -> usize {
    let mut count = 1;
    let mut last = 0;
    let mut last4 = vec![0; size];
    for c in input.chars() {
        last4[last] = c as u8;
        last = (last + 1) % size;

        // Create a hashset from the last 4 characters.
        let set = last4.iter().cloned().collect::<HashSet<_>>();

        // If the hashset has 4 elements, we have a marker. The first marker can only appear after
        // the first 4 characters, so we can safely ignore the first 3 characters.
        if set.len() == size && count >= size {
            break;
        }

        count += 1;
    }

    count
}

fn find_start_of_packet(input: &str) -> usize {
    find_marker(input, 4)
}

fn find_start_of_message(input: &str) -> usize {
    find_marker(input, 14)
}

fn main() {
    let input = include_str!("../../input/day6.txt");
    println!("First marker after {}", find_start_of_packet(input));
    println!("First marker after {}", find_start_of_message(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_1() {
        assert_eq!(find_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(
            find_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
        assert_eq!(find_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_day6_2() {
        assert_eq!(find_start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_start_of_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(
            find_start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
        assert_eq!(
            find_start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            26
        );
    }
}
