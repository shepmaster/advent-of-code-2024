use std::collections::BTreeMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");
const DIMENSIONS: [i32; 2] = [101, 103];

fn main() {
    assert_eq!(226179492, safety_factor(INPUT, DIMENSIONS, 100));

    let part_2 = search(INPUT, DIMENSIONS);
    // Random number just to know how much I need to look for
    assert!(part_2 < 1_000_000);
    // Oops, off-by-one on the time
    assert!(part_2 > 7501);
    assert_eq!(7502, part_2);
}

fn safety_factor(s: &str, dimensions: [i32; 2], seconds: i32) -> usize {
    let mut quads = BTreeMap::new();

    for l in s.lines() {
        let (p, v) = parse(l);

        let n1 = simulate(dimensions, p, v, seconds);

        if let Some(q) = Quadrant::categorize(dimensions, n1) {
            *quads.entry(q).or_insert(0) += 1;
        }
    }

    quads.values().product()
}

fn search(s: &str, dimensions: [i32; 2]) -> usize {
    let mut robots = s.lines().map(parse).collect::<Vec<_>>();

    let mut pos = BTreeMap::new();

    // loop {
    for k in 0..1_000_000 {
        pos.clear();

        for (p, v) in &mut robots {
            *p = simulate(dimensions, *p, *v, 1);
            *pos.entry(*p).or_insert(0) += 1;
        }

        let seconds = k + 1;

        // Any row/column completely filled?
        // Not in 1st million
        //
        // Any row/column completely empty?
        // Quite a few in first 1M
        //
        // Symmetric around X/Y axis?
        // Not obviously useful

        // Any rows of 10+ contiguous robots?

        let has_row_of_more_than_ten_robots = (0..dimensions[1]).any(|y| {
            let runs = (0..dimensions[0]).chunk_by(|&x| pos.contains_key(&[x, y]));
            runs.into_iter().any(|(k, i)| k && i.count() > 10)
        });

        if has_row_of_more_than_ten_robots {
            print(dimensions, &pos);
            return seconds;
        }
    }

    panic!("Did not find a result");
}

fn parse(l: &str) -> ([i32; 2], [i32; 2]) {
    let (p, v) = l.split_once(' ').expect("position / velocity malformed");

    let parse_one = |v: &str| {
        let (_, v) = v.split_once('=').expect("equal sign missing");
        let (x, y) = v.split_once(',').expect("comma missing");
        [x, y].map(|v| v.parse::<i32>().expect("number invalid"))
    };

    let p = parse_one(p);
    let v = parse_one(v);

    (p, v)
}

fn simulate(dimensions: [i32; 2], p: [i32; 2], v: [i32; 2], seconds: i32) -> [i32; 2] {
    // Distance moved in total
    let d = v.map(|v| v * seconds);

    // Next position
    let n = [p[0] + d[0], p[1] + d[1]];

    // Next position, wrapped around the grid edges
    [
        n[0].rem_euclid(dimensions[0]),
        n[1].rem_euclid(dimensions[1]),
    ]
}

fn print(dimensions: [i32; 2], pos: &BTreeMap<[i32; 2], i32>) {
    for y in 0..dimensions[1] {
        for x in 0..dimensions[0] {
            match pos.get(&[x, y]) {
                Some(_) => print!("#"),
                None => print!(" "),
            }
        }
        println!();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl Quadrant {
    fn categorize(dimensions: [i32; 2], point: [i32; 2]) -> Option<Self> {
        use std::cmp::Ordering::*;

        let [mx, my] = dimensions.map(|v| v / 2);
        let [x, y] = point;

        let q = match (x.cmp(&mx), y.cmp(&my)) {
            (_, Equal) | (Equal, _) => return None,

            (Less, Less) => Self::Q1,
            (Greater, Less) => Self::Q2,
            (Less, Greater) => Self::Q3,
            (Greater, Greater) => Self::Q4,
        };

        Some(q)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const DIMENSIONS: [i32; 2] = [11, 7];

    #[test]
    fn example() {
        assert_eq!(12, safety_factor(EXAMPLE, DIMENSIONS, 100));
    }
}
