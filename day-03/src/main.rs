use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(167650499, sum_of_products(INPUT));
}

fn sum_of_products(s: &str) -> u64 {
    let re = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).expect("Malformed regex");
    re.captures_iter(s)
        .map(|capture| {
            let (_, [l, r]) = capture.extract();
            let l: u64 = l.parse().expect("Left multiplicand is not a number");
            let r: u64 = r.parse().expect("Right multiplicand is not a number");
            l * r
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(161, sum_of_products(EXAMPLE));
    }
}
