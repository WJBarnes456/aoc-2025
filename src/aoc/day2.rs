use crate::Puzzle;

// Day2 implements day 2 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/2. 
pub struct Day2;

// "any ID which is made only of some sequence of digits repeated twice"
// i.e. any 2 digit multiple of 11, any 4 digit multiple of 101, any 6 digit multiple of 1001, any 8 digit multiple of 10001...
// must be an even number of digits anyway
// the brute force solution would be to list out all of the numbers and check if it's true for that value, but I think we can do it from just the start and end point of a range

fn is_duplicate_number_under_factor(v: u64, factor: u32) -> bool {
    // doubled numbers need to have an even number of digits
    let number_of_digits = v.ilog10() + 1;
    
    if number_of_digits % factor != 0 {
        return false;
    }

    // the number has an even number of digits, so now all we need to do is check that it's a multiple of 10*(1/2 digit count) + 1
    let prop_count = number_of_digits / factor;

    let mut divisor = 1;
    for _ in 1..factor {
        divisor = divisor * 10_u64.pow(prop_count) + 1;
    }

    return (v % divisor) == 0;

}

fn is_duplicate_number(v: u64, part2: bool) -> bool {
    if !part2 {
        return is_duplicate_number_under_factor(v, 2);
    }

    // otherwise, any factor of the digit count is a possibility
    // to be lazy, can we just iterate over every possible number 
    let number_of_digits = v.ilog10() + 1;
    
    for candidate in 2..=number_of_digits {
        //println!("considering candidate factor {candidate} for {v}");
        if is_duplicate_number_under_factor(v, candidate) {
            return true;
        }
    }
    return false;
}

fn duplicated_vals(start: u64, end: u64, part2: bool) -> Vec<u64> {
    let mut vec = Vec::new();

    println!("checking range {start}-{end}");

    for i in start..=end {
        //println!("checking if {} is a doubled number", i);
        if is_duplicate_number(i, part2) {
            //println!("{} is a doubled number", i);
            vec.push(i);
        }
    }

    return vec;
}

fn sum_vals_in_range(range: &str, part2: bool) -> u64 {
    let (start, end) = range.split_once('-') .unwrap();
    return duplicated_vals(
        start.parse::<u64>().unwrap(),
        end.parse::<u64>().unwrap(),
        part2,
    ).into_iter().sum();
}

impl Puzzle for Day2 {
    fn part1(&self, input: &String) -> String {
        return input.split(',')
                    .map(|x| sum_vals_in_range(x, false))
                    .sum::<u64>()
                    .to_string()
    }
    fn part2(&self, input: &String) -> String {
        return input.split(',')
                    .map(|x| sum_vals_in_range(x, true))
                    .sum::<u64>()
                    .to_string()
    }
}

