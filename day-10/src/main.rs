use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(566, trailhead_score_sum(INPUT));
}

fn trailhead_score_sum(s: &str) -> usize {
    let mut map = BTreeMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let c = c.to_digit(10).expect("Invalid digit");

            map.insert((x, y), c);

            max_x = x;
        }

        max_y = y;
    }

    let bounds = Bounds(max_x, max_y);

    let mut paths = BTreeMap::new();

    let coords_for_level = |level| {
        map.iter()
            .filter(move |&(_c, &l)| l == level)
            .map(|(c, _l)| c)
    };

    // Mark all level 9 coordindates as reachable by themselves
    for &c in coords_for_level(9) {
        paths.entry(c).or_insert_with(BTreeSet::new).insert(c);
    }

    // For each previous level, see if we can reach the next-higher level
    for level in (0..9).rev() {
        for &c in coords_for_level(level) {
            let mut found = BTreeSet::new();

            for n in bounds.neighbors(c) {
                if map.get(&n).copied() == Some(level + 1) {
                    if let Some(p) = paths.get(&n) {
                        found.extend(p.iter().copied());
                    }
                }
            }

            paths.insert(c, found);
        }
    }

    // Find how many unique level 9 squares we reached from each level 0
    coords_for_level(0)
        .map(|c| paths.get(c).map_or(0, |f| f.len()))
        .sum()
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Bounds(usize, usize);

impl Bounds {
    fn neighbors(&self, center: Coord) -> impl Iterator<Item = Coord> {
        let (x, y) = center;
        let u = y.checked_sub(1);
        let r = x.checked_add(1).filter(|&x| x <= self.0);
        let d = y.checked_add(1).filter(|&y| y <= self.1);
        let l = x.checked_sub(1);

        [
            u.map(|y| (x, y)),
            r.map(|x| (x, y)),
            d.map(|y| (x, y)),
            l.map(|x| (x, y)),
        ]
        .into_iter()
        .flatten()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../example-1.txt");
    const EXAMPLE_2: &str = include_str!("../example-2.txt");

    #[test]
    fn example_1() {
        assert_eq!(1, trailhead_score_sum(EXAMPLE_1));
    }

    #[test]
    fn example_2() {
        assert_eq!(36, trailhead_score_sum(EXAMPLE_2));
    }
}
