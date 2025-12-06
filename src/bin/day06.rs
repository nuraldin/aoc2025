/*
    day06:

    I need to help cephalopods do their math homework

    The puzzle input consists of a list of problems.
    Each problem has a group of numbers that need to be either added (+) or multiplied (*)
    Problems are arranged vertically and the last line is the symbol for the operation.

    Part 1:

    What is the grand total found by adding together all of the answers of the individual problems?

    Part 2:

    The final calculation remains the same, however the numbers are read differently.
    every column is a number with its most sgnificat digit at the top. e.g.

    12
    157
    1

    is read as 7, 25, 111
*/
use aoc2025::time_it;

fn main() {
    let input = include_str!("../inputs/day06.txt");

    let parsed_one = parse_columns_and_ops(input);
    let parsed_two = parse_vertical_columns_and_ops(input);

    let res_one = time_it!("part one: ", do_calculations(parsed_one));
    let res_two = time_it!("part two: ", do_calculations(parsed_two));

    println!("The grand total found for part one: {}", res_one);
    println!("The grand total found for part two: {}", res_two);
}

fn do_calculations((columns, ops): (Vec<Vec<i64>>, Vec<char>)) -> u128 {
    columns
        .into_iter()
        .zip(ops)
        .map(|(col, op)| {
            let values = col.into_iter().map(|x| x as u128);
            if op == '*' {
                values.into_iter().product::<u128>()
            } else {
                values.into_iter().sum::<u128>()
            }
        })
        .sum()
}

fn parse_columns_and_ops(input: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let mut lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    // The last line contains operators
    let ops_line = lines.pop().unwrap();

    // Parse operators (every non-space character)
    let ops: Vec<char> = ops_line.chars().filter(|c| !c.is_whitespace()).collect();

    // Parse number rows
    let number_rows: Vec<Vec<i64>> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|token| token.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    // Transpose rows → columns
    let cols_count = number_rows[0].len();
    let mut columns = vec![Vec::new(); cols_count];

    for row in number_rows {
        for (col_idx, value) in row.into_iter().enumerate() {
            columns[col_idx].push(value);
        }
    }

    (columns, ops)
}
fn parse_vertical_columns_and_ops(input: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    // 1) Collect non-empty lines, trimming only the right side.
    let mut lines: Vec<String> = input
        .lines()
        .map(|l| l.trim_end().to_string())
        .filter(|l| !l.trim().is_empty())
        .collect();

    assert!(!lines.is_empty(), "input must not be empty");

    // 2) Last line: contains the operators.
    let ops_line = lines.pop().unwrap();
    let mut ops = Vec::new();
    let mut op_positions = Vec::new();

    // We treat every non-space char in the last line as an operator,
    // and its byte index as the start of a block.
    for (idx, b) in ops_line.bytes().enumerate() {
        if !b.is_ascii_whitespace() {
            ops.push(b as char);
            op_positions.push(idx);
        }
    }

    assert!(!ops.is_empty(), "no operators found");

    // 3) Pad all number lines to the same width (so indexing is safe).
    let width = ops_line
        .len()
        .max(lines.iter().map(|l| l.len()).max().unwrap_or(0));
    let number_rows: Vec<Vec<u8>> = lines
        .into_iter()
        .map(|mut l| {
            if l.len() < width {
                l.push_str(&" ".repeat(width - l.len()));
            }
            l.into_bytes()
        })
        .collect();

    let row_count = number_rows.len();

    // 4) Compute block [start, end) ranges from operator positions.
    let starts = op_positions;
    let mut ends = starts.iter().skip(1).copied().collect::<Vec<usize>>();
    ends.push(width); // last block ends at end of line

    // 5) For each block, build its vertical numbers.
    let mut all_columns: Vec<Vec<i64>> = Vec::with_capacity(starts.len());

    for (&start, &end) in starts.iter().zip(ends.iter()) {
        let mut numbers_in_block = Vec::new();

        // For each character column inside this horizontal block:
        for col in start..end {
            let mut num_str = String::new();

            // Read digits top→bottom in this character column.
            for row in 0..row_count {
                let b = number_rows[row][col];
                if b.is_ascii_digit() {
                    num_str.push(b as char);
                }
            }

            if !num_str.is_empty() {
                let value: i64 = num_str.parse().unwrap();
                numbers_in_block.push(value);
            }
        }

        all_columns.push(numbers_in_block);
    }

    (all_columns, ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_input() {
        let test_input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
"#;
        let parsed = parse_columns_and_ops(test_input);

        assert_eq!(do_calculations(parsed), 4277556)
    }

    #[test]
    fn test_part_two_example_input() {
        let test_input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
"#;
        let parsed = parse_vertical_columns_and_ops(test_input);
        println!("{parsed:?}");
        assert_eq!(do_calculations(parsed), 3263827)
    }
}
