use std::collections::HashMap;

pub fn solve_part_one(input: &str) {
    let instructions: Vec<&str> = input.lines().collect();
    
    let mut wire_values: HashMap<&str, u16> = HashMap::new();
    let wires = build_wire_table(instructions);
    
    let final_signal = get_wire_signal("a", &wires, &mut wire_values);
    println!("Bobby's kit has a signal of {} on wire a.", final_signal);
}

pub fn solve_part_two(input: &str) {
    let instructions: Vec<&str> = input.lines().collect();
    
    let mut wires = build_wire_table(instructions);
    let mut wire_values: HashMap<&str, u16> = HashMap::new();
    
    let intmd_signal = get_wire_signal("a", &wires, &mut wire_values);
    println!("Got initial signal for a of {}.", &intmd_signal);
    
    wires.remove("b");
    wire_values.clear();
    wire_values.insert("b", intmd_signal);
    
    let final_signal = get_wire_signal("a", &wires, &mut wire_values);
    println!("After some malicious interference, Bobby's kit has a signal of {} on a.", final_signal);
}

fn build_wire_table(instructions: Vec<&str>) -> HashMap<&str, &str> {
    let mut wires: HashMap<&str, &str> = HashMap::new();
    for line in instructions {
        let tokens: Vec<&str> = line.split(" -> ").collect();

        let source = tokens[0];
        let target = tokens[1];
        wires.insert(target, source);
    }
    wires
}

fn get_wire_signal<'a>(wire_id: &'a str,
                       wires: &HashMap<&'a str, &'a str>,
                       mut found_values: &mut HashMap<&'a str, u16>)
                       -> u16 {

    match found_values.get(wire_id) {
        Some(v) => return *v,
        None => (),
    }

    let instruction = wires.get(wire_id).unwrap();
    let tokens: Vec<&str> = instruction.split(' ').collect();

    let signal_value = match tokens.len() {
        1 => {
            // no operator means integer operand/signal source
            let signal = tokens[0];
            match signal.parse::<u16>() {
                Ok(n) => n, // it's an integer source
                Err(_) => get_wire_signal(&signal, &wires, &mut found_values),
            }
        }
        2 => {
            // unary operator, only valid one is NOT

            let op = tokens[0];
            let rhs = tokens[1];
            match op {
                "NOT" => !get_wire_signal(&rhs, &wires, &mut found_values),
                _ => panic!("Incorrect length instruction received for {}!", wire_id),
            }
        }
        3 => {
            // binary operator

            let lhs_str = tokens[0];
            let op = tokens[1];
            let rhs_str = tokens[2];

            let lhs = match lhs_str.parse::<u16>() {
                Ok(n) => n, // integer operand
                Err(why) => get_wire_signal(&lhs_str, &wires, &mut found_values), //wire operand
            };

            let rhs = match rhs_str.parse::<u16>() {
                Ok(n) => n, // integer operand
                Err(why) => get_wire_signal(&rhs_str, &wires, &mut found_values), // wire operand
            };

            match op {
                "AND" => lhs & rhs,
                "OR" => lhs | rhs,
                "LSHIFT" => lhs << rhs,
                "RSHIFT" => lhs >> rhs,
                _ => {
                    panic!("Unrecognized instruction ({}) received for {}!",
                           op,
                           wire_id)
                }
            }
        }
        _ => panic!("Incorrect length instruction received for {}!", wire_id),
    };

    found_values.insert(wire_id, signal_value);
    signal_value
}