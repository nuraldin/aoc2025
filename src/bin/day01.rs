/*
   day01:

   We need to help the elves finish decorating the North Pole.
   Oh no! A password is missing!

   There is a dial with an arrow with numbers of 0 through 99. Each dial turn makes a click.
   The input is a sequence of rotations, one per line, which tells us how to open the safe
   They start with L or R to mark the direction the the distance which is the amount of clicks the dial should be rotated
   The dial starts always at position 50
   
   Part 1:

   The password is the amount of times the dial is left pointing at 0 after any rotation in the sequence

   Part 2:

   The password is calculating using the method 0x434C49434B which means the password is any time any click makes the dial be at 0
*/
const DIAL_START: i32 = 50;
const DIAL_SIZE: i32 = 100; // it goes from 0 to 99

#[derive(Debug)]
struct Instruction {
    direction: char,
    amount: i32,
}

// Counts anytime the dial goes through zero
fn count_zeroes_method_0x434c49434b(input: &Vec<Instruction>) -> i32 {
    let mut zeroes = 0;
    let mut dial_position = DIAL_START; 

    for instruction in input {
        let step = if instruction.direction == 'L' {
            -instruction.amount
        } else {
            instruction.amount
        };

        let old_position = dial_position;
        dial_position += step;

        zeroes += if dial_position > old_position {
            dial_position.div_euclid(DIAL_SIZE) - old_position.div_euclid(DIAL_SIZE)
        } else {
            (old_position - 1).div_euclid(DIAL_SIZE) - (dial_position - 1).div_euclid(DIAL_SIZE)
        };
    }

    zeroes
}

fn count_zeroes(input: &Vec<Instruction>) -> i32 {
    let mut zeroes = 0;
    let mut dial_position = DIAL_START; 

    for instruction in input {
        if instruction.direction == 'L' {
            dial_position -= instruction.amount;
        } else {
            dial_position += instruction.amount;
        }

        dial_position = dial_position.rem_euclid(100);
  
        if dial_position == 0 {
            zeroes += 1
        }
    }

    zeroes
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (dir, num) = line.trim().split_at(1);
            Instruction {
                direction: dir.chars().next().unwrap(), // safe because split_at(1)
                amount: num.parse().expect("invalid number in instruction"),
            }
        })
        .collect()
}

fn main() {
    let input = include_str!("../inputs/day01.txt");
    let instructions = parse_instructions(input);

    println!("times the dial was left at 0: {}", count_zeroes(&instructions));
    println!("times the dial passed over 0: {}", count_zeroes_method_0x434c49434b(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_result_is_three() {
        let example_input = r#"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "#;
        let parsed = parse_instructions(example_input);

        assert_eq!(count_zeroes(&parsed), 3);
    }

    #[test]
    fn exmaple_input_with_method_0x434c49434b() {
        let example_input = r#"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "#;
        let parsed = parse_instructions(example_input);

        assert_eq!(count_zeroes_method_0x434c49434b(&parsed), 6); 
    }

}
