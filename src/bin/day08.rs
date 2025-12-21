use aoc2025::day08::{INPUT, calculate_circuits, calculate_extension_size, parse_locations};

fn main() {
    let junction_boxes = parse_locations(INPUT);

    let res_one = calculate_circuits(&junction_boxes, 1000);
    let res_two = calculate_extension_size(&junction_boxes);

    println!("The first three longest circuits multiply to: {}", res_one);
    println!("The extension size needed for connecting the last two elements is: {}", res_two);
}
