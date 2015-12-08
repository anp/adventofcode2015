extern crate clap;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::{Arg, App};

mod day_one;
mod day_two;
mod day_three;

fn main() {
    let default_path = "input";
    let current_day = "3";
    let current_part = "2";

    let args = App::new("adventofcode-rs")
                   .version("0.000")
                   .author("Adam Perry <adam.n.perry@gmail.com>")
                   .about("http://adventofcode.com, in Rust stable.")
                   .arg(Arg::with_name("DAY")
                            .short("d")
                            .long("day")
                            .help("Pick which day's solution to run.")
                            .takes_value(true))
                   .arg(Arg::with_name("PART")
                            .short("p")
                            .long("part")
                            .help("Pick which half of the day's solution to run.")
                            .takes_value(true))
                   .arg(Arg::with_name("INPUT_PATH")
                            .short("f")
                            .long("folder")
                            .help("Path to folder containing sample challenge inputs.")
                            .takes_value(true))
                   .get_matches();

    let day = args.value_of("DAY")
                  .unwrap_or(current_day)
                  .parse::<i64>()
                  .expect("Entered invalid integral value for day.");

    let part = args.value_of("PART")
                   .unwrap_or(current_part)
                   .parse::<u8>()
                   .expect("Entered invalid value for part of day.");

    let entered_path = args.value_of("INPUT_PATH")
                           .unwrap_or(default_path);

    match part {
        1 | 2 => println!("Running solution for day {}, part {}...", day, part),
        _ => {
            panic!("Invalid part selected, there are only two parts for each day of the challenge.")
        }
    }

    let mut input_path = PathBuf::from(entered_path);
    input_path.push(format!("{}", day));

    println!("Reading challenge input from {}", input_path.display());

    let mut s = String::new();
    let mut f = File::open(&input_path)
                    .expect(&format!("Couldn't open file {}!", input_path.display()));

    f.read_to_string(&mut s).unwrap();
    let input = s.trim();

    match (day, part) {
        (1, 1) => day_one::solve_part_one(input),
        (1, 2) => day_one::solve_part_two(input),
        (2, 1) => day_two::solve_part_one(input),
        (2, 2) => day_two::solve_part_two(input),
        (3, 1) => day_three::solve_part_one(input),
        (3, 2) => day_three:: solve_part_two(input),
        (_, _) => unimplemented!(),
    }
}
