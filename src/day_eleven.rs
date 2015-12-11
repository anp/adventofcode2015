pub fn solve_part_one(input: &str) {
    let mut array = encode_password(input);

    while !password_is_valid(&array) {
        array = increment_password(array);
    }

    println!("Santa's next password will be \"{}\"",
             decode_password(&array));
}

fn password_is_valid(password: &[u8; 8]) -> bool {
    let mut has_straight = false;
    let mut has_confusion = false;
    let mut first_pair_start = 8;
    let mut second_pair_start = 8;

    // check for a straight
    for i in 0..6 {
        if password[i] + 1 == password[i + 1] && password[i] + 2 == password[i + 2] {
            has_straight = true;
        }
    }

    for i in 0..8 {
        if password[i] == 8 || password[i] == 11 || password[i] == 14 {
            has_confusion = true;
        }
    }

    for i in 0..7 {
        if password[i] == password[i + 1] {
            if first_pair_start < i - 1 {
                second_pair_start = i;
            } else if first_pair_start == 8 {
                first_pair_start = i;
            }
        }
    }

    has_straight && !has_confusion && second_pair_start < 8
}

fn encode_password(input: &str) -> [u8; 8] {
    assert_eq!(8, input.len());

    let mut i = 0;
    let mut result = [0; 8];
    for c in input.chars() {
        result[i] = match c {
            'a'...'z' => c as u8 - 97,
            _ => 0,
        };
        i += 1;
    }

    result
}

fn decode_password(input: &[u8; 8]) -> String {
    let mut buf = String::new();

    for i in 0..8 {
        buf.push((input[i] + 97) as char);
    }

    buf
}

fn increment_password(mut current: [u8; 8]) -> [u8; 8] {
    let mut i = 7;

    current[i] += 1;
    while current[i] > 25 {
        current[i] = 0;

        if i > 0 {
            i -= 1;
        } else {
            i = 7;
        }

        current[i] += 1;
    }

    current
}