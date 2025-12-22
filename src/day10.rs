/*
    day10:

    The Elves cannot figure out the initialization procedure of the factory machines.
    All that remains are joltage requirements for meach machine (the puzzle input)
    Each line has a light diagram in [] sqaure brackets. one or more button wiring schematics in () parenthesis.
    And joltage requirements in {} curly braces.
    A . means off and a # means on for the lights. They are all initially off.
    You can push any of the listed buttones which sayys which light it toggles.

    Part one:

    What is the fewest button presses required to correctly configure the indicator lights on all of the machines?

    Part two:



*/

pub const INPUT: &str = include_str!("inputs/day10.txt");

pub const TEST_INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub lights: Vec<bool>,        // '.' false, '#' true
    pub buttons: Vec<Vec<usize>>, // each button toggles these indices
}

pub fn fewest_buttons_all_machines(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|line| fewest_buttons_single_machine(line))
        .sum()
}

pub fn fewest_buttons_single_machine(line: &Line) -> usize {
    let min_buttons = min_buttons(line);
    println!("{min_buttons:?}");
    min_buttons.len()
}

// ---------- parsing (simple, assumes valid) ----------
pub fn parse_manual(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(input: &str) -> Line {
    let s = input.trim();
    let (lights_str, after_lights) = extract_first(s, '[', ']');

    let lights = lights_str.chars().map(|c| c == '#').collect::<Vec<_>>();

    let mut rest = after_lights;
    if let Some(i) = rest.find('{') {
        rest = &rest[..i]; // ignore joltage
    }

    let mut buttons = Vec::new();
    while let Some((btn, after)) = try_extract_first(rest, '(', ')') {
        let indices = btn
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|t| !t.is_empty())
            .map(|t| t.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        buttons.push(indices);
        rest = after;
    }

    Line { lights, buttons }
}

fn extract_first<'a>(s: &'a str, open: char, close: char) -> (&'a str, &'a str) {
    try_extract_first(s, open, close).unwrap()
}

fn try_extract_first<'a>(s: &'a str, open: char, close: char) -> Option<(&'a str, &'a str)> {
    let start = s.find(open)?;
    let after_open = &s[start + open.len_utf8()..];
    let end_rel = after_open.find(close)?;
    let inside = after_open[..end_rel].trim();
    let rest = &after_open[end_rel + close.len_utf8()..];
    Some((inside, rest))
}

// ---------- solving (≤ 16 lights) ----------

pub fn min_buttons(line: &Line) -> Vec<usize> {
    assert!(line.lights.len() <= 16);

    // target mask from diagram
    let mut target: u16 = 0;
    for (i, &on) in line.lights.iter().enumerate() {
        if on {
            target |= 1u16 << i;
        }
    }

    // button masks
    let btn_masks: Vec<u16> = line
        .buttons
        .iter()
        .map(|idxs| {
            let mut m = 0u16;
            for &i in idxs {
                m ^= 1u16 << i; // duplicates cancel automatically
            }
            m
        })
        .collect();

    // brute force all subsets; keep the one with smallest popcount
    let m = btn_masks.len();
    assert!(
        m <= 63,
        "this brute force uses u64 subset bits; raise if needed"
    );

    let mut best_subset: u64 = 0;
    let mut best_count: u32 = u32::MAX;

    for subset in 0u64..(1u64 << m) {
        // quick skip: if already worse than best, don't bother computing
        let count = subset.count_ones();
        if count >= best_count {
            continue;
        }

        // compute XOR of chosen buttons
        let mut x: u16 = 0;
        for i in 0..m {
            if ((subset >> i) & 1) == 1 {
                x ^= btn_masks[i];
            }
        }

        if x == target {
            best_subset = subset;
            best_count = count;
            if best_count == 0 {
                break;
            }
        }
    }

    // return indices of chosen buttons
    let mut chosen = Vec::new();
    for i in 0..m {
        if ((best_subset >> i) & 1) == 1 {
            chosen.push(i);
        }
    }
    chosen
}
