use aoc::*;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type Parsed<'a> = (HashMap<&'a str, bool>, Vec<(Op, &'a str, &'a str, &'a str)>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    And,
    Xor,
    Or,
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "XOR" => Ok(Self::Xor),
            "OR" => Ok(Self::Or),
            _ => Err(()),
        }
    }
}

impl Op {
    fn eval(&self, lval: bool, rval: bool) -> bool {
        match self {
            Self::And => lval & rval,
            Self::Xor => lval ^ rval,
            Self::Or => lval | rval,
        }
    }
}

#[derive(Debug)]
enum Node {
    Boolean { op: Op, left: usize, right: usize },
    Leaf { value: bool },
}

impl Node {
    fn evaluate(&self, arena: &[Node]) -> bool {
        match self {
            Self::Leaf { value } => *value,
            Self::Boolean { op, left, right } => {
                let lval = arena[*left].evaluate(arena);
                let rval = arena[*right].evaluate(arena);
                op.eval(lval, rval)
            }
        }
    }
}

fn parse_input(input: &str) -> Parsed {
    let (a, b) = input.split_once("\n\n").unwrap();

    let initial_values = a
        .lines()
        .map(|line| {
            let (wire, value) = line.split_once(": ").unwrap();
            (wire, value == "1")
        })
        .collect();

    let rules = b
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            (parse(parts[1]), parts[0], parts[2], parts[4])
        })
        .collect();

    (initial_values, rules)
}

fn part1((values, rules): &Parsed) -> impl std::fmt::Display {
    let (expr, node_indices) = build_expression(values, rules);
    value_from_prefix('z', &node_indices, &expr)
}

fn part2((values, rules): &Parsed) -> impl std::fmt::Display {
    let (_, node_indices) = build_expression(values, rules);
    let &last_z = node_indices
        .keys()
        .filter(|wire| wire.starts_with('z'))
        .max_by_key(|wire| {
            let num: u8 = wire.trim_start_matches('z').parse().unwrap();
            num
        })
        .unwrap();

    let mut wrong = HashSet::new();
    for &rule in rules {
        let (op, left, right, result) = rule;
        if result.starts_with('z') && op != Op::Xor && result != last_z {
            println!("1: {result} {rule:?}");
            wrong.insert(result);
        }

        if op == Op::Xor
            && !result.starts_with(['x', 'y', 'z'])
            && !left.starts_with(['x', 'y', 'z'])
            && !right.starts_with(['x', 'y', 'z'])
        {
            println!("2: {result} {rule:?}");
            wrong.insert(result);
        }
        //

        if op == Op::And && ![left, right].contains(&"x00") {
            for &(so, sl, sr, _) in rules {
                if (result == sl || result == sr) && so != Op::Or {
                    println!("3: {result} {rule:?}");
                    wrong.insert(result);
                }
            }
            //
        }

        if op == Op::Xor {
            for &(so, sl, sr, _) in rules {
                if (result == sl || result == sr) && so == Op::Or {
                    println!("4: {result} {rule:?}");
                    wrong.insert(result);
                }
            }
        }
    }

    let mut wrong: Vec<_> = wrong.into_iter().collect();
    wrong.sort();
    wrong.join(",")
}

fn build_expression<'a>(
    values: &HashMap<&'a str, bool>,
    rules: &[(Op, &'a str, &'a str, &'a str)],
) -> (Vec<Node>, HashMap<&'a str, usize>) {
    let mut expr: Vec<Node> = Vec::with_capacity(rules.len());
    let mut node_indices = HashMap::new();
    for &(op, left, right, result) in rules {
        let left_index = if let Some(&left_index) = node_indices.get(left) {
            left_index
        } else {
            let left_index = expr.len();
            let &value = values.get(left).unwrap_or(&false);
            node_indices.insert(left, left_index);
            expr.push(Node::Leaf { value });
            left_index
        };

        let right_index = if let Some(&right_index) = node_indices.get(right) {
            right_index
        } else {
            let right_index = expr.len();
            let &value = values.get(right).unwrap_or(&false);
            node_indices.insert(right, right_index);
            expr.push(Node::Leaf { value });
            right_index
        };

        if let Some(&result_node) = node_indices.get(result) {
            expr[result_node] = Node::Boolean {
                op,
                left: left_index,
                right: right_index,
            };
        } else {
            let result_node = expr.len();
            node_indices.insert(result, result_node);
            expr.push(Node::Boolean {
                op,
                left: left_index,
                right: right_index,
            })
        }
    }

    (expr, node_indices)
}

fn value_from_prefix(pre: char, node_indices: &HashMap<&str, usize>, expr: &[Node]) -> u64 {
    let mut zs: Vec<_> = node_indices
        .keys()
        .filter(|wire| wire.starts_with(pre))
        .copied()
        .collect();

    zs.sort();

    let mut result: u64 = 0;
    for wire in zs.into_iter().rev() {
        let index = node_indices[wire];
        let value = expr[index].evaluate(expr) as u64;

        result <<= 1;
        result |= value;
    }

    result
}

aoc::setup! {
    day24, parse_input;
    part1 == 2024,
    part2 == "ffh,hwm,kjc,mjb,ntg,rvg,tgd,wpb,z02,z03,z05,z06,z07,z08,z10,z11"
}
