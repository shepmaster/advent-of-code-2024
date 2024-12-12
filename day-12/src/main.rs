use bitflags::bitflags;
use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(1396562, fence_cost(INPUT));
}

fn fence_cost(s: &str) -> usize {
    let mut map = BTreeMap::new();

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            map.insert((x, y), c);
        }
    }

    let mut regions = Vec::new();
    let mut visited = BTreeSet::new();

    for &coord in map.keys() {
        explore(&map, coord, &mut visited, &mut regions);
    }

    regions.iter().map(Region::price).sum()
}

type Coord = (usize, usize);

type Map = BTreeMap<Coord, char>;

bitflags! {
    #[derive(Debug)]
    struct Direction: u8 {
        const U = 0b01;
        const L = 0b10;
    }
}

#[derive(Debug)]
struct Region {
    #[allow(dead_code)]
    label: char,
    tiles: BTreeSet<Coord>,
    perimeter: BTreeMap<Coord, Direction>,
}

impl Region {
    fn price(&self) -> usize {
        // Only have two total bits, so the cast to usize is fine
        let n_perimeters = self
            .perimeter
            .values()
            .map(|d| d.bits().count_ones() as usize)
            .sum::<usize>();
        n_perimeters * self.tiles.len()
    }
}

fn explore(map: &Map, coord: Coord, visited: &mut BTreeSet<Coord>, regions: &mut Vec<Region>) {
    if visited.contains(&coord) {
        return;
    }

    let label = map[&coord];
    let mut tiles = BTreeSet::new();
    let mut perimeter = BTreeMap::new();

    let mut to_explore = BTreeSet::from_iter([coord]);

    while let Some(coord) = to_explore.pop_first() {
        let newly_visited = visited.insert(coord);
        if !newly_visited {
            continue;
        }

        tiles.insert(coord);

        let (x, y) = coord;

        match y.checked_sub(1) {
            Some(y) => {
                if map[&(x, y)] == label {
                    to_explore.insert((x, y));
                } else {
                    perimeter
                        .entry(coord)
                        .or_insert_with(Direction::empty)
                        .insert(Direction::U);
                }
            }

            None => {
                // Walked off the grid
                perimeter
                    .entry(coord)
                    .or_insert_with(Direction::empty)
                    .insert(Direction::U);
            }
        }

        match x.checked_sub(1) {
            Some(x) => {
                if map[&(x, y)] == label {
                    to_explore.insert((x, y));
                } else {
                    perimeter
                        .entry(coord)
                        .or_insert_with(Direction::empty)
                        .insert(Direction::L);
                }
            }

            None => {
                // Walked off the grid
                perimeter
                    .entry(coord)
                    .or_insert_with(Direction::empty)
                    .insert(Direction::L);
            }
        }

        let q = (x, y + 1);
        match map.get(&q) {
            Some(&n_label) => {
                if n_label == label {
                    to_explore.insert(q);
                } else {
                    perimeter
                        .entry(q)
                        .or_insert_with(Direction::empty)
                        .insert(Direction::U);
                }
            }

            None => {
                // Walked off the grid
                perimeter
                    .entry(q)
                    .or_insert_with(Direction::empty)
                    .insert(Direction::U);
            }
        }

        let q = (x + 1, y);
        match map.get(&q) {
            Some(&n_label) => {
                if n_label == label {
                    to_explore.insert(q);
                } else {
                    perimeter
                        .entry(q)
                        .or_insert_with(Direction::empty)
                        .insert(Direction::L);
                }
            }

            None => {
                // Walked off the grid
                perimeter
                    .entry(q)
                    .or_insert_with(Direction::empty)
                    .insert(Direction::L);
            }
        }
    }

    regions.push(Region {
        label,
        tiles,
        perimeter,
    });
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../example-1.txt");
    const EXAMPLE_2: &str = include_str!("../example-2.txt");
    const EXAMPLE_3: &str = include_str!("../example-3.txt");

    #[test]
    fn example_1() {
        assert_eq!(140, fence_cost(EXAMPLE_1));
    }

    #[test]
    fn example_2() {
        assert_eq!(772, fence_cost(EXAMPLE_2));
    }

    #[test]
    fn example_3() {
        assert_eq!(1930, fence_cost(EXAMPLE_3));
    }
}
