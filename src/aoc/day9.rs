use crate::Puzzle;

// Day9 implements day 9 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/9. 
pub struct Day9;

impl Puzzle for Day9 {
    fn part1(&self, input: &String) -> String {
        return input.to_string();
    }
    fn part2(&self, input: &String) -> String {
        return input.chars().take(10).collect::<String>();
    }
}
