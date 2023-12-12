use std::{
    collections::HashSet,
    fmt::{Debug, Display, Write},
    str::FromStr,
};
use unicode_segmentation::UnicodeSegmentation;

pub fn run(example: bool) {
    let data1 = if example {
        include_str!("../data/example/day11.1.txt")
    } else {
        include_str!("../data/day11.1.txt")
    };

    tracing::info!("day 11 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 11 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data1)
    );
    tracing::info!("day 11 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 11 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data1)
    );
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Galaxy => "#",
                Tile::Empty => ".",
            }
        )
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "." {
            Ok(Tile::Empty)
        } else if s == "#" {
            Ok(Tile::Galaxy)
        } else {
            tracing::trace!("idk tile {:?}", s);
            Err(())
        }
    }
}

fn parse(data: &str) -> Vec<Vec<Tile>> {
    data.lines()
        .map(|line| {
            line.graphemes(true)
                .map(Tile::from_str)
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()
        .unwrap()
}

fn show(chart: &[Vec<impl Display>]) {
    let mut s = String::from("\n");
    for line in chart {
        for col in line {
            write!(s, "{}", col).unwrap();
        }
        write!(s, "\n").unwrap();
    }
    tracing::debug!("{}", s);
}

fn expand(chart: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut new_chart = Vec::new();

    for line in chart {
        if !line.iter().any(|&tile| tile == Tile::Galaxy) {
            new_chart.push(line.clone());
        }
        new_chart.push(line.clone());
    }

    let mut col = 0;
    while col < new_chart[0].len() {
        if !(0..new_chart.len()).any(|row| new_chart[row][col] == Tile::Galaxy) {
            (0..new_chart.len()).for_each(|row| new_chart[row].insert(col, Tile::Empty));
            col += 1;
        }
        col += 1;
    }

    new_chart
}

fn find_galaxies(chart: &[Vec<Tile>]) -> Vec<(usize, usize)> {
    let mut indexes = Vec::new();
    for (row, row_tiles) in chart.iter().enumerate() {
        for (col, &col_tile) in row_tiles.iter().enumerate() {
            if col_tile == Tile::Galaxy {
                indexes.push((row, col));
            }
        }
    }
    indexes
}

#[allow(dead_code)]
fn galaxy_lut(
    galaxies: &[(usize, usize)],
    universe_rows: usize,
    universe_cols: usize,
) -> Vec<Vec<bool>> {
    let mut lut = vec![vec![false; universe_cols]; universe_rows];
    for &(row, col) in galaxies {
        lut[row][col] = true;
    }
    lut
}

#[allow(dead_code)]
fn nearest_galaxy(lut: &[Vec<bool>], (to_row, to_col): (usize, usize)) -> (usize, usize) {
    let mut layer = 1;
    loop {
        for offset_a in 0..layer {
            let offset_b = layer - offset_a;

            if to_row >= offset_a && to_col >= offset_b {
                let (minus_minus_row, minus_minus_col) = (to_row - offset_a, to_col - offset_b);
                if lut[minus_minus_row][minus_minus_col] {
                    return (minus_minus_row, minus_minus_col);
                }
            }
            if to_row >= offset_a && to_col + offset_b < lut[0].len() {
                let (minus_plus_row, minus_plus_col) = (to_row - offset_a, to_col + offset_b);
                if lut[minus_plus_row][minus_plus_col] {
                    return (minus_plus_row, minus_plus_col);
                }
            }
            if to_row + offset_a < lut.len() && to_col >= offset_b {
                let (plus_minus_row, plus_minus_col) = (to_row + offset_a, to_col - offset_b);
                if lut[plus_minus_row][plus_minus_col] {
                    return (plus_minus_row, plus_minus_col);
                }
            }
            if to_row + offset_a < lut.len() && to_col + offset_b < lut[0].len() {
                let (plus_plus_row, plus_plus_col) = (to_row + offset_a, to_col + offset_b);
                if lut[plus_plus_row][plus_plus_col] {
                    return (plus_plus_row, plus_plus_col);
                }
            }
        }

        layer += 1;
        if layer == 9001 {
            panic!("only one galaxy in the universe?");
        }
    }
}

fn part1(data: &str) -> usize {
    let chart = expand(&parse(data));
    show(&chart);

    let galaxies = find_galaxies(&chart);
    //let mut lut = galaxy_lut(&galaxies, chart.len(), chart[0].len());

    tracing::debug!("{} galaxies", galaxies.len());

    let mut counted = HashSet::<(usize, usize, usize, usize)>::new();
    let mut sum = 0;
    for &(row_a, col_a) in galaxies.iter() {
        for &(row_b, col_b) in galaxies.iter() {
            if counted.contains(&(row_a, row_b, col_a, col_b))
                || counted.contains(&(row_b, row_a, col_b, col_a))
            {
                continue;
            }
            counted.insert((row_a, row_b, col_a, col_b));
            counted.insert((row_b, row_a, col_b, col_a));

            if row_a == row_b && col_a == col_b {
                continue;
            }

            let d_row = row_a.max(row_b) - row_b.min(row_a);
            let d_col = col_a.max(col_b) - col_b.min(col_a);
            let dist = d_row + d_col;

            tracing::trace!("between {},{} and {},{} is {}", row_a, col_a, row_b, col_b, dist);

            sum += dist;
        }
    }

    sum
}

fn space_between_rows(chart: &[Vec<Tile>], row_a: usize, row_b: usize) -> usize {
    let my_row_a = row_a.min(row_b);
    let my_row_b = row_a.max(row_b);
    let mut space = 0;
    for row in &chart[my_row_a..my_row_b] {
        if row.iter().all(|&tile| tile == Tile::Empty) {
            space += 1_000_000;
        } else {
            space += 1;
        }
    }
    space
}

fn space_between_cols(chart: &[Vec<Tile>], col_a: usize, col_b: usize) -> usize {
    let my_col_a = col_a.min(col_b);
    let my_col_b = col_a.max(col_b);
    let mut space = 0;
    for col in my_col_a..my_col_b {
        if (0..chart.len()).all(|row| chart[row][col] == Tile::Empty) {
            space += 1_000_000;
        } else {
            space += 1;
        }
    }
    space
}

fn part2(data: &str) -> usize {
    // haha, great punchline
    // let chart = expand(&parse(data));

    let chart = parse(data);
    let galaxies = find_galaxies(&chart);

    tracing::debug!("{} galaxies", galaxies.len());

    let mut counted = HashSet::<(usize, usize, usize, usize)>::new();

    let mut sum = 0;
    for &(row_a, col_a) in galaxies.iter() {
        for &(row_b, col_b) in galaxies.iter() {
            if counted.contains(&(row_a, row_b, col_a, col_b))
                || counted.contains(&(row_b, row_a, col_b, col_a))
            {
                continue;
            }
            counted.insert((row_a, row_b, col_a, col_b));
            counted.insert((row_b, row_a, col_b, col_a));

            if row_a == row_b && col_a == col_b {
                continue;
            }

            // to optimize, probably cache which columns are empty
            let d_row = space_between_rows(&chart, row_a, row_b);
            let d_col = space_between_cols(&chart, col_a, col_b);
            let dist = d_row + d_col;

            tracing::trace!("between {},{} and {},{} is {}", row_a, col_a, row_b, col_b, dist);

            sum += dist;
        }
    }

    sum
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day11.1.txt")), 9957702);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day11.1.txt")), 512240933238);
}
