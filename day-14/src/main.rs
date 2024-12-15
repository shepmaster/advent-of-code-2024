use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input.txt");
const DIMENSIONS: [i32; 2] = [101, 103];

fn main() {
    assert_eq!(226179492, safety_factor(INPUT, DIMENSIONS, 100));
}

fn safety_factor(s: &str, dimensions: [i32; 2], seconds: i32) -> usize {
    let mut quads = BTreeMap::new();

    for l in s.lines() {
        let (p, v) = l.split_once(' ').expect("position / velocity malformed");

        let parse_one = |v: &str| {
            let (_, v) = v.split_once('=').expect("equal sign missing");
            let (x, y) = v.split_once(',').expect("comma missing");
            [x, y].map(|v| v.parse::<i32>().expect("number invalid"))
        };

        let p = parse_one(p);
        let v = parse_one(v);

        // Distance moved in total
        let d = v.map(|v| v * seconds);

        // Next position
        let n = [p[0] + d[0], p[1] + d[1]];

        // Next position, wrapped around the grid edges
        let n1 = [
            n[0].rem_euclid(dimensions[0]),
            n[1].rem_euclid(dimensions[1]),
        ];

        if let Some(q) = Quadrant::categorize(dimensions, n1) {
            *quads.entry(q).or_insert(0) += 1;
        }
    }

    quads.values().product()
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
