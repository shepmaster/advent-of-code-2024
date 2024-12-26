use std::{collections::BTreeMap, fmt, sync::atomic::AtomicBool};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(55544677167336, decimal_output(INPUT));
    assert_eq!("gsd,kth,qnf,tbt,vpm,z12,z26,z32", find_swaps(INPUT));
}

fn decimal_output(s: &str) -> u64 {
    let mut c = parse(s);

    let z_names = names_starting_with(&c, 'z');

    let mut z = 0;
    for &z_name in z_names.iter().rev() {
        let enabled = resolve(&mut c, z_name);
        z <<= 1;
        z |= enabled as u64;
    }
    z
}

static LOGGING: AtomicBool = AtomicBool::new(false);

macro_rules! t {
    ($($t:tt)*) => {
        if LOGGING.load(std::sync::atomic::Ordering::SeqCst) {
            eprintln!($($t)*);
        }
    };
}

fn find_swaps(s: &str) -> String {
    let c = parse(s);
    let mut network = Network::new(c);

    let mut swaps = Vec::new();

    let z_names = names_starting_with(&network.connections, 'z');
    let mut to_see = z_names.iter().enumerate().skip(2);
    to_see.next_back(); // ignore the last output

    for (i, name) in to_see.clone() {
        // LOGGING.store(i == 36, std::sync::atomic::Ordering::SeqCst);

        if let Some([a, b]) = network.parse_z(name, i) {
            network.swap(a, b);
            swaps.extend([a, b]);
        }
    }

    for (i, name) in to_see {
        assert!(network.parse_z(name, i).is_none(), "Bit {i} remains broken");
    }

    assert_eq!(8, swaps.len());

    swaps.sort();
    swaps.join(",")
}

fn parse(s: &str) -> BTreeMap<&str, Connection<'_>> {
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

    c
}

fn resolve<'a>(connections: &mut BTreeMap<&'a str, Connection<'a>>, name: &'a str) -> bool {
    match connections[name] {
        Connection::Resolved(v) => v,
        Connection::Symbolic([l, r], op) => {
            let v = {
                let l = resolve(connections, l);
                let r = resolve(connections, r);

                match op {
                    Op::And => l & r,
                    Op::Or => l | r,
                    Op::Xor => l ^ r,
                }
            };
            connections.insert(name, Connection::Resolved(v));
            v
        }
    }
}

#[derive(Debug, Clone)]
struct Network<'a> {
    connections: BTreeMap<&'a str, Connection<'a>>,
    x_names: Vec<&'a str>,
    y_names: Vec<&'a str>,
    rx: Vec<&'a str>,
    c: Vec<&'a str>,
}

impl<'a> Network<'a> {
    fn new(connections: BTreeMap<&'a str, Connection<'a>>) -> Self {
        let x_names = names_starting_with(&connections, 'x');
        let y_names = names_starting_with(&connections, 'y');
        let [rx, c] = Self::find_rx_and_c(&connections, &x_names, &y_names);

        Self {
            connections,
            x_names,
            y_names,
            rx,
            c,
        }
    }

    // For a given output bit Z[n], there's a recurrence relation to construct the addition:
    //
    // Z[n]  = RX[n]  XOR Q[n]
    // RX[n] = X[n]   XOR Y[n]    "raw xor"
    // Q[n]  = C[n-1]  OR P[n]
    // C[n]  = X[n]   AND Y[n]    "carry"
    // P[n]  = Q[n-1] AND RX[n-1]
    //
    // There's some special edge cases:
    //
    // 1. Z[0] = Rx[0] -- there's no carry from a previous bit
    //
    // 2. Z[last] does not involve RX as there isn't one
    //
    // The general idea is to walk up through the bits, trying to
    // match up the circuit network to this pattern.
    //
    // - If Z[n] isn't an XOR, we look through all the connections for
    //   an XOR operation that is an XOR and otherwise matches the
    //   children network. Then Z[n] and the found node should be
    //   swapped.
    //
    // - If Z[n] is an XOR, and one of its children matches Q[n], then
    //   the other child should be RX[n]. Then the child and RX[n]
    //   should be swapped.
    //
    // This appears to be sufficient to handle my input.
    //
    // We can easily figure out what RX[n] and C[n] are because they
    // only involve X[n] and Y[n], so we precompute that. The rest is
    // a recursive task. We probably could memoize something, but
    // speed-wise we didn't need to.
    //
    // We do have to start at the low bits and work upwards, fixing as
    // we go. Otherwise higher bits wouldn't match the pattern because
    // the lower bits are broken.
    //
    // Manual inspection showed that Z[0] and Z[1] were well-formed,
    // so we can start after those edge cases. We discovered all the
    // swaps before we got to Z[last], so we ignore that.
    fn parse_z(&self, z_name: &'a str, bit_n: usize) -> Option<[&'a str; 2]> {
        let z_n = self.connections[z_name];
        t!("{bit_n:02} {z_name} parse_z {z_n:?}");

        let Connection::Symbolic([l, r], Op::Xor) = z_n else {
            t!("{bit_n:02} {z_name} parse_z - wrong pattern");

            t!("  -- Starting search --");

            for &name in self.connections.keys() {
                if self.is_z(name, bit_n) {
                    return Some([z_name, name]);
                }
            }

            panic!("Didn't find alternative for {z_name}");
        };

        let a0 = self.rx[bit_n] == l;
        let a1 = self.parse_q(r, bit_n);
        let b0 = self.rx[bit_n] == r;
        let b1 = self.parse_q(l, bit_n);

        let a = a0 && a1;
        let b = b0 && b1;

        if a1 && !a0 {
            return Some([l, self.rx[bit_n]]);
        }

        if b1 && !b0 {
            return Some([r, self.rx[bit_n]]);
        }

        t!("{bit_n:02} {z_name} parse_z - [{a0}, {a1}, {b0}, {b1}]");

        let v = a ^ b;
        t!("{bit_n:02} {z_name} parse_z - {v}");

        if v {
            None
        } else {
            panic!("Didn't find alternative for {z_name}");
        }
    }

