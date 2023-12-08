use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::Index,
};
use unicode_segmentation::UnicodeSegmentation;

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day3.1.txt")
    } else {
        include_str!("../data/day3.1.txt")
    };

    tracing::info!("day 3 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 3 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );
    tracing::info!("day 3 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 3 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}c{}", self.row, self.col)
    }
}

impl<T> Index<Coordinate> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self[index.row][index.col]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NumberCoordinate {
    number: usize,
    coord: Coordinate,
}

impl NumberCoordinate {
    fn from_data(data: &str) -> Vec<NumberCoordinate> {
        let mut coords = Vec::new();

        for (row, line) in data.lines().enumerate() {
            let mut col = 0;
            let mut ptr = &line[..];
            while ptr != "" {
                tracing::trace!("ptr={}", ptr);
                if ptr.as_bytes()[0].is_ascii_digit() {
                    let mut end_of_number = 0;
                    while end_of_number < ptr.as_bytes().len()
                        && ptr.as_bytes()[0 + end_of_number].is_ascii_digit()
                    {
                        end_of_number += 1;
                    }
                    let number = ptr[0..end_of_number].parse::<usize>().unwrap();
                    let coord = Coordinate { row, col };
                    coords.push(NumberCoordinate { number, coord });
                    tracing::debug!("found {} at {:?}", number, coord);
                    ptr = &ptr[end_of_number..];
                    col += end_of_number;
                } else {
                    ptr = &ptr[1..];
                    col += 1;
                }
            }
        }

        coords
    }

    fn neighbors(&self, width: usize, height: usize) -> Vec<Coordinate> {
        let num_length = format!("{}", self.number).len();

        let offsets = (-1isize..=num_length as isize)
            .map(|col_offset| (-1, col_offset))
            .chain([(0isize, -1)].into_iter())
            .chain([(0isize, num_length as isize)].into_iter())
            .chain((-1isize..=num_length as isize).map(|col_offset| (1, col_offset)));

        let mut coords = Vec::new();

        for (row_offset, col_offset) in offsets {
            if (row_offset == -1 && self.coord.row == 0)
                || (col_offset == -1 && self.coord.col == 0)
                || (self.coord.row as isize + row_offset) >= height as isize
                || (self.coord.col as isize + col_offset) >= width as isize
            {
                continue;
            }
            coords.push(Coordinate {
                row: (self.coord.row as isize + row_offset) as usize,
                col: (self.coord.col as isize + col_offset) as usize,
            });
        }

        coords
    }
}

impl Debug for NumberCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{:?}", self.number, self.coord)
    }
}

fn part1(data: &str) -> usize {
    let mut sum = 0;

    let schematic = data
        .lines()
        .map(|line| line.graphemes(true).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = schematic.len();
    let width = schematic[0].len();

    let numbers = NumberCoordinate::from_data(&data);

    let mut numbers_counted = HashSet::new();
    for number in numbers {
        let neighbors = number.neighbors(width, height);
        tracing::debug!("number {:?} surrounded by {:?}", number, neighbors);
        for neighbor in neighbors {
            tracing::trace!("{:?} = {}", neighbor, schematic[neighbor]);
            if schematic[neighbor] != "."
                && schematic[neighbor]
                    .bytes()
                    .all(|byte| !byte.is_ascii_digit())
            {
                if numbers_counted.contains(&number) {
                    tracing::trace!("already counted {:?}", number);
                } else {
                    numbers_counted.insert(number);
                    tracing::debug!(
                        "{:?} is {}, {} + {} = {}",
                        neighbor,
                        schematic[neighbor],
                        sum,
                        number.number,
                        sum + number.number,
                    );
                    sum += number.number;
                }
            }
        }
    }

    sum
}

fn part2(data: &str) -> usize {
    let mut sum = 0;

    let schematic = data
        .lines()
        .map(|line| line.graphemes(true).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = schematic.len();
    let width = schematic[0].len();

    let numbers = NumberCoordinate::from_data(&data);

    let mut gears_counted: HashMap<Coordinate, HashSet<NumberCoordinate>> = HashMap::new();
    for number in numbers {
        let neighbors = number.neighbors(width, height);
        tracing::debug!("number {:?} surrounded by {:?}", number, neighbors);
        for neighbor in neighbors {
            tracing::trace!("{:?} = {}", neighbor, schematic[neighbor]);
            if schematic[neighbor] == "*" {
                gears_counted
                    .entry(neighbor)
                    .or_insert(HashSet::default())
                    .insert(number);
                tracing::debug!("gear at {:?} has {:?}", neighbor, gears_counted[&neighbor]);
            }
        }
    }

    for (gear, numbers) in gears_counted.iter().filter_map(|(&gear, counted)| {
        if counted.len() == 2 {
            Some((gear, counted))
        } else {
            None
        }
    }) {
        let ratio = numbers
            .iter()
            .map(|number| number.number)
            .product::<usize>();
        tracing::debug!(
            "adding ratio of gear at {:?} {:?}: {} + {} = {}",
            gear,
            numbers,
            sum,
            ratio,
            sum + ratio
        );
        sum += ratio;
    }

    sum
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day3.1.txt")), 533775);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day3.1.txt")), 78236071);
}
