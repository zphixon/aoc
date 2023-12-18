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

enum NeResult {
    Equal,
    OneNotEqual(usize, usize),
    MoreThanOneNotEqual,
}

fn where_ne_col(note: &[Vec<Tile>], left: usize, right: usize) -> NeResult {
    let mut msg = format!("compare cols {} and {}\n", left, right);
    let mut iter = (0..note.len()).filter_map(|row_i| {
        msg += &format!("{:?} {:?}\n", note[row_i][left], note[row_i][right]);
        if note[row_i][left] != note[row_i][right] {
            Some((row_i, left))
        } else {
            None
        }
    });

    let result = if let Some((row, col)) = iter.next() {
        if let Some(_) = iter.next() {
            NeResult::MoreThanOneNotEqual
        } else {
            NeResult::OneNotEqual(row, col)
        }
    } else {
        NeResult::Equal
    };

    tracing::trace!("{}", msg);

    result
}

fn where_ne_row(note: &[Vec<Tile>], up: usize, down: usize) -> NeResult {
    tracing::trace!(
        "compare rows {} and {}\n{:?}\n{:?}",
        up,
        down,
        note[up],
        note[down]
    );

    let mut iter = (0..note[0].len()).filter_map(|col_i| {
        if note[up][col_i] != note[down][col_i] {
            Some((up, col_i))
        } else {
            None
        }
    });

    if let Some((row, col)) = iter.next() {
        if let Some(_) = iter.next() {
            NeResult::MoreThanOneNotEqual
        } else {
            NeResult::OneNotEqual(row, col)
        }
    } else {
        NeResult::Equal
    }
}

//fn num_ne_in_cols(note: &[Vec<Tile>], left: usize, right: usize) -> usize {
//    let mut msg = format!("compare cols {} and {}\n", left, right);
//    let result = (0..note.len())
//        .map(|row_i| note[row_i][left])
//        .zip((0..note.len()).map(|row_i| note[row_i][right]))
//        .filter(|(l, r)| {
//            msg += &format!("{:?} {:?}\n", l, r);
//            l != r
//        })
//        .count();
//    tracing::trace!("{}", msg);
//    result
//}
//
//fn num_ne_in_rows(note: &[Vec<Tile>], up: usize, down: usize) -> usize {
//    tracing::trace!(
//        "compare rows {} and {}\n{:?}\n{:?}",
//        up,
//        down,
//        note[up],
//        note[down]
//    );
//
//    note[up]
//        .iter()
//        .zip(note[down].iter())
//        .filter(|(u, d)| u != d)
//        .count()
//}

fn check_row_reflection(note: &[Vec<Tile>], use_smudge: bool) -> (usize, bool) {
    let mut smudged = false;
    let mut sum = 0;
    let mut start = 0;

    while start + 1 < note.len() {
        let mut up = start;
        let mut down = start + 1;

        loop {
            match where_ne_row(note, up, down) {
                NeResult::Equal => {
                    sum += 1;
                    tracing::trace!("nice {}", sum)
                }
                NeResult::OneNotEqual(row, col) if use_smudge && !smudged => {
                    smudged = true;
                    sum += 1;
                    tracing::trace!("smudged at {row},{col} {}", sum);
                }
                _ => {
                    sum = 0;
                    smudged = false;
                    tracing::trace!("oop");
                    break;
                }
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
    (mirrored, smudged)
}

fn check_col_reflection(note: &[Vec<Tile>], use_smudge: bool) -> (usize, bool) {
    let mut smudged = false;
    let mut sum = 0;
    let mut start = 0;
    let width = note[0].len();

    while start + 1 < width {
        let mut left = start;
        let mut right = start + 1;

        loop {
            match where_ne_col(note, left, right) {
                NeResult::Equal => {
                    sum += 1;
                    tracing::trace!("nice {}", sum)
                }
                NeResult::OneNotEqual(row, col) if use_smudge && !smudged => {
                    smudged = true;
                    sum += 1;
                    tracing::trace!("smudged at {row},{col} {}", sum);
                }
                _ => {
                    sum = 0;
                    smudged = false;
                    tracing::trace!("oop");
                    break;
                }
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
    (mirrored, smudged)
}

fn part1(data: &str) -> usize {
    let notes = parse(data);
    let mut row_sum = 0;
    let mut col_sum = 0;

    for note in notes.iter() {
        show_note(note);
        row_sum += check_row_reflection(note, false).0;
        col_sum += check_col_reflection(note, false).0;

        let above = check_row_reflection(note, false).0;
        let left = check_col_reflection(note, false).0;

        if above != 0 && left != 0 {
            unreachable!();
        }

        //tracing::info!("waiting for input");
        //let mut b = String::new();
        //std::io::stdin().read_line(&mut b).unwrap();
    }

    col_sum + (100 * row_sum)
}

fn part2(data: &str) -> usize {
    let notes = parse(data);
    let mut row_sum = 0;
    let mut col_sum = 0;

    for note in notes.iter() {
        show_note(note);

        // figure out where the mirror actually is
        let above = check_row_reflection(note, false).0;
        let left = check_col_reflection(note, false).0;

        if above != 0 && left != 0 {
            unreachable!();
        }

        if above != 0 {
            let new_above = check_row_reflection(note, true).0;
            tracing::debug!("above: {} to {}", above, new_above);
            row_sum += new_above;
        } else {
            let new_left = check_col_reflection(note, true).0;
            tracing::debug!("left: {} to {}", left, new_left);
            col_sum += new_left;
        }


        //if above_smudge == 0 {
        //    if left_smudge == 0 {
        //        unreachable!();
        //    } else {
        //        col_sum += left_smudge;
        //    }
        //} else {
        //    if left_smudge == 0 {
        //        row_sum += above_smudge;
        //    } else {
        //        if smudged_above {
        //            if smudged_left {
        //                unreachable!();
        //            } else {
        //                row_sum += above_smudge;
        //            }
        //        } else {
        //            if smudged_left {
        //                col_sum += left_smudge;
        //            } else {
        //                unreachable!();
        //            }
        //        }
        //    }
        //}

        //let (above, had_smudge) = check_row_reflection(note, true);
        //row_sum += above;

        //if above == 0 {
        //let (left, _) = check_col_reflection(note, !had_smudge);
        //col_sum += left;
        //}

        //tracing::info!("waiting for input");
        //let mut b = String::new();
        //std::io::stdin().read_line(&mut b).unwrap();
    }

    col_sum + (100 * row_sum)
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day13.1.txt")), 29846);
}
#[test]
fn test2() {
    panic!("this puzzle is poorly written");
}
