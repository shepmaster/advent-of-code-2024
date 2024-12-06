use bitflags::bitflags;
use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(5239, distinct_guard_positions(INPUT));

    let part_2 = new_obstruction_positions(INPUT);

    assert!(part_2 < 1932);
    // Forgot to check that we _could_ put the obstacle in the
    // upcoming square (i.e. that we hadn't previously walked through
    // the square).

    assert_eq!(1753, part_2);
}

fn distinct_guard_positions(s: &str) -> usize {
    use Direction::*;

    let (grid, max, mut guard) = parse(s);

    let mut direction = U;
    let mut visited = BTreeSet::new();

    loop {
        visited.insert(guard);

        let Some(next) = step(guard, direction, max) else {
            break;
        };

        if grid.contains(&next) {
            direction = direction.turn();
            continue;
        }

        guard = next;
    }

    visited.len()
}

fn new_obstruction_positions(s: &str) -> usize {
    use Direction::*;

    let (mut grid, max, mut guard) = parse(s);

    let mut direction = U;
    let mut visited = BTreeSet::new();
    let mut possible_loops = 0;

    loop {
        visited.insert(guard);

        let Some(next) = step(guard, direction, max) else {
            break;
        };

        if grid.contains(&next) {
            direction = direction.turn();
            continue;
        } else if !visited.contains(&next) {
            // But what if there _was_ an obstacle?
            grid.insert(next);
            if is_loop(&grid, guard, direction, max) {
                possible_loops += 1;
            }
            grid.remove(&next);
        }

        guard = next;
    }

    possible_loops
}

fn is_loop(grid: &Grid, mut guard: Coord, mut direction: Direction, max: Max) -> bool {
    let mut visited = BTreeMap::new();

    loop {
        let footprint = direction.to_footprint();
        let square = visited.entry(guard).or_insert_with(Footprint::empty);

        if square.contains(footprint) {
            return true;
        }

        square.insert(footprint);

        let Some(next) = step(guard, direction, max) else {
            return false;
        };

        if grid.contains(&next) {
            direction = direction.turn();
            continue;
        }

        guard = next;
    }
}

type Grid = BTreeSet<Coord>;

type Coord = (usize, usize);

#[derive(Copy, Clone)]
struct Max(usize, usize);

fn parse(s: &str) -> (Grid, Max, Coord) {
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

    let guard = guard.expect("Did not find a guard");
    (grid, Max(max_x, max_y), guard)
}

#[derive(Copy, Clone)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    fn turn(self) -> Self {
        use Direction::*;

        match self {
            U => R,
            R => D,
            D => L,
            L => U,
        }
    }

    fn to_footprint(self) -> Footprint {
        use Direction::*;

        match self {
            U => Footprint::U,
            R => Footprint::R,
            D => Footprint::D,
            L => Footprint::L,
        }
    }
}

fn step(coord: Coord, direction: Direction, max: Max) -> Option<Coord> {
    use Direction::*;

    let (x, y) = coord;

    let next = match direction {
        U => {
            // May walk off grid
            let y = y.checked_sub(1)?;
            (x, y)
        }

        R => {
            let x = x + 1;
            if x > max.0 {
                // Walked off grid
                return None;
            };
            (x, y)
        }

        D => {
            let y = y + 1;
            if y > max.1 {
                // Walked off grid
                return None;
            };
            (x, y)
        }

        L => {
            // May walk off grid
            let x = x.checked_sub(1)?;
            (x, y)
        }
    };
    Some(next)
}

bitflags! {
    #[derive(Copy, Clone)]
    struct Footprint: u8 {
        const U = 0b0001;
        const R = 0b0010;
        const D = 0b0100;
        const L = 0b1000;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(41, distinct_guard_positions(EXAMPLE));
    }

    #[test]
    fn example_positions() {
        assert_eq!(6, new_obstruction_positions(EXAMPLE));
    }
}
