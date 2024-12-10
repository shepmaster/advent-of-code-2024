use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(566, trailhead_score_sum(INPUT));
    assert_eq!(1324, trailhead_rating_sum(INPUT));
}

fn trailhead_score_sum(s: &str) -> usize {
    let (map, bounds) = parse(s);

    let mut paths = BTreeMap::new();

    let coords_for_level = |l| coords_for_level(&map, l);

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

fn trailhead_rating_sum(s: &str) -> usize {
    let (map, bounds) = parse(s);

    let coords_for_level = |l| coords_for_level(&map, l);

    let mut ratings = BTreeMap::new();

    // Mark all level 9 coordinates as having one path to itself
    ratings.extend(coords_for_level(9).map(|&c| (c, 1)));

    // Find coordindates for each previous level
    for level in (1..=9).rev() {
        for &c in coords_for_level(level) {
            // If we've visited this square
            if let Some(&paths_to_here) = ratings.get(&c) {
                // Check to see if the neighbors are one level away
                for n in bounds.neighbors(c) {
                    if map[&n] == level - 1 {
                        // And mark them as having a number of possible paths
                        *ratings.entry(n).or_insert(0) += paths_to_here;
                    }
                }
            }
        }
    }

    // Add up all possible paths
    coords_for_level(0)
        .map(|c| ratings.get(c).copied().unwrap_or(0))
        .sum()
}

type Coord = (usize, usize);

type Map = BTreeMap<Coord, u32>;

fn parse(s: &str) -> (Map, Bounds) {
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
    (map, bounds)
}

fn coords_for_level(map: &Map, level: u32) -> impl Iterator<Item = &Coord> {
    map.iter()
        .filter(move |&(_c, &l)| l == level)
        .map(|(c, _l)| c)
}

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

    #[test]
    fn example_ratings() {
        assert_eq!(81, trailhead_rating_sum(EXAMPLE_2));
    }
}
