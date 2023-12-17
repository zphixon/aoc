use std::fmt::{Debug, Write};
use unicode_segmentation::UnicodeSegmentation;

pub fn run(example: bool) {
    let data1 = if example {
        include_str!("../data/example/day13.1.txt")
    } else {
        include_str!("../data/day13.1.txt")
    };

    tracing::info!("day 13 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 13 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data1)
    );
    tracing::info!("day 13 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 13 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data1)
    );
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Ash,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Rock => "#",
                Tile::Ash => ".",
            },
        )
    }
}

fn parse(data: &str) -> Vec<Vec<Vec<Tile>>> {
    let mut notes = Vec::new();

    let mut note = Vec::new();
    for line in data.lines() {
        if line == "" {
            notes.push(note);
            note = Vec::new();
            continue;
        }

        let mut row = Vec::new();
        for tile in line.graphemes(true) {
            if tile == "#" {
                row.push(Tile::Rock);
            } else if tile == "." {
                row.push(Tile::Ash);
            } else {
                panic!("not ash or rock: {:?}", tile);
            }
        }
        note.push(row);
    }

    if !note.is_empty() {
        notes.push(note);
    }

    notes
}

fn show_note(note: &[Vec<Tile>]) {
    let mut dis = String::from("\n");
    for row in note {
        for tile in row {
            write!(dis, "{:?}", tile).unwrap();
        }
        dis += "\n";
    }
    tracing::trace!("{}", dis);
}

fn check_row_reflection(note: &[Vec<Tile>]) {
    let mut stack = Vec::new();

    let mut num_mirrored = 0;
    let mut mirrored = false;

    for (row_i, row) in note.iter().enumerate() {
        if stack.last() == Some(row) {
            tracing::trace!("{row:?} row reflection detected at {row_i}");
            mirrored = true;
            num_mirrored += 1;
            stack.pop();
        } else {
            tracing::trace!("{row:?} not reflected (yet?)");
            stack.push(row.clone());
            mirrored = false;
            num_mirrored = 0;
        }
    }

    if mirrored {

    }
}

fn part1(data: &str) -> usize {
    let notes = parse(data);
    for note in notes.iter() {
        show_note(note);
        check_row_reflection(note);
    }

    0
}

fn part2(data: &str) -> usize {
    0
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day13.1.txt")), 9957702);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day13.1.txt")), 512240933238);
}
