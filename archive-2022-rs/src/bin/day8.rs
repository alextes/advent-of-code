//! --- Day 8: Treetop Tree House ---
//! The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.
//!
//! First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.
//!
//! The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:
//!
//! ```
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//! Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.
//!
//! A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
//!
//! All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:
//!
//! The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
//! The top-middle 5 is visible from the top and right.
//! The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
//! The left-middle 5 is visible, but only from the right.
//! The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
//! The right-middle 3 is visible from the right.
//! In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
//! With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.
//!
//! Consider your map; how many trees are visible from outside the grid?

use advent_of_code::input::Input;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn count_visible_trees(map: &[Vec<u8>]) -> usize {
    let mut visible_trees = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            // Trees on the edge are always visible
            if x == 0 || y == 0 || x == row.len() - 1 || y == map.len() - 1 {
                visible_trees += 1;
                continue;
            }

            // Check if the tree is visible from any direction
            let visible_left = row[..x].iter().all(|t| *t < *tree);
            let visible_right = row[x + 1..].iter().all(|t| *t < *tree);
            let visible_up = map[..y].iter().all(|r| r[x] < *tree);
            let visible_down = map[y + 1..].iter().all(|r| r[x] < *tree);

            // If the tree is visible from any direction, count it
            if visible_left || visible_right || visible_up || visible_down {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

pub fn main() {
    println!("day 8 solutions");

    let input = Input::new(8).raw_string();
    let map = parse_input(&input);

    let visible_trees = count_visible_trees(&map);
    println!("part 1: {}", visible_trees);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_count_visible_trees() {
        let map = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(count_visible_trees(&map), 21);
    }
}
