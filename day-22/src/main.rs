use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, btree_map::Entry};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(15613157363, sum_of_secrets_after(INPUT, 2000));
    assert_eq!(1784, best_price(INPUT, 2000));
}

fn sum_of_secrets_after(s: &str, n_rounds: usize) -> i64 {
    s.lines()
        .map(|l| {
            let seed = l.parse().expect("Seed not a number");
            Rng(seed).nth(n_rounds - 1).unwrap()
        })
        .sum()
}

fn best_price(s: &str, n_rounds: usize) -> i64 {
    let sequences_prices = s
        .lines()
        .map(|l| {
            let seed = l.parse().expect("Seed not a number");
            sequence_prices(seed, n_rounds)
        })
        .collect::<Vec<_>>();
    let unique_sequences = sequences_prices
        .iter()
        .flat_map(|s| s.keys())
        .collect::<BTreeSet<_>>();

    unique_sequences
        .iter()
        .map(|&s| {
            sequences_prices
                .iter()
                .map(|q| q.get(s).copied().unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap()
}

fn sequence_prices(seed: i64, n_rounds: usize) -> BTreeMap<[i64; 4], i64> {
    let p = prices(seed).skip(1).take(n_rounds).collect::<Vec<_>>();
    let c = changes(seed).take(n_rounds).collect::<Vec<_>>();

    let mut sequence_to_price = BTreeMap::new();

    for (i, w) in c.windows(4).enumerate() {
        let w = <[_; 4]>::try_from(w).unwrap();
        let p = p[i + 4 - 1];

        if let Entry::Vacant(entry) = sequence_to_price.entry(w) {
            entry.insert(p);
        }
    }

    sequence_to_price
}

fn changes(seed: i64) -> impl Iterator<Item = i64> {
    prices(seed).tuple_windows().map(|(a, b)| b - a)
}

fn prices(seed: i64) -> impl Iterator<Item = i64> {
    [seed].into_iter().chain(Rng(seed)).map(|s| s % 10)
}

struct Rng(i64);

impl Iterator for Rng {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.step(self.0 * 64);
        self.step(self.0 / 32);
        self.step(self.0 * 2048);

        Some(self.0)
    }
}

impl Rng {
    fn step(&mut self, v: i64) {
        self.mix(v);
        self.prune();
    }

    fn mix(&mut self, v: i64) {
        self.0 ^= v;
    }

    fn prune(&mut self) {
        self.0 %= 16777216;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../example-1.txt");
    const EXAMPLE_2: &str = include_str!("../example-2.txt");

    #[test]
    fn example() {
        assert_eq!(37327623, sum_of_secrets_after(EXAMPLE_1, 2000));
    }

    #[test]
    fn example_best_price() {
        assert_eq!(23, best_price(EXAMPLE_2, 2000));
    }

    #[test]
    fn rng() {
        let seq = Rng(123).take(10).collect::<Vec<_>>();
        let expected = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        assert_eq!(expected, &*seq);
    }

    #[test]
    fn prices_example() {
        let expected = [3, 0, 6, 5, 4, 4, 6, 4, 4, 2];
        let prices = prices(123).take(expected.len()).collect::<Vec<_>>();
        assert_eq!(expected, &*prices);
    }

    #[test]
    fn changes_example() {
        let expected = [-3, 6, -1, -1, 0, 2, -2, 0, -2];
        let changes = changes(123).take(expected.len()).collect::<Vec<_>>();
        assert_eq!(expected, &*changes);
    }
}
