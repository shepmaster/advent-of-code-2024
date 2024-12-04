use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(2613, xmas_count(INPUT));
}

fn xmas_count(s: &str) -> usize {
    let mut grid = BTreeMap::new();

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid.insert((x, y), c);
        }
    }

    grid.iter()
        .map(|(&(x, y), &c)| {
            if c != 'X' {
                return 0;
            }

            Direction::ALL
                .iter()
                .filter(|d| {
                    let mut cursor = (x, y);
                    let to_find = ['M', 'A', 'S'];

                    for c_to_find in to_find {
                        let Some(new_cursor) = d.apply(cursor) else {
                            // Walked off the maximum possible board
                            return false;
                        };

                        let Some(&c) = grid.get(&new_cursor) else {
                            // Walked off what board we have
                            return false;
                        };

                        if c_to_find != c {
                            // Does not match
                            return false;
                        }

                        cursor = new_cursor;
                    }

                    // Ran out of characters to find, so we must have matched them all
                    true
                })
                .count()
        })
        .sum()
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
}
