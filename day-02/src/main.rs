use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(606, n_safe(INPUT));
}

fn n_safe(s: &str) -> usize {
    s.lines()
        .filter(|l| {
            let levels = l
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().expect("number was malformed"));

            let mut diffs = levels.tuple_windows().map(|(l, r)| l - r);

            let in_range = |v: i64| (1..=3).contains(&v.abs());

            let first = diffs.next().expect("Did not have at least two numbers");

            if !in_range(first) {
                return false;
            }

            diffs.all(|l| first.signum() == l.signum() && in_range(l))
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(2, n_safe(EXAMPLE));
    }
}
