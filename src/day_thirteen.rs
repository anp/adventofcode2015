use std::collections::HashMap;
use std::i64;

use permutohedron::Heap;

pub fn solve_part_one(input: &str) {
    println!("Solving challenge for day thirteen, part one.");
    
    let mut map = HashMap::new();
    
    for line in input.lines() {
        let tokens = line.split(' ').collect::<Vec<_>>();
        
        let person = tokens[0];
        let net = tokens[3].parse::<i64>().unwrap() * match tokens[2] {
            "gain" => 1,
            "lose" => -1,
            _ => i64::MAX,
        };
        let cause = tokens[tokens.len() - 1].trim_right_matches('.');
        
        if !map.contains_key(person) {
            map.insert(person.to_owned(), HashMap::new());
        }
        
        let mut person_map = map.get_mut(person).unwrap();
        
        person_map.insert(cause.to_owned(), net);
    }
    
    let map = map;
    let mut first_order = ["Alice", "Bob", "Carol", "David", "Eric", "Frank", "George", "Mallory"];
    let permuter = Heap::new(&mut first_order);
	let mut max_happiness = i64::MIN;
    
    for order in permuter {
        let current_happiness = table_happiness(&order, &map);
        if current_happiness > max_happiness {
            max_happiness = current_happiness;
        }
    }
    
    println!("The happiest arrangement of the table gives {} happiness units.", max_happiness);
}

fn table_happiness(order: &[&str; 8], lookup: &HashMap<String, HashMap<String, i64>>) -> i64{
    let mut happiness = 0;
    
    for i in 0..7 {
        happiness = happiness + lookup.get(order[i]).unwrap().get(order[i + 1]).unwrap();
    }
    
    for i in 1..8 {
        happiness = happiness + lookup.get(order[i]).unwrap().get(order[i - 1]).unwrap();
    }
    
    happiness = happiness + lookup.get(order[0]).unwrap().get(order[7]).unwrap();
    happiness = happiness + lookup.get(order[7]).unwrap().get(order[0]).unwrap();
    
    happiness
}