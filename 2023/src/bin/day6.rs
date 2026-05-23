//! --- Day 6: Wait For It ---
//!
//! Part 1
//! As part of signing up, you get a sheet of paper (your puzzle input) that lists the time allowed
//! for each race and also the best distance ever recorded in that race. To guarantee you win the
//! grand prize, you need to make sure you go farther in each race than the current record holder.
//!
//! The organizer brings you over to the area where the boat races are held. The boats are much
//! smaller than you expected - they're actually toy boats, each with a big button on top. Holding
//! down the button charges the boat, and releasing the button allows the boat to move. Boats move
//! faster if their button was held longer, but time spent holding the button counts against the
//! total race time. You can only hold the button at the start of the race, and boats don't move
//! until the button is released.
//!
//! For example:
//!
//! Time:      7  15   30
//! Distance:  9  40  200
//! This document describes three races:
//!
//! The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
//! The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
//! The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.
//! Your toy boat has a starting speed of zero millimeters per millisecond. For each whole
//! millisecond you spend at the beginning of the race holding down the button, the boat's speed
//! increases by one millimeter per millisecond.
//!
//! So, because the first race lasts 7 milliseconds, you only have a few options:
//!
//! Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race.
//! The boat won't move; it will have traveled 0 millimeters by the end of the race.
//! Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at
//! a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled
//! of 6 millimeters.
//! Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond.
//! It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters.
//! Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat
//! will have gone 12 millimeters.
//! Hold the button for 4 milliseconds. After its remaining 3 milliseconds of travel time, the boat
//! will have gone 12 millimeters.
//! Hold the button for 5 milliseconds, causing the boat to travel a total of 10 millimeters.
//! Hold the button for 6 milliseconds, causing the boat to travel a total of 6 millimeters.
//! Hold the button for 7 milliseconds. That's the entire duration of the race. You never let go of
//! the button. The boat can't move until you let go of the button. Please make sure you let go of
//! the button so the boat gets to move. 0 millimeters.
//! Since the current record for this race is 9 millimeters, there are actually 4 different ways
//! you could win: you could hold the button for 2, 3, 4, or 5 milliseconds at the start of the
//! race.
//!
//! In the second race, you could hold the button for at least 4 milliseconds and at most 11
//! milliseconds and beat the record, a total of 8 different ways to win.
//!
//! In the third race, you could hold the button for at least 11 milliseconds and no more than 19
//! milliseconds and still beat the record, a total of 9 ways you could win.
//!
//! To see how much margin of error you have, determine the number of ways you can beat the record
//! in each race; in this example, if you multiply these values together, you get 288 (4 * 8 * 9).
//!
//! Determine the number of ways you could beat the record in each race. What do you get if you
//! multiply these numbers together?
//!
//! --- Part Two ---
//! As the race is about to start, you realize the piece of paper with race times and record
//! distances you got earlier actually just has very bad kerning. There's really only one race
//! - ignore the spaces between the numbers on each line.
//!
//! So, the example from before:
//!
//! Time:      7  15   30
//! Distance:  9  40  200
//! ...now instead means this:
//!
//! Time:      71530
//! Distance:  940200
//! Now, you have to figure out how many ways there are to win this single race. In this example,
//! the race lasts for 71530 milliseconds and the record distance you need to beat is 940200
//! millimeters. You could hold the button anywhere from 14 to 71516 milliseconds and beat the
//! record, a total of 71503 ways!
//!
//! How many ways can you beat the record in this one much longer race?

fn find_number_of_ways_to_win_multiple(input: &str) -> usize {
    let mut total = 1;
    let lines = input.lines().collect::<Vec<_>>();
    let time_line = lines[0];
    let distance_line = lines[1];
    let times: Vec<u32> = time_line
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();
    let distance_records: Vec<u32> = distance_line
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect();

    // Find ways to win per race.
    for (time, record) in times.into_iter().zip(distance_records.into_iter()) {
        let mut ways = 0;
        for speed_candidate in 1..time {
            let distance = speed_candidate * (time - speed_candidate);
            if distance > record {
                ways += 1;
            }
        }
        total *= ways;
    }

    total
}

fn find_number_of_ways_to_win_single_multiple(input: &str) -> usize {
    let mut total = 0;
    let lines = input.lines().collect::<Vec<_>>();
    let time_line = lines[0];
    let distance_line = lines[1];
    let time = time_line
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let record = distance_line
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    dbg!(time, record);

    for speed_candidate in 1..time {
        let distance = speed_candidate * (time - speed_candidate);
        if distance > record {
            total += 1;
        }
    }

    total
}

fn main() {
    println!("day6");

    let input = include_str!("../../input/day6.txt");
    println!("{}", find_number_of_ways_to_win_multiple(input));

    let input = include_str!("../../input/day6.txt");
    println!("{}", find_number_of_ways_to_win_single_multiple(input));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_example_1() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        let solution = find_number_of_ways_to_win_multiple(input);

        assert_eq!(solution, 288);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day6.txt");

        let solution = find_number_of_ways_to_win_multiple(input);

        assert_eq!(solution, 741000);
    }

    #[test]
    fn test_example_2() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        let solution = find_number_of_ways_to_win_single_multiple(input);

        assert_eq!(solution, 71503);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day6.txt");

        let solution = find_number_of_ways_to_win_single_multiple(input);

        assert_eq!(solution, 38220708);
    }
}
