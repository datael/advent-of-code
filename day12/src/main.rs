const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> i32 {
    let mut remaining = input.as_bytes();
    let mut sum = 0;

    while !remaining.is_empty() {
        if let (Some(Value::Number(number)), rem) = parse_number(remaining) {
            sum += number;
            remaining = rem;
        } else {
            remaining = &remaining[1..];
        }
    }

    sum
}

fn parse_number(mut bytes: &[u8]) -> (Option<Value>, &[u8]) {
    let mut was_number = false;
    let mut number = 0;
    let mut maybe_negate = 1;

    if let Some(c) = bytes.first() {
        if *c == b'-' {
            maybe_negate = -1;
        }
    }

    loop {
        let first_of_negative = !was_number && maybe_negate == -1;
        let index = if first_of_negative { 1 } else { 0 };

        match bytes.iter().copied().nth(index) {
            Some(c @ b'0'..=b'9') => {
                bytes = &bytes[1..];

                if first_of_negative {
                    // also consume the '-' the first time we get here if there was one
                    bytes = &bytes[1..];
                }

                number = number * 10 + (c - b'0') as i32;
                was_number = true;
            }
            _ => break,
        }
    }

    let res = if was_number {
        Some(Value::Number(number * maybe_negate))
    } else {
        None
    };

    (res, bytes)
}

fn solve_part2(input: &str) -> i32 {
    if let (Some(obj), _) = parse_any(input.as_bytes()) {
        return obj.sum_values(&b"red".to_vec());
    }

    0
}

#[derive(Debug, PartialEq)]
enum Value {
    Number(i32),
    Array(Vec<Value>),
    Dictionary(Vec<(Vec<u8>, Value)>),
    String(Vec<u8>),
}

impl Value {
    fn sum_values(&self, skip: &Vec<u8>) -> i32 {
        match self {
            Value::Number(n) => *n,
            Value::Array(values) => values.iter().map(|v| v.sum_values(skip)).sum(),
            Value::Dictionary(values) => {
                let exclude = values
                    .iter()
                    .any(|(_, v)| matches!(v, Value::String(string) if string == skip));

                if exclude {
                    0
                } else {
                    values.iter().map(|(_, v)| v.sum_values(skip)).sum()
                }
            }
            _ => 0,
        }
    }
}

fn parse_any(bytes: &[u8]) -> (Option<Value>, &[u8]) {
    for parser in [parse_number, parse_string, parse_array, parse_dictionary].iter() {
        if let (Some(value), rem) = parser(bytes) {
            return (Some(value), rem);
        }
    }

    (None, bytes)
}

fn parse_array(mut bytes: &[u8]) -> (Option<Value>, &[u8]) {
    if bytes.first() != Some(&b'[') || bytes.len() < 2 {
        return (None, bytes);
    }

    let bytes_orig = bytes;
    bytes = &bytes[1..];

    let mut values = vec![];

    if bytes.first() == Some(&b']') {
        return (Some(Value::Array(values)), &bytes[1..]);
    }

    if let (Some(value), rem) = parse_any(bytes) {
        bytes = rem;
        values.push(value);

        loop {
            if bytes.first() == Some(&b',') {
                bytes = &bytes[1..];
            } else if bytes.first() == Some(&b']') {
                bytes = &bytes[1..];
                break;
            } else {
                return (None, bytes_orig);
            }

            if let (Some(value), rem) = parse_any(bytes) {
                values.push(value);
                bytes = rem;
            } else {
                return (None, bytes_orig);
            }
        }
    } else {
        return (None, bytes_orig);
    }

    (Some(Value::Array(values)), bytes)
}

fn parse_string(mut bytes: &[u8]) -> (Option<Value>, &[u8]) {
    if bytes.first() != Some(&b'"') || bytes.len() < 2 {
        return (None, bytes);
    }

    let bytes_orig = bytes;
    bytes = &bytes[1..];

    let mut escaped = false;
    let mut string = Vec::new();

    for (i, c) in bytes.iter().enumerate() {
        if escaped {
            escaped = false;
            string.push(*c);
        } else {
            match c {
                b'"' if !escaped => {
                    return (Some(Value::String(string)), &bytes[i + 1..]);
                }
                b'\\' => {
                    escaped = true;
                }
                c => string.push(*c),
            }
        }
    }

    (None, bytes_orig)
}

