mod aoc;

use std::env;
use std::fs::File;
use std::io::Read;
trait Puzzle {
    fn part1(&self, input: &String) -> String;
    fn part2(&self, input: &String) -> String;
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("aoc must be invoked as `aoc <day_number> <input_filename>`")
    }

    let day = &args[1];

    // make sure we have code for this day

    let day_impl: Box<dyn Puzzle> = match day.as_str() {
        "0" => Box::new(aoc::Day0),
        "1" => Box::new(aoc::Day1),
        "2" => Box::new(aoc::Day2),
        "3" => Box::new(aoc::Day3),
        "4" => Box::new(aoc::Day4),
        "5" => Box::new(aoc::Day5),
        "6" => Box::new(aoc::Day6),
        default => panic!("day number {} is not yet implemented", default),
    };
    
    // then get the input from the specified file
    let filename = &args[2];

    // NB: I know we should be unpacking the Result types here
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    // then actually execute it
    println!("Part 1: {}", day_impl.part1(&input)); 
    println!("Part 2: {}", day_impl.part2(&input));

    Ok(())
}
