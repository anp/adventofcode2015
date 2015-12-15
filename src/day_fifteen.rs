use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

pub fn solve_part_one(input: &str) {
    let mut line_iter = input.lines();
    let sprinkles = parse_ingredient(line_iter.next().unwrap());
    let butterscotch = parse_ingredient(line_iter.next().unwrap());
    let chocolate = parse_ingredient(line_iter.next().unwrap());
    let candy = parse_ingredient(line_iter.next().unwrap());
	
	let mut max_score = 0;

    for spr_qty in 0..101 {
        for but_qty in 0..101 - spr_qty {
            for cho_qty in 0..101 - (spr_qty + but_qty) {
				let mut can_qty = 100 - (spr_qty + but_qty + cho_qty);
				if can_qty < 0 {
					can_qty = 0;
				}
				
				let mut ingredients = HashMap::new();
				ingredients.insert(sprinkles, spr_qty);
				ingredients.insert(butterscotch, but_qty);
				ingredients.insert(chocolate, cho_qty);
				ingredients.insert(candy, can_qty);
				
				let current_score = get_cookie_score(ingredients);
				
				if current_score > max_score {
					max_score = current_score;
				}
            }
        }
    }
	
	println!("The winning cookie has a score of {}.", max_score);
}

fn get_cookie_score(ingredient_counts: HashMap<Ingredient, i32>) -> i32 {
    let mut capacity_score = 0;
    let mut durability_score = 0;
    let mut flavor_score = 0;
    let mut texture_score = 0;

    for (ingredient, quantity) in ingredient_counts {
        capacity_score += ingredient.capacity * quantity;
        durability_score += ingredient.durability * quantity;
        flavor_score += ingredient.flavor * quantity;
        texture_score += ingredient.texture * quantity;
    }
	
	if capacity_score < 0 {
		capacity_score = 0;
	}
	
	if durability_score < 0 {
		durability_score = 0;
	}
	
	if flavor_score < 0 {
		flavor_score = 0;
	}
	
	if texture_score < 0 {
		texture_score = 0;
	}

    capacity_score * durability_score * flavor_score * texture_score
}

fn parse_ingredient(line: &str) -> Ingredient {
    let tokens = line.split(' ')
                     .map(|s| s.trim_matches(':'))
                     .map(|s| s.trim_matches(','))
                     .collect::<Vec<_>>();

    let cap = tokens[2].parse::<i32>().unwrap();
    let dur = tokens[4].parse::<i32>().unwrap();
    let flv = tokens[6].parse::<i32>().unwrap();
    let tex = tokens[8].parse::<i32>().unwrap();
    let cal = tokens[10].parse::<i32>().unwrap();

    Ingredient {
        capacity: cap,
        durability: dur,
        flavor: flv,
        texture: tex,
        calories: cal,
    }
}

#[test]
fn test_cookie_scoring() {
	let butterscotch = Ingredient {
		capacity: -1,
		durability: -2,
		flavor: 6,
		texture: 3,
		calories: 8,
	};
	
	let cinnamon = Ingredient {
		capacity: 2,
		durability: 3,
		flavor: -2,
		texture: -1,
		calories: 3,
	};
	
	let mut ingredients = HashMap::new();
	ingredients.insert(butterscotch, 44);
	ingredients.insert(cinnamon, 56);
	
	assert_eq!(get_cookie_score(ingredients), 62842880);
}