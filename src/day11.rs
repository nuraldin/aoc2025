/*
    day10:

    there is a server rack which has devices and their outputs (puzzle input)
    the elves need to find the paths from myself marked as "you" to the end marked as "out"

    Part one:

    How many paths lead from "you" to "out"?

    Part two:

    for the second part, you start at svr and the paths need to cross both dac and fft in any order. the rest are discarded

*/
use std::collections::HashMap;

pub const INPUT: &str = include_str!("inputs/day11.txt");

pub const FIRST_PART_TEST_INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

pub const SECOND_PART_TEST_INPUT: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

const FFT: &str = "fft";
const DAC: &str = "dac";
const START: &str = "svr";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Seen {
    fft: bool,
    dac: bool,
}

pub fn count_traverse_fft_dac_paths(graph: &HashMap<String, Vec<String>>) -> u64 {
    fn dfs(
        graph: &HashMap<String, Vec<String>>,
        node: &str,
        seen: Seen,
        memo: &mut HashMap<(String, Seen), u64>,
    ) -> u64 {
        if node == "out" {
            return (seen.fft && seen.dac) as u64;
        }

        let key = (node.to_owned(), seen);
        if let Some(&v) = memo.get(&key) {
            return v;
        }

        let mut total = 0u64;
        for next in &graph[node] {
            let mut next_seen = seen;
            next_seen.fft |= next == FFT;
            next_seen.dac |= next == DAC;

            total += dfs(graph, next, next_seen, memo);
        }

        memo.insert(key, total);
        total
    }

    let mut memo = HashMap::new();
    dfs(
        graph,
        START,
        Seen {
            fft: false,
            dac: false,
        },
        &mut memo,
    )
}

pub fn traverse_paths(input: &HashMap<String, Vec<String>>) -> i32 {
    let start = input.get("you").unwrap();

    let mut paths = 0;

    let mut next_iteration = start.clone();
    while next_iteration.len() > 0 {
        let mut current_iteration = Vec::new();

        for branch in next_iteration {
            if branch != "out" {
                let next_branch = input.get(&branch).cloned().unwrap();
                current_iteration.extend(next_branch);
            } else {
                paths += 1; // Only count when path ends
            }
        }

        next_iteration = current_iteration;
    }

    paths
}

pub fn parse_devices(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (key, rest) = line.split_once(':').unwrap();
        let values = rest
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        map.insert(key.trim().to_string(), values);
    }

    map
}
