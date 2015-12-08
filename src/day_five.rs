pub fn solve_part_one(input: &str) {
    let mut num_nice_strings = 0u32;

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        if string_is_nice_old(line) {
            num_nice_strings += 1;
        }
    }

    println!("Santa's elvish intern would have found {} nice strings.",
             num_nice_strings);
}

fn string_is_nice_old(string: &str) -> bool {
    let mut num_vowels = 0;
    let mut twice_in_a_row = false;

    let (first_char, remaining_word) = string.split_at(1);
    let mut previous_character = first_char.chars().next().unwrap();

    match previous_character {
        'a' | 'e' | 'i' | 'o' | 'u' => num_vowels += 1,
        _ => (),
    }

    for letter in remaining_word.chars() {
        match letter {
            'a' | 'e' | 'i' | 'o' | 'u' => num_vowels += 1,
            _ => (),
        }

        if previous_character == letter {
            twice_in_a_row = true;
        }

        match (previous_character, letter) {
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return false,
            _ => (),
        }

        previous_character = letter;
    }

    num_vowels >= 3 && twice_in_a_row
}

pub fn solve_part_two(input: &str) {
    let mut num_nice_strings = 0u32;

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        if string_is_nice(line) {
            num_nice_strings += 1;
        }
    }

    println!("Santa's elvish intern would have found {} nice strings using the new method.",
             num_nice_strings);
}

fn string_is_nice(input: &str) -> bool {
    let mut double_pair = false;

    let mut spaced_iter = input.chars().peekable();
    let mut current_pos = 0;
    loop {

        let first = match spaced_iter.next() {
            Some(c) => c,
            None => break,
        };

        let second = match spaced_iter.peek() {
            Some(c) => c,
            None => break,
        };

        let search_pattern = format!("{}{}", first, second);

        match input.rfind(&search_pattern) {
            Some(loc) => {
                if (loc as isize - current_pos).abs() > 1 {
                    double_pair = true;
                    break;
                }
            }
            None => (),
        }

        current_pos += 1;
    }

    let mut spaced_pair = false;

    let mut double_iter = input.chars().peekable();
    let mut two_back = double_iter.next().unwrap();
    loop {

        let current = match double_iter.next() {
            Some(c) => c,
            None => break,
        };

        let ahead = match double_iter.peek() {
            Some(c) => c,
            None => break,
        };

        if two_back == *ahead {
            spaced_pair = true;
            break;
        }

        two_back = current;
    }

    double_pair && spaced_pair
}

#[test]
fn test_new_string_niceness() {
    assert!(string_is_nice("xyxy"));
    assert!(string_is_nice("qjhvhtzxzqqjkmpb"));
    assert!(string_is_nice("xxyxx"));

    assert!(!string_is_nice("aaa"));
    assert!(!string_is_nice("aabcdefgaa"));
    assert!(!string_is_nice("xyx"));
    assert!(!string_is_nice("abcdefeghi"));
    assert!(!string_is_nice("uurcxstgmygtbstg"));
    assert!(!string_is_nice("ieodomkazucvgmuy"));
}

#[test]
fn test_old_string_niceness() {
    assert!(string_is_nice_old("ugknbfddgicrmopn"));
    assert!(string_is_nice_old("aaa"));

    assert!(!string_is_nice_old("jchzalrnumimnmhp"));
    assert!(!string_is_nice_old("haegwjzuvuyypxyu"));
    assert!(!string_is_nice_old("dvszwmarrgswjxmb"));
}