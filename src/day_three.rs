use std::collections::HashSet;

pub fn solve_part_one(input: &str) {
    let mut x_pos = 0;
    let mut y_pos = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((x_pos, y_pos));

    for c in input.chars() {
        match c {
            '^' => y_pos += 1,
            '>' => x_pos += 1,
            'v' => y_pos -= 1,
            '<' => x_pos -= 1,
            _ => println!("Encountered unexpected direction ({}), ignoring.", c),
        }

        visited.insert((x_pos, y_pos));
    }

    println!("Santa visited {} unique locations at least once each.",
             visited.len());
}

pub fn solve_part_two(input: &str) {
    let mut santa_x_pos = 0;
    let mut santa_y_pos = 0;

    let mut robot_x_pos = 0;
    let mut robot_y_pos = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut santas_turn = true;

    for c in input.chars() {
        let mut x_move = 0;
        let mut y_move = 0;
        match c {
            '^' => y_move = 1,
            '>' => x_move = 1,
            'v' => y_move = -1,
            '<' => x_move = -1,
            _ => println!("Encountered unexpected direction ({}), ignoring.", c),
        }

        if santas_turn {
            santa_x_pos += x_move;
            santa_y_pos += y_move;
            visited.insert((santa_x_pos, santa_y_pos));
        } else {
            robot_x_pos += x_move;
            robot_y_pos += y_move;
            visited.insert((robot_x_pos, robot_y_pos));
        }
        santas_turn = !santas_turn;
    }

    println!("Santa and Robo-Santa together visited {} unique locations at least once each.",
             visited.len());
}