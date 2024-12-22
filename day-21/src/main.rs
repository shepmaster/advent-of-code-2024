use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    fmt,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(163086, sum_of_complexities(INPUT));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum KeypadDigit {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    A,
}

impl KeypadDigit {
    const ALL: [Self; 11] = [
        Self::N0,
        Self::N1,
        Self::N2,
        Self::N3,
        Self::N4,
        Self::N5,
        Self::N6,
        Self::N7,
        Self::N8,
        Self::N9,
        Self::A,
    ];

    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    fn mapping() -> BTreeMap<(Self, Self), Direction> {
        use Direction::*;
        use KeypadDigit as K;

        let one_direction = [
            // Horizontal
            ((K::N7, K::N8), R),
            ((K::N8, K::N9), R),
            ((K::N4, K::N5), R),
            ((K::N5, K::N6), R),
            ((K::N1, K::N2), R),
            ((K::N2, K::N3), R),
            ((K::N0, K::A), R),
            // Vertical
            ((K::N7, K::N4), D),
            ((K::N4, K::N1), D),
            ((K::N8, K::N5), D),
            ((K::N5, K::N2), D),
            ((K::N2, K::N0), D),
            ((K::N9, K::N6), D),
            ((K::N6, K::N3), D),
            ((K::N3, K::A), D),
        ];

        let other_direction = one_direction
            .into_iter()
            .map(|((a, b), d)| ((b, a), d.invert()));

        one_direction.into_iter().chain(other_direction).collect()
    }

