use crate::Puzzle;

// Day3 implements day 23 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/3. 
pub struct Day3;

impl Puzzle for Day3 {
    fn part1(&self, input: &String) -> String {
        return input.to_string();
    }
    fn part2(&self, input: &String) -> String {
        return input.chars().rev().collect::<String>();
    }
}

