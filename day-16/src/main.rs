use std::collections::{BTreeSet, BinaryHeap};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(91464, best_path_score(INPUT));
}

fn best_path_score(s: &str) -> usize {
    let mut start = None;
    let mut end = None;
    let mut path = BTreeSet::new();

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let coord = [x, y];
            match c {
                '#' => { /* Wall */ }

                '.' => {
                    path.insert(coord);
                }

                'S' => {
                    path.insert(coord);
                    assert!(start.is_none());
                    start = Some(coord);
                }

                'E' => {
                    path.insert(coord);
                    assert!(end.is_none());
                    end = Some(coord);
                }

                o => unreachable!("Unknown square at {coord:?}: {o}"),
            }
        }
    }

    let start = start.expect("Start not found");
    let end = end.expect("End not found");

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Candidate {
        coord: start,
        direction: Direction::E,
        score: 0,
    });

    let mut visited = BTreeSet::new();

    while let Some(candidate) = to_visit.pop() {
        let Candidate {
            coord,
            direction,
            score,
        } = candidate;

        if coord == end {
            return score;
        }

        let newly_visited = visited.insert((candidate.coord, candidate.direction));
        if !newly_visited {
            continue;
        }

        let forward = direction.advance(coord);
        if path.contains(&forward) {
            to_visit.push(Candidate {
                coord: forward,
                direction,
                score: score + 1,
            });
        }

        let cw = direction.clockwise();
        to_visit.push(Candidate {
            coord,
            direction: cw,
            score: score + 1000,
        });

        let ccw = direction.counter_clockwise();
        to_visit.push(Candidate {
            coord,
            direction: ccw,
            score: score + 1000,
        });
    }

    panic!("No path found");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn advance(self, coord: Coord) -> Coord {
        use Direction::*;

        let [x, y] = coord;

        match self {
            N => [x, y - 1],
            E => [x + 1, y],
            S => [x, y + 1],
            W => [x - 1, y],
        }
    }

    fn clockwise(self) -> Self {
        use Direction::*;

        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }

    fn counter_clockwise(self) -> Self {
        use Direction::*;

        match self {
            N => W,
            E => N,
            S => E,
            W => S,
        }
    }
}

type Coord = [usize; 2];

#[derive(Debug, PartialEq, Eq)]
struct Candidate {
    coord: Coord,
    direction: Direction,
    score: usize,
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../example-1.txt");
    const EXAMPLE_2: &str = include_str!("../example-2.txt");

    #[test]
    fn example_1() {
        assert_eq!(7036, best_path_score(EXAMPLE_1));
    }

    #[test]
    fn example_2() {
        assert_eq!(11048, best_path_score(EXAMPLE_2));
    }
}
