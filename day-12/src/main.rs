use bitflags::bitflags;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(1396562, fence_cost(INPUT));

    let part_2 = fence_cost_bulk(INPUT);

    // Was treating regions that touched at a diagonal as having one
    // (shared) side.
    assert!(part_2 > 836796);
    assert_eq!(844132, part_2);
}

fn fence_cost(s: &str) -> usize {
    let map = parse(s);
    let regions = find_regions(&map);

    regions.iter().map(Region::price).sum()
}

fn fence_cost_bulk(s: &str) -> usize {
    let map = parse(s);
    let regions = find_regions(&map);

    regions.iter().map(Region::price_bulk).sum()
}

type Coord = (usize, usize);

type Map = BTreeMap<Coord, char>;

bitflags! {
    #[derive(Debug, Copy, Clone)]
    struct Direction: u8 {
        const U = 0b01;
        const L = 0b10;
    }
}

fn parse(s: &str) -> Map {
    let mut map = BTreeMap::new();

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            map.insert((x, y), c);
        }
    }

    map
}

fn find_regions(map: &Map) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut visited = BTreeSet::new();

    for &coord in map.keys() {
        explore(map, coord, &mut visited, &mut regions);
    }

    regions
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
        self.n_perimeters() * self.tiles.len()
    }

    fn price_bulk(&self) -> usize {
        self.n_sides() * self.tiles.len()
    }

    fn n_perimeters(&self) -> usize {
        // Only have two total bits, so the cast to usize is fine
        self.perimeter
            .values()
            .map(|d| d.bits().count_ones() as usize)
            .sum()
    }

    fn n_sides(&self) -> usize {
        let mut bounds = None;

        for &(x, y) in self.perimeter.keys() {
            let (min_x, max_x, min_y, max_y) = bounds.get_or_insert((x, x, y, y));
            *min_x = usize::min(*min_x, x);
            *max_x = usize::max(*max_x, x);
            *min_y = usize::min(*min_y, y);
            *max_y = usize::max(*max_y, y);
        }

        let (min_x, max_x, min_y, max_y) = bounds.expect("No perimeter");

        let xs = min_x..=max_x;
        let ys = min_y..=max_y;

        let vert_sides = xs
            .clone()
            .map(|x| {
                let mut cross_edges = 0;

                let chunks = ys.clone().chunk_by(|&y| {
                    let coord = (x, y);
                    let direction = self
                        .perimeter
                        .get(&coord)
                        .copied()
                        .unwrap_or(Direction::empty());

                    if direction.contains(Direction::U) {
                        cross_edges += 1;
                    }

                    (direction.contains(Direction::L), cross_edges % 2)
                });

                chunks.into_iter().filter(|&((v, _), _)| v).count()
            })
            .sum::<usize>();

        let horz_sides = ys
            .map(|y| {
                let mut cross_edges = 0;

                let chunks = xs.clone().chunk_by(|&x| {
                    let coord = (x, y);
                    let direction = self
                        .perimeter
                        .get(&coord)
                        .copied()
                        .unwrap_or(Direction::empty());

                    if direction.contains(Direction::L) {
                        cross_edges += 1;
                    }

                    (direction.contains(Direction::U), cross_edges % 2)
                });

                chunks.into_iter().filter(|&((v, _), _)| v).count()
            })
            .sum::<usize>();

        vert_sides + horz_sides
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

    // E-shaped region full of type E plants
    const EXAMPLE_4: &str = include_str!("../example-4.txt");

    // Two regions of type B plants and a single region of type A plants
    const EXAMPLE_5: &str = include_str!("../example-5.txt");

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

    #[test]
    fn example_1_bulk() {
        assert_eq!(80, fence_cost_bulk(EXAMPLE_1));
    }

    #[test]
    fn example_2_bulk() {
        assert_eq!(436, fence_cost_bulk(EXAMPLE_2));
    }

    #[test]
    fn example_3_bulk() {
        assert_eq!(1206, fence_cost_bulk(EXAMPLE_3));
    }

    #[test]
    fn example_4_bulk() {
        assert_eq!(236, fence_cost_bulk(EXAMPLE_4));
    }

    #[test]
    fn example_5_bulk() {
        assert_eq!(368, fence_cost_bulk(EXAMPLE_5));
    }
}
