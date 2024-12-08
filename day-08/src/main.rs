use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(413, unique_antinode_locations(INPUT));
    assert_eq!(1417, unique_antinode_locations_resonant(INPUT));
}

fn unique_antinode_locations(s: &str) -> usize {
    let (antennas, max) = parse(s);

    let mut antinodes = BTreeSet::new();

    for transmitters in antennas.values() {
        // For each pair of transmitters
        for (t1, t2) in transmitters.iter().tuple_combinations() {
            // Find the delta of the positions
            let dx = t1.0 - t2.0;
            let dy = t1.1 - t2.1;

            // Use the delta to compute the potential antinode locations
            let an1 = (t1.0 + dx, t1.1 + dy);
            let an2 = (t2.0 - dx, t2.1 - dy);

            // eprintln!("{t1:?} <-> {t2:?} [({dx},{dy})]: {an1:?} & {an2:?}");

            for an in [an1, an2] {
                if max.is_in_bounds(an) {
                    antinodes.insert(an);
                }
            }
        }
    }

    antinodes.len()
}

fn unique_antinode_locations_resonant(s: &str) -> usize {
    let (antennas, max) = parse(s);

    let mut antinodes = BTreeSet::new();

    for transmitters in antennas.values() {
        // For each pair of transmitters
        for (&t1, &t2) in transmitters.iter().tuple_combinations() {
            // Find the delta of the positions
            let dx = t1.0 - t2.0;
            let dy = t1.1 - t2.1;

            // Starting at a transmitter, walk the grid until we fall
            // off. Each location is an antinode.
            let mut current = t1;
            while max.is_in_bounds(current) {
                antinodes.insert(current);

                current.0 += dx;
                current.1 += dy;
            }

            // Same thing in the opposite direction.
            let mut current = t1;
            while max.is_in_bounds(current) {
                antinodes.insert(current);

                current.0 -= dx;
                current.1 -= dy;
            }
        }
    }

    antinodes.len()
}

type Coord = (i32, i32);

type Grid = BTreeMap<char, Vec<Coord>>;

#[derive(Copy, Clone)]
struct Max(i32, i32);

impl Max {
    fn is_in_bounds(self, c: Coord) -> bool {
        (0..=self.0).contains(&c.0) && (0..=self.1).contains(&c.1)
    }
}

fn parse(s: &str) -> (Grid, Max) {
    let mut max_x = 0;
    let mut max_y = 0;

    let mut antennas = BTreeMap::new();

    for (y, l) in s.lines().enumerate() {
        let y = i32::try_from(y).expect("Y out of range");
        max_y = y;

        for (x, c) in l.chars().enumerate() {
            let x = i32::try_from(x).expect("X out of range");
            max_x = x;

            if c == '.' {
                continue;
            }

            antennas.entry(c).or_insert_with(Vec::new).push((x, y));
        }
    }

    (antennas, Max(max_x, max_y))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(14, unique_antinode_locations(EXAMPLE));
    }

    #[test]
    fn example_resonance() {
        assert_eq!(34, unique_antinode_locations_resonant(EXAMPLE));
    }
}