fn parse_dictionary(mut bytes: &[u8]) -> (Option<Value>, &[u8]) {
    if bytes.first() != Some(&b'{') || bytes.len() < 2 {
        return (None, bytes);
    }

    let bytes_orig = bytes;
    bytes = &bytes[1..];

    let mut values = vec![];

    if bytes.first() == Some(&b'}') {
        return (Some(Value::Dictionary(values)), &bytes[1..]);
    }

    if let (Some(Value::String(key)), rem) = parse_string(bytes) {
        bytes = rem;

        if bytes.first() != Some(&b':') {
            return (None, bytes_orig);
        }

        bytes = &bytes[1..];

        if let (Some(value), rem) = parse_any(bytes) {
            bytes = rem;
            values.push((key, value));

            loop {
                if bytes.first() == Some(&b',') {
                    bytes = &bytes[1..];
                } else if bytes.first() == Some(&b'}') {
                    bytes = &bytes[1..];
                    break;
                } else {
                    return (None, bytes_orig);
                }

                if let (Some(Value::String(key)), rem) = parse_string(bytes) {
                    bytes = rem;

                    if bytes.first() != Some(&b':') {
                        return (None, bytes_orig);
                    }

                    bytes = &bytes[1..];

                    if let (Some(value), rem) = parse_any(bytes) {
                        values.push((key, value));
                        bytes = rem;
                    } else {
                        return (None, bytes_orig);
                    }
                } else {
                    return (None, bytes_orig);
                }
            }

            (Some(Value::Dictionary(values)), bytes)
        } else {
            (None, bytes_orig)
        }
    } else {
        (None, bytes_orig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        for (input, expected) in [
            (r#""#, (None, [].as_slice())),
            (r#"a"#, (None, b"a".as_slice())),
            (r#"0"#, (Some(Value::Number(0)), [].as_slice())),
            (r#"-1"#, (Some(Value::Number(-1)), [].as_slice())),
            (r#"-2?"#, (Some(Value::Number(-2)), b"?".as_slice())),
            (r#"-3\?"#, (Some(Value::Number(-3)), b"\\?".as_slice())),
            (r#".-3\?"#, (None, b".-3\\?".as_slice())),
        ] {
            assert_eq!(parse_number(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_part1() {
        for (input, expected) in [
            (r#"[1,2,3]"#, 6),
            (r#"{"a":2,"b":4}"#, 6),
            (r#"[[[3]]]"#, 3),
            (r#"{"a":{"b":4},"c":-1}"#, 3),
            (r#"{"a":[-1,1]}"#, 0),
            (r#"[-1,{"a":1}]"#, 0),
            (r#"[]"#, 0),
            (r#"{}"#, 0),
        ] {
            assert_eq!(solve_part1(input), expected);
        }
    }

    #[test]
    fn test_parse_string() {
        for (input, expected) in [
            (r#""#, (None, [].as_slice())),
            (r#"""#, (None, b"\"".as_slice())),
            (r#""""#, (Some(Value::String(b"".to_vec())), [].as_slice())),
            (
                r#""hello""#,
                (Some(Value::String(b"hello".to_vec())), [].as_slice()),
            ),
            (
                r#""he\"llo""#,
                (Some(Value::String(b"he\"llo".to_vec())), [].as_slice()),
            ),
            (
                r#""he\"llo"rem"#,
                (Some(Value::String(b"he\"llo".to_vec())), "rem".as_bytes()),
            ),
            (
                r#""he\\"llo"!\!"#,
                (
                    Some(Value::String(b"he\\".to_vec())),
                    "llo\"!\\!".as_bytes(),
                ),
            ),
            (
                r#""he\\\"llo"??"#,
                (Some(Value::String(b"he\\\"llo".to_vec())), "??".as_bytes()),
            ),
            (r#""hello"#, (None, "\"hello".as_bytes())),
        ] {
            assert_eq!(parse_string(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_parse_array() {
        for (input, expected) in [
            (r#""#, (None, [].as_slice())),
            (r#"""#, (None, b"\"".as_slice())),
            (r#"{"#, (None, b"{".as_slice())),
            (r#"["#, (None, b"[".as_slice())),
            (r#"[]"#, (Some(Value::Array(vec![])), [].as_slice())),
            (r#"[["#, (None, b"[[".as_slice())),
            (r#"[[]"#, (None, b"[[]".as_slice())),
            (r#"[]!"#, (Some(Value::Array(vec![])), b"!".as_slice())),
            (r#"[][!"#, (Some(Value::Array(vec![])), b"[!".as_slice())),
            (r#"[[],[!"#, (None, b"[[],[!".as_slice())),
            (r#"[[,],[!"#, (None, b"[[,],[!".as_slice())),
            (
                r#"[[],[]]]"#,
                (
                    Some(Value::Array(vec![
                        Value::Array(vec![]),
                        Value::Array(vec![]),
                    ])),
                    b"]".as_slice(),
                ),
            ),
            (
                r#"[0]"#,
                (Some(Value::Array(vec![Value::Number(0)])), [].as_slice()),
            ),
            (
                r#"[0]?"#,
                (Some(Value::Array(vec![Value::Number(0)])), b"?".as_slice()),
            ),
            (
                r#"[-1],"#,
                (Some(Value::Array(vec![Value::Number(-1)])), b",".as_slice()),
            ),
            (
                r#"[-1,1,2,3]"#,
                (
                    Some(Value::Array(vec![
                        Value::Number(-1),
                        Value::Number(1),
                        Value::Number(2),
                        Value::Number(3),
                    ])),
                    [].as_slice(),
                ),
            ),
            (
                r#"[-1,[1,2],3]"#,
                (
                    Some(Value::Array(vec![
                        Value::Number(-1),
                        Value::Array(vec![Value::Number(1), Value::Number(2)]),
                        Value::Number(3),
                    ])),
                    [].as_slice(),
                ),
            ),
            (
                r#"[-1,[1,[2]],3]"#,
                (
                    Some(Value::Array(vec![
                        Value::Number(-1),
                        Value::Array(vec![Value::Number(1), Value::Array(vec![Value::Number(2)])]),
                        Value::Number(3),
                    ])),
                    [].as_slice(),
                ),
            ),
        ] {
            assert_eq!(parse_array(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_parse_object() {
        for (input, expected) in [
            (r#""#, (None, [].as_slice())),
            (r#"""#, (None, b"\"".as_slice())),
            (r#"{"#, (None, b"{".as_slice())),
            (r#"["#, (None, b"[".as_slice())),
            (r#"{}"#, (Some(Value::Dictionary(vec![])), [].as_slice())),
            (r#"{]"#, (None, b"{]".as_slice())),
            (r#"{{"#, (None, b"{{".as_slice())),
            (r#"{{}"#, (None, b"{{}".as_slice())),
            (r#"{}!"#, (Some(Value::Dictionary(vec![])), b"!".as_slice())),
            (
                r#"{}{!"#,
                (Some(Value::Dictionary(vec![])), b"{!".as_slice()),
            ),
            (r#"{{},{!"#, (None, b"{{},{!".as_slice())),
            (r#"{{,},{!"#, (None, b"{{,},{!".as_slice())),
            (r#"{{:},{!"#, (None, b"{{:},{!".as_slice())),
            (
                r#"{"a":{}}"#,
                (
                    Some(Value::Dictionary(vec![(
                        b"a".to_vec(),
                        Value::Dictionary(vec![]),
                    )])),
                    [].as_slice(),
                ),
            ),
            (
                r#"{"a":{},"b":-1,"c":[]}"#,
                (
                    Some(Value::Dictionary(vec![
                        (b"a".to_vec(), Value::Dictionary(vec![])),
                        (b"b".to_vec(), Value::Number(-1)),
                        (b"c".to_vec(), Value::Array(vec![])),
                    ])),
                    [].as_slice(),
                ),
            ),
            (
                r#"{"a":{"b":{}}}"#,
                (
                    Some(Value::Dictionary(vec![(
                        b"a".to_vec(),
                        Value::Dictionary(vec![(b"b".to_vec(), Value::Dictionary(vec![]))]),
                    )])),
                    [].as_slice(),
                ),
            ),
        ] {
            assert_eq!(parse_dictionary(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_parse_any() {
        for (input, expected) in [
            (r#""#, (None, [].as_slice())),
            (r#"""#, (None, b"\"".as_slice())),
            (r#"{"#, (None, b"{".as_slice())),
            (r#"["#, (None, b"[".as_slice())),
            (r#"{}"#, (Some(Value::Dictionary(vec![])), [].as_slice())),
            (r#"[]"#, (Some(Value::Array(vec![])), [].as_slice())),
            (
                r#"[{}]"#,
                (
                    Some(Value::Array(vec![Value::Dictionary(vec![])])),
                    [].as_slice(),
                ),
            ),
            (r#"1"#, (Some(Value::Number(1)), [].as_slice())),
        ] {
            assert_eq!(parse_any(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expected) in [
            (r#"[1,2,3]"#, 6),
            (r#"[1,{"c":"red","b":2},3]"#, 4),
            (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
            (r#"[1,"red",5]"#, 6),
        ] {
            assert_eq!(solve_part2(input), expected);
        }
    }
}
