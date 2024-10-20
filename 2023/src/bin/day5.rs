//! Day 5: If You Give A Seed A Fertilizer
//! The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.
//!
//! The rest of the almanac contains a list of maps which describe how to convert numbers from
//! a source category into numbers in a destination category. That is, the section that starts with
//! seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the
//! destination). This lets the gardener and his team know which soil to use with which seeds,
//! which water to use with which fertilizer, and so on.
//!
//! Rather than list every source number and its corresponding destination number one by one, the
//! maps describe entire ranges of numbers that can be converted. Each line within a map contains
//! three numbers: the destination range start, the source range start, and the range length.
//!
//! Consider again the example seed-to-soil map:
//!
//! 50 98 2
//! 52 50 48
//!
//! The first line has a destination range start of 50, a source range start of 98, and a range
//! length of 2. This line means that the source range starts at 98 and contains two values: 98 and
//! 99. The destination range is the same length, but it starts at 50, so its two values are 50 and
//! 51. With this information, you know that seed number 98 corresponds to soil number 50 and that
//! seed number 99 corresponds to soil number 51.
//!
//! The second line means that the source range starts at 50 and contains 48 values: 50, 51, ...,
//! 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values:
//! 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.
//!
//! Any source numbers that aren't mapped correspond to the same destination number. So, seed
//! number 10 corresponds to soil number 10.
//!
//! --- Part Two ---
//! Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it
//! looks like the seeds: line actually describes ranges of seed numbers.
//!
//! The values on the initial seeds: line come in pairs. Within each pair, the first value is the
//! start of the range and the second value is the length of the range. So, in the first line of
//! the example above:
//!
//! seeds: 79 14 55 13
//! This line describes two ranges of seed numbers to be planted in the garden. The first range
//! starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts
//! with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.
//!
//! Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.
//!
//! In the above example, the lowest location number can be obtained from seed number 82, which
//! corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and
//! location 46. So, the lowest location number is 46.
//!
//! Consider all of the initial seed numbers listed in the ranges on the first line of the almanac.
//! What is the lowest location number that corresponds to any of the initial seed numbers?

use std::collections::{HashMap, HashSet};

fn build_map(section: &str) -> HashMap<std::ops::Range<u64>, std::ops::Range<u64>> {
    let mut map = HashMap::new();
    let contents = section.lines().skip(1).collect::<Vec<&str>>();
    for line in contents {
        let mut parts = line.split_whitespace();
        let dest_start: u64 = parts.next().unwrap().parse().unwrap();
        let source_start: u64 = parts.next().unwrap().parse().unwrap();
        let range_length: u64 = parts.next().unwrap().parse().unwrap();

        let source_range = source_start..source_start + range_length;
        let dest_range = dest_start..dest_start + range_length;

        map.insert(source_range, dest_range);
    }

    map
}

fn translate_with_map(
    number: u64,
    map: &HashMap<std::ops::Range<u64>, std::ops::Range<u64>>,
) -> u64 {
    map.iter()
        .find(|(source_range, _)| source_range.contains(&number))
        .map(|(source_range, dest_range)| {
            let offset = number - source_range.start;
            dest_range.start + offset
        })
        .unwrap_or(number)
}

/// To find the lowest location, you need to start with the seed number 0 and follow the maps in
/// order. We start by building the maps. Then follow them to find each location number. Then take
/// their min.
fn find_lowest_location(input: &str) -> u64 {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let seed_numbers = sections[0]
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();

    let seed_to_soil_map = build_map(sections[1]);
    let soil_to_fertilizer_map = build_map(sections[2]);
    let fertilizer_to_water_map = build_map(sections[3]);
    let water_to_light_map = build_map(sections[4]);
    let light_to_temperature_map = build_map(sections[5]);
    let temperature_to_humidity_map = build_map(sections[6]);
    let humidity_to_location_map = build_map(sections[7]);

    let mut locations = Vec::new();

    for seed in seed_numbers {
        let soil = translate_with_map(seed, &seed_to_soil_map);
        let fertilizer = translate_with_map(soil, &soil_to_fertilizer_map);
        let water = translate_with_map(fertilizer, &fertilizer_to_water_map);
        let light = translate_with_map(water, &water_to_light_map);
        let temperature = translate_with_map(light, &light_to_temperature_map);
        let humidity = translate_with_map(temperature, &temperature_to_humidity_map);
        let location = translate_with_map(humidity, &humidity_to_location_map);

        locations.push(location);
    }

    *locations.iter().min().unwrap()
}

