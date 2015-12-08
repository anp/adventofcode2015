pub fn solve_part_one(input: &str) {
    let instructions: Vec<&str> = input.lines().collect();

    let mut lights = [[false; 1000]; 1000];

    for line in instructions {
        let mut tokens: Vec<&str> = line.split(' ').collect();
        tokens.reverse();

        let mut action = tokens.pop().unwrap();
        if action == "turn" {
            action = tokens.pop().unwrap();
        }

        let first_coord_str = tokens.pop().unwrap();
        tokens.pop();
        let second_coord_str = tokens.pop().unwrap();

        let (first_x_str, first_y_str) = first_coord_str.split_at(first_coord_str.find(',')
                                                                                 .unwrap());

        let (second_x_str, second_y_str) = second_coord_str.split_at(second_coord_str.find(',')
                                                                                     .unwrap());

        let first_x = first_x_str.parse::<usize>().unwrap();
        let first_y = first_y_str.trim_matches(',').parse::<usize>().unwrap();

        let second_x = second_x_str.parse::<usize>().unwrap();
        let second_y = second_y_str.trim_matches(',').parse::<usize>().unwrap();

        for x in first_x..(second_x + 1) {
			for y in first_y..(second_y + 1) {
				
				match action {
					"on" => lights[x][y] = true,
					"off" => lights[x][y] = false,
					"toggle" => lights[x][y] = !lights[x][y],
					_ => println!("Invalid action specified: {}", action),
				}
				
			}
		}
    }
	
	let mut lights_on = 0;
	for x in 0..1000 {
		for y in 0..1000 {
			if lights[x][y] {
				lights_on += 1;
			}
		}
	}
	
	println!("After following Santa's instructions, there are {} lights on.", lights_on);
}