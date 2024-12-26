use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, sync::atomic::{AtomicBool, Ordering},
};

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

        c.insert(name, Connection::Symbolic([input_a, input_b], op, None));
    }

    // input has a tree depth of 88

    let mut network = Network::new(c);

    {
        let mut network = network.clone();

        let mut swaps = Vec::new();

        for i in 2..45 { // hack
            //LOGGING.store(i == 36, Ordering::SeqCst);

            if let Some([a, b]) = network.parse_z(i) {
                eprintln!("Found a swap {a} <> {b}");

                let (k0, v0) = network.connections.remove_entry(&*a).unwrap();
                let (k1, v1) = network.connections.remove_entry(&*b).unwrap();

                swaps.extend([k0, k1]);

                network.connections.insert(k1, v0);
                network.connections.insert(k0, v1);
                network.reindex_rx_and_c();

                assert!(network.parse_z(i).is_none(), "Bit {i} remains broken");
            }
        }

        for i in 2..36 { // hack
            assert!(network.parse_z(i).is_none());
        }

        swaps.sort();
        panic!("{}", swaps.join(","));

        // check if output N involves only input <= N
        // for ((&x, &y), &z) in network.x_names.iter().zip(&network.y_names).zip(&network.z_names) {
        //     eprintln!("{x} {y} {z}");
        // }

        // for (i, &z) in network.z_names.iter().enumerate() {
        //     // let inputs = network.roots(z);
        //     // for i in 0..i {
        //     //     dbg!(z, i, inputs.contains(network.x_names[i]), inputs.contains(network.y_names[i]));
        //     // }

        //     let d = network.depth(z);
        //     eprintln!("{z}: {d}");
        // }

        // for (n, c) in &network.connections {
        //     let &Connection::Symbolic(mut i, Op::Xor, _) = c else { continue };
        //     i.sort();
        //     let [x, y] = i;

        //     let Some(x) = x.strip_prefix('x') else { continue };
        //     let Some(y) = y.strip_prefix('y') else { continue };

        //     if x == y {
        //         eprintln!("{n} is the raw xor for {x} / {y}");
        //     }
        // }

        // 0 and 1 are ok by inspection and slightly different anyway

        // // Set Y to zero
        // let y = 0;
        // network.set_y(y);

        // // Set each bit of X to one in turn.
        // for bit in 0..network.x_names.len() {
        //     let x = 1 << bit;

        //     network.set_x(x);

        //     network.unresolve();
        //     let z = network.resolve_z();

        //     let z_expected = x + y;

        //     if z != z_expected {
        //         let bit_expected = (z_expected >> bit) & 1;
        //         eprintln!("{bit} / {x} + {y} = {z} ({z_expected} ... {bit_expected})");
        //         network.dig_in(network.z_names[bit], bit_expected == 1);
        //     }
        // }

        // // Set X to zero
        // let x = 0;
        // network.set_x(x);

        // // Set each bit of Y to one in turn.
        // for bit in 0..network.y_names.len() {
        //     let y = 1 << bit;

        //     network.set_y(y);

        //     network.unresolve();
        //     let z = network.resolve_z();

        //     let z_expected = x + y;

        //     if z != z_expected {
        //         let bit_expected = (z_expected >> bit) & 1;
        //         eprintln!("{bit} / {x} + {y} = {z} ({z_expected} ... {bit_expected})");
        //         network.dig_in(network.z_names[bit], bit_expected == 1);
        //     }
        // }

        // Set each bit of X and Y to one in turn.
        // for bit in 0..network.y_names.len() {
        //     let x = 1 << bit;
        //     let y = 1 << bit;

        //     network.set_x(x);
        //     network.set_y(y);

        //     network.unresolve();
        //     let z = network.resolve_z();

        //     let z_expected = x + y;

        //     if z != z_expected {
        //         eprintln!("{x} + {y} = {z} ({z_expected})");
        //         for bit in 0..network.x_names.len() {
        //             let bit_expected = (z_expected >> bit) & 1;
        //             let bit_actual = (z >> bit) & 1;

        //             if bit_actual != bit_expected {
        //                 network.dig_in(network.z_names[bit], bit_expected == 1);
        //             }
        //         }
        //     }
        // }
    }

    // {
    //     // Set each bit of X and Y to one in turn.
    //     for bit in 0..network.y_names.len() {
    //         let x = 1 << bit;
    //         let y = 1 << bit;

    //         let mut network = network.clone();

    //         network.set_x(x);
    //         network.set_y(y);
    //         let z = network.resolve_z();

    //         if z != (x + y) {
    //             eprint!("* {bit}");
    //         }

    //         dbg!(z);
    //     }
    // }

    network.resolve_z()
}


