use crate::Puzzle;

use std::collections::{HashMap, HashSet};

// Day7 implements day 7 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/7. 
pub struct Day7;

// important info:
// starts at S, splits at ^, passes through .
// beams always move downward (i.e. we don't need to model every single location, just where there are splitters)
// beams are not duplicated - positions must be tracked distinctly
// what data structure - we really want a sparse lookup of row number to splitter positions
// we could just as easily do a dense one instead, and honestly since we can iterate over the rows it's not too bad at all
// then we just need a set to maintain the... well, set of different beam positions

// part 1 - how many times will it be split - i.e. the number of times we have a beam hit a splitter

impl Puzzle for Day7 {
    fn part1(&self, input: &String) -> String {
        let mut lines = input.lines();
        let first_pos = lines.next().unwrap().chars().position(|x| x == 'S').unwrap();
        let mut positions = HashSet::<usize>::new();
        positions.insert(first_pos);

        let mut total_splits = 0;
        for (y, line) in lines.enumerate() {
            let splitter_indices = line.chars().enumerate().filter(|(_, c)| *c == '^').map(|(i, _)| i);
            for i in splitter_indices {
                // if there is a beam at the specified position, then remove it and add the new positions
                if positions.remove(&i) {
                    // NB: we might need to handle hitting the edge of the map in the general case but neither the test nor my puzzle input include that 
                    positions.insert(i-1);
                    positions.insert(i+1);
                    total_splits += 1;
                    println!("beam split at position {i}");
                }
            }

            println!("beam positions at y={y}: {positions:?}");
        }

        return total_splits.to_string();
    }
    fn part2(&self, input: &String) -> String {
        let mut lines = input.lines();
        let first_pos = lines.next().unwrap().chars().position(|x| x == 'S').unwrap();
        let mut positions = HashMap::<usize, usize>::new();
        positions.insert(first_pos, 1);

        for (y, line) in lines.enumerate() {
            let splitter_indices = line.chars().enumerate().filter(|(_, c)| *c == '^').map(|(i, _)| i);
            for i in splitter_indices {
                // if there is a beam at the specified position, then remove it and add the new positions
                match positions.remove(&i) {
                    None => continue,
                    Some(v) => {
                        // it may have already been possible to get to either of the neighbouring positions, so we need to attempt to pull out and update the neighbouring values
                        positions.insert(i-1, positions.get(&(i-1)).unwrap_or(&0) + v);
                        positions.insert(i+1, positions.get(&(i+1)).unwrap_or(&0) + v);
                    }
                }
            }

            println!("beam positions at y={y}: {positions:?}");
        }

        // the number of different timelines total is the sum of the number of timelines that can get to each position
        return positions.iter().map(|(_,v)| v).sum::<usize>().to_string();
    }
}
