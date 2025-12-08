use crate::Puzzle;

// Day8 implements day 8 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/8. 
pub struct Day8;

impl Puzzle for Day8 {
    fn part1(&self, input: &String) -> String {
        return input.to_string();
    }
    fn part2(&self, input: &String) -> String {
        return input.chars().rev().collect::<String>();
    }
}

