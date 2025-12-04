use crate::Puzzle;

// Day3 implements day 23 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/3. 
pub struct Day3;

fn add_val(vec: &mut Vec<u64>, value: u64, top_n: u32) {
    // if we don't already have the first n, add it
    if vec.len() < top_n.try_into().unwrap() {
        vec.push(value);
        return;
    }

    // otherwise, consider whether it'd be more valuable to have this new value in the array
    // we can drop any value in the array arbitrarily to get the largest value (not only first/last), which would shift every other element across
    // so consider the value after dropping each element
    let orig_val = as_val(vec);
    let mut best_val = orig_val;
    let mut best_index_to_drop: Option<usize> = None;

    for i in 0..vec.len() {
        let new_val = orig_val - as_val(&vec[i..]) + as_val(&vec[i+1..]) * 10 + value;
        if new_val > best_val {
            best_val = new_val;
            best_index_to_drop = Some(i);
        }
    }
    
    match best_index_to_drop {
        None => return,
        Some(x) => {
            // we have a best index to drop, so actually drop it
            let length = vec.len();
            for i in x..length-1 {
                vec[i] = vec[i+1];
            }
            vec[length - 1] = value;
        }
    }
}

fn as_val(vec: &[u64]) -> u64 {
    let mut base = 0;
    for i in vec {
        base = base * 10 + i;
    }
    return base;
}

fn bank_joltage(line: &str, top_n: u32) -> u64 {
    let mut top_n_vals: Vec<u64> = Vec::new();
    
    // convert to ints and try and add each to the top_n array
    let char_count = line.chars()
                                .map(|x| add_val(&mut top_n_vals, x.to_digit(10).unwrap().into(), top_n))
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
                    .sum::<u64>()
                    .to_string()
    }
    fn part2(&self, input: &String) -> String {
        return input.split('\n')
                    .map(|line| bank_joltage(line.trim(), 12))
                    .sum::<u64>()
                    .to_string()
    }
}

