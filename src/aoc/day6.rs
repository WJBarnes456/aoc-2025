use std::ptr::null;

use crate::Puzzle;

// Day6 implements day 6 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/6. 
pub struct Day6;

#[derive(Clone, Copy)]
enum OperationKind {
    Add,
    Multiply,
}

enum Row {
    Vals(Vec<usize>),
    Ops(Vec<OperationKind>),
}

fn parse_op(v: char) -> Option<OperationKind> {
    match v {
        '*' => Some(OperationKind::Multiply),
        '+' => Some(OperationKind::Add),
        _ => None,
    }
}

fn parse_line(line: &str) -> Row {
    let vals_or_ops = line.split_whitespace();

    let mut vals = Vec::new();
    let mut ops = Vec::new();

    vals_or_ops.map(|v: &str| {
        match (parse_op(v[0])) {
            Some(op) => ops.push(op),
            // we deliberately expect this unwrap to error if it's not an op or a val
            None => vals.push(v.parse::<usize>().unwrap()),
        }    
    })

    match (vals.len(), ops.len()) {
        (0, 0) => panic!("received empty line"),
        (0, l) => Row::Ops(ops),
        (l, 0) => Row::Vals(vals),
        (l_v, l_o) => panic!("invalid line contained both {l_v} vals and {l_o} ops"),
    }
}

fn parse_input(input: &str) -> Vec<(Vec<usize>, OperationKind)> {
    let mut problem_vals: Vec<Vec<usize>> = Vec::new();

    let mut problem_ops = None;
    for row in input.lines().map(|x| parse_line(x.trim())) {
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

    let mut out: Vec<(Vec<usize>, OperationKind)> = Vec::new();
    // this feels like it should be implementable as a collect but isn't because
    // I'll need to do a bit more reasoning about how we can transfer these values out, for now it's fast enough to just clone at the end here
    // we could sidestep this but be less ergonomic if we just returned problem_vals and problem_ops instead and let the user zip them together
    for (vals, op) in problem_vals.iter().zip(problem_ops.unwrap().iter()) {
        out.push((vals.clone(), *op));
    }

    return out;

}

impl Puzzle for Day6 {
    fn part1(&self, input: &String) -> String {
        return parse_input(input).into_iter().map(|(vals, op)| {

        }).sum();
    }
    fn part2(&self, input: &String) -> String {
        return input.chars().take(10).collect::<String>();
    }
}

