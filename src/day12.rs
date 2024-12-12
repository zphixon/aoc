use crate::util;
use std::collections::{BTreeMap, BTreeSet};

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day12.1.txt")
    } else {
        include_str!("../data/day12.1.txt")
    };

    tracing::info!("day 12 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 12 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    tracing::info!("day 12 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 12 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data),
    );
}

fn regions(plane: &[Vec<char>]) -> Vec<BTreeSet<(usize, usize)>> {
    let mut chars = BTreeMap::<char, BTreeSet<(usize, usize)>>::new();

    for (row, lat) in plane.iter().enumerate() {
        for (col, c) in lat.iter().enumerate() {
            chars.entry(*c).or_default().insert((row, col));
        }
    }

    let mut regions = Vec::new();
    for (c, mut plots) in chars {
        let (row, col) = plots.first().cloned().unwrap();

        let region = util::flood_fill(&plane, row, col);
        region.iter().for_each(|plot| {
            plots.remove(plot);
        });
        tracing::debug!("{:?} {:?} {:?}", c, region, plots);
        regions.push(region);

        while !plots.is_empty() {
            let (row, col) = plots.pop_first().unwrap();
            let region = util::flood_fill(&plane, row, col);
            region.iter().for_each(|plot| {
                plots.remove(plot);
            });
            tracing::debug!("{:?} {:?} {:?}", c, region, plots);
            regions.push(region);
        }
    }

    regions
}

fn part1(data: &str) -> usize {
    let plane: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let regions = regions(&plane);

    let mut total_price = 0;
    for region in regions.iter() {
        let (row, col) = region.first().cloned().unwrap();
        let plot_plant = plane[row][col];
        tracing::debug!("{:?} area {}", plot_plant, region.len());

        // +4     +3       +2         +1
        //  -      -        -
        // |o|    |o| x4   |o  x4     |o  x4
        //  -
        //                 |o| x2

        let mut perimeter = 0;
        for plot in region.iter() {
            let (row, col) = *plot;
            perimeter += 4 - util::surrounding_cardinal(&plane, row, col)
                .filter(|(plant, _, _, _)| **plant == plot_plant)
                .count();
        }

        total_price += region.len() * perimeter;
    }

    total_price
}

#[rustfmt::skip]
fn count(
    nw: bool, n: bool, ne: bool,
     w: bool,           e: bool,
    sw: bool, s: bool, se: bool,
) -> usize {
    #[derive(Clone, Copy, PartialEq, Debug)]
    enum Plant {
        X,
        O,
    }
    use Plant::*;
    impl From<bool> for Plant{
        fn from(value: bool) -> Self {
            if value {
                Plant::X
            } else {
                Plant::O
            }
        }
    }

    match (
        Plant::from(nw), Plant::from(n), Plant::from(ne),
        Plant::from(w),  Plant::X,       Plant::from(e),
        Plant::from(sw), Plant::from(s), Plant::from(se)
    ) {
        // filled inside corners
        (   X,X,O,
            X,X,X,
            X,X,X,
        ) | (
            X,X,X,
            X,X,X,
            X,X,O,
        ) | (
            X,X,X,
            X,X,X,
            O,X,X,
        ) | (
            O,X,X,
            X,X,X,
            X,X,X,
        ) => {
            1
        }

        // double inside corners
        (   X,X,O,
            X,X,X,
            O,X,X,
        ) | (
            O,X,X,
            X,X,X,
            X,X,O,
        ) | (
            O,X,O,
            X,X,X,
            X,X,X,
        ) | (
            X,X,O,
            X,X,X,
            X,X,O,
        ) | (
            X,X,X,
            X,X,X,
            O,X,O,
        ) | (
            O,X,X,
            X,X,X,
            O,X,X,
        ) => {
            2
        }

        // triple inside corners
        (   X,X,O,
            X,X,X,
            O,X,O,
        ) | (
            O,X,X,
            X,X,X,
            O,X,O,
        ) | (
            O,X,O,
            X,X,X,
            X,X,O,
        ) | (
            O,X,O,
            X,X,X,
            O,X,X,
        ) => {
            3
        }

        // quadruple inside corner
        (   O,X,O,
            X,X,X,
            O,X,O,
        ) => {
            3
        }

        // island
        (   O,O,O,
            O,X,O,
            O,O,O,
        ) => {
            3
        }

        // inside+outside corners
        (   O,O,O,
            O,X,X,
            O,X,O,
        ) | (
            O,O,O,
            X,X,O,
            O,X,O,
        ) | (
            O,X,O,
            X,X,O,
            O,O,O,
        ) | (
            O,X,O,
            O,X,X,
            O,O,O,
        ) => {
            2
        }

        // outside corners
        (   O,O,O,
            X,X,O,
            X,X,O,
        ) | (
            X,X,O,
            X,X,O,
            O,O,O,
        ) | (
            O,X,X,
            O,X,X,
            O,O,O,
        ) | (
            O,O,O,
            O,X,X,
            O,X,X,
        ) => {
            1
        }

        // peninsulae
        (   O,O,O,
            O,X,O,
            O,X,O,
        ) | (
            O,O,O,
            X,X,O,
            O,O,O,
        ) | (
            O,X,O,
            O,X,O,
            O,O,O,
        ) | (
            O,O,O,
            O,X,X,
            O,O,O,
        ) => {
            2
        }

        // Ts
        (   O,X,O,
            O,X,X,
            O,X,O,
        ) | (
            O,O,O,
            X,X,X,
            O,X,O,
        ) | (
            O,X,O,
            X,X,O,
            O,X,O,
        ) | (
            O,X,O,
            X,X,X,
            O,O,O,
        ) => {
            2
        }

        (a,b,c,d,e,f,g,h,i) => {
            //tracing::trace!("zero: \n{:?},{:?},{:?},\n{:?},{:?},{:?},\n{:?},{:?},{:?},", a,b,c,d,e,f,g,h,i);
            0
        }
    }
}

fn part2(data: &str) -> usize {
    let plane: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let regions = regions(&plane);

    let mut total_price = 0;
    for region in regions.iter() {
        let (row, col) = region.first().cloned().unwrap();
        let plant = plane[row][col];

        let mut edges = 0;
        for plot in region.iter() {
            let (row, col) = *plot;
            let dirs = util::surrounding_all(&plane, row, col)
                .filter(|(_, row, col, _)| region.contains(&(*row, *col)))
                .map(|(c, _, _, dir)| (dir, *c))
                .collect::<BTreeMap<_, _>>();

            tracing::debug!("{:?} around {},{} is {:?}", plant, row, col, dirs);

            use util::Direction::*;
            let new = count(
                dirs.get(&NW) == Some(&plant),
                dirs.get(&N) == Some(&plant),
                dirs.get(&NE) == Some(&plant),
                dirs.get(&W) == Some(&plant),
                dirs.get(&E) == Some(&plant),
                dirs.get(&SW) == Some(&plant),
                dirs.get(&S) == Some(&plant),
                dirs.get(&SE) == Some(&plant),
            );
            if new != 0 {
            tracing::trace!("adds {} edges", new);
            }
            edges += new;
        }

        tracing::debug!("{:?} area {} edges {}", plant, region.len(), edges);
        total_price += region.len() * edges;
    }

    total_price
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day12.1.txt")), 0);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day12.1.txt")), 0);
}
