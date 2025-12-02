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
        let mut count = 0;
        let end_pos= input.split('\n') // convert to lines
                    .map(|x| to_val(x.trim())) // convert each line into the corresponding offset
                    .fold(50, |x, y| {
                        // if the new value is a rotation of more than 100, then we can add those clicks already to reduce to the case where it's less than 100
                        let hundred_change = (y / 100).abs();
                        if hundred_change > 0 {
                            count += hundred_change;
                            println!("crossed 0 {} times in a move of {} from {} (total {})", hundred_change, y, x, count);
                        }
                        
                        let offset = y % 100;

                        if x != 0 && ((x + offset <= 0) || (x + offset >= 100)) {
                            count += 1;
                            println!("crossed 0 moving from {}: {} (total {})", x, y, count);
                        }

                        // and return the next value
                        // I spent SO LONG on this being a buggy modulo where I was doing (x+y+100)%100 which doesn't work when y is greater than 100... 
                        // I should have spotted this should have been offset (ended up debugging based on the endpos ending up negative for L200)
                        return (x+offset+100)%100;
                    });
        
        println!("end_pos was {}", end_pos);

        return count.to_string();
    }
}
