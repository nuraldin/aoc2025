use aoc2025::day08::*;

fn main() {
    let junction_boxes = parse_locations(INPUT);

    let res_one = calculate_circuits(&junction_boxes, 1000);
    let res_two = calculate_extension_size(&junction_boxes);

    println!("The first three longest circuits multiply to: {}", res_one);
    println!(
        "The extension size needed for connecting the last two elements is: {}",
        res_two
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08_part_one_example_input() {
        let junction_boxes = parse_locations(TEST_INPUT);

        assert_eq!(calculate_circuits(&junction_boxes, 10), 40);
    }

    #[test]
    fn test_day08_part_two_example_input() {
        let junction_boxes = parse_locations(TEST_INPUT);

        assert_eq!(calculate_extension_size(&junction_boxes), 25272);
    }
}
