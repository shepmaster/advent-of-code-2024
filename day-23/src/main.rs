use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(1314, n_sets_where_starts_with_t(INPUT));
}

fn n_sets_where_starts_with_t(s: &str) -> usize {
    let mut connections = BTreeMap::new();
    let mut add_connection = |a, b| {
        connections.entry(a).or_insert_with(BTreeSet::new).insert(b);
    };
    for l in s.lines() {
        let (l, r) = l.split_once('-').expect("malformed line");
        add_connection(l, r);
        add_connection(r, l);
    }

    connections
        .keys()
        .flat_map(|&n| reachable(&connections, n, n, 3))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|path| path.iter().any(|&n| n.starts_with('t')))
        .count()
}

fn reachable<'a>(
    connections: &BTreeMap<&'a str, BTreeSet<&'a str>>,
    current: &'a str,
    target: &'a str,
    steps: usize,
) -> Vec<BTreeSet<&'a str>> {
    if steps == 0 {
        return if current == target {
            vec![BTreeSet::new()]
        } else {
            vec![]
        };
    }

    connections[current]
        .iter()
        .flat_map(|n| {
            let mut r = reachable(connections, n, target, steps - 1);
            for path in &mut r {
                path.insert(current);
            }
            r
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(7, n_sets_where_starts_with_t(EXAMPLE));
    }
}
