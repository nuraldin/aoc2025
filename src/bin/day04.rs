/*
    day04:

    There are rolls of papers given in the input in locations.
    A roll is accessible if less than four rolls of papers are around in the 8 directions.

    Part 1:

    How many are accessible?

    Part 2:

    How many could I remove until I can't, if I remove the accessible ones.
*/
fn main() {
    let input = include_str!("../inputs/day04.txt");
    let grid = parse_grid(input);

    println!(
        "reachable roll of papers: {}",
        reachable_roll_of_papers(&grid)
    );
    println!(
        "removable roll of papers: {}",
        removable_roll_of_papers(grid)
    );
}

fn removable_roll_of_papers(mut grid: Vec<Vec<char>>) -> i32 {
    let mut removable = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    let mut can_still_remove = true;
    while can_still_remove {
        let mut removed = 0;

        for row_idx in 0..rows {
            for col_idx in 0..cols {
                let mut neighbours_count = 0;
                for (nr, nc) in neighbours(row_idx, col_idx, rows, cols) {
                    if grid[nr][nc] == '@' {
                        neighbours_count += 1;
                    }
                }

                if neighbours_count < 4 && grid[row_idx][col_idx] == '@' {
                    grid[row_idx][col_idx] = '.';
                    removed += 1;
                    removable += 1;
                }
            }
        }

        if removed == 0 {
            can_still_remove = false;
        }
    }

    removable
}

fn reachable_roll_of_papers(grid: &[Vec<char>]) -> i32 {
    let mut reachable_count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for row_idx in 0..rows {
        for col_idx in 0..cols {
            let mut neighbours_count = 0;
            for (nr, nc) in neighbours(row_idx, col_idx, rows, cols) {
                if grid[nr][nc] == '@' {
                    neighbours_count += 1;
                }
            }

            if neighbours_count < 4 && grid[row_idx][col_idx] == '@' {
                reachable_count += 1;
            }
        }
    }

    reachable_count
}

fn neighbours(
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    const DIRS: [(isize, isize); 8] = [
        (-1, 0),  // up
        (1, 0),   // down
        (0, -1),  // left
        (0, 1),   // right
        (-1, -1), // up-left
        (-1, 1),  // up-right
        (1, -1),  // down-left
        (1, 1),   // down-right
    ];

    let r = row as isize;
    let c = col as isize;
    let rows = rows as isize;
    let cols = cols as isize;

    DIRS.into_iter().filter_map(move |(dr, dc)| {
        let rr = r + dr;
        let cc = c + dc;

        if (0..rows).contains(&rr) && (0..cols).contains(&cc) {
            Some((rr as usize, cc as usize))
        } else {
            None
        }
    })
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty()) // skip empty lines just in case
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part_example_input() {
        const TEST_INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

        let grid = parse_grid(TEST_INPUT);
        assert_eq!(reachable_roll_of_papers(&grid), 13);
    }

    #[test]
    fn test_second_part_example_input() {
        const TEST_INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

        let grid = parse_grid(TEST_INPUT);
        assert_eq!(removable_roll_of_papers(grid), 43);
    }
}
