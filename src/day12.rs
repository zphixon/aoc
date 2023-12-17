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

    // remove leading and trailing .s
    while !states.is_empty() && states[0] == State::Working {
        states = &states[1..];
    }
    while !states.is_empty() && states[states.len() - 1] == State::Working {
        states = &states[..states.len() - 1];
    }

    if states.is_empty() && runs.is_empty() {
        // it looks like this never happens?
        tracing::trace!("{indent}nice 1");
        return 1;
    }

    if states.iter().all(State::maybe_working) && runs.is_empty() {
        // everything left is ? or . and no more runs left
        tracing::trace!("{indent}nice 2");
        return 1;
    }

    if states.is_empty() || runs.is_empty() {
        tracing::trace!("{indent}empty");
        return 0;
    }

    let run = runs[0];
    tracing::trace!("{indent}run={run} trimmed={states:?}");

    if run > states.len() {
        tracing::trace!("{indent}not enough states left");
        return 0;
    }

    if runs.len() == 1
        && run <= states.len()
        && states[0..run].iter().all(State::maybe_broken)
        && states[run..].iter().all(|&state| state == State::Working)
    {
        // everything within the run is ? or # and everything beyond is .
        // (should this also count #, idk?)
        tracing::trace!("{indent}nice 3");
        return 1;
    }

    let mut left = None;
    let mut right = states.get(run).copied();
    let mut sum = 0;
    for (window_num, window) in states.windows(run).enumerate() {
        tracing::trace!(
            "{indent}run={run} left={left:?} window={window:?} right={right:?} window_num={window_num}"
        );
        
        let must_go_here = window[0] == State::Broken;
        if must_go_here {
            tracing::trace!("{indent}must go here");
        }

        // in the case where window is ??##
        //   L??##R....
        //    ^^^^
        // it can fit there only if L and R are None or not #
        if left != Some(State::Broken)
            && window[..run].iter().all(State::maybe_broken)
            && right != Some(State::Broken)
        {
            if window_num + run + 1 >= states.len() {
                if runs.len() == 1 {
                    // right will be None after this, and we cannot recur, but
                    // it fits here
                    tracing::trace!("{indent}fits nicely");
                    sum += 1;
                } else {
                    tracing::trace!("{indent}fits but more runs left");
                }
            } else {
                // check states after R (....) for the next run
                tracing::trace!("{indent}fits");
                sum +=
                    num_arrangements_rec(&states[window_num + run + 1..], &runs[1..], indent_n + 1);
            }
        }

        left = Some(window[0]);
        right = states.get(window_num + run + 1).copied();

        if must_go_here {
            // the first spring in this window is broken, the run cannot appear
            // later. for example, with run length 3,
            //   ??#?????
            //     ^^^
            // if we were to check
            //   ??#?????
            //       ^^^
            // we would accidentally be introducing a run of 1
            break;
        }
    }

    sum
}

fn num_arrangements(states: &[State], runs: &[usize]) -> usize {
    num_arrangements_rec(states, runs, 0)
}

fn part1(data: &str) -> usize {
    let mut sum = 0;

    for record in parse(data) {
        let arrangements = num_arrangements(&record.states, &record.runs);
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
    assert_eq!(part1(include_str!("../data/day12.1.txt")), 7939);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day12.1.txt")), 512240933238);
}
