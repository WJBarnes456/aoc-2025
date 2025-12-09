use crate::Puzzle;

use itertools::Itertools;
// Day9 implements day 9 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/9. 
pub struct Day9;

// after really overcomplicating yesterday I'm going to keep it simple today. Thanks to Wyn for the inspiration on using itertools for this, that combinations function is lush.
// I'm not even sure what structure would allow you to do a clever thing here - the area between two points is not a metric (exhibit e.g. a triangle, the area between each pair of points is low other than the hypotenuse)
//
// part 2 is interesting - we really need a fast way of checking whether a rectangle is valid or not
// we could store all the valid locations in the space, but that seems very wasteful. recomputing it on each pass is also wasteful, however
// do we want to go back to the start on it? we can traverse the set of points and solve this by finding each area bounded by the set of points
// we know that any line is always valid, though combining any pair is tricky...
// I think I might just leave this one. it feels much more like an algorithmic trick than anything where I'll be able to develop my Rust by solving it, unfortunately.
// (I suspect there is going to be something in iterating around the outside of the shape and checking if any tile in the square sits outside of it)
// nb. the shape will always be convex since we draw a single line. that might be something we can use.
#[derive(Clone, Copy, Debug)]
struct RedTile {
    x: usize,
    y: usize,
}

impl RedTile {
    fn area_separating(self: &RedTile, other: &RedTile) -> usize {
        let (big_x, small_x) = match self.x > other.x {
            true => (self.x, other.x),
            false => (other.x, self.x),
        };

        let (big_y, small_y) = match self.y > other.y {
            true => (self.y, other.y),
            false => (other.y, self.y),
        };

        // we need to add one since the rectangle size is non-zero if they're on the same dimension 
        let area =  (big_x - small_x + 1) * (big_y - small_y + 1);

        //println!("area between {self:?} and {other:?} is {area}");

        return area;
    }
}

fn line_to_redtile(line: &str) -> RedTile {
    let coords = line.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    if coords.len() != 2 {
        panic!("row should have 2 coords, had {}", coords.len());
    }
    return RedTile { x: coords[0], y: coords[1] }
}

impl Puzzle for Day9 {
    fn part1(&self, input: &String) -> String {
        return input.lines()
                    .map(|l| line_to_redtile(l))
                    .combinations(2)
                    .map(|v| v[0].area_separating(&v[1]))
                    .max().unwrap()
                    .to_string();
    }
    fn part2(&self, input: &String) -> String {
        return input.chars().take(10).collect::<String>();
    }
}
