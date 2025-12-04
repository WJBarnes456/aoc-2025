use std::collections::VecDeque;

use crate::Puzzle;

// Day3 implements day 23 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/3. 
pub struct Day3;

fn add_val(vec: &mut VecDeque<u32>, value: u32, top_n: u32) {
    // if we don't already have the first n, add it
    if vec.len() < top_n.try_into().unwrap() {
        vec.push_back(value);
        return;
    }

    // otherwise, consider whether it'd be more valuable to have this new value in the array
    // two options: either we replace the last value with it (NB: we can't drop an intermediate value), or shift the whole array along, and we want to pick whichever would lead to a bigger value
    let current_val = as_val(vec);
    let replace_last_val = (current_val + value) - vec.back().unwrap();
    let shift_along_val = (current_val - vec.front().unwrap() * 10_u32.pow(top_n - 1)) * 10 + value;

    //println!("current val {current_val}. considering replacing_last (new val {replace_last_val}) or shifting along (new val {shift_along_val}) for {value} to {vec:?}");

    // if neither change is an improvement, do nothing
    if current_val >= replace_last_val && current_val >= shift_along_val {
        return;
    }

    // otherwise do whichever is more of an improvement, preferring replacing last since it's quicker
    if replace_last_val >= shift_along_val {
        *vec.back_mut() .unwrap() = value;
    } else if shift_along_val > replace_last_val {
        vec.pop_front();
        vec.push_back(value);
    }
}

fn as_val(vec: &VecDeque<u32>) -> u32 {
    let mut base = 0;
    for i in vec {
        base = base * 10 + i;
    }
    return base;
}

fn bank_joltage(line: &str, top_n: u32) -> u32 {
    let mut top_n_vals: VecDeque<u32> = VecDeque::new();
    
    // convert to ints and try and add each to the top_n array
    let char_count = line.chars()
                                .map(|x| add_val(&mut top_n_vals, x.to_digit(10).unwrap(), top_n))
                                .count();
    
    println!("processed a line with {char_count} elements");
    println!("line {line} had top vals {top_n_vals:?}");
    
    // then return the value of the array 
    return as_val(&top_n_vals);
}

impl Puzzle for Day3 {
    fn part1(&self, input: &String) -> String {
        return input.split('\n')
                    .map(|line| bank_joltage(line.trim(), 2))
                    .sum::<u32>()
                    .to_string()
    }
    fn part2(&self, input: &String) -> String {
        return "".to_string();
    }
}

