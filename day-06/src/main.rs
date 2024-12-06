use std::collections::BTreeSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(5239, distinct_guard_positions(INPUT));
}

fn distinct_guard_positions(s: &str) -> usize {
    let mut grid = BTreeSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut guard = None;

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '^' => {
                    assert!(guard.is_none());
                    guard = Some((x, y));
                }

                '#' => {
                    grid.insert((x, y));
                }

                '.' => { /* no-op */ }

                o => panic!("Unknown sigil {o}"),
            }

            max_x = x;
        }

        max_y = y;
    }

    let mut guard = guard.expect("Did not find a guard");

    use Direction::*;
    let mut direction = U;
    let mut visited = BTreeSet::new();

    loop {
        visited.insert(guard);

        let (x, y) = guard;
        match direction {
            U => {
                let Some(y) = y.checked_sub(1) else {
                    // Walked off grid
                    break;
                };
                if grid.contains(&(x, y)) {
                    direction = R;
                    continue;
                }
                guard.1 = y;
            }

            R => {
                let x = x + 1;
                if x > max_x {
                    // Walked off grid
                    break;
                };
                if grid.contains(&(x, y)) {
                    direction = D;
                    continue;
                }
                guard.0 = x;
            }

            D => {
                let y = y + 1;
                if y > max_y {
                    // Walked off grid
                    break;
                };
                if grid.contains(&(x, y)) {
                    direction = L;
                    continue;
                }
                guard.1 = y;
            }

            L => {
                let Some(x) = x.checked_sub(1) else {
                    // Walked off grid
                    break;
                };
                if grid.contains(&(x, y)) {
                    direction = U;
                    continue;
                }
                guard.0 = x;
            }
        }
    }

    visited.len()
}

enum Direction {
    U,
    R,
    D,
    L,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(41, distinct_guard_positions(EXAMPLE));
    }
}
