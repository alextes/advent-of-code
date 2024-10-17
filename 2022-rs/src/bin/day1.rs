use advent_of_code::input::Input;

fn group_strings_by_empty_line(lines: &[String]) -> Vec<Vec<&String>> {
    let mut groups = vec![vec![]];
    for line in lines {
        if line.is_empty() {
            groups.push(vec![]);
        } else {
            let last_group = groups
                .last_mut()
                .expect("expect groups to have at least one group");
            last_group.push(line);
        }
    }
    groups
}

/// Returns the summed calories per elf. Sorted. Biggest first.
fn get_calorie_sums(lines: &[String]) -> Vec<u32> {
    let mut calorie_sums = group_strings_by_empty_line(lines)
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|str| str.parse::<u32>().expect("expect each str to be a u32"))
                .sum()
        })
        .collect::<Vec<_>>();
    calorie_sums.sort();
    calorie_sums.reverse();
    calorie_sums
}

fn get_biggest_sum(lines: &[String]) -> u32 {
    let calorie_sums = get_calorie_sums(lines);
    calorie_sums
        .first()
        .expect("expect at least one calorie group")
        .to_owned()
}

fn get_biggest_three_sum(lines: &[String]) -> u32 {
    let calorie_sums = get_calorie_sums(lines);
    calorie_sums[0..3].iter().sum()
}

fn main() {
    let input = Input::new(1);

    let biggest_sum = get_biggest_sum(input.lines());
    println!("answer1: {biggest_sum}");

    let biggest_three_sum = get_biggest_three_sum(input.lines());
    println!("answer2: {biggest_three_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biggest_sum_test() {
        let input = Input::new(1);
        let biggest_sum = get_biggest_sum(input.lines());
        assert_eq!(biggest_sum, 71471)
    }

    #[test]
    fn biggest_three_sum_test() {
        let input = Input::new(1);
        let biggest_three_sum = get_biggest_three_sum(input.lines());
        assert_eq!(biggest_three_sum, 211189)
    }
}
