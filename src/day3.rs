//! --- Day 3: Gear Ratios ---
//!
//! You and the Elf eventually reach a gondola lift station; he says the gondola
//! lift will take you up to the water source, but this is as far as he can
//! bring you. You go inside.
//!
//! It doesn't take long to find the gondolas, but there seems to be a problem:
//! they're not moving.
//!
//! "Aaah!"
//!
//! You turn around to see a slightly-greasy Elf with a wrench and a look of
//! surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working
//! right now; it'll still be a while before I can fix it." You offer to help.
//!
//! The engineer explains that an engine part seems to be missing from the
//! engine, but nobody can figure out which one. If you can add up all the part
//! numbers in the engine schematic, it should be easy to work out which part is
//! missing.
//!
//! The engine schematic (your puzzle input) consists of a visual representation
//! of the engine. There are lots of numbers and symbols you don't really
//! understand, but apparently any number adjacent to a symbol, even diagonally,
//! is a "part number" and should be included in your sum. (Periods (.) do not
//! count as a symbol.)
//!
//! Here is an example engine schematic:
//!
//! 467..114..
//! ...*......
//! ..35..633.
//! ......#...
//! 617*......
//! .....+.58.
//! ..592.....
//! ......755.
//! ...$.*....
//! .664.598..
//!
//! In this schematic, two numbers are not part numbers because they are not
//! adjacent to a symbol: 114 (top right) and 58 (middle right). Every other
//! number is adjacent to a symbol and so is a part number; their sum is 4361.
//!
//! Of course, the actual engine schematic is much larger. What is the sum of
//! all of the part numbers in the engine schematic?
//!
//!
//! --- Part Two ---
//!
//! The engineer finds the missing part and installs it in the engine! As the
//! engine springs to life, you jump in the closest gondola, finally ready to
//! ascend to the water source.
//!
//! You don't seem to be going very fast, though. Maybe something is still
//! wrong? Fortunately, the gondola has a phone labeled "help", so you pick it
//! up and the engineer answers.
//!
//! Before you can explain the situation, she suggests that you look out the
//! window. There stands the engineer, holding a phone in one hand and waving
//! with the other. You're going so slowly that you haven't even left the
//! station. You exit the gondola.
//!
//! The missing part wasn't the only issue - one of the gears in the engine is
//! wrong. A gear is any * symbol that is adjacent to exactly two part numbers.
//! Its gear ratio is the result of multiplying those two numbers together.
//!
//! This time, you need to find the gear ratio of every gear and add them all up
//! so that the engineer can figure out which gear needs to be replaced.
//!
//! Consider the same engine schematic again:
//!
//! 467..114..
//! ...*......
//! ..35..633.
//! ......#...
//! 617*......
//! .....+.58.
//! ..592.....
//! ......755.
//! ...$.*....
//! .664.598..
//!
//! In this schematic, there are two gears. The first is in the top left; it has
//! part numbers 467 and 35, so its gear ratio is 16345. The second gear is in
//! the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a
//! gear because it is only adjacent to one part number.) Adding up all of the
//! gear ratios produces 467835.
//!
//! What is the sum of all of the gear ratios in your engine schematic?

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::Index,
};
use unicode_segmentation::UnicodeSegmentation;

pub fn run(example: bool) {
    tracing::info!("day 3 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 3 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(crate::day_data!(example, 1)),
    );
    tracing::info!("day 3 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 3 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(crate::day_data!(example, 1)), // same data
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

fn part1(data: String) -> usize {
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

fn part2(data: String) -> usize {
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
