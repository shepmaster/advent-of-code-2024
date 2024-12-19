use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(317, possible_designs(INPUT));
}

fn possible_designs(s: &str) -> usize {
    let (avail, desired) = s.split_once("\n\n").expect("input malformed");
    let avail = avail.split(',').map(|s| s.trim()).collect::<Vec<_>>();

    let mut cache = Default::default();

    desired
        .lines()
        .filter(|desired| can_be_built_from(&mut cache, &avail, desired))
        .count()
}

fn can_be_built_from<'a>(
    cache: &mut BTreeMap<&'a str, bool>,
    available: &[&str],
    desired: &'a str,
) -> bool {
    if desired.is_empty() {
        return true;
    }

    if let Some(&cached) = cache.get(desired) {
        return cached;
    }

    let can_be = available.iter().any(|&a| {
        desired
            .strip_prefix(a)
            .is_some_and(|tail| can_be_built_from(cache, available, tail))
    });

    cache.insert(desired, can_be);
    can_be
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(6, possible_designs(EXAMPLE));
    }
}
