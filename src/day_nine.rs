use std::u32;

use permutohedron::Heap;

pub fn solve_part_one(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let paths = build_adjacency_matrix(lines);

    let mut indices = [0, 1, 2, 3, 4, 5, 6, 7];
    let permuter = Heap::new(&mut indices); // terrible hack
    let mut shortest_path = 1000000;

    for order in permuter {
        let mut distance = 0;

        for i in 1..8 {
            distance += paths[order[i - 1]][order[i]];
        }

        if distance < shortest_path {
            shortest_path = distance;
        }
    }

    println!("The shortest path for Santa to take is {}.", shortest_path);
}

pub fn solve_part_two(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let paths = build_adjacency_matrix(lines);

    let mut indices = [0, 1, 2, 3, 4, 5, 6, 7];
    let permuter = Heap::new(&mut indices); // terrible hack
    let mut longest_path = 0;

    for order in permuter {
        let mut distance = 0;

        for i in 1..8 {
            distance += paths[order[i - 1]][order[i]];
        }

        if distance > longest_path {
            longest_path = distance;
        }
    }

    println!("The longest path for Santa to take is {}.", longest_path);
}

fn build_adjacency_matrix(lines: Vec<&str>) -> [[u32; 8]; 8] {
    let mut paths = [[u32::MAX; 8]; 8];
    for line in lines {
        let mut tokens = line.split(' ');

        let first_city_str = tokens.next().unwrap();
        tokens.next();
        let second_city_str = tokens.next().unwrap();
        tokens.next();

        let first_city_index = match first_city_str {
            "Arbre" => 0, 
            "Tristram" => 1, 
            "Snowdin" => 2, 
            "Faerun" => 3, 
            "Straylight" => 4, 
            "AlphaCentauri" => 5, 
            "Norrath" => 6, 
            "Tambi" => 7,
            _ => panic!(""),
        };

        let second_city_index = match second_city_str {
            "Arbre" => 0, 
            "Tristram" => 1, 
            "Snowdin" => 2, 
            "Faerun" => 3, 
            "Straylight" => 4, 
            "AlphaCentauri" => 5, 
            "Norrath" => 6, 
            "Tambi" => 7,
            _ => panic!(""),
        };

        let distance_str = tokens.next().unwrap();
        let distance = distance_str.parse::<u32>().expect("blarg");

        paths[first_city_index][second_city_index] = distance;
        paths[second_city_index][first_city_index] = distance;
    }
    paths
}