use std::cmp;

pub fn solve_part_one(input: &str) {
	println!("Solving challenge for day two, part one...");
	
	let presents: Vec<&str> = input.lines().collect();
	let mut paper_needed = 0u32;
	
	for present in presents {
		let dimensions: Vec<&str> = present.split('x').collect();
		
		let l = dimensions[0].parse::<u32>().expect("Invalid integer received!");
		let w = dimensions[1].parse::<u32>().expect("Invalid integer received!");
		let h = dimensions[2].parse::<u32>().expect("Invalid integer received!");
		
		paper_needed += wrapping_paper_needed(l, w, h);
	}
	println!("The elves need {} square feet of wrapping paper.", paper_needed);
}

fn wrapping_paper_needed(l: u32, w: u32, h: u32) -> u32{
	let lw_area = 2 * l * w;
	let wh_area = 2 * w * h;
	let hl_area = 2 * h * l;
	
	let extra = cmp::min(cmp::min(lw_area, wh_area), hl_area) / 2;
	
	lw_area + wh_area + hl_area + extra
}

#[test]
fn test_wrapping_paper_needed() {
	assert_eq!(wrapping_paper_needed(2, 3, 4), 58);
	assert_eq!(wrapping_paper_needed(1, 1, 10), 43);
}