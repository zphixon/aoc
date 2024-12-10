use std::collections::BTreeSet;
use crate::util;

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day10.1.txt")
    } else {
        include_str!("../data/day10.1.txt")
    };

    tracing::info!("day 10 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 10 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 10 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 10 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

fn parse(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(1)).collect())
        .collect()
}

fn trailheads(map: &[Vec<u32>]) -> impl Iterator<Item = (usize, usize)> + '_ {
    map.iter().enumerate().flat_map(|(row, lat)| {
        lat.iter()
            .enumerate()
            .filter_map(move |(col, place)| if *place == 0 { Some((row, col)) } else { None })
    })
}

fn accessible_peaks(
    map: &[Vec<u32>],
    current_height: u32,
    current_row: usize,
    current_col: usize,
    paths: &mut usize,
    peaks: &mut BTreeSet<(usize, usize)>,
) {
    tracing::trace!(
        "at height {}, check {},{} for height {}",
        current_height,
        current_row,
        current_col,
        current_height + 1
    );
    for (next_height, next_row, next_col, _) in
        util::surrounding_cardinal(map, current_row, current_col)
            .filter(|(next_height, _, _, _)| **next_height == current_height + 1)
    {
        if *next_height == 9 {
            tracing::trace!("found peak at {},{}", next_row, next_col);
            *paths += 1;
            peaks.insert((next_row, next_col));
        } else {
            accessible_peaks(map, current_height + 1, next_row, next_col, paths, peaks);
        }
    }
}

fn part1(data: &str) -> usize {
    let map = parse(data);
    let mut total_score = 0;
    for (trailhead_row, trailhead_col) in trailheads(&map) {
        let mut peaks = BTreeSet::new();
        let mut paths = 0;
        accessible_peaks(&map, 0, trailhead_row, trailhead_col, &mut paths, &mut peaks);
        total_score += peaks.len();
    }
    total_score
}

fn part2(data: &str) -> usize {
    let map = parse(data);
    let mut total_rating = 0;
    for (trailhead_row, trailhead_col) in trailheads(&map) {
        let mut peaks = BTreeSet::new();
        let mut paths = 0;
        accessible_peaks(&map, 0, trailhead_row, trailhead_col, &mut paths, &mut peaks);
        total_rating += paths;
    }
    total_rating
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day10.1.txt")), 552);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day10.1.txt")), 1225);
}
