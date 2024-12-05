use std::{cmp::Ordering, collections::BTreeMap};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(5713, sum_of_valid_middle_page(INPUT));
}

fn sum_of_valid_middle_page(s: &str) -> u64 {
    let mut lines = s.lines();

    let mut rules = BTreeMap::new();

    let rule_lines = lines.by_ref().take_while(|l| !l.trim().is_empty());
    for l in rule_lines {
        let (l, r) = l.split_once('|').expect("Rule malformed");
        let [l, r] = [l, r].map(|s| s.parse::<u8>().expect("Rule value not a number"));

        rules.insert((l, r), Ordering::Less);
        rules.insert((r, l), Ordering::Greater);
    }

    lines
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<u8>().expect("Update value not a number"))
                .collect::<Vec<_>>()
        })
        .filter(|update| check_update_validity(&rules, update))
        .map(|update| update[update.len() / 2])
        .map(u64::from)
        .sum()
}

type Rules = BTreeMap<(u8, u8), Ordering>;

fn check_update_validity(rules: &Rules, update: &[u8]) -> bool {
    let mut q = update;

    while let Some((&head, tail)) = q.split_first() {
        let in_order = tail
            .iter()
            .map(|&e| (head, e))
            .all(|key| rules.get(&key).is_none_or(|&o| o == Ordering::Less));

        if !in_order {
            return false;
        }

        q = tail;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(143, sum_of_valid_middle_page(EXAMPLE));
    }
}