#[derive(Debug, Clone)]
struct Network<'a> {
    connections: BTreeMap<&'a str, Connection<'a>>,
    x_names: Vec<&'a str>,
    y_names: Vec<&'a str>,
    z_names: Vec<&'a str>,
    rx: Vec<&'a str>,
    c: Vec<&'a str>,
}

static LOGGING: AtomicBool = AtomicBool::new(false);

macro_rules! t {
    ($($t:tt)*) => {
        if LOGGING.load(std::sync::atomic::Ordering::SeqCst) {
            eprintln!($($t)*);
        }
    };
}

#[allow(dead_code)]
impl<'a> Network<'a> {
    fn new(connections: BTreeMap<&'a str, Connection<'a>>) -> Self {
        let x_names = names_starting_with(&connections, 'x');
        let y_names = names_starting_with(&connections, 'y');
        let z_names = names_starting_with(&connections, 'z');
        let [rx, c] = Self::find_rx_and_c(&connections, &x_names, &y_names);

        Self {
            connections,
            x_names,
            y_names,
            z_names,
            rx,
            c,
        }
    }

    fn set_x(&mut self, arg: u64) {
        for (i, name) in self.x_names.iter().enumerate() {
            *self.connections.get_mut(name).unwrap() = Connection::Resolved((arg >> i) & 1 == 1);
        }
    }

    fn set_y(&mut self, arg: u64) {
        for (i, name) in self.y_names.iter().enumerate() {
            *self.connections.get_mut(name).unwrap() = Connection::Resolved((arg >> i) & 1 == 1);
        }
    }

    fn resolve_z(&mut self) -> u64 {
        let mut z = 0;

        for &z_name in self.z_names.iter().rev() {
            let enabled = Self::resolve(&mut self.connections, z_name);
            z <<= 1;
            z |= enabled as u64;
        }

        z
    }

    fn involved_in(&self, names: &mut BTreeSet<&'a str>, name: &'a str) {
        match self.connections[name] {
            Connection::Resolved(_) => {}
            Connection::Symbolic([l, r], _, _) => {
                names.insert(name);
                self.involved_in(names, l);
                self.involved_in(names, r);
            }
        }
    }

    fn resolve(connections: &mut BTreeMap<&'a str, Connection<'a>>, name: &'a str) -> bool {
        match connections[name] {
            Connection::Resolved(v) => v,
            Connection::Symbolic(_, _, Some(v)) => v,
            Connection::Symbolic([l, r], op, None) => {
                let v = {
                    let l = Self::resolve(connections, l);
                    let r = Self::resolve(connections, r);

                    match op {
                        Op::And => l & r,
                        Op::Or => l | r,
                        Op::Xor => l ^ r,
                    }
                };
                connections.insert(name, Connection::Symbolic([l, r], op, Some(v)));
                v
            }
        }
    }

    fn unresolve(&mut self) {
        for (_, v) in &mut self.connections {
            if let Connection::Symbolic(_, _, v) = v {
                *v = None;
            }
        }
    }

