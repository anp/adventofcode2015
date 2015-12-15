use std::collections::HashMap;
use std::i64;

use permutohedron::Heap;

pub fn solve_part_one(input: &str) {
    println!("Solving challenge for day thirteen, part one.");

    let map = build_happiness_map(input);
    let mut first_order = ["Alice", "Bob", "Carol", "David", "Eric", "Frank", "George", "Mallory"];
    let permuter = Heap::new(&mut first_order);
    let mut max_happiness = i64::MIN;

    for order in permuter {
        let current_happiness = table_happiness(&order, &map);
        if current_happiness > max_happiness {
            max_happiness = current_happiness;
        }
    }

    println!("The happiest arrangement of the table gives {} happiness units.",
             max_happiness);
}

pub fn solve_part_two(input: &str) {
    println!("Solving challenge for day thirteen, part two.");

    let mut map = build_happiness_map(input);
    let mut first_order = ["Myself", "Alice", "Bob", "Carol", "David", "Eric", "Frank", "George",
                           "Mallory"];

    for (_, person_map) in map.iter_mut() {
        person_map.insert("Myself".to_string(), 0);
    }

    let mut my_map = HashMap::new();
    my_map.insert("Alice".to_string(), 0);
    my_map.insert("Bob".to_string(), 0);
    my_map.insert("Carol".to_string(), 0);
    my_map.insert("David".to_string(), 0);
    my_map.insert("Eric".to_string(), 0);
    my_map.insert("Frank".to_string(), 0);
    my_map.insert("George".to_string(), 0);
    my_map.insert("Mallory".to_string(), 0);
    map.insert("Myself".to_string(), my_map);

    let permuter = Heap::new(&mut first_order);
    let mut max_happiness = i64::MIN;

    for order in permuter {
        let current_happiness = table_happiness(&order, &map);
        if current_happiness > max_happiness {
            max_happiness = current_happiness;
        }
    }

    println!("The happiest arrangement of the table (including myself) gives {} happiness units.",
             max_happiness);
}

fn build_happiness_map(input: &str) -> HashMap<String, HashMap<String, i64>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let tokens = line.split(' ').collect::<Vec<_>>();

        let person = tokens[0];
        let net = tokens[3].parse::<i64>().unwrap() *
                  match tokens[2] {
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
    map
}

fn table_happiness(order: &[&str], lookup: &HashMap<String, HashMap<String, i64>>) -> i64 {
    let mut happiness = 0;

    for i in 0..order.len() - 1 {
        happiness = happiness + lookup.get(order[i]).unwrap().get(order[i + 1]).unwrap();
    }

    for i in 1..order.len() {
        happiness = happiness + lookup.get(order[i]).unwrap().get(order[i - 1]).unwrap();
    }

    happiness = happiness +
                lookup.get(order[0])
                      .unwrap()
                      .get(order[order.len() - 1])
                      .unwrap();
    happiness = happiness +
                lookup.get(order[order.len() - 1])
                      .unwrap()
                      .get(order[0])
                      .unwrap();

    happiness
}