    fn neighbors() -> BTreeMap<Self, Vec<Self>> {
        let mut neighbors = BTreeMap::new();
        for (a, b) in Self::mapping().into_keys() {
            neighbors.entry(a).or_insert_with(Vec::new).push(b);
        }
        neighbors
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum KeypadDir {
    U,
    R,
    D,
    L,
    A,
}

impl KeypadDir {
    const ALL: [Self; 5] = [Self::U, Self::R, Self::D, Self::L, Self::A];

    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    fn mapping() -> BTreeMap<(Self, Self), Direction> {
        use Direction::*;
        use KeypadDir as K;

        let one_direction = [
            // Horizontal
            ((K::U, K::A), R),
            ((K::L, K::D), R),
            ((K::D, K::R), R),
            // Vertical
            ((K::U, K::D), D),
            ((K::A, K::R), D),
        ];

        let other_direction = one_direction
            .into_iter()
            .map(|((a, b), d)| ((b, a), d.invert()));

        one_direction.into_iter().chain(other_direction).collect()
    }

    fn neighbors() -> BTreeMap<Self, Vec<Self>> {
        let mut neighbors = BTreeMap::new();
        for (a, b) in Self::mapping().into_keys() {
            neighbors.entry(a).or_insert_with(Vec::new).push(b);
        }
        neighbors
    }

    #[allow(dead_code)]
    fn as_string(this: &[Self]) -> String {
        this.iter().map(|s| s.as_char()).collect()
    }

    #[allow(dead_code)]
    fn as_char(self) -> char {
        use KeypadDir::*;

        match self {
            U => '^',
            R => '>',
            D => 'v',
            L => '<',
            A => 'A',
        }
    }
}

impl From<Direction> for KeypadDir {
    fn from(value: Direction) -> Self {
        use Direction as D;
        use KeypadDir as K;

        match value {
            D::U => K::U,
            D::R => K::R,
            D::D => K::D,
            D::L => K::L,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    fn invert(self) -> Self {
        use Direction::*;

        match self {
            U => D,
            R => L,
            D => U,
            L => R,
        }
    }
}

fn sum_of_complexities(s: &str) -> usize {
    s.lines()
        .map(|l| {
            let buttons = l
                .chars()
                .map(|c| {
                    use KeypadDigit::*;
                    match c {
                        '0' => N0,
                        '1' => N1,
                        '2' => N2,
                        '3' => N3,
                        '4' => N4,
                        '5' => N5,
                        '6' => N6,
                        '7' => N7,
                        '8' => N8,
                        '9' => N9,
                        'A' => A,
                        _ => panic!("Unknown character {c}"),
                    }
                })
                .collect();

            let value = l.trim().trim_end_matches('A').parse::<usize>().unwrap();
            let minimum_buttons = minimum_button_presses(buttons);

            value * minimum_buttons
        })
        .sum()
}

fn minimum_button_presses(buttons: Vec<KeypadDigit>) -> usize {
    let digit_paths = all_paths_digits();
    let dir_paths = all_paths_directions();

    let mut buttons_set = buttons_to_keypad_directions(vec![buttons], &digit_paths, KeypadDigit::A);

    for _ in 0..2 {
        buttons_set = buttons_to_keypad_directions(buttons_set, &dir_paths, KeypadDir::A);
    }

    buttons_set.iter().map(|b| b.len()).min().unwrap()
}

fn buttons_to_keypad_directions<T: Copy + Ord>(
    buttons_set: Vec<Vec<T>>,
    paths: &BTreeMap<(T, T), BTreeSet<Vec<Direction>>>,
    start: T,
) -> Vec<Vec<KeypadDir>> {
    buttons_set
        .into_iter()
        .flat_map(|buttons| {
            let mut current = start;
            let mut my_paths = vec![vec![]];

            for b in buttons {
                // Just push the same button again
                if current == b {
                    for p in &mut my_paths {
                        p.push(KeypadDir::A);
                    }
                    continue;
                }

                let new_paths = &paths[&(current, b)];

                my_paths = my_paths
                    .into_iter()
                    .flat_map(|p| {
                        new_paths.iter().map(move |np| {
                            let mut p = p.clone();
                            p.extend(np.iter().map(|&d| KeypadDir::from(d)));
                            p.push(KeypadDir::A);
                            p
                        })
                    })
                    .collect();

                current = b;
            }

            my_paths
        })
        .collect()
}

fn all_paths_digits() -> BTreeMap<(KeypadDigit, KeypadDigit), BTreeSet<Vec<Direction>>> {
    let mapping = KeypadDigit::mapping();
    let neighbors = KeypadDigit::neighbors();

    let pairs = KeypadDigit::ALL
        .into_iter()
        .cartesian_product(KeypadDigit::ALL);

    all_paths(mapping, neighbors, pairs)
}

fn all_paths_directions() -> BTreeMap<(KeypadDir, KeypadDir), BTreeSet<Vec<Direction>>> {
    let mapping = KeypadDir::mapping();
    let neighbors = KeypadDir::neighbors();

    let pairs = KeypadDir::ALL.into_iter().cartesian_product(KeypadDir::ALL);

    all_paths(mapping, neighbors, pairs)
}

fn all_paths<T: fmt::Debug + Copy + Ord>(
    mapping: BTreeMap<(T, T), Direction>,
    neighbors: BTreeMap<T, Vec<T>>,
    pairs: impl Iterator<Item = (T, T)>,
) -> BTreeMap<(T, T), BTreeSet<Vec<Direction>>> {
    let mut commands = BTreeMap::new();

    for (start, end) in pairs {
        // Could be smarter: if A,B are already known and we are
        // looking for B,A could invert the steps, but eh.

        if start == end {
            continue;
        }

        let mut to_visit = BinaryHeap::from_iter([Candidate {
            coord: start,
            score: 0,
        }]);

        let mut visited = BTreeMap::new();

        // Relatively normal grid visiting...
        while let Some(candidate) = to_visit.pop() {
            if candidate.coord == end {
                break;
            }

            for &neighbor in &neighbors[&candidate.coord] {
                let visited = visited
                    .entry(neighbor)
                    .or_insert_with(|| (candidate.score, Vec::new()));

                match candidate.score.cmp(&visited.0) {
                    Ordering::Less => {
                        visited.0 = candidate.score;
                        visited.1.clear();
                        visited.1.push(candidate.coord);
                    }
                    Ordering::Equal => {
                        visited.1.push(candidate.coord);
                    }
                    Ordering::Greater => continue,
                }

                to_visit.push(Candidate {
                    coord: neighbor,
                    score: candidate.score + 1,
                });
            }
        }

        // Walk backwards through our visited data to collect all paths
        let mut paths = vec![vec![end]];

        loop {
            let mut next_paths = Vec::new();
            let mut modified = false;

            for p in paths {
                let end = *p.last().unwrap();

                if end == start {
                    next_paths.push(p);
                } else {
                    let (_, from) = &visited[&end];
                    next_paths.extend(from.iter().map(|&f| {
                        let mut p = p.clone();
                        p.push(f);
                        p
                    }));
                    modified = true;
                }
            }

            paths = next_paths;

            if !modified {
                break;
            }
        }

        // With all the paths, we figure out what directions each step would be
        for p in paths {
            let start = *p.last().unwrap();
            let end = *p.first().unwrap();

            let directions = p
                .iter()
                .rev()
                .tuple_windows()
                .map(|(&a, &b)| mapping[&(a, b)])
                .collect();

            commands
                .entry((start, end))
                .or_insert_with(BTreeSet::new)
                .insert(directions);
        }
    }

    commands
}

#[derive(Debug, PartialEq, Eq)]
struct Candidate<T> {
    coord: T,
    score: usize,
}

impl<T: Ord> Ord for Candidate<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl<T: Ord> PartialOrd for Candidate<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(126384, sum_of_complexities(EXAMPLE));
        panic!()
    }
}
