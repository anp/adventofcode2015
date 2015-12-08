use std::u64;

use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn solve_part_one(input: &str) {
    println!("Solving challenge for day four, part one...");

    let solution = first_md5_that_starts_with(input, "00000");
    println!("Santa can use {} as the integer input for his AdventCoin function.",
             solution);
}

pub fn solve_part_two(input: &str) {
    println!("Solving challenge for day four, part two...");
    let solution = first_md5_that_starts_with(input, "000000");
    println!("Santa can use {} as the integer input for his AdventCoin function.",
             solution);
}

fn first_md5_that_starts_with(base: &str, needle: &str) -> u64 {
    let mut stomach = Md5::new();
    for i in u64::MIN..u64::MAX {
        let hash_input = &format!("{}{}", &base, &i);

        stomach.input_str(hash_input);
        let result = stomach.result_str();
        let (first_five, _) = result.split_at(needle.len());

        if first_five == needle {
            return i;
        }

        stomach.reset();
    }
    return u64::MAX;
}