const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(15613157363, sum_of_secrets_after(INPUT, 2000));
}

fn sum_of_secrets_after(s: &str, n_rounds: usize) -> u64 {
    s.lines()
        .map(|l| {
            let seed = l.parse().expect("Seed not a number");
            Rng(seed).nth(n_rounds - 1).unwrap()
        })
        .sum()
}

struct Rng(u64);

impl Iterator for Rng {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.step(self.0 * 64);
        self.step(self.0 / 32);
        self.step(self.0 * 2048);

        Some(self.0)
    }
}

impl Rng {
    fn step(&mut self, v: u64) {
        self.mix(v);
        self.prune();
    }

    fn mix(&mut self, v: u64) {
        self.0 ^= v;
    }

    fn prune(&mut self) {
        self.0 %= 16777216;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(37327623, sum_of_secrets_after(EXAMPLE, 2000));
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
}
