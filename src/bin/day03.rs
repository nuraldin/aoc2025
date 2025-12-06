/*
    day03:

    Template scaffold for later implementation.
    Parses signed integers from the input and feeds them into placeholder
    solvers. Swap out the solver bodies with the real puzzle logic once ready.
*/
use aoc2025::time_it;

fn main() {
    let input = include_str!("../inputs/day03.txt");
    let numbers = parse_numbers(input);

    let part_one = time_it!("part one: ", solve_part_one(&numbers));
    let part_two = time_it!("part two: ", solve_part_two(&numbers));

    println!("day03 part one: {part_one}");
    println!("day03 part two: {part_two}");
}

fn parse_numbers(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.parse::<i64>().expect("invalid number in input"))
            }
        })
        .collect()
}

fn solve_part_one(numbers: &[i64]) -> i64 {
    numbers.iter().sum()
}

fn solve_part_two(numbers: &[i64]) -> i64 {
    numbers.iter().map(|n| n * n).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ignores_empty_lines() {
        let input = "1\n \n-2\n";
        let parsed = parse_numbers(input);
        assert_eq!(parsed, vec![1, -2]);
    }

    #[test]
    fn placeholder_solvers_work() {
        let nums = vec![1, -2, 3];
        assert_eq!(solve_part_one(&nums), 2);
        assert_eq!(solve_part_two(&nums), 14);
    }
}
