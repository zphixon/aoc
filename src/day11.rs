use std::collections::BTreeMap;
use crate::util;

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day11.1.txt")
    } else {
        include_str!("../data/day11.1.txt")
    };

    tracing::info!("day 11 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 11 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 11 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 11 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

fn split(num: u64) -> (u64, u64) {
    let left_ones = 10u64.pow(util::num_digits(num) / 2);
    let left = num / left_ones;
    let right = num - (left * left_ones);
    (left, right)
}

fn blink(times: usize, value: u64) -> u64 {
    fn inner_blink(
        times: usize,
        value: u64,
        depth: usize,
        cache: &mut BTreeMap<(u64, usize), u64>,
    ) -> u64 {
        //tracing::trace!("{:indent$}{}", "", value, indent = depth);
        if depth > times {
            return 1;
        }
        if let Some(stones) = cache.get(&(value, depth)) {
            return *stones;
        }

        let stones = if value == 0 {
            inner_blink(times, 1, depth + 1, cache)
        } else if util::num_digits(value) % 2 != 0 {
            inner_blink(times, value * 2024, depth + 1, cache)
        } else {
            let (left, right) = split(value);
            inner_blink(times, left, depth + 1, cache) + inner_blink(times, right, depth + 1, cache)
        };
        cache.insert((value, depth), stones);
        stones
    }
    let mut cache = BTreeMap::new();
    inner_blink(times, value, 1, &mut cache)
}

fn part1(data: &str) -> u64 {
    let mut total = 0;
    for root in data
        .split_whitespace()
        .map(|word| word.parse::<u64>().unwrap())
    {
        total += blink(25, root);
    }
    total
}

fn part2(data: &str) -> u64 {
    let mut total = 0;
    for root in data
        .split_whitespace()
        .map(|word| word.parse::<u64>().unwrap())
    {
        total += blink(75, root);
    }
    total
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day11.1.txt")), 203457);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day11.1.txt")), 241394363462435);
}
