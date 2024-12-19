use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(317, possible_designs(INPUT));
    assert_eq!(883443544805484, possible_design_counts(INPUT));
}

fn possible_designs(s: &str) -> usize {
    let (avail, desired) = parse(s);

    let mut cache = Default::default();

    desired
        .filter(|desired| can_be_built_from(&mut cache, &avail, desired) != 0)
        .count()
}

fn possible_design_counts(s: &str) -> usize {
    let (avail, desired) = parse(s);

    let mut cache = Default::default();

    desired
        .map(|desired| can_be_built_from(&mut cache, &avail, desired))
        .sum()
}

fn parse(s: &str) -> (Vec<&str>, impl Iterator<Item = &str>) {
    let (avail, desired) = s.split_once("\n\n").expect("input malformed");

    let avail = avail.split(',').map(|s| s.trim()).collect::<Vec<_>>();
    let desired = desired.lines();

    (avail, desired)
}

fn can_be_built_from<'a>(
    cache: &mut BTreeMap<&'a str, usize>,
    available: &[&str],
    desired: &'a str,
) -> usize {
    if desired.is_empty() {
        return 1;
    }

    if let Some(&cached) = cache.get(desired) {
        return cached;
    }

    let can_be = available
        .iter()
        .map(|&a| {
            desired
                .strip_prefix(a)
                .map(|tail| can_be_built_from(cache, available, tail))
                .unwrap_or(0)
        })
        .sum();

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

    #[test]
    fn example_counts() {
        assert_eq!(16, possible_design_counts(EXAMPLE));
    }
}
