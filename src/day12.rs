use std::{
    collections::HashMap,
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Working,
    Broken,
    Unknown,
}

unsafe impl Send for State {}

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

impl Record {
    fn unfold(&self) -> Record {
        let mut states = self.states.clone();
        for _ in 2..=5 {
            states.push(State::Unknown);
            states.extend(self.states.iter().copied());
        }

        let mut runs = self.runs.clone();
        for _ in 2..=5 {
            runs.extend(self.runs.iter().copied());
        }

        Record { states, runs }
    }
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

fn num_arrangements_rec<'data>(
    memo: &mut HashMap<(&'data [State], &'data [usize]), usize>,
    states: &'data [State],
    runs: &'data [usize],
    indent_n: usize,
) -> usize {
    if let Some(&memoed) = memo.get(&(states, runs)) {
        return memoed;
    }

    let indent = "|  ".repeat(indent_n);

    tracing::trace!("{indent}states={states:?} runs={runs:?}");

    let mut trimmed = states;
    // remove leading and trailing .s
    while !trimmed.is_empty() && trimmed[0] == State::Working {
        trimmed = &trimmed[1..];
    }
    while !trimmed.is_empty() && trimmed[trimmed.len() - 1] == State::Working {
        trimmed = &trimmed[..trimmed.len() - 1];
    }

    if trimmed.is_empty() && runs.is_empty() {
        // it looks like this never happens?
        tracing::trace!("{indent}nice 1");
        return 1;
    }

    if trimmed.iter().all(State::maybe_working) && runs.is_empty() {
        // everything left is ? or . and no more runs left
        tracing::trace!("{indent}nice 2");
        return 1;
    }

    if trimmed.is_empty() || runs.is_empty() {
        tracing::trace!("{indent}empty");
        return 0;
    }

    let run = runs[0];
    tracing::trace!("{indent}run={run} trimmed={trimmed:?}");

    if run > trimmed.len() {
        tracing::trace!("{indent}not enough states left");
        return 0;
    }

    if runs.len() == 1
        && run <= trimmed.len()
        && trimmed[0..run].iter().all(State::maybe_broken)
        && trimmed[run..].iter().all(|&state| state == State::Working)
    {
        // everything within the run is ? or # and everything beyond is .
        // (should this also count #, idk?)
        tracing::trace!("{indent}nice 3");
        return 1;
    }

    let mut left = None;
    let mut right = trimmed.get(run).copied();
    let mut sum = 0;
    for (window_num, window) in trimmed.windows(run).enumerate() {
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
            if window_num + run + 1 >= trimmed.len() {
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
                sum += num_arrangements_rec(
                    memo,
                    &trimmed[window_num + run + 1..],
                    &runs[1..],
                    indent_n + 1,
                );
            }
        }

        left = Some(window[0]);
        right = trimmed.get(window_num + run + 1).copied();

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

    memo.insert((states, runs), sum);
    sum
}

fn num_arrangements(states: &[State], runs: &[usize]) -> usize {
    let mut memo = HashMap::new();
    num_arrangements_rec(&mut memo, states, runs, 0)
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
    parse(data)
        .into_iter()
        .map(|record| {
            let Record { states, runs } = record.unfold();
            num_arrangements(&states, &runs)
        })
        .sum()
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day12.1.txt")), 7939);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day12.1.txt")), 850504257483930);
}
