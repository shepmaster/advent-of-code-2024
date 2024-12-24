use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(55544677167336, decimal_output(INPUT));
}

fn decimal_output(s: &str) -> u64 {
    let mut c = BTreeMap::new();

    let mut lines = s.lines();
    for l in &mut lines {
        if l.is_empty() {
            break;
        }
        let (name, value) = l.split_once(": ").expect("literal malformed");
        c.insert(name, Connection::Resolved(value == "1"));
    }

    for l in lines {
        let (input, name) = l.split_once(" -> ").expect("symbolic malformed");
        let mut input = input.split(' ');
        let input_a = input.next().expect("input a missing");
        let input_op = input.next().expect("input op missing");
        let input_b = input.next().expect("input b missing");

        let op = match input_op {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("unknown operator {input_op}"),
        };

        c.insert(name, Connection::Symbolic([input_a, input_b], op));
    }

    let z_names = c
        .keys()
        .copied()
        .filter(|n| n.starts_with('z'))
        .collect::<Vec<_>>();
    let mut z = 0;

    for &z_name in z_names.iter().rev() {
        let enabled = resolve(&mut c, z_name);
        z <<= 1;
        z |= enabled as u64;
    }

    z
}

fn resolve<'a>(c: &mut BTreeMap<&'a str, Connection<'a>>, name: &'a str) -> bool {
    match c[name] {
        Connection::Resolved(v) => v,
        Connection::Symbolic([l, r], op) => {
            let l = resolve(c, l);
            let r = resolve(c, r);
            let v = match op {
                Op::And => l & r,
                Op::Or => l | r,
                Op::Xor => l ^ r,
            };
            c.insert(name, Connection::Resolved(v));
            v
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Connection<'a> {
    Resolved(bool),
    Symbolic([&'a str; 2], Op),
}

#[derive(Debug, Copy, Clone)]
enum Op {
    And,
    Or,
    Xor,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../example-1.txt");

    #[test]
    fn example_1() {
        assert_eq!(4, decimal_output(EXAMPLE_1));
    }
}
