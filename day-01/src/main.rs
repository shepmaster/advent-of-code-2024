const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(2285373, compare(INPUT));
}

fn compare(s: &str) -> u64 {
    let [mut l, mut r] = parse(s);

    l.sort_unstable();
    r.sort_unstable();

    l.iter()
        .zip(&r)
        .map(|(&l, &r)| {
            let [small, big] = {
                let mut v = [l, r];
                v.sort_unstable();
                v
            };
            big - small
        })
        .sum()
}

fn parse(s: &str) -> [Vec<u64>; 2] {
    let mut list_l = Vec::new();
    let mut list_r = Vec::new();

    for line in s.lines() {
        let mut line = line.split_ascii_whitespace();
        let l = line.next().expect("Data malformed; missing left number");
        let r = line.next().expect("Data malformed; missing right number");
        let l = l
            .parse()
            .expect("Left number malformed; not a valid integer");
        let r = r
            .parse()
            .expect("Right number malformed; not a valid integer");
        list_l.push(l);
        list_r.push(r);
    }

    [list_l, list_r]
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(11, compare(EXAMPLE));
    }
}
