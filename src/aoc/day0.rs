use crate::Puzzle;

// Day0 is a puzzle implementation which does basic string operations as a test of the overall framework.
pub struct Day0;

impl Puzzle for Day0 {
    fn part1(&self, input: &String) -> String {
        return input.to_string();
    }
    fn part2(&self, input: &String) -> String {
        return input.chars().rev().collect::<String>();
    }
}
