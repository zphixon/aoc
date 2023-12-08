use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

pub fn run(example: bool) {
    let data1 = if example {
        include_str!("../data/example/day8.1.txt")
    } else {
        include_str!("../data/day8.1.txt")
    };

    let data2 = if example {
        include_str!("../data/example/day8.2.txt")
    } else {
        include_str!("../data/day8.1.txt")
    };

    tracing::info!("day 8 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 8 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data1)
    );
    tracing::info!("day 8 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 8 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data2)
    );
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    L,
    R,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "L" {
            Ok(Direction::L)
        } else if s == "R" {
            Ok(Direction::R)
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pair {
    left: usize,
    right: usize,
}

#[derive(Debug)]
struct Map<'d> {
    directions: Vec<Direction>,
    rules: Vec<(&'d str, Pair)>,
    lines: fnv::FnvHashMap<&'d str, (usize, &'d str, &'d str)>,
}

impl<'d> From<&'d str> for Map<'d> {
    fn from(s: &'d str) -> Self {
        let s_lines = s.lines().collect::<Vec<_>>();
        let directions = s_lines[0]
            .trim()
            .graphemes(true)
            .map(Direction::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let mut rule_lines = Vec::new();
        for pair_line in s_lines[2..].iter() {
            let [name, left_right] = pair_line.split(" = ").collect::<Vec<_>>()[..] else {
                tracing::error!("didn't split on = {:?}", pair_line);
                panic!();
            };

            let [left, right] = left_right
                .trim_matches(|c| c == '(' || c == ')')
                .split(", ")
                .collect::<Vec<_>>()[..]
            else {
                tracing::error!("didn't split or trim {:?}", left_right);
                panic!();
            };

            rule_lines.push((name, left, right));
        }

        let mut lines = fnv::FnvHashMap::default();
        for (i, (name, left, right)) in rule_lines.iter().enumerate() {
            lines.insert(*name, (i, *left, *right));
        }

        // build integer-based table for much faster lookup
        let mut rules = Vec::new();
        for (name, left, right) in rule_lines.iter() {
            rules.push((
                *name,
                Pair {
                    left: lines[left].0,
                    right: lines[right].0,
                },
            ));
        }

        Map {
            directions,
            rules,
            lines,
        }
    }
}

fn num_steps(map: &Map, start: &str) -> usize {
    tracing::debug!("start at {}", start);
    let mut steps = 0;
    let mut current = map.lines[start].0;
    let mut directions = map.directions.iter().cycle();
    loop {
        let dir = directions.next().unwrap();
        let next;
        match dir {
            Direction::L => next = map.rules[current].1.left,
            Direction::R => next = map.rules[current].1.right,
        }
        steps += 1;
        tracing::trace!(
            "{} L:{} R:{} -> {:?} {} ({} -> {})",
            map.rules[current].0,
            map.lines[map.rules[current].0].1,
            map.lines[map.rules[current].0].2,
            dir,
            map.rules[next].0,
            current,
            next,
        );
        current = next;
        if map.rules[current].0.ends_with("Z") {
            tracing::debug!("done with {}", map.rules[current].0);
            break;
        }
    }

    steps
}

fn part1(data: &str) -> usize {
    num_steps(&Map::from(data), "AAA")
}

fn part2(data: &str) -> usize {
    let map = Map::from(data);

    let starts = map
        .lines
        .iter()
        .filter_map(|(&name, _)| {
            if name.ends_with("A") {
                Some(name)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let periods = starts
        .iter()
        .map(|&start| num_steps(&map, start))
        .fold(1, |acc, steps| num::integer::lcm(acc, steps));

    tracing::trace!("lcm is {:?}", periods);

    periods
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day8.1.txt")), 22411);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day8.1.txt")), 11188774513823);
}
