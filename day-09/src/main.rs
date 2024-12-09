use std::{cmp::Ordering, mem};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    assert_eq!(6340197768906, filesystem_checksum(INPUT.trim()));
    assert_eq!(6363913128533, filesystem_checksum_whole_file(INPUT.trim()));
}

fn filesystem_checksum(s: &str) -> u64 {
    let mut disk = parse(s);

    loop {
        // dump(&disk);

        let mut src = match disk.pop() {
            Some(Content::Free { .. }) => {
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

        let dst_idx = disk.iter().position(|c| matches!(c, Content::Free { .. }));

        // No more free space, done compacting
        let Some(dst_idx) = dst_idx else {
            disk.push(src);
            break;
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

    checksum(&disk)
}

fn filesystem_checksum_whole_file(s: &str) -> u64 {
    let mut disk = parse(s);

    let id = disk
        .iter()
        .rev()
        .find_map(Content::to_file_id)
        .expect("No files present");

    for id in (0..=id).rev() {
        // dump(&disk);

        let src_idx = disk
            .iter()
            .rposition(|c| c.to_file_id() == Some(id))
            .expect("Source file not found");
        let src = &disk[src_idx];

        let dst_idx = disk
            .iter()
            .position(|c| c.free_space().is_some_and(|f| f >= src.len()));
        if let Some(dst_idx) = dst_idx {
            // Only move to the left
            if dst_idx < src_idx {
                let src = disk[src_idx].take();

                let dst = &mut disk[dst_idx];
                let leftover = dst.split_at(src.len());
                *dst = src;
                disk.insert(dst_idx + 1, leftover);
            }
        }

        // Coalesce free space
        {
            let mut i = 0;
            while i + 1 < disk.len() {
                if disk[i].is_free() && disk[i + 1].is_free() {
                    let b = disk.remove(i + 1);
                    disk[i].absorb(b);
                } else {
                    i += 1;
                }
            }
        }
    }

    checksum(&disk)
}

fn parse(s: &str) -> Vec<Content> {
    let mut id = 0;

    s.chars()
        .enumerate()
        .map(|(i, c)| {
            let len = c.to_digit(10).expect("Not a valid digit").into();

            if i % 2 == 0 {
                let c = Content::File { len, id };
                id += 1;
                c
            } else {
                Content::Free { len }
            }
        })
        .collect()
}

fn checksum(disk: &[Content]) -> u64 {
    let mut block = 0;

    disk.iter()
        .map(|c| {
            let sum = match *c {
                Content::File { len, id } => (0..len).map(|i| (i + block) * id).sum::<u64>(),
                _ => 0,
            };
            block += c.len();
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

    fn take(&mut self) -> Self {
        let empty = Content::Free { len: self.len() };
        mem::replace(self, empty)
    }

    fn free_space(&self) -> Option<u64> {
        match *self {
            Content::File { .. } => None,
            Content::Free { len } => Some(len),
        }
    }

    fn to_file_id(&self) -> Option<u64> {
        match *self {
            Content::File { id, .. } => Some(id),
            Content::Free { .. } => None,
        }
    }

    fn is_free(&self) -> bool {
        match self {
            Content::File { .. } => false,
            Content::Free { .. } => true,
        }
    }

    fn absorb(&mut self, other: Content) {
        match self {
            Content::File { .. } => panic!("Can't absorb into a file"),
            Content::Free { len } => *len += other.len(),
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

    #[test]
    fn example_whole_file() {
        assert_eq!(2858, filesystem_checksum_whole_file(EXAMPLE));
    }
}
