use itertools::Itertools;
use std::{convert::Infallible, str::FromStr};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!([4, 6, 1, 4, 2, 1, 3, 1, 6], &*run_program(INPUT));

    let part_2 = part_2();

    // Was only looking at 8 numbers
    assert!(part_2 > 25295828419877);
    // Pasted the penultimate number, not the ultimate number
    assert!(part_2 > 25295828419909);

    assert_eq!(202366627359274, part_2);
}

fn run_program(s: &str) -> Vec<u64> {
    let (_, program, registers) = parse(s);
    run(program, registers)
}

fn parse(s: &str) -> (Vec<u64>, Vec<Opcode>, [u64; 3]) {
    use Opcode::*;

    let mut l = s.lines();

    let mut parse_reg = || {
        let l = l.next().expect("Missing register");
        let (_, v) = l.split_once(':').expect("Register malformed");
        v.trim()
            .parse::<u64>()
            .expect("Register value not a number")
    };

    let a = parse_reg();
    let b = parse_reg();
    let c = parse_reg();

    l.next().unwrap();

    let p = l.next().expect("Missing program");
    let (_, p) = p.split_once(':').expect("Program malformed");

    let raw_program = p
        .trim()
        .split(',')
        .map(|c| c.parse().expect("Invalid number"))
        .collect();

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
        .collect();

    (raw_program, program, [a, b, c])
}

fn run(program: Program, mut registers: Registers) -> Vec<u64> {
    use Opcode::*;

    let mut ip = 0;
    let mut output = vec![];

    while let Some(opcode) = program.get(ip) {
        fn division(arg: &Combo, registers: &Registers) -> u64 {
            let arg = arg.value(registers);
            let numer = registers[REG_A];
            let denom = 2u64.pow(arg as u32);
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
type Registers = [u64; 3];
type Program = Vec<Opcode>;

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
    fn value(&self) -> u64 {
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
    fn value(self, registers: &Registers) -> u64 {
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

// My specific input disassembles to
//
// BST A
// BXL 1
// CDV B
// BXC
// BXL 4
// ADV 3
// OUT B
// JNZ 0
//
// Logic-wise, that's
//
// B = A % 8
// B ^= 1
// C = A / 2**B
// B ^= C
// B ^= 4
// A = A / 8
// OUT B % 8
// JNZ 0
//
// Applying and substituting each step
//
// A = A0, B = B0, C = C0
// 1. B = A0 % 8
// 2. B = (A0 % 8) ^ 1
// 3. C = A0 / 2**((A0 % 8) ^ 1)
// 4. B = ((A0 % 8) ^ 1) ^ (A0 / 2**((A0 % 8) ^ 1))
// 5. B = (((A0 % 8) ^ 1) ^ (A0 / 2**((A0 % 8) ^ 1))) ^ 4
// 6. A = A0 / 8
// 7. OUT ((((A0 % 8) ^ 1) ^ (A0 / 2**((A0 % 8) ^ 1))) ^ 4) % 8
// 8. JNZ 0
//
// Combined, each loop's total logic is
//
// A = A0 / 8
// B = (((A0 % 8) ^ 1) ^ (A0 / 2**((A0 % 8) ^ 1))) ^ 4
// OUT B % 8
// JNZ 0

fn part_2() -> u64 {
    let (raw_program, program, mut registers) = parse(INPUT);

    // Starts at zero so the program would exit the loop
    let mut a0 = 0;

    // Walk backwards to find each `a0` (the starting value of `a`)
    // that would output the desired `b`.

    'outer: for &output_b in raw_program.iter().rev() {
        // A = A0 / 8
        // So multiply by 8 and then start searching upward from there
        for a in (a0 * 8).. {
            // B = (((A0 % 8) ^ 1) ^ (A0 / 2**((A0 % 8) ^ 1))) ^ 4
            let k = u8::try_from((a % 8) ^ 1).unwrap();
            let b = (u64::from(k) ^ (a / 2u64.pow(k.into()))) ^ 4;

            if b % 8 == output_b {
                a0 = a;
                continue 'outer;
            }
        }

        panic!("Did not find a viable a");
    }

    registers[REG_A] = a0;
    let output = run(program, registers);
    assert_eq!(raw_program, output);

    a0
}
