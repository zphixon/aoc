use itertools::Itertools;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use unicode_segmentation::UnicodeSegmentation;

pub fn run(example: bool) {
    let data1 = if example {
        include_str!("../data/example/day12.1.txt")
    } else {
        include_str!("../data/day12.1.txt")
    };

    tracing::info!("day 12 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 12 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data1)
    );
    tracing::info!("day 12 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 12 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data1)
    );
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Working,
    Broken,
    Unknown,
}

impl State {
    fn as_str(&self) -> &'static str {
        match self {
            State::Working => ".",
            State::Broken => "#",
            State::Unknown => "?",
        }
    }

    fn maybe_broken(&self) -> bool {
        match self {
            State::Working => false,
            State::Broken | State::Unknown => true,
        }
    }

    fn maybe_working(&self) -> bool {
        match self {
            State::Broken => false,
            State::Working | State::Unknown => true,
        }
    }
}

impl FromStr for State {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(State::Working),
            "#" => Ok(State::Broken),
            "?" => Ok(State::Unknown),
            _ => Err(()),
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

struct Record {
    states: Vec<State>,
    runs: Vec<usize>,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for state in self.states.iter() {
            write!(f, "{}", state.as_str())?;
        }
        write!(f, " {:?}", self.runs)
    }
}

impl Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn parse(data: &str) -> Vec<Record> {
    data.lines()
        .map(|line| {
            let [springs, runs] = line.split_whitespace().collect::<Vec<_>>()[..] else {
                panic!();
            };
            Record {
                states: springs
                    .graphemes(true)
                    .map(State::from_str)
                    .collect::<Result<_, _>>()
                    .unwrap(),
                runs: runs
                    .split(",")
                    .map(|run| run.parse())
                    .collect::<Result<_, _>>()
                    .unwrap(),
            }
        })
        .collect()
}

fn num_arrangements_rec(mut states: &[State], runs: &[usize], indent_n: usize) -> usize {
    let indent = "|  ".repeat(indent_n);

    tracing::trace!("{indent}states={states:?} runs={runs:?}");

    while !states.is_empty() && states[0] == State::Working {
        states = &states[1..];
    }
    while !states.is_empty() && states[states.len() - 1] == State::Working {
        states = &states[..states.len() - 1];
    }

    if states.is_empty() && runs.is_empty() {
        tracing::trace!("{indent}double nice");
        return 2;
    }
    if states.iter().all(State::maybe_working) && runs.is_empty() {
        tracing::trace!("{indent}nice");
        return 1;
    }
    if states.is_empty() || runs.is_empty() {
        return 0;
    }

    let run = runs[0];

    tracing::trace!("{indent}run={run} trimmed={states:?}");

    if run > states.len() {
        return 0;
    }

    if runs.len() == 1
        && run <= states.len()
        && states[0..run].iter().all(State::maybe_broken)
        && states[run..].iter().all(|&state| state == State::Working)
    {
        tracing::trace!("{indent}nice");
        return 1;
    }

    let mut pushed = None;
    let mut sum = 0;
    for (window_num, window) in states.windows(run + 1).enumerate() {
        tracing::trace!("{indent}window={window:?} window_num={window_num} pushed={pushed:?}");

        if pushed != Some(State::Broken)
            && window[..run].iter().all(State::maybe_broken)
            && window[window.len() - 1] != State::Broken
        {
            let must_go_here = window[..run].iter().all(|&state| state == State::Broken);
            if must_go_here {
                tracing::trace!("{indent}must go here");
            } else {
                tracing::trace!("{indent}fits");
            }
            sum += num_arrangements_rec(&states[window_num + run + 1..], &runs[1..], indent_n + 1);
            if must_go_here {
                break;
            }
        }

        pushed = Some(window[0]);
    }

    sum
}

fn num_arrangements(mut states: &[State], runs: &[usize]) -> usize {
    num_arrangements_rec(states, runs, 0)
}

fn placements_rec(mut states: &[State], mut runs: &[usize], indent_n: usize) -> usize {
    // ...###...##...##.... 3,2,2
    // 0 1 1
    // ###.##.##...........
    // 0 1 2
    // ###.##..##..........
    // 0 1 3
    // ###.##...##.........
    // 0 1 x
    // ###.##............##
    // 0 2 1
    // ###..##.##..........
    // 0 2 x
    // ###..##...........##
    // 0 3 1
    // ###...##.##.........
    // 0 y x
    // ###............##.##
    // 1 1 1
    // .###.##.##..........

    let runs_len = runs.iter().sum::<usize>() + runs.len() - 1;
    tracing::trace!("runs_len={runs_len}");

    for run_start in 0..states.len() - runs_len + 1{
        let mut starts = vec![run_start; runs.len()];
        for i in 1..runs.len() {
            starts[i] = starts[i-1] + runs[i-1] + 1;
        }
        tracing::trace!("runs={runs:?}");
        tracing::trace!("starts={starts:?}");
    }

    0
}

fn placements(mut states: &[State], runs: &[usize]) -> usize {
    placements_rec(states, runs, 0)
}

fn part1(data: &str) -> usize {
    let mut sum = 0;

    for record in parse(data) {
        //let mut arrangements = 0;
        //for perm in record
        //    .runs
        //    .iter()
        //    .cloned()
        //    .permutations(record.runs.len())
        //    .unique()
        //{
        //    arrangements += num_arrangements(&record.states, perm.as_slice());
        //}
        let arrangements = placements(&record.states, &record.runs);
        //let arrangements = placements(&record.states, &record.runs);
        sum += arrangements;
        tracing::debug!("{} {} arrangements", record, arrangements);
    }

    sum
}

fn part2(data: &str) -> usize {
    0
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day12.1.txt")), 9957702);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day12.1.txt")), 512240933238);
}