    fn dig_in(&self, name: &str, expected: bool) {
        let q = self.connections[name];
        let Connection::Symbolic([l, r], op, _) = q else {
            eprintln!("{name} is not symbolic...; caller must be bad?");
            return;
        };

        let ll = self.assume_resolved(l);
        let rr = self.assume_resolved(r);

        eprintln!("{name}: {l}[{ll}] {op} {r}[{rr}] ==> {expected}");

        match (op, ll, rr, expected) {
            (Op::And, true, true, true) => todo!(),
            (Op::And, true, true, false) => { /*not enough info*/ }
            (Op::And, true, false, true) => self.dig_in(r, true),
            (Op::And, true, false, false) => todo!(),
            (Op::And, false, true, true) => self.dig_in(l, true),
            (Op::And, false, true, false) => todo!(),
            (Op::And, false, false, true) => { /*not enough info*/ }
            (Op::And, false, false, false) => todo!(),
            (Op::Or, true, true, true) => todo!(),
            (Op::Or, true, true, false) => todo!(),
            (Op::Or, true, false, true) => todo!(),
            (Op::Or, true, false, false) => self.dig_in(l, false),
            (Op::Or, false, true, true) => todo!(),
            (Op::Or, false, true, false) => todo!(),
            (Op::Or, false, false, true) => { /*not enough info*/ }
            (Op::Or, false, false, false) => todo!(),
            (Op::Xor, true, true, true) => todo!(),
            (Op::Xor, true, true, false) => todo!(),
            (Op::Xor, true, false, true) => todo!(),
            (Op::Xor, true, false, false) => { /*not enough info*/ }
            (Op::Xor, false, true, true) => todo!(),
            (Op::Xor, false, true, false) => { /*not enough info*/ }
            (Op::Xor, false, false, true) => { /*not enough info*/ }
            (Op::Xor, false, false, false) => { /*not enough info*/ }
        }
    }

    fn assume_resolved(&self, name: &str) -> bool {
        match self.connections[name] {
            Connection::Resolved(v) => v,
            Connection::Symbolic(_, _, Some(v)) => v,
            _ => unreachable!(),
        }
    }

