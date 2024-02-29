const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(decode_line_and_compare)
        .fold(0, |acc, (original_len, decoded_len)| {
            acc + original_len - decoded_len
        })
}

fn decode_line_and_compare(line: &str) -> (usize, usize) {
    let original_len = line.len();

    let mut rem = line[1..line.len() - 1].chars().collect::<Vec<_>>();
    let mut rem = &mut rem[..];
    let mut res = vec![];

    while !rem.is_empty() {
        let (num_consumed, parsed) = match rem[..std::cmp::min(rem.len(), 4)] {
            ['\\', 'x', upper, lower] => {
                let upper = from_hex(upper);
                let lower = from_hex(lower);

                let c = (upper * 16 + lower) as char;

                (4, c)
            }
            ['\\', c, ..] => (2, c),
            [c, ..] => (1, c),
            [] => unreachable!(),
        };

        res.push(parsed);
        rem = &mut rem[num_consumed..];
    }

    (original_len, res.len())
}

fn from_hex(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - b'0',
        'a'..='f' => c as u8 - b'a' + 10,
        _ => unreachable!(),
    }
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(encode_line_and_compare)
        .fold(0, |acc, (original_len, decoded_len)| {
            acc + decoded_len - original_len
        })
}

fn encode_line_and_compare(line: &str) -> (usize, usize) {
    let original_len = line.len();

    let mut encoded_len = 2; // for the surrounding quotes

    for c in line.chars() {
        match c {
            '"' | '\\' => encoded_len += 2,
            _ => encoded_len += 1,
        }
    }

    (original_len, encoded_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (input, expected) in [
            ("\"\"", (2, 0)),
            ("\"abc\"", (5, 3)),
            ("\"aaa\\\"aaa\"", (10, 7)),
            ("\"\\x27\"", (6, 1)),
        ] {
            assert_eq!(decode_line_and_compare(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [
            ("\"\"", (2, 6)),
            ("\"abc\"", (5, 9)),
            ("\"aaa\\\"aaa\"", (10, 16)),
            ("\"\\x27\"", (6, 11)),
        ] {
            assert_eq!(encode_line_and_compare(input), expected);
        }
    }
}
