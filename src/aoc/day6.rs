use std::iter::once;

use crate::Puzzle;

// Day6 implements day 6 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/6. 
pub struct Day6;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
}

enum Row {
    Vals(Vec<usize>),
    Ops(Vec<Operation>),
}

fn parse_op(v: char) -> Option<Operation> {
    match v {
        '*' => Some(Operation::Multiply),
        '+' => Some(Operation::Add),
        _ => None,
    }
}

fn parse_line(line: &str) -> Row {
    let vals_or_ops = line.split_whitespace();

    let mut vals = Vec::new();
    let mut ops = Vec::new();

    vals_or_ops.map(|v: &str| {
        match parse_op(v.chars().nth(0).unwrap()) {
            Some(op) => ops.push(op),
            // we deliberately expect this unwrap to error if it's not an op or a val
            None => vals.push(v.parse::<usize>().unwrap()),
        }    
    }).count();

    match (vals.len(), ops.len()) {
        (0, 0) => panic!("received empty line"),
        (0, _l) => Row::Ops(ops),
        (_l, 0) => Row::Vals(vals),
        (l_v, l_o) => panic!("invalid line contained both {l_v} vals and {l_o} ops"),
    }
}

// map_input maps from cephalopod math to natural math 
fn map_input(input: &str) -> Vec<(Vec<usize>, Operation)> {
    let mut lines = input.lines().collect::<Vec<&str>>();

    // the last line should be all the operations, so pop it off
    let ops = match parse_line(lines.pop().unwrap()) {
        Row::Ops(o) => o,
        _ => panic!("final row was not ops"),
    };
    
    // we shouldn't need to read the whole input into an array, this is wasteful, but easier to write
    let mut line_iters: Vec<std::str::Chars<'_>> = lines.iter().map(|line| line.chars()).collect();
    
    let mut problems = Vec::new();
    let mut problem = Vec::new();
    let mut non_space = false;
    loop {
        let mut new_val = "".to_string();
        for j in 0..lines.len() {
            match line_iters[j].nth_back(0) {
                // None => we exhausted the rows, so return the problem
                // we parsed the problems in reverse, so we need to reverse ops too
                None => {
                    problems.push(problem.clone());
                    return problems.into_iter().zip(ops.into_iter().rev()).collect();
                },
                // a space - not interesting
                Some(' ') => continue,
                Some(c) => {
                    non_space = true;
                    new_val.push(c);
                }
            }
        }

        // this was a value other than all spaces, so it's a new value for the problem 
        if non_space {
            problem.push(new_val.parse::<usize>().unwrap());
            non_space = false;
        } else {
            // this _was_ all spaces, so we hit the end of this problem
            problems.push(problem.clone());
            problem.clear();
        }
    } 


}

fn parse_input<'a, I>(lines: I) -> Vec<(Vec<usize>, Operation)>
where I : Iterator<Item = String> {
    let mut problem_vals: Vec<Vec<usize>> = Vec::new();

    let mut problem_ops = None;

    for row in lines.map(|x| parse_line(x.trim())) {
        match row {
            Row::Vals(vals) => {
                // if this is the first row, we need to initialise the vals for each row
                if problem_vals.len() == 0 {
                    for v in vals {
                        let mut new_problem = Vec::new();
                        new_problem.push(v);
                        problem_vals.push(new_problem);
                    }
                } else {
                    for (i, v) in vals.into_iter().enumerate() {
                        problem_vals[i].push(v);
                    }
                }
            }
            Row::Ops(ops) => problem_ops = Some(ops),
        }
    }

    if problem_ops.is_none() {
        panic!("input had no operations row");
    }

    // zip each set of vals with its corresponding op, and return that
    return problem_vals.into_iter().zip(
        problem_ops.unwrap().into_iter()
    ).collect();

}

impl Puzzle for Day6 {
    fn part1(&self, input: &String) -> String {
        return parse_input(input.lines().map(|x| x.to_string())).into_iter().map(|(vals, op)| {
            match op {
                Operation::Add => vals.iter().sum::<usize>(),
                Operation::Multiply => vals.iter().product(),
            }
        }).sum::<usize>().to_string();
    }
    fn part2(&self, input: &String) -> String {
        return map_input(input).into_iter().map(|(vals, op)| {
            println!("evaluating {vals:?} under {op:?}");
            match op {
                Operation::Add => vals.iter().sum::<usize>(),
                Operation::Multiply => vals.iter().product(),
            }
        }).sum::<usize>().to_string();
    }
}

