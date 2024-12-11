use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    let mut layout = Layout::from(input);
    layout.compact();

    Checksum::generate(&layout)
}

struct Layout(Vec<Option<FileID>>);

impl<S> From<S> for Layout
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let mut inner = Vec::<Option<FileID>>::new();

        let mut is_file = true;
        let mut file_id = 0;
        for c in value
            .as_ref()
            .lines()
            .next()
            .expect("one-line input")
            .chars()
        {
            let length = c as u8 - b'0';

            if is_file {
                for _ in 0..length {
                    inner.push(Some(FileID(file_id)));
                }
                file_id += 1;
            } else {
                for _ in 0..length {
                    inner.push(None);
                }
            }
            is_file = !is_file;
        }

        Self(inner)
    }
}

impl Deref for Layout {
    type Target = Vec<Option<FileID>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Layout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for maybe_file_id in self.iter() {
            match maybe_file_id {
                Some(file_id) => f.write_fmt(format_args!("{}", **file_id))?,
                _ => f.write_char('.')?,
            }
        }

        Ok(())
    }
}

impl Layout {
    fn compact(&mut self) {
        let mut idx_left = 0;
        let mut idx_right = self.len() - 1;

        while idx_right >= idx_left {
            while idx_left < idx_right && self[idx_left].is_some() {
                idx_left += 1;
            }

            self.swap(idx_left, idx_right);
            idx_right -= 1;
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct FileID(usize);

impl Deref for FileID {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FileID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct Checksum;

impl Checksum {
    fn generate(layout: &[Option<FileID>]) -> usize {
        layout
            .iter()
            .enumerate()
            .map(|(i, file_id)| i * file_id.as_ref().map(FileID::deref).unwrap_or(&0))
            .sum()
    }
}

fn solve_part2(input: &str) -> usize {
    let mut layout = Layout::from(input);
    layout.compact_filewise();

    Checksum::generate(&layout)
}

impl Layout {
    fn compact_filewise(&mut self) {
        let mut idx_right = self.len() - 1;

        // We are going to move in from the end, and must only process one file at a time.
        // Files are in order at the start, so we can therefore count downwards as we find files
        // and if we see a file that is greater than the one we are looking for,
        // we know that it has already been moved.
        let mut max_file_id = {
            let mut curr_file = self[idx_right];
            while curr_file == None {
                idx_right -= 1;
                curr_file = self[idx_right];
            }

            curr_file.expect("We should have found at least something")
        };

        'done: while max_file_id >= FileID(0) {
            let mut curr_file_id = self[idx_right];
            while curr_file_id.is_none() || curr_file_id > Some(max_file_id) {
                idx_right -= 1;
                curr_file_id = self[idx_right];

                // if idx_right == 0 && curr_file == None {
                //     break 'done;
                // }
            }

            let file_end = idx_right;
            while self[idx_right] == curr_file_id {
                if idx_right == 0 {
                    break;
                }

                idx_right -= 1;
            }

            idx_right += 1;
            let file_start = idx_right;
            let file_length = file_end - file_start + 1;

            let mut idx_left = 0;
            let mut curr_block = self[idx_left];
            while idx_left < file_start {
                while curr_block.is_some() {
                    idx_left += 1;
                    curr_block = self[idx_left];
                }

                let free_space_start = idx_left;
                while curr_block.is_none() && idx_left < file_start {
                    idx_left += 1;
                    curr_block = self[idx_left];
                }

                if idx_left > file_start {
                    if *max_file_id == 0 {
                        break 'done;
                    }
                    max_file_id.0 -= 1;
                    break;
                }

                idx_left -= 1;
                let free_space_end = idx_left;
                let free_space_length = free_space_end - free_space_start + 1;

                if free_space_length >= file_length {
                    let (free_space_slice, file_slice) = self.split_at_mut(file_start);
                    free_space_slice[free_space_start..free_space_start + file_length]
                        .swap_with_slice(&mut file_slice[0..file_length]);
                    max_file_id.0 -= 1;

                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        for (input, expected) in [
            (r"0099811188827773336446555566", 1928),
            (r"00992111777.44.333....5555.6666.....8888..", 2858),
        ] {
            assert_eq!(
                Checksum::generate(
                    &input
                        .chars()
                        .map(|c| match c {
                            '0'..='9' => Some(FileID((c as u8 - b'0') as usize)),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                ),
                expected
            );
        }
    }

    #[test]
    fn test_part1() {
        for (input, expected) in [(r"2333133121414131402", 1928)] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [(r"2333133121414131402", 2858)] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
