use itertools::Either;
use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(203228, stones_after_blinks(INPUT, 25));
    assert_eq!(203228, stones_after_blinks_memo(INPUT, 25));
    assert_eq!(240884656550923, stones_after_blinks_memo(INPUT, 75));
}

fn stones_after_blinks(s: &str, n_blinks: usize) -> usize {
    let mut stones: Vec<u64> = s
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("Invalid number"))
        .collect();

    for _ in 0..n_blinks {
        stones = stones
            .into_iter()
            .flat_map(|stone| {
                if stone == 0 {
                    return Either::Left([1].into_iter());
                }

                let n_digits = stone.ilog10() + 1;

                if n_digits % 2 == 0 {
                    let factor = 10u64.pow(n_digits / 2);

                    let l = stone / factor;
                    let r = stone % factor;

                    return Either::Right([l, r].into_iter());
                }

                Either::Left([stone * 2024].into_iter())
            })
            .collect();
    }

    stones.len()
}

fn stones_after_blinks_memo(s: &str, n_blinks: usize) -> usize {
    let stones = s
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().expect("Invalid number"));

    let mut memo = BTreeMap::new();

    stones.map(|stone| delve(&mut memo, stone, n_blinks)).sum()
}

fn delve(memo: &mut BTreeMap<(u64, usize), usize>, stone: u64, depth: usize) -> usize {
    let key = (stone, depth);

    if let Some(&count) = memo.get(&key) {
        return count;
    }

    let count = {
        if depth == 0 {
            1
        } else if stone == 0 {
            delve(memo, 1, depth - 1)
        } else {
            let n_digits = stone.ilog10() + 1;

            if n_digits % 2 == 0 {
                let factor = 10u64.pow(n_digits / 2);

                let l = stone / factor;
                let r = stone % factor;

                let l = delve(memo, l, depth - 1);
                let r = delve(memo, r, depth - 1);

                l + r
            } else {
                delve(memo, stone * 2024, depth - 1)
            }
        }
    };

    memo.insert(key, count);

    count
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = "0 1 10 99 999";
    const EXAMPLE_2: &str = "125 17";

    #[test]
    fn example_1() {
        assert_eq!(7, stones_after_blinks(EXAMPLE_1, 1));
    }

    #[test]
    fn example_2() {
        assert_eq!(22, stones_after_blinks(EXAMPLE_2, 6));
        assert_eq!(55312, stones_after_blinks(EXAMPLE_2, 25));
    }

    #[test]
    fn example_1_memo() {
        assert_eq!(7, stones_after_blinks_memo(EXAMPLE_1, 1));
    }

    #[test]
    fn example_2_memo() {
        assert_eq!(22, stones_after_blinks_memo(EXAMPLE_2, 6));
        assert_eq!(55312, stones_after_blinks_memo(EXAMPLE_2, 25));
    }
}
