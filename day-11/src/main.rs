use itertools::Either;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(203228, stones_after_blinks(INPUT, 25));
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
}
