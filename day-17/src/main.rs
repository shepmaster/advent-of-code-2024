use itertools::Itertools;
use std::{convert::Infallible, str::FromStr};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!([4, 6, 1, 4, 2, 1, 3, 1, 6], &*run_program(INPUT));
}

fn run_program(s: &str) -> Vec<i32> {
    use Opcode::*;

    let mut l = s.lines();

    let mut parse_reg = || {
        let l = l.next().expect("Missing register");
        let (_, v) = l.split_once(':').expect("Register malformed");
        v.trim()
            .parse::<i32>()
            .expect("Register value not a number")
    };

    let a = parse_reg();
    let b = parse_reg();
    let c = parse_reg();

    l.next().unwrap();

    let p = l.next().expect("Missing program");
    let (_, p) = p.split_once(':').expect("Program malformed");
    let program = p
        .trim()
        .split(',')
        .tuples()
        .map(|(op, a)| match op {
            "0" => Adv(a.parse().unwrap()),
            "1" => Bxl(a.parse().unwrap()),
            "2" => Bst(a.parse().unwrap()),
            "3" => Jnz(a.parse().unwrap()),
            "4" => Bxc(a.parse().unwrap()),
            "5" => Out(a.parse().unwrap()),
            "6" => Bdv(a.parse().unwrap()),
            "7" => Cdv(a.parse().unwrap()),
            _ => panic!("Unknown opcode `{op}`"),
        })
        .collect::<Vec<_>>();

    let mut registers = [a, b, c];
    let mut ip = 0;
    let mut output = vec![];

    while let Some(opcode) = program.get(ip) {
        fn division(arg: &Combo, registers: &Registers) -> i32 {
            let arg = arg.value(registers);
            let numer = registers[REG_A];
            let denom = 2i32.pow(arg as u32);
            numer / denom
        }

        match opcode {
            Adv(arg) => {
                registers[REG_A] = division(arg, &registers);
            }

            Bxl(arg) => {
                registers[REG_B] ^= arg.value();
            }

            Bst(arg) => {
                registers[REG_B] = arg.value(&registers) % 8;
            }

            Jnz(arg) => {
                if registers[REG_A] != 0 {
                    ip = arg.value() as usize;
                    continue;
                }
            }

            Bxc(_arg) => {
                registers[REG_B] ^= registers[REG_C];
            }

            Out(arg) => {
                let v = arg.value(&registers) % 8;
                output.push(v);
            }

            Bdv(arg) => {
                registers[REG_B] = division(arg, &registers);
            }

            Cdv(arg) => {
                registers[REG_C] = division(arg, &registers);
            }
        }

        ip += 1;
    }

    output
}

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;
type Registers = [i32; 3];

#[derive(Debug)]
enum Opcode {
    Adv(Combo),

    Bxl(Lit),

    Bst(Combo),

    Jnz(Lit),

    Bxc(Lit),

    Out(Combo),

    Bdv(Combo),

    Cdv(Combo),
}

#[derive(Debug, Copy, Clone)]
struct Lit(u8);
impl Lit {
    fn value(&self) -> i32 {
        self.0.into()
    }
}

impl FromStr for Lit {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l = s.parse().expect("Invalid value");
        assert!(l < 8);
        Ok(Self(l))
    }
}

#[derive(Debug, Copy, Clone)]
enum Combo {
    Lit(u8),
    A,
    B,
    C,
    Reserved,
}

impl FromStr for Combo {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = match s.parse().expect("Invalid value") {
            v @ 0..=3 => Combo::Lit(v),
            4 => Combo::A,
            5 => Combo::B,
            6 => Combo::C,
            7 => Combo::Reserved,
            _ => panic!("Invalid value"),
        };

        Ok(v)
    }
}

impl Combo {
    fn value(self, registers: &Registers) -> i32 {
        use Combo::*;

        match self {
            Lit(v) => v.into(),
            A => registers[REG_A],
            B => registers[REG_B],
            C => registers[REG_C],
            Reserved => unreachable!("Not possible"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!([4, 6, 3, 5, 6, 3, 5, 2, 1, 0], &*run_program(EXAMPLE));
    }
}
