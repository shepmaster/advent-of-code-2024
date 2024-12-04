use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(2613, xmas_count(INPUT));
    assert_eq!(1905, cross_mas_count(INPUT));
}

fn xmas_count(s: &str) -> usize {
    let grid = parse(s);

    grid.iter()
        .map(|(&(x, y), &c)| {
            if c != 'X' {
                return 0;
            }

            Direction::ALL
                .iter()
                .filter(|&&d| {
                    let mut cursor = (x, y);
                    let to_find = ['M', 'A', 'S'];

                    for c_to_find in to_find {
                        match check_in_direction(&grid, cursor, d, c_to_find) {
                            Some(new_cursor) => cursor = new_cursor,
                            _ => return false,
                        };
                    }

                    // Ran out of characters to find, so we must have matched them all
                    true
                })
                .count()
        })
        .sum()
}

fn cross_mas_count(s: &str) -> usize {
    let grid = parse(s);

    grid.iter()
        .filter(|&(&(x, y), &c)| {
            if c != 'A' {
                return false;
            }

            type Diag = [Direction; 2];

            const DIAG_1: Diag = [Direction::UL, Direction::DR];
            const DIAG_2: [Direction; 2] = [Direction::UR, Direction::DL];

            let start = (x, y);

            let check_simple = |dir, c| check_in_direction(&grid, start, dir, c).is_some();

            let check_once = |[a, b]: Diag| check_simple(a, 'M') && check_simple(b, 'S');

            let check_diag = |[a, b]: Diag| check_once([a, b]) || check_once([b, a]);

            check_diag(DIAG_1) && check_diag(DIAG_2)
        })
        .count()
}

type Grid = BTreeMap<(usize, usize), char>;

fn parse(s: &str) -> Grid {
    let mut grid = BTreeMap::new();

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid.insert((x, y), c);
        }
    }

    grid
}

fn check_in_direction(grid: &Grid, start: Coord, d: Direction, c_to_find: char) -> Option<Coord> {
    // Did we walk off the maximum possible board?
    let coord = d.apply(start)?;

    // Did we walk off what board we have?
    let &c = grid.get(&coord)?;

    if c_to_find != c {
        // Does not match
        return None;
    }

    Some(coord)
}

type Coord = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    U,
    UR,
    R,
    DR,
    D,
    DL,
    L,
    UL,
}

impl Direction {
    const ALL: [Self; 8] = [
        Self::U,
        Self::UR,
        Self::R,
        Self::DR,
        Self::D,
        Self::DL,
        Self::L,
        Self::UL,
    ];

    fn apply(self, coord: Coord) -> Option<Coord> {
        use Direction::*;

        let (x, y) = coord;

        let x_left = x.checked_sub(1);
        let x_right = x.checked_add(1);

        let y_up = y.checked_sub(1);
        let y_down = y.checked_add(1);

        let c = match self {
            U => (x, y_up?),
            UR => (x_right?, y_up?),
            R => (x_right?, y),
            DR => (x_right?, y_down?),
            D => (x, y_down?),
            DL => (x_left?, y_down?),
            L => (x_left?, y),
            UL => (x_left?, y_up?),
        };

        Some(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(18, xmas_count(EXAMPLE));
    }

    #[test]
    fn example_cross() {
        assert_eq!(9, cross_mas_count(EXAMPLE));
    }
}
