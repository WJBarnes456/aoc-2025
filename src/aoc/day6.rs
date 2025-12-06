use crate::Puzzle;

// Day6 implements day 6 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/6. 
pub struct Day6;

#[derive(Clone, Copy)]
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
        (0, l) => Row::Ops(ops),
        (l, 0) => Row::Vals(vals),
        (l_v, l_o) => panic!("invalid line contained both {l_v} vals and {l_o} ops"),
    }
}

fn parse_input(input: &str) -> Vec<(Vec<usize>, Operation)> {
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

    // zip each set of vals with its corresponding op, and return that
    return problem_vals.into_iter().zip(
        problem_ops.unwrap().into_iter()
    ).collect();

}

impl Puzzle for Day6 {
    fn part1(&self, input: &String) -> String {
        return parse_input(input).into_iter().map(|(vals, op)| {
            match op {
                Operation::Add => vals.iter().sum::<usize>(),
                Operation::Multiply => vals.iter().product(),
            }
        }).sum::<usize>().to_string();
    }
    fn part2(&self, input: &String) -> String {
        return input.chars().take(10).collect::<String>();
    }
}

