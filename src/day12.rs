/*
    day10:

    The Elves are worried their presents won't fit under the trees.
    The puzzle input is a summary of the situation, the shapes are measured in standard units.
    It contans first a list of the present's shapes.
    Second the size of the region and a list of the number of presents needed to fit into the region.
    Shapes need to be rotated and flipped as necessary tom ake the mfit in the region grid.
    # cannot go in the asme place on the grid.
    The Elves want to know how many of the regions can fit the presents listed.

    Part one:

    How many of the regions can fit the needed  shapes?

    Part two:



*/
pub const INPUT: &str = include_str!("inputs/day12.txt");

pub const TEST_INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

// -------------------------
// Core types
// -------------------------

#[derive(Clone, Debug)]
struct Shape {
    id: usize,
    cells: Vec<(i32, i32)>, // '#' only, normalized to top-left
}

#[derive(Clone, Debug)]
struct RegionSpec {
    w: usize,
    h: usize,
    counts: Vec<usize>, // counts[shape_id]
}

#[derive(Debug)]
pub struct Puzzle {
    shapes: Vec<Shape>,
    regions: Vec<RegionSpec>,
}

// -------------------------
// Parsing (single input containing shapes + regions)
// -------------------------

pub fn parse_puzzle(input: &str) -> Puzzle {
    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<RegionSpec> = Vec::new();

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        let line = line.trim_end();
        if line.trim().is_empty() {
            continue;
        }

        let Some((lhs, rhs)) = line.split_once(':') else {
            continue;
        };
        let lhs = lhs.trim();
        let rhs = rhs.trim();

        // Region: "{w}x{h}: counts..."
        if let Some((w_str, h_str)) = lhs.split_once('x') {
            if let (Ok(w), Ok(h)) = (w_str.trim().parse::<usize>(), h_str.trim().parse::<usize>()) {
                let counts: Vec<usize> = rhs
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().expect("count"))
                    .collect();

                regions.push(RegionSpec { w, h, counts });
                continue;
            }
        }

        // Shape header: "{id}:" with empty rhs
        if rhs.is_empty() {
            let id: usize = lhs.parse().expect("shape id");

            let mut grid: Vec<Vec<char>> = Vec::new();

            // Read until blank line or next header (shape or region).
            while let Some(&peek) = lines.peek() {
                let p = peek.trim_end();
                if p.trim().is_empty() {
                    lines.next(); // consume blank line
                    break;
                }
                if looks_like_header(p) {
                    break;
                }
                grid.push(p.chars().collect());
                lines.next();
            }

            let cells = normalize_cells(extract_hash_cells(&grid));
            shapes.push(Shape { id, cells });
        }
    }

    shapes.sort_by_key(|s| s.id);

    for r in &regions {
        assert!(
            r.counts.len() == shapes.len(),
            "Region {}x{} has {} counts, but there are {} shapes",
            r.w,
            r.h,
            r.counts.len(),
            shapes.len()
        );
    }

    Puzzle { shapes, regions }
}

fn looks_like_header(line: &str) -> bool {
    let l = line.trim();
    let Some((lhs, _)) = l.split_once(':') else {
        return false;
    };
    let lhs = lhs.trim();

    if lhs.parse::<usize>().is_ok() {
        return true; // "N:"
    }
    if let Some((w_str, h_str)) = lhs.split_once('x') {
        return w_str.trim().parse::<usize>().is_ok() && h_str.trim().parse::<usize>().is_ok();
    }
    false
}

fn extract_hash_cells(grid: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut cells = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == '#' {
                cells.push((x as i32, y as i32));
            }
        }
    }
    cells
}

fn normalize_cells(mut cells: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if cells.is_empty() {
        return cells;
    }

    let (minx, miny) = cells
        .iter()
        .fold((i32::MAX, i32::MAX), |(ax, ay), &(x, y)| {
            (ax.min(x), ay.min(y))
        });

    for c in &mut cells {
        c.0 -= minx;
        c.1 -= miny;
    }

    cells.sort_unstable();
    cells
}

// -------------------------
// AoC-style fast solvability test
// -------------------------
//
// Assumptions this test uses (matches what you described):
// 1) Holes in the region are allowed (we don't need full coverage).
// 2) Only '#' cells occupy space; '.' is empty.
// 3) All pieces fit inside a 3x3 bounding box (rotations/flips allowed).
//
// Then:
// - Necessary condition: total occupied area <= region area
// - Sufficient condition: there are enough disjoint 3x3 "slots" to place all pieces
//
// If your AoC input is designed kindly, regions will fall into either "obviously no"
// or "obviously yes" by these two checks, avoiding NP-hard search.

fn shape_areas(shapes: &[Shape]) -> Vec<usize> {
    let mut areas = vec![0usize; shapes.len()];
    for s in shapes {
        areas[s.id] = s.cells.len();
    }
    areas
}

fn piece_count(region: &RegionSpec) -> usize {
    region.counts.iter().sum()
}

fn fits_in_3x3(shape: &Shape) -> bool {
    // shape.cells are normalized, so max x/y determine bbox
    let (mut maxx, mut maxy) = (0i32, 0i32);
    for &(x, y) in &shape.cells {
        maxx = maxx.max(x);
        maxy = maxy.max(y);
    }
    (maxx + 1) <= 3 && (maxy + 1) <= 3
}

fn region_is_solvable_fast(shapes: &[Shape], region: &RegionSpec) -> bool {
    // Enforce the key assumption explicitly, so you don't get false positives silently.
    assert!(
        shapes.iter().all(fits_in_3x3),
        "Fast test assumes every shape fits in a 3x3 bounding box"
    );

    let areas = shape_areas(shapes);

    let needed_area: usize = region
        .counts
        .iter()
        .enumerate()
        .map(|(sid, &cnt)| cnt * areas[sid])
        .sum();

    let region_area = region.w * region.h;

    // Necessary: must have enough cells to host all occupied '#' cells.
    if needed_area > region_area {
        return false;
    }

    // Sufficient (under the 3x3-bbox + holes-allowed model):
    // if we can carve enough disjoint 3x3 blocks, we can place one piece per block.
    let k = piece_count(region);
    let slots = (region.w / 3) * (region.h / 3);

    slots >= k
}

// -------------------------
// Runner
// -------------------------

pub fn get_possible_regions(puzzle: &Puzzle) -> i32 {
    let mut count = 0;

    for (i, region) in puzzle.regions.iter().enumerate() {
        if region_is_solvable_fast(&puzzle.shapes, region) {
            count += 1;

            println!("Region #{i} {}x{}: SOLVABLE", region.w, region.h,);
        }
    }

    count
}
