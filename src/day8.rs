use crate::util;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day8.1.txt")
    } else {
        include_str!("../data/day8.1.txt")
    };

    tracing::info!("day 8 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 8 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 8 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 8 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

#[derive(Debug, Clone, Copy)]
enum Location {
    Empty,
    Antenna(char),
}

fn parse(data: &str) -> Vec<Vec<Location>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Location::Empty,
                    _ => Location::Antenna(c),
                })
                .collect()
        })
        .collect()
}

fn freq_locations(plane: &[Vec<Location>]) -> BTreeMap<char, BTreeSet<(usize, usize)>> {
    let mut locations = BTreeMap::<char, BTreeSet<(usize, usize)>>::new();

    for (row, r) in plane.iter().enumerate() {
        for (col, l) in r.iter().enumerate() {
            if let Location::Antenna(a) = *l {
                locations.entry(a).or_default().insert((row, col));
            }
        }
    }

    locations
}

fn antinodes(plane: &[Vec<Location>], start: usize, iterations: usize) -> BTreeSet<(isize, isize)> {
    let mut antinodes = BTreeSet::new();

    let freq_locations = freq_locations(plane);
    for (antenna, locations) in freq_locations.iter() {
        for combo in locations.iter().combinations(2) {
            let [(a_row, a_col), (b_row, b_col)] = combo[..] else {
                tracing::error!("not combo size 2");
                return antinodes;
            };
            let (a_row, a_col, b_row, b_col) = (
                *a_row as isize,
                *a_col as isize,
                *b_row as isize,
                *b_col as isize,
            );

            let row_diff = a_row - b_row;
            let col_diff = a_col - b_col;

            tracing::trace!(
                "{:?} at {},{} ({},{}) {},{}",
                antenna,
                a_row,
                a_col,
                row_diff,
                col_diff,
                b_row,
                b_col,
            );

            for factor in start..=(start + iterations) {
                let mut had_in_bounds = false;

                let factor = factor as isize;
                let a_antinode_row = a_row + (row_diff * factor);
                let a_antinode_col = a_col + (col_diff * factor);
                if util::in_bounds(&plane, a_antinode_row, a_antinode_col) {
                    tracing::trace!("antinode at {},{}", a_antinode_row, a_antinode_col);
                    antinodes.insert((a_antinode_row, a_antinode_col));
                    had_in_bounds = true;
                } else {
                    tracing::trace!("antinode not in bounds");
                }

                let b_antinode_row = b_row - (row_diff * factor);
                let b_antinode_col = b_col - (col_diff * factor);
                if util::in_bounds(&plane, b_antinode_row, b_antinode_col) {
                    tracing::trace!("antinode at {},{}", b_antinode_row, b_antinode_col);
                    antinodes.insert((b_antinode_row, b_antinode_col));
                    had_in_bounds = true;
                } else {
                    tracing::trace!("antinode not in bounds");
                }

                if !had_in_bounds {
                    tracing::trace!("no more in-bounds antinodes");
                    break;
                }
            }
        }
    }

    antinodes
}

fn debug_vis(plane: &[Vec<Location>], antinodes: &BTreeSet<(isize, isize)>) {
    for (row, r) in plane.iter().enumerate() {
        let mut line = String::new();
        for (col, l) in r.iter().enumerate() {
            line.push(match l {
                Location::Empty => {
                    if antinodes.contains(&(row as isize, col as isize)) {
                        '#'
                    } else {
                        '.'
                    }
                }
                Location::Antenna(a) => {
                    if antinodes.contains(&(row as isize, col as isize)) {
                        '*'
                    } else {
                        *a
                    }
                }
            });
        }
        tracing::debug!("{}", line);
    }
}

fn part1(data: &str) -> usize {
    let plane = parse(data);
    let antinodes = antinodes(&plane, 1, 0);
    debug_vis(&plane, &antinodes);
    antinodes.len()
}

fn part2(data: &str) -> usize {
    let plane = parse(data);
    let antinodes = antinodes(&plane, 0, 100);
    debug_vis(&plane, &antinodes);
    antinodes.len()
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day8.1.txt")), 396);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day8.1.txt")), 1200);
}
