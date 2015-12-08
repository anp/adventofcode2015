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
	
	println!("Santa visited {} unique locations at least once each.", visited.len());
}