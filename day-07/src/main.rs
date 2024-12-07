use itertools::Itertools;
use std::iter;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(1582598718861, total_calibration(INPUT));
    assert_eq!(165278151522644, total_calibration_concat(INPUT));
}

fn total_calibration(s: &str) -> u64 {
    total_calibration_with_operators(s, &Op::LIMITED)
}

fn total_calibration_concat(s: &str) -> u64 {
    total_calibration_with_operators(s, &Op::ALL)
}

fn total_calibration_with_operators(s: &str, op_choices: &[Op]) -> u64 {
    s.lines()
        .map(parse_line)
        .filter(|(test, numbers)| test_with_operators(numbers, op_choices, *test))
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

fn test_with_operators(numbers: &[u64], op_choices: &[Op], test: u64) -> bool {
    let (&head, tail) = numbers.split_first().expect("Need more than one number");

    iter::repeat_n(op_choices, tail.len())
        .multi_cartesian_product()
        .any(|ops| {
            let computed = ops
                .into_iter()
                .zip(tail)
                .fold(head, |acc, (op, &n)| op.apply(acc, n));

            test == computed
        })
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    const ALL: [Self; 3] = [Op::Add, Op::Mul, Op::Concat];
    const LIMITED: [Self; 2] = [Op::Add, Op::Mul];

    fn apply(self, l: u64, r: u64) -> u64 {
        match self {
            Op::Add => l + r,
            Op::Mul => l * r,
            Op::Concat => {
                let n_r_digits = r.ilog10() + 1;
                let f = 10u64.pow(n_r_digits);
                l * f + r
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(3749, total_calibration(EXAMPLE));
    }

    #[test]
    fn example_concat() {
        assert_eq!(11387, total_calibration_concat(EXAMPLE));
    }
}
