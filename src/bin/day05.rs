use std::cmp::Ordering;

/*
    day05:

    Elves cannot figure out what ingredients are fresh and which are spoiled.
    The puzzle input is a copy of their database, which operates on ingredient IDs.
    It consists on a list of fresh ingredient ID ranges,a blank line and a list of available ingredient IDs.
    the ID ranges are inclusive.
    Any ingredient is fresh if it is in any range.

    Part one:

    How many of the available ingredient IDs are fresh?

    Part two:

    How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges?


*/
use aoc2025::time_it;

fn main() {
    let input = include_str!("../inputs/day05.txt");

    let parsed = parse_ranges_and_numbers_fixed(input);

    let res_one = time_it!("part one: ", get_fresh_ingredients(&parsed)); // approx runtime: 220 us
    let res_two = time_it!("part two: ", get_all_fresh(parsed.0)); // approx run 2 us

    println!("There are this amount of fresh ingredients: {res_one}");
    println!("There are this amount of considered to be fresh ingredientes: {res_two}");
}

fn get_all_fresh(ranges: Vec<(u64, u64)>) -> u128 {
    let mut fresh = 0;

    for (start, end) in ranges {
        fresh += (end as u128 - start as u128) + 1
    }

    fresh
}

/// Counts how many `ingredients` fall inside the inclusive `ranges`.
///
/// Expects `ranges` to be sorted, non-overlapping, and inclusive so a binary
/// search can quickly determine membership for each ingredient.
fn get_fresh_ingredients((ranges, ingredients): &(Vec<(u64, u64)>, Vec<u64>)) -> i32 {
    let mut count = 0;

    for ingredient in ingredients {
        if ranges
            .binary_search_by(|(start, end)| {
                if ingredient < start {
                    Ordering::Greater
                } else if ingredient > end {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .is_ok()
        {
            count += 1;
        }
    }

    count
}

/// Merge overlapping AND touching inclusive ranges.
///
/// Input: arbitrary (possibly unsorted, overlapping) ranges (start, end)
/// Output: sorted, non-overlapping ranges with touching ones merged.
fn merge_ranges_touching(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    // Ensure each range has start <= end (just in case input is dirty)
    for (start, end) in &mut ranges {
        if *start > *end {
            std::mem::swap(start, end);
        }
    }

    // Sort by start
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];

    for (start, end) in ranges.into_iter().skip(1) {
        let (cur_start, cur_end) = current;

        // Overlap if start <= cur_end (inclusive ranges)
        let overlaps = start <= cur_end;
        // "Touch" if start == cur_end + 1, but avoid overflow when cur_end == u64::MAX
        let touches = cur_end != u64::MAX && start == cur_end + 1;

        if overlaps || touches {
            // Extend current range
            let new_end = cur_end.max(end);
            current = (cur_start, new_end);
        } else {
            // No overlap or touch → push current and start a new one
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    merged
}

fn parse_ranges_and_numbers_fixed(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut parts = input.split("\n\n");

    let ranges_block = parts.next().expect("missing ranges block");
    let nums_block = parts.next().expect("missing numbers block");

    let ranges = ranges_block
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').expect("range must contain '-'");
            (
                a.trim().parse::<u64>().unwrap(),
                b.trim().parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut ranges = merge_ranges_touching(ranges);
    ranges.sort_by_key(|&(start, _)| start);

    let numbers = nums_block
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    (ranges, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part_example_input() {
        let test_input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
        let parsed = parse_ranges_and_numbers_fixed(test_input);

        assert_eq!(get_fresh_ingredients(&parsed), 3);
    }

    #[test]
    fn test_second_part_example_input() {
        let test_input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
        let parsed = parse_ranges_and_numbers_fixed(test_input);

        assert_eq!(get_all_fresh(parsed.0), 14);
    }
}
