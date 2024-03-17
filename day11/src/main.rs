const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> String {
    solve::<1>(input)
}

fn solve<const NTH: usize>(input: &str) -> String {
    let input = &mut input.trim().to_owned();
    // SAFETY: We know that the input is ASCII, and we're only going to be
    // incrementing within the range of lowercase ascii characters
    let password = unsafe { input.as_bytes_mut() };

    let mut n = 0;
    loop {
        increment_password(password);

        if is_valid_password(password) {
            n += 1;
            if n == NTH {
                return String::from_utf8_lossy(password).to_string();
            }
        }
    }
}

fn increment_password(password: &mut [u8]) {
    for i in (0..password.len()).rev() {
        match password[i] {
            b'z' => password[i] = b'a',
            // skip i, o, l
            b'h' | b'n' | b'k' => {
                password[i] += 2;
                return;
            }
            _ => {
                password[i] += 1;
                return;
            }
        }
    }
}

fn is_valid_password(password: &[u8]) -> bool {
    has_increasing_straight(password)
        && has_no_invalid_characters(password)
        && has_non_overlapping_groups(password)
}

fn has_increasing_straight(password: &[u8]) -> bool {
    for w in password.windows(3) {
        match w {
            [a, b, c] => {
                if *b == a + 1 && *c == a + 2 {
                    return true;
                }
            }
            _ => unreachable!(),
        }
    }

    false
}

fn has_no_invalid_characters(password: &[u8]) -> bool {
    for c in password {
        match c {
            b'i' | b'o' | b'l' => return false,
            _ => {}
        }
    }

    true
}

fn has_non_overlapping_groups(password: &[u8]) -> bool {
    let mut found = 0;
    let mut windows = password.windows(2);

    while let Some(w) = windows.next() {
        match w {
            [a, b] if a == b => {
                windows.next();
                found += 1;
            }
            _ => {}
        }
    }

    found >= 2
}

fn solve_part2(input: &str) -> String {
    solve::<2>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_increasing_straight() {
        for (input, expected) in [
            ("hijklmmn", true),
            ("abbceffg", false),
            ("abbcegjk", false),
            ("abcdffaa", true),
            ("ghjaabcc", true),
        ] {
            assert_eq!(has_increasing_straight(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_has_no_invalid_characters() {
        for (input, expected) in [
            ("hijklmmn", false),
            ("abbceffg", true),
            ("abbcegjk", true),
            ("abcdffaa", true),
            ("ghjaabcc", true),
        ] {
            assert_eq!(has_no_invalid_characters(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_has_non_overlapping_groups() {
        for (input, expected) in [
            ("hijklmmn", false),
            ("abbceffg", true),
            ("abbcegjk", false),
            ("abcdffaa", true),
            ("ghjaabcc", true),
            ("ghjaaacd", false),
        ] {
            assert_eq!(has_non_overlapping_groups(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_increment_password() {
        for (input, expected) in [
            ("abcdefgg", "abcdefgh"),
            ("abcdefgh", "abcdefgj"),
            ("abcdhzzz", "abcdjaaa"),
        ] {
            let password = &mut input.to_owned();
            let expected = &mut expected.to_owned();

            let password = unsafe { password.as_bytes_mut() };
            let expected = unsafe { expected.as_bytes_mut() };

            increment_password(password);

            assert_eq!(password, expected);
        }
    }

    #[test]
    fn test_part1_definitions() {
        for (input, expected) in [
            ("hijklmmn", false),
            ("abbceffg", false),
            ("abbcegjk", false),
            ("abcdffaa", true),
            ("ghjaabcc", true),
        ] {
            assert_eq!(is_valid_password(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_part1() {
        for (input, expected) in [("abcdefgh", "abcdffaa"), ("ghijklmn", "ghjaabcc")] {
            assert_eq!(solve_part1(input), expected);
        }
    }
}
