use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(606, n_safe(INPUT));
    assert_eq!(644, n_safe_dampened(INPUT));
}

fn n_safe(s: &str) -> usize {
    s.lines().filter(|l| levels_are_safe(parse(l))).count()
}

fn n_safe_dampened(s: &str) -> usize {
    s.lines()
        .filter(|l| {
            let levels = parse(l).collect::<Vec<_>>();

            if levels_are_safe(levels.iter().copied()) {
                return true;
            }

            (0..levels.len()).any(|to_drop| {
                let dropped_one = levels
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &l)| (i != to_drop).then_some(l));
                levels_are_safe(dropped_one)
            })
        })
        .count()
}

fn parse(s: &str) -> impl Iterator<Item = i64> {
    s.split_ascii_whitespace()
        .map(|n| n.parse::<i64>().expect("number was malformed"))
}

fn levels_are_safe(levels: impl Iterator<Item = i64>) -> bool {
    let mut diffs = levels.tuple_windows().map(|(l, r)| l - r);

    let in_range = |v: i64| (1..=3).contains(&v.abs());

    let first = diffs.next().expect("Did not have at least two numbers");

    if !in_range(first) {
        return false;
    }

    diffs.all(|l| first.signum() == l.signum() && in_range(l))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(2, n_safe(EXAMPLE));
    }

    #[test]
    fn example_dampened() {
        assert_eq!(4, n_safe_dampened(EXAMPLE));
    }
}
