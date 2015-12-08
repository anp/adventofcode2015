pub fn solve_part_one(input: &str) {
    let entries: Vec<&str> = input.lines().collect();

    let mut total_encoded_chars = 0;
    let mut total_unquoted_chars = 0;

    for entry in entries {
        total_encoded_chars += num_chars(entry);
        total_unquoted_chars += num_chars_unescaped(entry);
    }

    println!("Santa's list has {} characters of encoding overhead.",
             total_encoded_chars - total_unquoted_chars);
}

pub fn solve_part_two(input: &str) {
    let entries: Vec<&str> = input.lines().collect();

    let mut difference = 0;
    for entry in entries {
        let escaped = escape_string(entry);
        difference += num_chars(&escaped) - num_chars(entry);
    }

    println!("Santa's list has {} characters of re-encoding overhead.",
             difference);
}

fn num_chars(string: &str) -> i32 {
    string.chars().collect::<Vec<_>>().len() as i32
}

fn num_chars_unescaped(string: &str) -> i32 {
    let mut chars: Vec<char> = string.chars().collect();
    let mut actual_chars = 0;

    chars.remove(0);
    chars.pop();

    let mut char_iter = chars.iter();
    loop {
        // should loop once for each "actual" character in memory
        match char_iter.next() {
            Some(c) => {
                actual_chars += 1;

                match *c {
                    '\\' => {
                        match char_iter.next() { // we know that we're skipping at least one char
                            Some(n) => {
                                if *n == 'x' {
                                    // we skipped an x, but we still have two hex to go
                                    char_iter.next();
                                    char_iter.next();
                                }
                            }
                            None => (),
                        }
                    }
                    _ => (),
                }
            }
            None => break,
        }
    }
    actual_chars
}

fn escape_string(string: &str) -> String {
    let mut buf = String::from("\"");

    for c in string.chars() {
        match c {
            '"' => {
                buf.push('\\');
                buf.push('\"');
            }
            '\\' => {
                buf.push('\\');
                buf.push('\\');
            }
            _ => buf.push(c),
        }
    }

    buf.push('\"');
    buf
}

#[test]
fn test_string_escape() {
    assert_eq!("\"\\\"\\\"\"", escape_string("\"\""));
    assert_eq!("\"\\\"abc\\\"\"", escape_string("\"abc\""));
    assert_eq!("\"\\\"aaa\\\\\\\"aaa\\\"\"",
               escape_string("\"aaa\\\"aaa\""));
    assert_eq!("\"\\\"\\\\x27\\\"\"", escape_string("\"\\x27\""));
}

#[test]
fn test_encoded_char_count() {
    assert_eq!(2, num_chars("\"\""));
    assert_eq!(5, num_chars("\"abc\""));
    assert_eq!(10, num_chars("\"aaa\\\"aaa\""));
    assert_eq!(6, num_chars("\"\\x27\""));
    assert_eq!(6, num_chars("\"\\\\\\\\\""));
}

#[test]
fn test_unquoted_char_count() {
    assert_eq!(0, num_chars_unescaped("\"\""));
    assert_eq!(3, num_chars_unescaped("\"abc\""));
    assert_eq!(7, num_chars_unescaped("\"aaa\\\"aaa\""));
    assert_eq!(1, num_chars_unescaped("\"\\x27\""));
    assert_eq!(2, num_chars_unescaped("\"\\\\\\\\\""));
}