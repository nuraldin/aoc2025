/*
    day08:

    The elves have a large number of small electrical junction boxes.
    In order to allow electricity to pass through them they need to be connected by a string of lights.
    They want to figure out which junctions boxes to connect so that electricity can reach every junction box.
    Their positions in 3D is the puzzle input.
    The Elves would like to focus on joining the closest possible according to straight line idstance.
    When joined, these become a circuit. Also if not joined to anything, these are considered a circuit.


    Part one:

    For the example, only connect 10 closest circuits.
    For the challenge, connect 1000.
    What do you get if you multiply together the sizes of the three largest circuits?

    Part two:

    What is the product of the x coordinates of the two junction boxes that if connect would close all circuits together.?

*/
use disjoint::DisjointSet;
use std::collections::HashMap;

pub const INPUT: &str = include_str!("inputs/day08.txt");

pub const TEST_INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

#[derive(Clone, Debug)]
pub struct JunctionBox {
    x: f64,
    y: f64,
    z: f64,
}

impl JunctionBox {
    fn distance(&self, rhs: &JunctionBox) -> f64 {
        let dx = rhs.x - self.x;
        let dy = rhs.y - self.y;
        let dz = rhs.z - self.z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Clone, Copy, Debug)]
struct LightString {
    l_box: usize,
    r_box: usize,
    distance: f64,
}

pub fn calculate_extension_size(junction_boxes: &[JunctionBox]) -> i64 {
    let n = junction_boxes.len();
    let edges = calculate_distances(junction_boxes);

    let mut uf = DisjointSet::with_len(n);

    let mut components = n;

    let mut last_join: Option<(usize, usize)> = None;

    for e in edges {
        if components == 1 {
            break;
        }

        if !uf.is_joined(e.l_box, e.r_box) {
            uf.join(e.l_box, e.r_box);
            components -= 1;
            last_join = Some((e.l_box, e.r_box));
        }
    }

    let (l, r) = last_join.expect("no merging edge found (graph may be disconnected)");
    // Change the type/cast here to match your coordinate type.
    // If x is i32/i64/usize, adjust accordingly.
    let x1: i64 = junction_boxes[l].x as i64;
    let x2: i64 = junction_boxes[r].x as i64;

    x1 * x2
}

/// Add shortest edges, skipping those that would connect vertices already in the same component.
/// Returns the chosen edges (if you want to “mark joined pairs”) and a map root->component_size.
fn connect_circuits(junction_boxes: &[JunctionBox], edges_to_add: usize) -> Vec<usize> {
    let n = junction_boxes.len();
    let edges = calculate_distances(junction_boxes);

    let mut uf = DisjointSet::with_len(n); // :contentReference[oaicite:1]{index=1}

    for idx in 0..edges_to_add {
        let e = edges[idx];

        if !uf.is_joined(e.l_box, e.r_box) {
            uf.join(e.l_box, e.r_box);
        }
    }

    // Compute component sizes (root->size) by querying representatives.
    // DisjointSet doesn't store sizes for you, so we count.
    let mut size_by_root: HashMap<usize, usize> = HashMap::new();
    for v in 0..n {
        // DisjointSet exposes a representative; in this crate it’s called `root` / representative.
        // In 0.8, you can use `uf.find(v)` via the method the crate provides.
        // We'll use `uf.root(v)` pattern defensively:
        let r = uf.root_of(v); // see note below
        *size_by_root.entry(r).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = size_by_root.into_values().collect();
    sizes.sort_unstable();
    sizes
}

pub fn calculate_circuits(junction_boxes: &Vec<JunctionBox>, n: usize) -> usize {
    let mut sizes = connect_circuits(junction_boxes, n);

    let first = sizes.pop().unwrap();
    let second = sizes.pop().unwrap();
    let third = sizes.pop().unwrap();

    first * second * third
}

fn calculate_distances(junction_boxes: &[JunctionBox]) -> Vec<LightString> {
    let n = junction_boxes.len();

    let mut result = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..n {
            let d = junction_boxes[i].distance(&junction_boxes[j]);
            result.push(LightString {
                l_box: i,
                r_box: j,
                distance: d,
            });
        }
    }

    result.sort_unstable_by(|s1, s2| s1.distance.partial_cmp(&s2.distance).unwrap());
    result
}

pub fn parse_locations(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');

            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();

            JunctionBox { x, y, z }
        })
        .collect()
}
