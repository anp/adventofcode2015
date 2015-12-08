extern crate clap;

use clap::{Arg, App};

fn main() {
    let args = App::new("adventofcode-rs")
        .version("0.000")
        .author("Adam Perry <adam.n.perry@gmail.com>")
        .about("http://adventofcode.com")
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
        .get_matches();
    
    let day = args.value_of("DAY")
        .unwrap_or("1")
        .parse::<i64>()
        .expect("Entered invalid integral value for day.");
        
    let part = args.value_of("PART")
        .unwrap_or("1")
        .parse::<u8>()
        .expect("Entered invalid value for part of day.");
        
    match part {
        1 | 2 => println!("Running solution for day {}, part {}...", day, part),
        _ => panic!("Invalid part selected, there are only two parts for each day of the challenge."),
    }
    
    //TODO call solution function based on day, part pattern match
}