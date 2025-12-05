use crate::Puzzle;

// Day5 implements day 5 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/5. 
pub struct Day5;

impl Puzzle for Day5 {
    fn part1(&self, input: &String) -> String {
        return input.to_string();
    }
    fn part2(&self, input: &String) -> String {
        return input.chars().rev().collect::<String>();
    }
}
