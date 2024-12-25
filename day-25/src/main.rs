use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(0, unique_fitting_pairs(INPUT));
}

fn unique_fitting_pairs(example: &str) -> usize {
    let chunks = example.split("\n\n");

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for chunk in chunks {
        let mut l = chunk.lines();
        let h = l.next().expect("malformed chunk");
        let is_lock = h.starts_with("#");
        let kind = if is_lock { &mut locks } else { &mut keys };

        let mut depths = [0; 5];
        for l in l.take(5) {
            for (i, c) in l.chars().enumerate() {
                if c == '#' {
                    depths[i] += 1;
                }
            }
        }

        kind.push(depths);
    }

    locks
        .iter()
        .cartesian_product(&keys)
        .filter(|&(lock, key)| lock.iter().zip(key).all(|(l, k)| l + k <= 5))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(3, unique_fitting_pairs(EXAMPLE));
    }
}
