/*
    day09:

    The Elves give us a list of red tiles locations in a grid.

    Part one:

    What is the biggest red tile rectangle one can do with these locations?
    Using the red tiles as opposite corners.

    Part two:

    what is the largest area of any rectangle you can make using only red and green tiles?
*/
pub const INPUT: &str = include_str!("inputs/day08.txt");

pub const TEST_INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

#[derive(Clone, Debug)]
pub struct Location {
    x: u32,
    y: u32,
}

impl Location {
    fn diff(&self, rhs: &Location) -> Location {
        Location {
            x: self.x.abs_diff(rhs.x),
            y: self.y.abs_diff(rhs.y),
        }
    }

    fn rectangle_size(&self, rhs: &Location) -> u64 {
        let diff = self.diff(rhs);

        let dx = diff.x as u64 + 1;
        let dy = diff.y as u64 + 1;

        dx * dy
    }
}

pub fn largest_area(input: &[Location]) -> u64 {
    let n = input.len();

    let mut max = 0;
    for idx in 0..(n - 1) {
        for jdx in idx..n {
            let lhs = &input[idx];
            let rhs = &input[jdx];

            let current_size = lhs.rectangle_size(&rhs);

            if current_size > max {
                max = current_size;
            }
        }
    }

    max
}

pub fn parse_locations(input: &str) -> Vec<Location> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .expect("invalid location format, expected `x,y`");

            Location {
                x: x.trim().parse::<u32>().expect("invalid x coordinate"),
                y: y.trim().parse::<u32>().expect("invalid y coordinate"),
            }
        })
        .collect()
}