fn build_map_2(section: &str) -> Vec<(u64, u64, u64)> {
    let mut map = Vec::new();
    let contents = section.lines().skip(1).collect::<Vec<&str>>();
    for line in contents {
        let mut parts = line.split_whitespace();
        let dest_start: u64 = parts.next().unwrap().parse().unwrap();
        let source_start: u64 = parts.next().unwrap().parse().unwrap();
        let range_length: u64 = parts.next().unwrap().parse().unwrap();

        map.push((source_start, dest_start, range_length));
    }
    map
}

fn inverse_map_number(number: u64, mapping_intervals: &[(u64, u64, u64)]) -> Vec<u64> {
    let mut possible_sources = Vec::new();
    let mut unmapped = true;

    for &(s_start, d_start, length) in mapping_intervals {
        let d_end = d_start + length - 1;

        if number >= d_start && number <= d_end {
            let offset = number - d_start;
            let source_number = s_start + offset;
            possible_sources.push(source_number);
            unmapped = false;
        }
    }

    // If the number isn't in any mapping interval, it maps to itself
    if unmapped {
        possible_sources.push(number);
    }

    possible_sources
}

fn is_number_in_seed_ranges(number: u64, seed_ranges: &[(u64, u64)]) -> bool {
    for &(start, end) in seed_ranges {
        if number >= start && number <= end {
            return true;
        }
    }
    false
}

fn find_lowest_location_from_seed_ranges(input: &str) -> u64 {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let seed_ranges_input = sections[0]
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    // Collect every two values into a range
    let seed_ranges = seed_ranges_input
        .chunks(2)
        .map(|chunk| {
            let start: u64 = chunk[0].parse().unwrap();
            let length: u64 = chunk[1].parse().unwrap();
            let end = start + length - 1;
            (start, end)
        })
        .collect::<Vec<(u64, u64)>>();

    let mapping_functions = sections[1..]
        .iter()
        .map(|section| build_map_2(section))
        .collect::<Vec<_>>();

    // Start from the lowest possible location number
    let mut location_number = 0;

    loop {
        // Initialize the set of possible numbers at the current step
        let mut current_numbers = vec![location_number];

        // We will work backwards through the mappings
        for mapping_intervals in mapping_functions.iter().rev() {
            let mut next_numbers = HashSet::new();

            for &number in &current_numbers {
                let sources = inverse_map_number(number, mapping_intervals);
                for source in sources {
                    next_numbers.insert(source);
                }
            }

            // Update the current numbers for the next iteration
            current_numbers = next_numbers.into_iter().collect();
        }

        // At this point, current_numbers contains possible seed numbers
        for &seed_number in &current_numbers {
            if is_number_in_seed_ranges(seed_number, &seed_ranges) {
                // Found the lowest location number
                return location_number;
            }
        }

        // Move to the next location number
        location_number += 1;
    }
}

fn main() {
    println!("Day 5: If You Give A Seed A Fertilizer");

    let input = include_str!("../../input/day5.txt");

    let solution = find_lowest_location(input);
    println!("Lowest location: {}", solution);

    let solution = find_lowest_location_from_seed_ranges(input);
    println!("Lowest location from seed ranges: {}", solution);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_1: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_example_1() {
        let solution = find_lowest_location(EXAMPLE_1);

        assert_eq!(solution, 35);
    }

    #[test]
    fn test_solution_1() {
        let input = include_str!("../../input/day5.txt");

        let solution = find_lowest_location(input);

        assert_eq!(solution, 662197086);
    }

    #[test]
    fn test_example_2() {
        let solution = find_lowest_location_from_seed_ranges(EXAMPLE_1);

        assert_eq!(solution, 46);
    }

    #[test]
    fn test_solution_2() {
        let input = include_str!("../../input/day5.txt");

        let solution = find_lowest_location_from_seed_ranges(input);

        assert_eq!(solution, 52510809);
    }
}
