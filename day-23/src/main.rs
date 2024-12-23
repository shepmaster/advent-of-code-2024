use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(1314, n_sets_where_starts_with_t(INPUT));
    assert_eq!("bg,bu,ce,ga,hw,jw,nf,nt,ox,tj,uu,vk,wp", biggest_set(INPUT));
}

fn n_sets_where_starts_with_t(s: &str) -> usize {
    let connections = parse(s);

    connections
        .keys()
        .flat_map(|&n| reachable(&connections, n, n, 3))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|path| path.iter().any(|&n| n.starts_with('t')))
        .count()
}

fn biggest_set(s: &str) -> String {
    let connections = parse(s);

    let p_set = connections.keys().copied().collect();
    let mut maximal_cliques = Default::default();

    bron_kerbosch(
        &connections,
        Default::default(),
        p_set,
        Default::default(),
        &mut maximal_cliques,
    );

    let max_size = maximal_cliques
        .iter()
        .map(|c| c.len())
        .max()
        .expect("No maximum size found");

    let max_clique = maximal_cliques
        .into_iter()
        .find(|c| c.len() == max_size)
        .expect("No maximum clique found");

    max_clique.into_iter().collect::<Vec<_>>().join(",")
}

fn parse(s: &str) -> BTreeMap<&str, BTreeSet<&str>> {
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

// https://en.wikipedia.org/wiki/Bron–Kerbosch_algorithm#Without_pivoting
fn bron_kerbosch<'a>(
    connections: &BTreeMap<&'a str, BTreeSet<&'a str>>,
    r_set: BTreeSet<&'a str>,
    mut p_set: BTreeSet<&'a str>,
    mut x_set: BTreeSet<&'a str>,
    output: &mut BTreeSet<BTreeSet<&'a str>>,
) {
    if p_set.is_empty() && x_set.is_empty() {
        output.insert(r_set);
        return;
    }

    while let Some(v) = p_set.pop_first() {
        let neighbors = &connections[v];

        let next_r_set = {
            let mut r_set = r_set.clone();
            r_set.insert(v);
            r_set
        };
        let next_p_set = p_set.intersection(neighbors).copied().collect();
        let next_x_set = x_set.intersection(neighbors).copied().collect();

        bron_kerbosch(connections, next_r_set, next_p_set, next_x_set, output);

        // v has already been removed from p_set
        x_set.insert(v);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(7, n_sets_where_starts_with_t(EXAMPLE));
    }

    #[test]
    fn example_biggest() {
        assert_eq!("co,de,ka,ta", biggest_set(EXAMPLE));
    }
}
