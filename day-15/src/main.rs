const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(1515788, part1::gps_sum(INPUT));
    assert_eq!(1516544, part2::gps_sum(INPUT));
}

mod part1 {
    use std::{collections::BTreeMap, iter};

    use super::{Coord, Direction};

    pub fn gps_sum(s: &str) -> usize {
        let (m, i) = s.split_once("\n\n").expect("input malformed");

        let mut robot = None;
        let mut map = BTreeMap::new();
        for (y, l) in m.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '@' {
                    assert!(robot.is_none());
                    robot = Some([x, y]);
                } else if c != '.' {
                    let p = Piece::try_from(c).unwrap();
                    map.insert([x, y], p);
                }
            }
        }
        let mut robot = robot.expect("No robot found");

        let instructions = i
            .lines()
            .flat_map(|l| l.chars())
            .map(|c| Direction::try_from(c).unwrap());

        // print(&map, robot);

        for i in instructions {
            use Piece::*;

            let pos = iter::successors(Some(robot), |&r| i.move_it(r)).skip(1);

            let mut to_push = 0;
            for c in pos {
                match map.get(&c) {
                    None => {
                        // eprintln!("Found a free spot, moving {to_push} boxes");
                        robot = i.move_it(robot).unwrap();

                        let old = map.remove(&robot);
                        assert_ne!(old, Some(Wall));

                        if to_push > 0 {
                            let old = map.insert(c, Box);
                            assert!(old.is_none());
                        }

                        break;
                    }
                    Some(Box) => {
                        to_push += 1;
                    }
                    Some(Wall) => {
                        // eprintln!("Found a wall, doing nothing");
                        break;
                    }
                }
            }

            // println!("\n{i:?}\n");
            // print(&map, robot);
        }

        map.iter()
            .filter(|&(_, &p)| p == Piece::Box)
            .map(|(&[x, y], _)| y * 100 + x)
            .sum()
    }

    #[allow(dead_code)]
    fn print(map: &BTreeMap<Coord, Piece>, robot: Coord) {
        let (&[max_x, max_y], _) = map.last_key_value().unwrap();

        for y in 0..=max_y {
            for x in 0..=max_x {
                if robot == [x, y] {
                    print!("@");
                } else {
                    match map.get(&[x, y]) {
                        None => print!("."),
                        Some(Piece::Wall) => print!("#"),
                        Some(Piece::Box) => print!("O"),
                    }
                }
            }
            println!();
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum Piece {
        Wall,
        Box,
    }

    impl TryFrom<char> for Piece {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            use Piece::*;

            let v = match value {
                '#' => Wall,
                'O' => Box,
                _ => return Err("Unknown piece"),
            };
            Ok(v)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        const EXAMPLE_1: &str = include_str!("../example-1.txt");
        const EXAMPLE_2: &str = include_str!("../example-2.txt");

        #[test]
        fn example_1() {
            assert_eq!(10092, gps_sum(EXAMPLE_1));
        }

        #[test]
        fn example_2() {
            assert_eq!(2028, gps_sum(EXAMPLE_2));
        }
    }
}

mod part2 {
    use std::collections::{BTreeMap, BTreeSet};

    use super::{Coord, Direction};

    pub fn gps_sum(s: &str) -> usize {
        use Piece::*;

        let (m, i) = s.split_once("\n\n").expect("malformed input");

        let mut map = BTreeMap::new();
        let mut robot = None;

        for (y, l) in m.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let x = x * 2;

                match c {
                    '.' => {}

                    '@' => {
                        assert!(robot.is_none());
                        robot = Some([x, y]);
                    }

                    '#' => {
                        map.insert([x, y], Wall);
                        map.insert([x + 1, y], Wall);
                    }

                    'O' => {
                        map.insert([x, y], BoxLeft);
                        map.insert([x + 1, y], BoxRight);
                    }

                    _ => panic!("Unknown character {c}"),
                }
            }
        }

        let mut robot = robot.expect("No robot found");

        // print(&map, robot);

        let instructions = i
            .lines()
            .flat_map(|l| l.chars())
            .map(|c| Direction::try_from(c).unwrap());

        let mut to_move = BTreeSet::new();
        let mut pieces_to_move = Vec::new();

        for i in instructions {
            to_move.clear();
            pieces_to_move.clear();

            let target = i.move_it(robot).unwrap();

            if can_move(&map, &mut to_move, target, i) {
                // Remove the pieces that are moving
                let old_pieces = to_move
                    .iter()
                    .map(|&c| map.remove(&c).expect("Moving something that isn't there"));

                pieces_to_move.extend(old_pieces);

                // Re-insert at their new position
                for (&c, &p) in to_move.iter().zip(&pieces_to_move) {
                    let nc = i.move_it(c).unwrap();
                    map.insert(nc, p);
                }

                // Move the robot
                robot = target;
            }

            // println!("\n=== {i:?}");
            // print(&map, robot);
        }

        map.iter()
            .filter(|&(_, &p)| p == BoxLeft)
            .map(|(&[x, y], _)| y * 100 + x)
            .sum()
    }

    fn can_move(map: &Map, to_move: &mut BTreeSet<Coord>, target: Coord, i: Direction) -> bool {
        use Direction::*;
        use Piece::*;

        let mut split_move = |l_target, r_target| {
            to_move.insert(l_target);
            to_move.insert(r_target);

            let l_target = i.move_it(l_target).unwrap();
            let r_target = i.move_it(r_target).unwrap();

            can_move(map, to_move, l_target, i) && can_move(map, to_move, r_target, i)
        };

        let v = match (map.get(&target), i) {
            (Some(Wall), _) => false,

            (Some(BoxLeft), R) | (Some(BoxRight), L) => {
                to_move.insert(target);
                let target = i.move_it(target).unwrap();
                to_move.insert(target);
                let target = i.move_it(target).unwrap();
                can_move(map, to_move, target, i)
            }

            (Some(BoxLeft), U | D) => split_move(target, R.move_it(target).unwrap()),

            (Some(BoxRight), U | D) => split_move(L.move_it(target).unwrap(), target),

            (None, _) => true,

            o => unreachable!("{o:?} is impossible"),
        };

        // eprintln!("{target:?} {i:?}: {v}");

        v
    }

    type Map = BTreeMap<Coord, Piece>;

    #[allow(dead_code)]
    fn print(map: &Map, robot: Coord) {
        use Piece::*;

        let (&[max_x, max_y], _) = map.last_key_value().expect("map empty");

        for y in 0..=max_y {
            for x in 0..=max_x {
                let c = [x, y];

                let s = if robot == c {
                    "@"
                } else {
                    match map.get(&c) {
                        Some(Wall) => "#",
                        Some(BoxLeft) => "[",
                        Some(BoxRight) => "]",
                        None => ".",
                    }
                };

                print!("{s}");
            }
            println!();
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum Piece {
        Wall,
        BoxLeft,
        BoxRight,
    }

    #[cfg(test)]
    mod test {
        use super::*;

        const EXAMPLE_1: &str = include_str!("../example-1.txt");
        const EXAMPLE_2: &str = include_str!("../example-2.txt");
        const EXAMPLE_3: &str = include_str!("../example-3.txt");

        #[test]
        fn example_1() {
            assert_eq!(9021, gps_sum(EXAMPLE_1));
        }

        #[test]
        fn example_2() {
            gps_sum(EXAMPLE_2);
        }

        #[test]
        fn example_3() {
            gps_sum(EXAMPLE_3);
        }
    }
}

type Coord = [usize; 2];

#[derive(Debug, Copy, Clone)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction::*;

        let v = match value {
            '^' => U,
            '>' => R,
            'v' => D,
            '<' => L,
            _ => return Err("Unknown direction"),
        };
        Ok(v)
    }
}

impl Direction {
    fn move_it(self, c: Coord) -> Option<Coord> {
        let [x, y] = c;

        let u = y.checked_sub(1)?;
        let r = x.checked_add(1)?;
        let d = y.checked_add(1)?;
        let l = x.checked_sub(1)?;

        let c = match self {
            Direction::U => [x, u],
            Direction::R => [r, y],
            Direction::D => [x, d],
            Direction::L => [l, y],
        };
        Some(c)
    }
}
