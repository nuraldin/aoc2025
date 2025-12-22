use aoc2025::day10::*;

fn main() {
    let parsed_input = parse_manual(INPUT);

    let res_one = fewest_buttons_all_machines(&parsed_input);
    // let res_two = time_it!("part two: ", count_timelines(&parsed));

    println!("The sum of all fewest button presses is {}", res_one);
    // println!("The particle would generate {} timelines", res_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_input() {
        let parsed_input = parse_manual(TEST_INPUT);

        assert_eq!(fewest_buttons_all_machines(&parsed_input), 7);
    }

    #[test]
    fn test_part_two_example_input() {}
}
