use crate::Puzzle;
use std::cmp::min;
use std::cmp::max;
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

// add_to_ranges adds a new range to the ranges that already exist
// we maintain this list in sorted order since adding a new range might cause 2 other ranges to merge together
// e.g. consider we have ranges 1-2 and 4-5. if we insert range 3-3 this becomes a single range 1-5
// I checked my input playing around in Python and the input ranges _are_ overlapping so we have to do this
fn add_to_ranges(ranges: &mut Vec<(usize, usize)>, new_start_p: &usize, new_end_p: &usize) {
    let (new_start, new_end) = (*new_start_p, *new_end_p);
    let mut i = 0;
    while i < ranges.len() {
        let (existing_start, existing_end) = ranges[i];
        // cases
        // no overlap, too early - new_end < existing_start - when we encounter this for the first time, this is a new value! so we need to push it into the array at this point
        if (new_end < existing_start) {
            ranges.insert(i, (new_start, new_end));
            return;
        }

        // some overlap - existing_start <= new_end <= existing_end or existing_start <= new_start <= existing_end.
        // in this case, we have a new range we want to insert - it's inefficient, but correct, to remove the old range, and try and insert this new one instead
        if ((existing_start <= new_start) && (new_start <= existing_end)) || ((existing_start <= new_end) && (new_end <= existing_end)) {
            ranges.remove(i);
            add_to_ranges(ranges, &min(existing_start, new_start), &max(existing_end, new_end));
            return;
        }

        i+=1;
    }

    // no overlap, too late
    // we iterated over the entire array and didn't have any overlap, so this is a new value at the very end
    ranges.push((new_start, new_end));
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
        let (starts, ends, _) = process_input(input);

        let mut ranges: Vec<(usize, usize)> = Vec::new();

        let range_count = starts.iter().zip(ends.iter())
            .map(|(start, end)| add_to_ranges(&mut ranges, start, end))
            .count();

        println!("added {range_count} ranges to ranges");

        for (start, end) in ranges.iter() {
            println!("range: ({start}, {end}), size {}", end - start + 1);
        }

        return ranges.iter().map(|(start, end)| end - start + 1).sum::<usize>().to_string();
    }
}
