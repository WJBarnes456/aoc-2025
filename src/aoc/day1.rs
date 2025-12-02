use crate::Puzzle;

// Day1 implements day 1 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/1. 
pub struct Day1;

fn to_val(line: &str) -> i32 {
    if line.len() == 0 {
        return 0;
    }
    // get the direction
    let direction = match line.chars().nth(0) {
        Some('L') => -1,
        Some('R') => 1,
        default => panic!("invalid direction {:?}", default)
    };
    
    // convert to an i32 and multiply by the direction
    return match line[1..].parse::<i32>() {
        Ok(v) => v * direction,
        Err(_) => panic!("invalid line {}", line),
    }
}

impl Puzzle for Day1 {
    fn part1(&self, input: &String) -> String {
        let mut count = 0;
        let end_pos= input.split('\n') // convert to lines
                    .map(|x| to_val(x.trim())) // convert each line into the corresponding offset
                    .fold(50, |x, y| {
                        let v = (x+y+100)%100;
                        if v == 0 { count+= 1;}
                        return v;
                    });
        
        println!("end_pos was {}", end_pos);

        return count.to_string();
    }
    fn part2(&self, input: &String) -> String {
        return self.part1(input)
    }
}
