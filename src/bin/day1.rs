use advent_of_code::input::{FirstOrSecond, Input};

/// Returns the summed calories per elf. Sorted. Biggest first.
fn get_calorie_sums(lines: &Vec<String>) -> Vec<u32> {
    let mut calorie_groups = vec![];
    let mut sum = 0;
    for line in lines {
        match line.as_str() {
            "" => {
                calorie_groups.push(sum);
                sum = 0;
            }
            num_str => {
                let num: u32 = num_str
                    .parse()
                    .expect("expect newlines or numerical strings");
                sum = sum + num;
            }
        }
    }

    calorie_groups.sort();
    calorie_groups.reverse();
    calorie_groups
}

fn get_biggest_sum(lines: &Vec<String>) -> u32 {
    let calorie_sums = get_calorie_sums(lines);
    calorie_sums
        .first()
        .expect("expect at least one calorie group")
        .to_owned()
}

fn get_biggest_three_sum(lines: &Vec<String>) -> u32 {
    let calorie_sums = get_calorie_sums(lines);
    calorie_sums[0..3].iter().sum()
}

fn main() {
    let input = Input::new(1, FirstOrSecond::First);

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
        let input = Input::new(1, FirstOrSecond::First);
        let biggest_sum = get_biggest_sum(input.lines());
        assert_eq!(biggest_sum, 71471)
    }

    #[test]
    fn biggest_three_sum_test() {
        let input = Input::new(1, FirstOrSecond::First);
        let biggest_three_sum = get_biggest_three_sum(input.lines());
        assert_eq!(biggest_three_sum, 211189)
    }
}
