use std::collections::{BTreeMap, BTreeSet, BinaryHeap, btree_map::Entry};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(1395, n_cheats(INPUT, 2, 100));
    assert_eq!(993178, n_cheats(INPUT, 20, 100));
}

fn n_cheats(s: &str, max_cheats: usize, at_least_ps: usize) -> usize {
    let mut start = None;
    let mut end = None;
    let mut walls = BTreeSet::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let coord = [x, y];

            match c {
                '.' => { /* Path */ }

                '#' => {
                    walls.insert(coord);
                }

                'S' => {
                    assert!(start.is_none());
                    start = Some(coord)
                }

                'E' => {
                    assert!(end.is_none());
                    end = Some(coord)
                }

                _ => panic!("Unknown map square {c}"),
            }

            max_x = x;
        }

        max_y = y;
    }

    let start = start.expect("start not found");
    let end = end.expect("end not found");
    let walls = Walls::new(walls, max_x, max_y);

    // Idea: Walk backwards from the end to every place, tracking the
    // distance, then walk forwards from the start using cheating to
    // find how much we can save.

    let distances = distances(&walls, end);
    let saved = cheating_paths(&walls, start, end, distances, max_cheats);

    saved
        .into_iter()
        .filter(|&(k, _)| k >= at_least_ps)
        .map(|(_, v)| v)
        .sum()
}

type Coord = [usize; 2];

struct Walls {
    coords: BTreeSet<Coord>,
    max_x: usize,
    max_y: usize,
}

impl Walls {
    fn new(coords: BTreeSet<Coord>, max_x: usize, max_y: usize) -> Self {
        Self {
            coords,
            max_x,
            max_y,
        }
    }

    fn valid(&self, coord: Coord) -> bool {
        let Self {
            ref coords,
            max_x,
            max_y,
        } = *self;
        let [x, y] = coord;
        x <= max_x && y <= max_y && !coords.contains(&coord)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Candidate {
    coord: Coord,
    score: usize,
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score
            .cmp(&other.score)
            .reverse()
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

type Distances = BTreeMap<Coord, usize>;

fn distances(walls: &Walls, end: Coord) -> Distances {
    let mut to_visit = BTreeSet::from_iter([Candidate {
        coord: end,
        score: 0,
    }]);

    let mut visited = BTreeMap::new();

    while let Some(candidate) = to_visit.pop_first() {
        match visited.entry(candidate.coord) {
            Entry::Vacant(entry) => {
                entry.insert(candidate.score);
            }
            Entry::Occupied(_) => continue,
        }

        let new_to_visit = neighbors(candidate.coord)
            .filter(|&n| walls.valid(n))
            .map(|coord| Candidate {
                coord,
                score: candidate.score + 1,
            });
        to_visit.extend(new_to_visit);
    }

    visited
}

fn cheating_paths(
    walls: &Walls,
    start: Coord,
    end: Coord,
    distances: Distances,
    max_cheats: usize,
) -> BTreeMap<usize, usize> {
    let mut to_visit = BinaryHeap::from_iter([Candidate {
        coord: start,
        score: 0,
    }]);

    let mut visited = BTreeSet::new();
    let mut shortcuts = BTreeMap::new();

    while let Some(candidate) = to_visit.pop() {
        if candidate.coord == end {
            break;
        }

        let newly_visited = visited.insert(candidate.coord);
        if !newly_visited {
            continue;
        }

        // Check the cheating jumps
        {
            let start_distance = distances[&candidate.coord];

            let mut cheat_area = BTreeMap::from_iter([(candidate.coord, 0)]);
            for _ in 0..max_cheats {
                let new_cheats = cheat_area
                    .iter()
                    .flat_map(|(&c, &d)| neighbors(c).map(move |n| (n, d + 1)))
                    .collect::<Vec<_>>();

                for (c, d) in new_cheats {
                    cheat_area.entry(c).or_insert(d);
                }
            }

            let cheating_coords = cheat_area.into_iter().filter(|&(c, _)| walls.valid(c));

            for (cheat_coord, distance) in cheating_coords {
                let saved = (|| {
                    let cheat_distance = *distances.get(&cheat_coord)?;
                    let delta = start_distance.checked_sub(cheat_distance)?;
                    let saved = delta.checked_sub(distance)?;
                    (saved > 0).then_some(saved)
                })();

                if let Some(saved) = saved {
                    *shortcuts.entry(saved).or_insert(0) += 1;
                }
            }
        }

        let new_to_visit = neighbors(candidate.coord)
            .filter(|&n| walls.valid(n))
            .map(|coord| Candidate {
                coord,
                score: candidate.score + 1,
            });
        to_visit.extend(new_to_visit);
    }

    shortcuts
}

fn neighbors(coord: Coord) -> impl Iterator<Item = Coord> {
    let [x, y] = coord;

    let u = y.checked_sub(1);
    let r = x.checked_add(1);
    let d = y.checked_add(1);
    let l = x.checked_sub(1);

    [
        u.map(|y| [x, y]),
        r.map(|x| [x, y]),
        d.map(|y| [x, y]),
        l.map(|x| [x, y]),
    ]
    .into_iter()
    .flatten()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(5, n_cheats(EXAMPLE, 2, 20));
    }

    #[test]
    fn example_20() {
        assert_eq!(285, n_cheats(EXAMPLE, 20, 50));
    }
}
