use itertools::Itertools;
use std::iter;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(1582598718861, total_calibration(INPUT));
}

fn total_calibration(s: &str) -> u64 {
    s.lines()
        .map(parse_line)
        .filter(|&(test, ref numbers)| {
            let (&head, tail) = numbers.split_first().expect("Need more than one number");

            let ops = [Op::Add, Op::Mul];

            iter::repeat_n(&ops, tail.len())
                .multi_cartesian_product()
                .any(|ops| {
                    let computed = ops
                        .into_iter()
                        .zip(tail)
                        .fold(head, |acc, (op, &n)| match op {
                            Op::Add => acc + n,
                            Op::Mul => acc * n,
                        });

                    test == computed
                })
        })
        .map(|(test, _)| test)
        .sum()
}

fn parse_line(l: &str) -> (u64, Vec<u64>) {
    let (test, numbers) = l.split_once(':').expect("Missing test value");

    let test = test.parse().expect("Test value not a number");
    let numbers = numbers
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("Number value not a number"))
        .collect();

    (test, numbers)
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(3749, total_calibration(EXAMPLE));
    }
}
