/*
    day02:

    Template scaffold for later implementation.
    Reads the input, parses non-empty lines, and runs placeholder solutions for
    both parts. Replace `solve_part_one` and `solve_part_two` with the real
    logic when ready.
*/
use aoc2025::time_it;

fn main() {
    let input = include_str!("../inputs/day02.txt");
    let parsed = parse_input(input);

    let part_one = time_it!("part one: ", solve_part_one(&parsed));
    let part_two = time_it!("part two: ", solve_part_two(&parsed));

    println!("day02 part one: {part_one}");
    println!("day02 part two: {part_two}");
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_owned())
            }
        })
        .collect()
}

fn solve_part_one(data: &[String]) -> i64 {
    data.len() as i64
}

fn solve_part_two(data: &[String]) -> i64 {
    data.iter().map(|line| line.len() as i64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_strips_blank_lines() {
        let input = "alpha\n \n beta\n";
        let parsed = parse_input(input);
        assert_eq!(parsed, vec!["alpha".to_string(), "beta".to_string()]);
    }

    #[test]
    fn placeholder_solutions_return_counts() {
        let data = vec!["abc".into(), "d".into()];
        assert_eq!(solve_part_two(&data), 4);
    }
}
