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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }

    // Check if this range overlaps with another range
    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    // Compute the intersection of two ranges
    fn intersect(&self, other: &Range) -> Option<Range> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if start <= end {
            Some(Range::new(start, end))
        } else {
            None
        }
    }
}

struct RangeMapping {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl RangeMapping {
    fn source_range(&self) -> Range {
        Range::new(self.source_start, self.source_start + self.length - 1)
    }

    fn map_range(&self, source_range: &Range) -> Option<Range> {
        let mapping_source_range = self.source_range();
        if let Some(overlap) = source_range.intersect(&mapping_source_range) {
            let offset_start = overlap.start - self.source_start;
            let offset_end = overlap.end - self.source_start;
            Some(Range::new(
                self.dest_start + offset_start,
                self.dest_start + offset_end,
            ))
        } else {
            None
        }
    }

    fn contains(&self, number: u64) -> bool {
        number >= self.source_start && number < self.source_start + self.length
    }

    fn map(&self, number: u64) -> u64 {
        let offset = number - self.source_start;
        self.dest_start + offset
    }
}

fn map_number_with_range_mapping(table: &[RangeMapping], number: u64) -> u64 {
    table
        .iter()
        .find(|mapping| mapping.contains(number))
        .map_or(number, |mapping| mapping.map(number))
}

/// To find the lowest location, you need to start with the seed number 0 and follow the maps in
/// order. We start by building the maps. Then follow them to find each location number. Then take
/// their min.
fn find_lowest_location_from_seed_numbers(input: &str) -> u64 {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let seed_numbers = sections[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

    let tables = sections
        .into_iter()
        .skip(1)
        .map(parse_range_mapping)
        .collect::<Vec<_>>();

    let locations: Vec<_> = seed_numbers
        .iter()
        .map(|n| {
            let mut number = *n;
            for table in &tables {
                number = map_number_with_range_mapping(table, number);
            }
            number
        })
        .collect();

    *locations.iter().min().unwrap()
}

fn parse_range_mapping(section: &str) -> Vec<RangeMapping> {
    section
        .lines()
        .skip(1)
        .map(|line| {
            let mut parts = line.split_whitespace().map(|n| n.parse::<u64>().unwrap());
            let dest_start = parts.next().unwrap();
            let source_start = parts.next().unwrap();
            let range_length = parts.next().unwrap();
            RangeMapping {
                dest_start,
                source_start,
                length: range_length,
            }
        })
        .collect()
}
// Function to process a single range through a mapping table
fn map_range_with_range_mappings(range: &Range, mapping_table: &[RangeMapping]) -> Vec<Range> {
    // Collect breakpoints: start and end of the range, and start and end of overlapping mappings
    let mut breakpoints = vec![range.start, range.end + 1];
    for mapping in mapping_table {
        let mapping_source = mapping.source_range();
        if let Some(overlap) = range.intersect(&mapping_source) {
            breakpoints.push(overlap.start);
            breakpoints.push(overlap.end + 1);
        }
    }
    // Remove duplicates and sort breakpoints
    breakpoints.sort_unstable();
    breakpoints.dedup();

    let mut new_ranges = Vec::new();

    // For each interval between breakpoints, determine if it is covered by any mapping
    for i in 0..breakpoints.len() - 1 {
        let interval = Range::new(breakpoints[i], breakpoints[i + 1] - 1);
        if interval.start > interval.end {
            continue;
        }
        let mut mapped = false;
        for mapping in mapping_table {
            let mapping_source = mapping.source_range();
            if interval.overlaps(&mapping_source) {
                // Map the interval
                if let Some(mapped_range) = mapping.map_range(&interval) {
                    new_ranges.push(mapped_range);
                    mapped = true;
                    break;
                }
            }
        }
        if !mapped {
            // Unmapped interval maps to itself
            new_ranges.push(interval);
        }
    }
    new_ranges
}

fn find_lowest_location_from_seed_ranges(input: &str) -> u64 {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let seed_ranges_input = sections[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    // Collect every two values into a seed range
    let seed_ranges = seed_ranges_input
        .chunks(2)
        .map(|chunk| {
            let start: u64 = chunk[0].parse().unwrap();
            let length: u64 = chunk[1].parse().unwrap();
            let end = start + length - 1;
            Range::new(start, end)
        })
        .collect::<Vec<Range>>();

    let mapping_tables = sections
        .into_iter()
        .skip(1)
        .map(parse_range_mapping)
        .collect::<Vec<Vec<RangeMapping>>>();

    // Initialize current ranges with seed ranges
    let mut current_ranges = seed_ranges;

    for mapping_table in mapping_tables {
        let mut new_ranges = Vec::new();
        for range in &current_ranges {
            let mapped_ranges = map_range_with_range_mappings(range, &mapping_table);
            new_ranges.extend(mapped_ranges);
        }
        current_ranges = new_ranges;
    }

    // Now, current_ranges contains ranges in the location category
    // Find the minimum start among all ranges
    current_ranges.into_iter().map(|r| r.start).min().unwrap()
}

fn main() {
    println!("Day 5: If You Give A Seed A Fertilizer");

    let input = include_str!("../../input/day5.txt");

    let solution = find_lowest_location_from_seed_numbers(input);
    println!("Lowest location: {solution}");

    let solution = find_lowest_location_from_seed_ranges(input);
    println!("Lowest location from seed ranges: {solution}");
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
        let solution = find_lowest_location_from_seed_numbers(EXAMPLE_1);

        assert_eq!(solution, 35);
    }

    #[test]
    fn test_solution_1() {
        let input = include_str!("../../input/day5.txt");

        let solution = find_lowest_location_from_seed_numbers(input);

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
