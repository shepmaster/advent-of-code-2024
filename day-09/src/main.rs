use std::cmp::Ordering;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(6340197768906, filesystem_checksum(INPUT.trim()));
}

fn filesystem_checksum(s: &str) -> u64 {
    use Content::*;

    let mut id = 0;

    let mut disk = s
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let len = c.to_digit(10).expect("Not a valid digit").into();

            if i % 2 == 0 {
                let c = File { len, id };
                id += 1;
                c
            } else {
                Free { len }
            }
        })
        .collect::<Vec<_>>();

    loop {
        // dump(&disk);

        let mut src = match disk.pop() {
            Some(Free { .. }) => {
                // ignoring free space
                continue;
            }
            Some(c) => {
                if c.is_empty() {
                    // ignoring empty contents
                    continue;
                }
                c
            }
            None => panic!("Ran out of contents"),
        };

        let dst_idx = disk.iter().position(|c| matches!(c, Free { .. }));

        // No more free space, done compacting
        let Some(dst_idx) = dst_idx else {
            disk.push(src);
            break
        };
        let dst = &mut disk[dst_idx];

        match dst.len().cmp(&src.len()) {
            Ordering::Equal => {
                *dst = src;
            }

            Ordering::Less => {
                let leftover = src.split_at(dst.len());
                *dst = src;
                // Put the remainder back at the end
                disk.push(leftover);
            }

            Ordering::Greater => {
                let leftover = dst.split_at(src.len());
                *dst = src;
                // Put the remainder back after where it started
                disk.insert(dst_idx + 1, leftover);
            }
        }
    }

    let mut block = 0;

    disk.iter()
        .map(|c| {
            let File { len, id } = *c else {
                panic!("Free space left")
            };
            let sum = (0..len).map(|i| (i + block) * id).sum::<u64>();
            block += len;
            sum
        })
        .sum()
}

#[derive(Debug)]
enum Content {
    File { len: u64, id: u64 },

    Free { len: u64 },
}

impl Content {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> u64 {
        match *self {
            Content::File { len, .. } | Content::Free { len } => len,
        }
    }

    fn split_at(&mut self, part_len: u64) -> Self {
        assert!(self.len() >= part_len, "{} >= {}", self.len(), part_len);
        match self {
            Content::File { len, id } => {
                let leftover = Content::File {
                    len: *len - part_len,
                    id: *id,
                };
                *len = part_len;
                leftover
            }
            Content::Free { len } => {
                let leftover = Content::Free {
                    len: *len - part_len,
                };
                *len = part_len;
                leftover
            }
        }
    }
}

#[allow(unused)]
fn dump(d: &[Content]) {
    for c in d {
        match *c {
            Content::File { len, id } => {
                for _ in 0..len {
                    eprint!("{id}");
                }
            }
            Content::Free { len } => {
                for _ in 0..len {
                    eprint!(".");
                }
            }
        }
    }
    eprintln!();
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_tiny() {
        assert_eq!(60, filesystem_checksum("12345"));
    }

    #[test]
    fn example() {
        assert_eq!(1928, filesystem_checksum(EXAMPLE));
    }
}
