use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use crate::util;

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day7.1.txt")
    } else {
        include_str!("../data/day7.1.txt")
    };

    tracing::info!("day 7 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 7 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 7 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 7 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

#[derive(Debug)]
struct Equation {
    value: u64,
    factors: Vec<u64>,
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        let Some(value_str) = split.next() else {
            return Err(String::from("missing value"));
        };
        let value = value_str[..value_str.len() - 1]
            .parse()
            .map_err(|_| String::from("value not a number"))?;

        Ok(Equation {
            value,
            factors: split
                .map(|part| part.parse())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| String::from("part not a number"))?,
        })
    }
}

fn parse(data: &str) -> Vec<Equation> {
    data.lines()
        .filter(|&line| !line.is_empty())
        .map(Equation::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn part1(data: &str) -> u64 {
    let equations = parse(data);
    tracing::debug!("{:?}", equations);

    #[derive(Clone, Copy, Debug)]
    enum Op {
        Add,
        Mul,
    }

    impl Display for Op {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match self {
                Op::Add => write!(f, "+"),
                Op::Mul => write!(f, "*"),
            }
        }
    }

    let mut sum = 0;
    for Equation { value, factors } in equations {
        tracing::debug!("{}: {:?} solving", value, factors);

        fn solve(
            value: u64,
            factors: &[u64],
            factor: usize,
            acc: u64,
            solution: &mut Vec<Op>,
        ) -> bool {
            if factor == factors.len() {
                return false;
            }

            tracing::trace!(
                "{}: {} + {} = {}, {} * {} = {}",
                value,
                acc,
                factors[factor],
                acc + factors[factor],
                acc,
                factors[factor],
                acc * factors[factor]
            );
            if factor + 1 == factors.len() {
                if acc + factors[factor] == value {
                    tracing::trace!("solution found");
                    solution.push(Op::Add);
                    true
                } else if acc * factors[factor] == value {
                    tracing::trace!("solution found");
                    solution.push(Op::Mul);
                    true
                } else {
                    tracing::trace!("not a solution");
                    false
                }
            } else if solve(value, factors, factor + 1, acc + factors[factor], solution) {
                if factor != 0 {
                    solution.push(Op::Add);
                } else {
                    solution.reverse();
                }
                true
            } else if solve(value, factors, factor + 1, acc * factors[factor], solution) {
                if factor != 0 {
                    solution.push(Op::Mul);
                } else {
                    solution.reverse();
                }
                true
            } else {
                false
            }
        }

        let mut solution = Vec::new();
        if solve(value, &factors, 0, 0, &mut solution) {
            assert_eq!(
                factors.len() - 1,
                solution.len(),
                "invalid number of ops for factors"
            );
            let mut check_value = factors[0];
            for (&factor, &op) in factors.iter().skip(1).zip(solution.iter()) {
                match op {
                    Op::Add => check_value += factor,
                    Op::Mul => check_value *= factor,
                }
            }
            assert_eq!(check_value, value, "solution invalid");

            tracing::debug!(
                "{}: {:?} solved: {} {}",
                value,
                factors,
                factors
                    .iter()
                    .map(ToString::to_string)
                    .zip(solution.iter().map(ToString::to_string))
                    .map(|(factor, op)| format!("{} {}", factor, op))
                    .reduce(|mut prev, next| {
                        prev.push(' ');
                        prev.push_str(&next);
                        prev
                    })
                    .unwrap(),
                factors.last().unwrap(),
            );

            sum += value;
        } else {
            tracing::debug!("{}: {:?} no solution", value, factors);
        }
    }

    sum
}

fn concat(l: u64, r: u64) -> u64 {
    l * 10u64.pow(util::num_digits(r)) + r
}

fn part2(data: &str) -> u64 {
    let equations = parse(data);
    tracing::debug!("{:?}", equations);

    #[derive(Clone, Copy, Debug)]
    enum Op {
        Add,
        Mul,
        Con,
    }

    impl Display for Op {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match self {
                Op::Add => write!(f, "+"),
                Op::Mul => write!(f, "*"),
                Op::Con => write!(f, "|"),
            }
        }
    }

    let mut sum = 0;
    for Equation { value, factors } in equations {
        tracing::debug!("{}: {:?} solving", value, factors);

        fn solve(
            value: u64,
            factors: &[u64],
            factor_index: usize,
            acc: u64,
            solution: &mut Vec<Op>,
        ) -> bool {
            if factor_index == factors.len() {
                return false;
            }
            let factor = factors[factor_index];

            tracing::trace!(
                "{v}: {a} + {f} = {}, {a} * {f} = {}, {a} | {f} = {}",
                acc + factor,
                acc * factor,
                concat(acc, factor),
                v = value,
                a = acc,
                f = factor,
            );
            if factor_index + 1 == factors.len() {
                if acc + factor == value {
                    tracing::trace!("solution found");
                    solution.push(Op::Add);
                    true
                } else if acc * factor == value {
                    tracing::trace!("solution found");
                    solution.push(Op::Mul);
                    true
                } else if concat(acc, factor) == value {
                    tracing::trace!("solution found");
                    solution.push(Op::Con);
                    true
                } else {
                    tracing::trace!("not a solution");
                    false
                }
            } else if solve(value, factors, factor_index + 1, acc + factor, solution) {
                if factor_index != 0 {
                    solution.push(Op::Add);
                } else {
                    solution.reverse();
                }
                true
            } else if solve(value, factors, factor_index + 1, acc * factor, solution) {
                if factor_index != 0 {
                    solution.push(Op::Mul);
                } else {
                    solution.reverse();
                }
                true
            } else if solve(
                value,
                factors,
                factor_index + 1,
                concat(acc, factor),
                solution,
            ) {
                if factor_index != 0 {
                    solution.push(Op::Con);
                } else {
                    solution.reverse();
                }
                true
            } else {
                false
            }
        }

        let mut solution = Vec::new();
        if solve(value, &factors, 0, 0, &mut solution) {
            assert_eq!(
                factors.len() - 1,
                solution.len(),
                "invalid number of ops for factors"
            );
            let mut check_value = factors[0];
            for (&factor, &op) in factors.iter().skip(1).zip(solution.iter()) {
                match op {
                    Op::Add => check_value += factor,
                    Op::Mul => check_value *= factor,
                    Op::Con => check_value = concat(check_value, factor),
                }
            }
            assert_eq!(check_value, value, "solution invalid");

            tracing::debug!(
                "{}: {:?} solved: {} {}",
                value,
                factors,
                factors
                    .iter()
                    .map(ToString::to_string)
                    .zip(solution.iter().map(ToString::to_string))
                    .map(|(factor, op)| format!("{} {}", factor, op))
                    .reduce(|mut prev, next| {
                        prev.push(' ');
                        prev.push_str(&next);
                        prev
                    })
                    .unwrap(),
                factors.last().unwrap(),
            );

            sum += value;
        } else {
            tracing::debug!("{}: {:?} no solution", value, factors);
        }
    }

    sum
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day7.1.txt")), 945512582195);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day7.1.txt")), 271691107779347);
}
