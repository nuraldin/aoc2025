use aoc2025::day12::*;

fn main() {
    let puzzle = parse_puzzle(INPUT);

    println!(
        "The amount of regions that are solvable are: {}",
        get_possible_regions(&puzzle)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_input() {
        let puzzle = parse_puzzle(TEST_INPUT);
        assert_eq!(get_possible_regions(&puzzle), 2);
    }

    #[test]
    fn test_part_two_example_input() {}
}
