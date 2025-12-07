/*
    day07:

    Our tachyon minifolds have an issue. The diagram of these is our puzzle input.
    The beam enters the minifold at the location marked 'S', beams always move downward.
    The beams passe freely through empty spaces '.', However, if they found a splitter (^) the beam is stopped and it splits immediately.
    They go left and right of the splitter.
    The process continues until all the tachyoun beams reach a splitter or exit the manifold

    Part one:

    Howe many times will the beam be split?

    Part two:

    After all, it was not a normal tachyon manifold, it was a quantum tachyon manifold! This time it's only a particle that its flowing.
    The particle could go right or left, each path creating a new timeline.

    How many different timelines would a single tachyon particle end up on?

*/
use aoc2025::time_it;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../inputs/day07.txt");

    let parsed = parse_manifold(input);

    let res_one = time_it!("part one: ", count_splits(&parsed));
    let res_two = time_it!("part two: ", count_timelines(&parsed));

    println!("The beam will be split {} times", res_one);
    println!("The particle would generate {} timelines", res_two);
}

fn count_timelines(
    (start, carets, rows): &(Option<(usize, usize)>, Vec<(usize, usize)>, usize),
) -> usize {
    let mut beams = HashMap::new();
    beams.insert(start.unwrap(), 1);

    let mut hits = 0;

    while !beams.is_empty() {
        let mut next_beams = HashMap::new();

        for (beam_pos, count) in beams {
            let next_row = beam_pos.0 + 1;
            if next_row >= *rows {
                continue;
            }

            if carets.contains(&(next_row, beam_pos.1)) {
                hits += count;

                // spawn two beams, count
                *next_beams.entry((next_row, beam_pos.1 + 1)).or_insert(0) += count;
                *next_beams.entry((next_row, beam_pos.1 - 1)).or_insert(0) += count;
            } else {
                // just go straight
                *next_beams.entry((next_row, beam_pos.1)).or_insert(0) += count;
            }
        }

        beams = next_beams;
    }

    hits + 1
}

fn count_splits(
    (start, carets, rows): &(Option<(usize, usize)>, Vec<(usize, usize)>, usize),
) -> usize {
    let mut beams = HashSet::from([start.unwrap()]);
    let mut hitted_carets = HashSet::new();

    while !beams.is_empty() {
        let current_beams = beams.clone();
        let mut next_beams = HashSet::new();

        for beam in &current_beams {
            // if the beam is in the last row or left, skip it
            if beam.0 + 1 >= *rows {
                continue;
            }

            // if next beam position is a caret, split it. otherwise just advance
            if carets.contains(&(beam.0 + 1, beam.1)) {
                next_beams.insert((beam.0 + 1, beam.1 + 1)); // to the right
                next_beams.insert((beam.0 + 1, beam.1 - 1)); // to the left
                hitted_carets.insert((beam.0 + 1, beam.1));
            } else {
                next_beams.insert((beam.0 + 1, beam.1)); // just advance down
            }
        }

        beams = next_beams;
    }

    hitted_carets.len()
}

fn parse_manifold(input: &str) -> (Option<(usize, usize)>, Vec<(usize, usize)>, usize) {
    let mut start: Option<(usize, usize)> = None;
    let mut carets: Vec<(usize, usize)> = Vec::new();

    let mut rows = 0;
    for (row, line) in input.lines().enumerate() {
        // Skip empty lines that appear because of leading newline in raw string
        if line.trim().is_empty() {
            continue;
        }

        rows += 1;
        for (col, ch) in line.chars().enumerate() {
            match ch {
                'S' => start = Some((row, col)),
                '^' => carets.push((row, col)),
                _ => {}
            }
        }
    }

    (start, carets, rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_input() {
        let test_input = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

        let parsed = parse_manifold(test_input);
        assert_eq!(count_splits(&parsed), 21);
    }

    #[test]
    fn test_part_two_example_input() {
        let test_input = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

        let parsed = parse_manifold(test_input);
        assert_eq!(count_timelines(&parsed), 40);
    }
}