    fn roots(&self, name: &'a str) -> BTreeSet<&'a str> {
        let mut set = BTreeSet::new();

        self.roots_(&mut set, name);

        set
    }

    fn roots_(&self, set: &mut BTreeSet<&'a str>, name: &'a str) {
        match self.connections[name] {
            Connection::Resolved(_) => {
                set.insert(name);
            }
            Connection::Symbolic([l, r], _, _) => {
                self.roots_(set, l);
                self.roots_(set, r);
            }
        }
    }

    fn depth(&self, name: &str) -> usize {
        match self.connections[name] {
            Connection::Resolved(_) => 0,
            Connection::Symbolic([l, r], _, _) => {
                let l = self.depth(l);
                let r = self.depth(r);
                usize::max(l, r) + 1
            }
        }
    }

    fn parse_z(&self, bit_n: usize) -> Option<[String; 2]> {
        static LOL: AtomicBool = AtomicBool::new(false);

        if bit_n == 36 && !LOL.load(Ordering::SeqCst) {
            LOL.store(true, Ordering::SeqCst);
            return Some(["qnf".into(), "vpm".into()]);
        }

        let z_name = format!("z{bit_n:02}");
        let found = self.parse_z_inner(&z_name, bit_n);
        if found {
            None
        } else {
            t!("  -- Starting search --");
            for &name in self.connections.keys() {
                if self.parse_z_inner(name, bit_n) {
                    return Some([z_name, name.into()]);
                }
            }
            panic!("Didn't find alternative for {z_name}");
        }
    }

    fn parse_z_inner(&self, name: &str, bit_n: usize) -> bool {
        let z_n = self.connections[name];
        t!("{bit_n:02} {name} parse_z {z_n:?}");

        let Connection::Symbolic([l, r], Op::Xor, _) = z_n else {
            t!("{bit_n:02} {name} parse_z - wrong pattern");
            return false;
        };

        let a0 = self.rx[bit_n] == l;
        let a1 = self.parse_q(r, bit_n);
        let b0 = self.rx[bit_n] == r;
        let b1 = self.parse_q(l, bit_n);

        let a = a0 && a1;
        let b = b0 && b1;

        if b1 && !b0 {
            eprintln!("swap {r} and {} (cx is {})", self.rx[bit_n], self.c[bit_n]);
        }

        t!("{bit_n:02} {name} parse_z - [{a0}, {a1}, {b0}, {b1}]");

        let v = a ^ b;
        t!("{bit_n:02} {name} parse_z - {v}");
        v
    }

    // fn parse_rx(&self, name: &str, bit_n: usize) -> bool {
    //     let found = self.parse_rx_inner(name, bit_n);

    //     // if !found {
    //     //     t!("   --- Starting search ---");
    //     //     for &sname in self.connections.keys() {
    //     //         if self.parse_rx_inner(sname, bit_n) {
    //     //             eprintln!("*** return Some([{name}, {sname}.into()])");
    //     //         }
    //     //     }
    //     // }

    //     found
    // }

    // // rx and c can be precomputed and searched (careful with the swaps?)

    // fn parse_rx_inner(&self, name: &str, bit_n: usize) -> bool {
    //     let rx_n = self.connections[name];
    //     t!("{bit_n:02} {name} parse_rx {rx_n:?}");
    //     let Connection::Symbolic(mut i, Op::Xor, _) = rx_n else {
    //         t!("{bit_n:02} {name} parse_rx - wrong pattern");
    //         return false;
    //     };
    //     i.sort();
    //     let [x, y] = i;

    //     let [x_expected, y_expected] = self.x_y_names(bit_n);

    //     let v = x == x_expected && y == y_expected;
    //     t!("{bit_n:02} {name} parse_rx - {v}");
    //     v
    // }

    fn parse_q(&self, name: &str, bit_n: usize) -> bool {
        // this is special and we assume it's well-formed
        if bit_n <= 1 { return true }


        let q_n = self.connections[name];
        t!("{bit_n:02} {name} parse_q {q_n:?}");
        let Connection::Symbolic([l, r], Op::Or, _) = q_n else {
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

    // fn parse_c(&self, name: &str, bit_n: usize) -> bool {
    //     let c_n = self.connections[name];
    //     t!("{bit_n:02} {name} parse_c {c_n:?}");
    //     let Connection::Symbolic(mut i, Op::And, _) = c_n else {
    //         t!("{bit_n:02} {name} parse_c - wrong pattern");
    //         return false;
    //     };
    //     i.sort();
    //     let [x, y] = i;

    //     let [x_expected, y_expected] = self.x_y_names(bit_n);
    //     let v = x == x_expected && y == y_expected;
    //     t!("{bit_n:02} {name} parse_c - {v}");
    //     v
    // }

    fn parse_p(&self, name: &str, bit_n: usize) -> bool {
        let p_n = self.connections[name];
        t!("{bit_n:02} {name} parse_p {p_n:?}");
        let Connection::Symbolic([l, r], Op::And, _) = p_n else {
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

    fn x_y_names(&self, bit_n: usize) -> [String; 2] {
        let x = format!("x{bit_n:02}");
        let y = format!("y{bit_n:02}");
        [x, y]
    }

    fn reindex_rx_and_c(&mut self) {
        let [rx, c] = Self::find_rx_and_c(&self.connections, &self.x_names, &self.y_names);
        self.rx = rx;
        self.c = c;
    }

    fn find_rx_and_c(connections: &BTreeMap<&'a str, Connection<'a>>, x_names: &[&'a str], y_names: &[&'a str]) -> [Vec<&'a str>; 2] {
        let mut rx = Vec::new();
        let mut c = Vec::new();

        for (n, (&x_tgt, &y_tgt)) in x_names.iter().zip(y_names).enumerate() {
            let mut found_rx = false;
            let mut found_c = false;

            for (&k, v) in connections {
                if let &Connection::Symbolic(mut i, Op::Xor, _) = v {
                    i.sort();
                    let [x, y] = i;

                    if x == x_tgt && y == y_tgt {
                        assert!(!found_rx);
                        found_rx = true;
                        rx.push(k);
                    }
                }

                if let &Connection::Symbolic(mut i, Op::And, _) = v {
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
    Symbolic([&'a str; 2], Op, Option<bool>),
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
