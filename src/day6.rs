use crate::util::Direction;
use std::{collections::{BTreeSet, BTreeMap}, fmt::Display};

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day6.1.txt")
    } else {
        include_str!("../data/day6.1.txt")
    };

    tracing::info!("day 6 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 6 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 6 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 6 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Obstacle(bool),
    Guard(Direction),
}

impl Tile {
    fn dir(&self) -> Option<Direction> {
        match self {
            Tile::Guard(dir) => Some(*dir),
            _ => None,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Obstacle(false) => write!(f, "#"),
            Tile::Obstacle(true) => write!(f, "O"),
            Tile::Guard(direction) => match direction {
                Direction::N => write!(f, "^"),
                Direction::S => write!(f, "v"),
                Direction::E => write!(f, ">"),
                Direction::W => write!(f, "<"),
                _ => write!(f, "what? {:?}", direction),
            },
        }
    }
}

fn parse(data: &str) -> Vec<Vec<Tile>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Obstacle(false),
                    '^' => Tile::Guard(Direction::N),
                    'v' => Tile::Guard(Direction::S),
                    '>' => Tile::Guard(Direction::E),
                    '<' => Tile::Guard(Direction::W),
                    '.' => Tile::Empty,
                    _ => panic!("bad tile: {:?}", c),
                })
                .collect()
        })
        .collect()
}

fn visited(mut plane: Vec<Vec<Tile>>) -> (BTreeSet<(usize, usize, Direction)>, bool) {
    //let mut vis: Vec<Vec<String>> = plane
    //    .iter()
    //    .map(|row| row.iter().map(|tile| format!("{}", tile)).collect())
    //    .collect();

    let mut guard = None;
    for (rowi, row) in plane.iter().enumerate() {
        for (coli, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::Guard(_)) {
                guard = Some((rowi, coli));
            }
        }
    }

    let Some((mut guard_row, mut guard_col)) = guard else {
        tracing::error!("no guard");
        return (BTreeSet::default(), false);
    };
    let mut guard_dir = plane[guard_row][guard_col].dir().unwrap();

    let mut visited = BTreeSet::default();

    visited.insert((guard_row, guard_col, guard_dir));
    //vis[guard_row][guard_col] = String::from("X");

    let mut cycle = false;
    let mut hit_obstacles = BTreeMap::<(usize, usize), usize>::default();

    while let Some((next_row, next_col)) =
        guard_dir.apply_index(plane.as_slice(), guard_row, guard_col)
    {
        if hit_obstacles.get(&(next_row, next_col)) == Some(&2) {
            cycle = true;
            tracing::debug!("cycle");
            break;
        }

        tracing::trace!(
            "at {},{} looking {:?} at {},{}",
            guard_row,
            guard_col,
            guard_dir,
            next_row,
            next_col
        );
        //for row in vis.iter() {
        //    tracing::trace!("{}", row.join(""));
        //}

        match guard_dir
            .apply(plane.as_slice(), guard_row, guard_col)
            .unwrap()
        {
            Tile::Obstacle(_) => {
                tracing::trace!(
                    "obstacle {:?} at {},{}: {:?} next",
                    guard_dir,
                    next_row,
                    next_col,
                    guard_dir.right_90()
                );
                guard_dir = guard_dir.right_90();
                *hit_obstacles.entry((next_row, next_col)).or_default() += 1;
            }

            Tile::Empty => {
                tracing::trace!("empty {:?} at {},{}", guard_dir, next_row, next_col);
                plane[guard_row][guard_col] = Tile::Empty;
                plane[next_row][next_col] = Tile::Guard(guard_dir);
                visited.insert((next_row, next_col, guard_dir));
                guard_row = next_row;
                guard_col = next_col;
                //vis[guard_row][guard_col] = String::from("X");
            }

            _ => unreachable!("guards plural?"),
        }
    }

    if cycle {
        //for row in vis.iter() {
        //    tracing::debug!("{}", row.join(""));
        //}
    }

    (visited, cycle)
}

fn part1(data: &str) -> usize {
    let plane = parse(data);

    visited(plane)
        .0
        .into_iter()
        .map(|(row, col, _)| (row, col))
        .collect::<BTreeSet<_>>()
        .len()
}

fn part2(data: &str) -> usize {
    let plane = parse(data);
    let visited_dirs = visited(plane.clone()).0;
    tracing::debug!(
        "visited {} locations with different directions",
        visited_dirs.len()
    );

    let mut guard = None;
    for (rowi, row) in plane.iter().enumerate() {
        for (coli, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::Guard(_)) {
                guard = Some((rowi, coli));
            }
        }
    }
    let Some((guard_row, guard_col)) = guard else {
        tracing::error!("no guard");
        return 0;
    };

    // brute force go brrrrr
    let mut cycles = BTreeSet::default();
    for (visited_row, visited_col, _) in visited_dirs {
        if (visited_row, visited_col) == (guard_row, guard_col) {
            continue;
        }
        let mut plane = plane.clone();
        plane[visited_row][visited_col] = Tile::Obstacle(true);
        let (tiles, cycle) = visited(plane);
        if cycle {
            tracing::debug!("cycle?");
            cycles.insert(tiles.into_iter().collect::<std::collections::BTreeSet<_>>());
        }
    }

    cycles.len()
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day6.1.txt")), 5067);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day6.1.txt")), 1793);
}