    fn is_z(&self, name: &str, bit_n: usize) -> bool {
        let z_n = self.connections[name];
        t!("{bit_n:02} {name} is_z {z_n:?}");

        let Connection::Symbolic([l, r], Op::Xor) = z_n else {
            t!("{bit_n:02} {name} is_z - wrong pattern");
            return false;
        };

        let a0 = self.rx[bit_n] == l;
        let a1 = self.parse_q(r, bit_n);
        let b0 = self.rx[bit_n] == r;
        let b1 = self.parse_q(l, bit_n);

        let a = a0 && a1;
        let b = b0 && b1;

        t!("{bit_n:02} {name} is_z - [{a0}, {a1}, {b0}, {b1}]");

        let v = a ^ b;
        t!("{bit_n:02} {name} is_z - {v}");
        v
    }

    fn parse_q(&self, name: &str, bit_n: usize) -> bool {
        // this is special and we assume it's well-formed
        if bit_n <= 1 {
            return true;
        }

        let q_n = self.connections[name];
        t!("{bit_n:02} {name} parse_q {q_n:?}");
        let Connection::Symbolic([l, r], Op::Or) = q_n else {
            t!("{bit_n:02} {name} parse_q - wrong pattern");
            return false;
        };

        let a0 = self.c[bit_n - 1] == l;
        let a1 = self.parse_p(r, bit_n);
        let b0 = self.c[bit_n - 1] == r;
        let b1 = self.parse_p(l, bit_n);

        let a = a0 && a1;
        let b = b0 && b1;

        t!("{bit_n:02} {name} parse_q - [{a0}, {a1}, {b0}, {b1}]");

        let v = a ^ b;
        t!("{bit_n:02} {name} parse_q - {v}");
        v
    }

    fn parse_p(&self, name: &str, bit_n: usize) -> bool {
        let p_n = self.connections[name];
        t!("{bit_n:02} {name} parse_p {p_n:?}");
        let Connection::Symbolic([l, r], Op::And) = p_n else {
            t!("{bit_n:02} {name} parse_p - wrong pattern");
            return false;
        };

        let a0 = self.parse_q(l, bit_n - 1);
        let a1 = self.rx[bit_n - 1] == r;
        let b0 = self.parse_q(r, bit_n - 1);
        let b1 = self.rx[bit_n - 1] == l;

        let a = a0 && a1;
        let b = b0 && b1;

        t!("{bit_n:02} {name} parse_p - [{a0}, {a1}, {b0}, {b1}]");

        let v = a ^ b;
        t!("{bit_n:02} {name} parse_p - {v}");
        v
    }

    fn swap(&mut self, a: &str, b: &str) {
        let (k0, v0) = self.connections.remove_entry(a).unwrap();
        let (k1, v1) = self.connections.remove_entry(b).unwrap();

        self.connections.insert(k1, v0);
        self.connections.insert(k0, v1);

        self.reindex_rx_and_c();
    }

    fn reindex_rx_and_c(&mut self) {
        let [rx, c] = Self::find_rx_and_c(&self.connections, &self.x_names, &self.y_names);
        self.rx = rx;
        self.c = c;
    }

    fn find_rx_and_c(
        connections: &BTreeMap<&'a str, Connection<'a>>,
        x_names: &[&'a str],
        y_names: &[&'a str],
    ) -> [Vec<&'a str>; 2] {
        let mut rx = Vec::new();
        let mut c = Vec::new();

        for (n, (&x_tgt, &y_tgt)) in x_names.iter().zip(y_names).enumerate() {
            let mut found_rx = false;
            let mut found_c = false;

            for (&k, v) in connections {
                if let &Connection::Symbolic(mut i, Op::Xor) = v {
                    i.sort();
                    let [x, y] = i;

                    if x == x_tgt && y == y_tgt {
                        assert!(!found_rx);
                        found_rx = true;
                        rx.push(k);
                    }
                }

                if let &Connection::Symbolic(mut i, Op::And) = v {
                    i.sort();
                    let [x, y] = i;

                    if x == x_tgt && y == y_tgt {
                        assert!(!found_c);
                        found_c = true;
                        c.push(k);
                    }
                }
            }

            assert!(found_rx, "{n:02} {x_tgt} / {y_tgt}");
            assert!(found_c, "{n:02} {x_tgt} / {y_tgt}");
        }

        [rx, c]
    }
}

fn names_starting_with<'a>(c: &BTreeMap<&'a str, Connection<'_>>, pat: char) -> Vec<&'a str> {
    c.keys().copied().filter(|n| n.starts_with(pat)).collect()
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

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Op::And => '&',
            Op::Or => '|',
            Op::Xor => '^',
        };
        c.fmt(f)
    }
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
