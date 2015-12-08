pub fn solve_part_one(input: &str) {
    let mut num_nice_strings = 0u32;

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        if string_is_nice(line) {
            num_nice_strings += 1;
        }
    }
	
	println!("Santa's elvish intern would have found {} nice strings.", num_nice_strings);
}

fn string_is_nice(string: &str) -> bool {
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

#[test]
fn test_string_niceness() {
    assert!(string_is_nice("ugknbfddgicrmopn"));
    assert!(string_is_nice("aaa"));

    assert!(!string_is_nice("jchzalrnumimnmhp"));
    assert!(!string_is_nice("haegwjzuvuyypxyu"));
    assert!(!string_is_nice("dvszwmarrgswjxmb"));
}