pub fn solve_part_one(input: &str) {
    println!("Solving challenge for day one, part one...");

    let mut floor = 0;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => println!("Found unexpected character ({}), skipping.", c),
        }
    }
    println!("Santa will end up on floor {}.", floor);
}

pub fn solve_part_two(input: &str) {
    println!("Solving challenge for day one, part two...");

    let mut floor = 0;
    let mut char_pos = 1;

    for c in input.chars() {
        match c {
            '(' => {
                floor += 1;
                char_pos += 1;
            }
            ')' => {
                floor -= 1;
                if floor < 0 {
                    println!("Santa first entered the basement at instruction {}.",
                             char_pos);
                    break;
                }
                char_pos += 1;
            }
            _ => println!("Found unexpected character ({}), skipping.", c),
        }
    }
}