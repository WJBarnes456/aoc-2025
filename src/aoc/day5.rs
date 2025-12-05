use crate::Puzzle;
// Day5 implements day 5 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/5. 
pub struct Day5;

fn process_input(input: &String) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut lines = input.lines();

    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let fresh_count = lines.by_ref().take_while(|line| line.trim().len() != 0)
        .map(|line| { 
            let (start, end) = line.trim().split_once('-').unwrap();
            starts.push(start.parse::<usize>().unwrap());
            ends.push(end.parse::<usize>().unwrap());
        }).count();

    println!("consumed {fresh_count} pairs of fresh ingredients");

    let available = lines.map(|x| x.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
    println!("consumed {} available ingredients", available.len());

    return (starts, ends, available);
}

impl Puzzle for Day5 {
    fn part1(&self, input: &String) -> String {
        let (starts, ends, available) = process_input(input);

        let count = available
            .iter()
            .filter(|ingredient| {
                starts.iter().zip(ends.iter())
                .any(|(start, end)| start <= ingredient && end >= ingredient)
            })
            .count();

        return count.to_string();
    }
    fn part2(&self, input: &String) -> String {
        let (start, _) = input.split_at(10); 
        return start.to_string();
    }
}
