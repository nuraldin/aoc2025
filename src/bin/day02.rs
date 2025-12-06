/*
    day02:

    I've got some product ID ranges that are my puzzle input that I have to check.
    The ranges are separated by commas (,) and each range gives the first to last ID separated by (-)
    There are invalid IDs that are only made of a sequene of digits repeated twice. e.g. 55 (5 twice), 6464 (64 twice), 123123 (123 twice)..
    No number has leading zeroes.

    Part one:

    What do you get if you sum up the invalid IDs?

    Part two:

    The issue here is that it is a squence that is repeated at least twice, it could be more e.g. 123123123 (123 three times), 111 (1 three times), etc.
*/
use aoc2025::time_it;

fn main() {
    let input = include_str!("../inputs/day02.txt");
    let parsed = parse_range_list(input);

    let part_one = time_it!("part one: ", sum_invalid_ids(&parsed));
    let part_two = time_it!("part two: ", sum_invalid_ids_corrected(&parsed));

    println!("The sum of invalid IDs is: {part_one}");
    println!("The corrected sum of invalid IDs is: {part_two}");
}

fn sum_invalid_ids_corrected(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .map(|&(start, end)| {
            (start..=end)
                .filter(|n| {
                    let s = n.to_string();
                    let len = s.len();

                    let doubled = s.repeat(2);

                    doubled[1..2 * len - 1].contains(&s)
                })
                .sum::<u64>()
        })
        .sum()
}

fn sum_invalid_ids(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .map(|&(start, end)| {
            (start..=end)
                .filter(|n| {
                    let s = n.to_string();
                    let len = s.len();

                    if len % 2 != 0 {
                        return false;
                    }

                    let half = len / 2;
                    s[..half] == s[half..]
                })
                .sum::<u64>()
        })
        .sum()
}

fn parse_range_list(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|range_str| {
            let range_str = range_str.trim();

            // Must contain "-"
            let (start, end) = range_str.split_once('-').expect("range must contain '-'");

            let mut a = start
                .trim()
                .parse::<u64>()
                .expect("invalid number in range");
            let mut b = end.trim().parse::<u64>().expect("invalid number in range");

            // Normalize (ensure inclusive start <= end)
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }

            (a, b)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_input() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

        let parsed = parse_range_list(input);
        assert_eq!(sum_invalid_ids(&parsed), 1227775554);
    }

    #[test]
    fn test_part_two_example_input() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

        let parsed = parse_range_list(input);
        assert_eq!(sum_invalid_ids_corrected(&parsed), 4174379265);
    }
}
