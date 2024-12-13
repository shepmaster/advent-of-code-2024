use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1 = minimum_tokens(INPUT);

    // Was not double-checking exact equation fit
    assert!(part_1 < 33704);
    assert_eq!(29023, part_1);
}

fn minimum_tokens(s: &str) -> u32 {
    let parse_coord = |x: &str, y: &str| {
        let x = x.parse::<u32>().expect("X not a number");
        let y = y.parse::<u32>().expect("Y not a number");
        (x, y)
    };

    let button_regex = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").expect("Invalid button regex");
    let parse_button = |btn: &str| {
        let (_, [x, y]) = button_regex
            .captures(btn)
            .expect("Match not found")
            .extract();
        parse_coord(x, y)
    };

    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").expect("Invalid prize regex");
    let parse_prize = |prz: &str| {
        let (_, [x, y]) = prize_regex
            .captures(prz)
            .expect("Match not found")
            .extract();
        parse_coord(x, y)
    };

    s.split("\n\n")
        .map(|g| {
            let mut l = g.lines();

            let a = l.next().expect("Missing A");
            let b = l.next().expect("Missing B");
            let p = l.next().expect("Missing prize");

            let a = parse_button(a);
            let b = parse_button(b);
            let prize = parse_prize(p);

            Behavior { a, b, prize }
        })
        .flat_map(|b| b.minimum_tokens())
        .sum()
}

#[derive(Debug)]
struct Behavior {
    a: (u32, u32),
    b: (u32, u32),
    prize: (u32, u32),
}

impl Behavior {
    fn minimum_tokens(&self) -> Option<u32> {
        // n_a * a_x + n_b * b_x = p_x
        // n_a * a_y + n_b * b_y = p_y
        //
        // n_a * (a_x + a_y) + n_b * (b_x + b_y) = p_x + p_y
        // n_a * a_k         + n_b * b_k         = p

        let Self {
            a: (a_x, a_y),
            b: (b_x, b_y),
            prize: (p_x, p_y),
        } = *self;

        let a_k = a_x + a_y;
        let b_k = b_x + b_y;
        let p = p_x + p_y;

        (0..=100)
            .flat_map(|n_a| {
                let leftover = p.checked_sub(n_a * a_k)?;
                if leftover % b_k == 0 {
                    let n_b = leftover / b_k;
                    Some((n_a, n_b))
                } else {
                    None
                }
            })
            .filter(|&(n_a, n_b)| n_a <= 100 && n_b <= 100)
            .filter(|&(n_a, n_b)| n_a * a_x + n_b * b_x == p_x)
            .filter(|&(n_a, n_b)| n_a * a_y + n_b * b_y == p_y)
            .map(|(n_a, n_b)| n_a * 3 + n_b)
            .min()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(480, minimum_tokens(EXAMPLE));
    }
}
