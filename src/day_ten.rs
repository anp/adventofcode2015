pub fn solve_part_one(input: &str) {
    let mut result = input.to_string();

    for _ in 0..40 {
        result = look_and_say(&result);
    }

    println!("Look-and-say 40x over produces a string of {} characters.",
             result.len());
}

fn look_and_say(input: &str) -> String {
    let mut buf = String::new();

    let mut input_iter = input.chars();
    let mut count = 1;
    let mut last = input_iter.next().unwrap();

    for c in input_iter {
        if c == last {
            count += 1;
        } else {
            buf.push_str(&count.to_string());
            buf.push(last);

            last = c;
            count = 1;
        }
    }
    buf.push_str(&count.to_string());
    buf.push(last);

    buf
}

#[test]
fn test_look_and_say() {
    assert_eq!("1221", &look_and_say("211"));
    assert_eq!("11", &look_and_say("1"));
    assert_eq!("21", &look_and_say("11"));
    assert_eq!("1211", &look_and_say("21"));
    assert_eq!("111221", &look_and_say("1211"));
    assert_eq!("312211", &look_and_say("111221"));
}