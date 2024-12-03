use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(167650499, sum_of_products(INPUT));
    assert_eq!(95846796, sum_of_products_conditional(INPUT));
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

fn sum_of_products_conditional(s: &str) -> u64 {
    let re = Regex::new(
        r#"(?x)
        (?:(?<do>do)\(\))

        |

        (?:(?<dont>don't)\(\))

        |

        (?:(?<mul>mul)\((?<l>\d{1,3}),(?<r>\d{1,3})\))
    "#,
    )
    .expect("Malformed regex");

    let mut enabled = true;

    re.captures_iter(s)
        .map(|capture| {
            if capture.name("do").is_some() {
                enabled = true;
                0
            } else if capture.name("dont").is_some() {
                enabled = false;
                0
            } else {
                if enabled {
                    let l = capture
                        .name("l")
                        .expect("Left multiplicand is missing")
                        .as_str();
                    let r = capture
                        .name("r")
                        .expect("right multiplicand is missing")
                        .as_str();
                    let l: u64 = l.parse().expect("Left multiplicand is not a number");
                    let r: u64 = r.parse().expect("Right multiplicand is not a number");
                    l * r
                } else {
                    0
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE_2: &str = include_str!("../example-2.txt");

    #[test]
    fn example() {
        assert_eq!(161, sum_of_products(EXAMPLE));
    }

    #[test]
    fn example_conditional() {
        assert_eq!(48, sum_of_products_conditional(EXAMPLE_2));
    }
}
