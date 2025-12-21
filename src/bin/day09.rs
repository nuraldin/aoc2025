use aoc2025::day09::*;

fn main() {
    let parsed_input = parse_locations(INPUT);

    let res_one = largest_area(&parsed_input);
    // let res_two = time_it!("part two: ", count_timelines(&parsed));

    println!("The largest rectangle size is {}", res_one);
    // println!("The particle would generate {} timelines", res_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_input() {
        let parsed_input = parse_locations(TEST_INPUT);

        assert_eq!(largest_area(&parsed_input), 50);
    }

    #[test]
    fn test_part_two_example_input() {}
}
