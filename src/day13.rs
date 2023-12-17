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

fn col_eq(note: &[Vec<Tile>], left: usize, right: usize) -> bool {
    let mut msg = format!("compare cols {} and {}\n", left, right);
    let result = (0..note.len())
        .map(|row_i| note[row_i][left])
        .zip((0..note.len()).map(|row_i| note[row_i][right]))
        .all(|(l, r)| {
            msg += &format!("{:?} {:?}\n", l, r);
            l == r
        });
    tracing::trace!("{}", msg);
    result
}

fn row_eq(note: &[Vec<Tile>], up: usize, down: usize) -> bool {
    tracing::trace!(
        "compare rows {} and {}\n{:?}\n{:?}",
        up,
        down,
        note[up],
        note[down]
    );
    note[up] == note[down]
}

fn check_row_reflection(note: &[Vec<Tile>]) -> usize {
    let mut sum = 0;
    let mut start = 0;

    while start + 1 < note.len() {
        let mut up = start;
        let mut down = start + 1;

        loop {
            if row_eq(note, up, down) {
                sum += 1;
                tracing::trace!("nice {}", sum);
            } else {
                sum = 0;
                tracing::trace!("oop");
                break;
            }

            if up == 0 {
                break;
            }

            up -= 1;
            down += 1;

            if down == note.len() {
                break;
            }
        }

        if (up == 0 || down == note.len()) && sum != 0 {
            tracing::trace!("heyo");
            break;
        }

        start += 1;
    }

    let mirrored = if sum != 0 { start + 1 } else { 0 };
    tracing::debug!("mirrored {} rows up", mirrored);
    mirrored
}

fn check_col_reflection(note: &[Vec<Tile>]) -> usize {
    let mut sum = 0;
    let mut start = 0;
    let width = note[0].len();

    while start + 1 < width {
        let mut left = start;
        let mut right = start + 1;

        loop {
            if col_eq(note, left, right) {
                sum += 1;
                tracing::trace!("nice {}", sum);
            } else {
                sum = 0;
                tracing::trace!("oop");
                break;
            }

            if left == 0 {
                break;
            }

            left -= 1;
            right += 1;

            if right == width {
                break;
            }
        }

        if (left == 0 || right == width) && sum != 0 {
            tracing::trace!("heyo start={start}");
            break;
        }

        start += 1;
    }

    let mirrored = if sum == 0 { 0 } else { start + 1 };
    tracing::debug!("mirrored {} cols left", mirrored);
    mirrored
}

fn part1(data: &str) -> usize {
    let notes = parse(data);
    let mut row_sum = 0;
    let mut col_sum = 0;

    for note in notes.iter() {
        show_note(note);
        row_sum += check_row_reflection(note);
        col_sum += check_col_reflection(note);

        //tracing::info!("waiting for input");
        //let mut b = String::new();
        //std::io::stdin().read_line(&mut b).unwrap();
    }

    col_sum + (100 * row_sum)
}

fn part2(data: &str) -> usize {
    0
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day13.1.txt")), 29846);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day13.1.txt")), 512240933238);
}
