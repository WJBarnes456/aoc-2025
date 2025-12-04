use crate::Puzzle;

// Day4 implements day 4 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/4. 
pub struct Day4;

fn to_grid(input: &String) -> Vec<Vec<bool>> {
    return input.split('\n')
                .map(|line| line.trim().chars().map(|c| c == '@').collect::<Vec<bool>>())
                .collect::<Vec<Vec<bool>>>();
}

fn surrounding_rolls(grid: &Vec<Vec<bool>>, x: isize, y: isize) -> usize {
    let mut count = 0;
    for y_offset in -1..=1 {
        // don't go off the grid in the y direction
        if y + y_offset < 0 || y + y_offset >= grid.len() as isize {
            continue;
        }

        let new_y = (y + y_offset) as usize;

        for x_offset in -1..=1 {
            // don't go off the grid in the x direction either
            if x + x_offset < 0 || (x + x_offset) >= grid[new_y].len() as isize {
                continue;
            }

            // and don't consider the square we're currently looking at
            if (x_offset == 0 && y_offset == 0) {
                continue;
            }

            let new_x = (x + x_offset) as usize;

            if grid[new_y][new_x] {
                count += 1;
            }
        }
    }
    return count;
}

impl Puzzle for Day4 {
    fn part1(&self, input: &String) -> String {
        let grid = to_grid(input);
        let mut accessible = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] && surrounding_rolls(&grid, x as isize, y as isize) < 4 {
                    println!("accessible roll at ({x},{y})");
                    accessible += 1;
                } 
            }
        }
        return accessible.to_string();
    }
    fn part2(&self, input: &String) -> String {
        return input.chars().rev().collect::<String>();
    }
}

