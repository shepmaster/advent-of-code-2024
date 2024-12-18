use std::collections::{BTreeSet, BinaryHeap};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(438, simulate(INPUT, [70, 70], 1024));
}

type Coord = [usize; 2];

fn simulate(s: &str, max: Coord, n_steps: usize) -> usize {
    let positions = s.lines().map(|l| {
        let (x, y) = l.split_once(',').expect("line malformed");
        [x, y].map(|v| v.parse::<usize>().expect("coordinate malformed"))
    });

    let corrupted = positions.take(n_steps).collect::<BTreeSet<_>>();

    let start = [0, 0];
    let mut to_visit = BinaryHeap::from_iter([Candidate {
        coord: start,
        steps: 0,
    }]);
    let mut visited = BTreeSet::new();

    while let Some(candidate) = to_visit.pop() {
        let Candidate { coord, steps } = candidate;

        if coord == max {
            return steps;
        }

        let newly_inserted = visited.insert(coord);
        if !(newly_inserted) {
            continue;
        }

        for n in neighbors(coord, max) {
            if !corrupted.contains(&n) {
                to_visit.push(Candidate {
                    coord: n,
                    steps: steps + 1,
                });
            }
        }
    }

    panic!("No path found");
}

#[derive(Debug, PartialEq, Eq)]
struct Candidate {
    coord: Coord,
    steps: usize,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps).reverse()
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(coord: Coord, max: Coord) -> impl Iterator<Item = Coord> {
    let [x, y] = coord;
    let [max_x, max_y] = max;

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
    .filter(move |&[x, y]| x <= max_x && y <= max_y)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(22, simulate(EXAMPLE, [6, 6], 12));
    }
}
