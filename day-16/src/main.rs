use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(91464, best_path_score(INPUT));
    assert_eq!(494, best_seats(INPUT));
}

fn best_path_score(s: &str) -> usize {
    let (start, end, path) = parse(s);

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Candidate {
        coord: start,
        direction: Direction::E,
        score: 0,
        my_path: BTreeSet::new(),
    });

    let mut visited = BTreeSet::new();

    while let Some(candidate) = to_visit.pop() {
        if candidate.coord == end {
            return candidate.score;
        }

        let newly_visited = visited.insert((candidate.coord, candidate.direction));
        if !newly_visited {
            continue;
        }

        add_candidates(&path, candidate, &mut to_visit);
    }

    panic!("No path found")
}

fn best_seats(s: &str) -> usize {
    let (start, end, path) = parse(s);

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Candidate {
        coord: start,
        direction: Direction::E,
        score: 0,
        my_path: BTreeSet::new(),
    });

    let mut visited = BTreeMap::new();

    while let Some(candidate) = to_visit.pop() {
        if candidate.coord == end {
            let min_score = candidate.score;

            let mut best_path_squares = BTreeSet::from_iter([start, end]);

            best_path_squares.extend(candidate.path_squares());

            // Find the rest
            while let Some(candidate) = to_visit.pop() {
                // Doesn't end in the right spot
                if candidate.coord != end {
                    continue;
                }

                // No longer a best path
                if candidate.score > min_score {
                    break;
                }

                best_path_squares.extend(candidate.path_squares());
            }

            return best_path_squares.len();
        }

        let min_score = *visited
            .entry((candidate.coord, candidate.direction))
            .or_insert(candidate.score);
        if candidate.score > min_score {
            // We've visited this coordinate / direction before and it
            // was cheaper, so don't care about this path.
            continue;
        }

        add_candidates(&path, candidate, &mut to_visit);
    }

    panic!("No path found");
}

type Coord = [usize; 2];

type Path = BTreeSet<Coord>;

fn parse(s: &str) -> (Coord, Coord, Path) {
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

    (start, end, path)
}

fn add_candidates(path: &Path, candidate: Candidate, to_visit: &mut BinaryHeap<Candidate>) {
    let Candidate {
        coord,
        direction,
        score,
        my_path,
    } = candidate;

    let forward = direction.advance(coord);
    if path.contains(&forward) {
        let mut my_path = my_path.clone();
        my_path.insert((forward, direction));

        to_visit.push(Candidate {
            coord: forward,
            direction,
            score: score + 1,
            my_path,
        });
    }

    let cw = direction.clockwise();
    to_visit.push(Candidate {
        coord,
        direction: cw,
        score: score + 1000,
        my_path: my_path.clone(),
    });

    let ccw = direction.counter_clockwise();
    to_visit.push(Candidate {
        coord,
        direction: ccw,
        score: score + 1000,
        my_path,
    });
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

#[derive(Debug, Eq)]
struct Candidate {
    coord: Coord,
    direction: Direction,
    score: usize,
    my_path: BTreeSet<(Coord, Direction)>,
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.direction == other.direction && self.score == other.score
    }
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

impl Candidate {
    fn path_squares(&self) -> impl Iterator<Item = Coord> + '_ {
        self.my_path.iter().map(|&(s, _)| s)
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

    #[test]
    fn example_1_best_seats() {
        assert_eq!(45, best_seats(EXAMPLE_1));
    }

    #[test]
    fn example_2_best_seats() {
        assert_eq!(64, best_seats(EXAMPLE_2));
    }
}
