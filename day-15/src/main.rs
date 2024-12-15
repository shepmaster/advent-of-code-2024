use std::{collections::BTreeMap, iter};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(1515788, gps_sum(INPUT));
}

fn gps_sum(s: &str) -> usize {
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

type Coord = [usize; 2];

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
