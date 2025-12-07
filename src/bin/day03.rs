/*
    day03:

    The security checks are offline. There are batteries that can help and are labed with their joltage rating, a value from 1 to 9. The raings are the puzzle input.
    The batteries are arranged into banks (a line of digits)
    I need to turn on exactly two batteries.The joltage that a bank produces is equal to the number formed by the digits on the batteries turned on. e.f. 24 if 12345 has 2 and 4 active.
    I need to find the largest possible joltage each bank can produce.

    Part one:

    What is the total output joltage? Which is the sum of the maximum joltage for each bank.

    Part two:

    Now the largest joltage is made by turning exactly twelve batteries. What is the new total output joltage

*/
use aoc2025::time_it;

fn main() {
    let input = include_str!("../inputs/day03.txt");
    let numbers = parse_digit_grid(input);

    let part_one = time_it!("part one: ", total_output_joltage_k_digits(&numbers, 2));
    let part_two = time_it!("part two: ", total_output_joltage_k_digits(&numbers, 12));

    println!("The total output joltage for part one is: {part_one}");
    println!("The total output joltage for part two is: {part_two}");
}

fn find_largest_k_digits(bank: &[u32], k: usize) -> Option<u64> {
    if bank.len() < k {
        return None; // Cannot pick k digits if the size is smaller than k
    }

    let mut to_remove = bank.len() - k;
    let mut stack: Vec<u32> = Vec::with_capacity(bank.len());

    for &d in bank {
        // d is assumed to be a single digit 0..=9
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(d);
    }

    // If we didn't remove enough (e.g. monotonically decreasing input), drop from the end.
    stack.truncate(k);

    // Turn the k digits into a number
    let mut result: u64 = 0;
    for &d in &stack {
        result = result * 10 + d as u64;
    }

    Some(result)
}

fn total_output_joltage_k_digits(banks: &[Vec<u32>], k: usize) -> u64 {
    banks
        .iter()
        .map(|bank| find_largest_k_digits(bank, k).unwrap())
        .sum()
}

fn parse_digit_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("non-digit in input"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_input() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        let parsed = parse_digit_grid(input);
        assert_eq!(total_output_joltage_k_digits(&parsed, 2), 357);
    }

    #[test]
    fn test_part_two_example_input() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        let parsed = parse_digit_grid(input);
        assert_eq!(total_output_joltage_k_digits(&parsed, 12), 3121910778619);
    }
}
