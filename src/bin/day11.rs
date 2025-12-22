use aoc2025::day11::*;

fn main() {
    let parsed_input = parse_devices(INPUT);

    println!(
        "The paths that lead to out are: {}",
        traverse_paths(&parsed_input)
    );
    println!(
        "The paths that visit both dac and fft are: {}",
        count_traverse_fft_dac_paths(&parsed_input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_input() {
        let parsed_input = parse_devices(FIRST_PART_TEST_INPUT);

        assert_eq!(traverse_paths(&parsed_input), 5);
    }

    #[test]
    fn test_part_two_example_input() {
        let parsed_input = parse_devices(SECOND_PART_TEST_INPUT);

        assert_eq!(count_traverse_fft_dac_paths(&parsed_input), 2);
    }
}
