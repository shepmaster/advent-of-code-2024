use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(413, unique_antinode_locations(INPUT));
}

fn unique_antinode_locations(s: &str) -> usize {
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

    let mut antinodes = BTreeSet::new();

    let is_in_bounds = |c: Coord| (0..=max_x).contains(&c.0) && (0..=max_y).contains(&c.1);

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
                if is_in_bounds(an) {
                    antinodes.insert(an);
                }
            }
        }
    }

    antinodes.len()
}

type Coord = (i32, i32);

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(14, unique_antinode_locations(EXAMPLE));
    }
}
