use std::{
    collections::VecDeque,
    fmt::{Debug, Display, Write},
    str::FromStr,
};
use unicode_segmentation::UnicodeSegmentation;

pub fn run(example: bool) {
    let data1 = if example {
        include_str!("../data/example/day10.1.txt")
    } else {
        include_str!("../data/day10.1.txt")
    };

    let data2 = if example {
        include_str!("../data/example/day10.2.txt")
    } else {
        include_str!("../data/day10.1.txt")
    };

    tracing::info!("day 10 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 10 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data1)
    );
    tracing::info!("day 10 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 10 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data2)
    );
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground(bool),
    Start,
}

impl Tile {
    fn connects_south(&self) -> bool {
        matches!(
            self,
            Tile::Start | Tile::Vertical | Tile::SouthEast | Tile::SouthWest
        )
    }
    fn connects_north(&self) -> bool {
        matches!(
            self,
            Tile::Start | Tile::Vertical | Tile::NorthEast | Tile::NorthWest
        )
    }
    fn connects_west(&self) -> bool {
        matches!(
            self,
            Tile::Start | Tile::Horizontal | Tile::NorthWest | Tile::SouthWest
        )
    }
    fn connects_east(&self) -> bool {
        matches!(
            self,
            Tile::Start | Tile::Horizontal | Tile::NorthEast | Tile::SouthEast
        )
    }
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "|" => Tile::Vertical,
            "-" => Tile::Horizontal,
            "L" => Tile::NorthEast,
            "J" => Tile::NorthWest,
            "F" => Tile::SouthEast,
            "7" => Tile::SouthWest,
            "." => Tile::Ground(true),
            "S" => Tile::Start,
            _ => {
                tracing::error!("no tile type {:?}", s);
                return Err(());
            }
        })
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Vertical => "│",
                Tile::Horizontal => "─",
                Tile::NorthEast => "└",
                Tile::NorthWest => "┘",
                Tile::SouthEast => "┌",
                Tile::SouthWest => "┐",
                Tile::Ground(_) => " ",
                Tile::Start => "█",
            }
        )
    }
}

fn parse(data: &str) -> Vec<Vec<Tile>> {
    data.lines()
        .filter_map(|line| {
            if line.starts_with("#") || line.is_empty() {
                None
            } else {
                Some(
                    line.graphemes(true)
                        .map(Tile::from_str)
                        .collect::<Result<_, _>>(),
                )
            }
        })
        .collect::<Result<_, _>>()
        .unwrap()
}

fn show_map(map: &[Vec<impl Display>]) {
    let mut map_str = String::from("\n");
    for line in map {
        for tile in line {
            write!(map_str, "{}", tile).unwrap();
        }
        write!(map_str, "\n").unwrap();
    }
    tracing::trace!("{}", map_str);
}

fn find_start_row_col(map: &[Vec<Tile>]) -> Option<(usize, usize)> {
    for (row, line) in map.iter().enumerate() {
        for (col, &tile) in line.iter().enumerate() {
            if tile == Tile::Start {
                return Some((row, col));
            }
        }
    }
    None
}

fn above_below_left_right(
    map: &[Vec<Tile>],
    (row, col): (usize, usize),
) -> [Option<(Tile, (usize, usize))>; 4] {
    [
        if row == 0 {
            None
        } else {
            Some((map[row - 1][col], (row - 1, col)))
        },
        if row + 1 >= map.len() {
            None
        } else {
            Some((map[row + 1][col], (row + 1, col)))
        },
        if col == 0 {
            None
        } else {
            Some((map[row][col - 1], (row, col - 1)))
        },
        if col + 1 >= map[0].len() {
            None
        } else {
            Some((map[row][col + 1], (row, col + 1)))
        },
    ]
}

fn calculate_distances(map: &Vec<Vec<Tile>>) -> (Vec<Vec<usize>>, usize) {
    let mut map = map.clone();

    let max_row = map.len();
    let max_col = map[0].len();
    let mut distance = 0;
    let mut distances = vec![vec![0usize; max_col]; max_row];
    let (start_row, start_col) = find_start_row_col(&map).expect("no start?");
    let mut current_row = start_row;
    let mut current_col = start_col;
    let mut seen_start = false;
    let mut prev_direction = None;

    loop {
        let current_tile = map[current_row][current_col];
        let [above, below, left, right] = above_below_left_right(&map, (current_row, current_col));
        tracing::trace!(
            "map[{}][{}]={} distance={} above={:?} below={:?} left={:?} right={:?}",
            current_row,
            current_col,
            current_tile,
            distance,
            above,
            below,
            left,
            right,
        );

        macro_rules! next {
            ($row:expr, $col:expr) => {
                current_row = $row;
                current_col = $col;
                distance += 1;
                distances[current_row][current_col] = distance;
                continue;
            };
        }

        macro_rules! arm {
            ($tile:expr, $connects:ident, $connects_opposite:ident, $go_dir:expr, $go_dir_opposite:expr) => {
                match $tile {
                    Some((tile, (row, col)))
                        if tile.$connects()
                            && current_tile.$connects_opposite()
                            && prev_direction != Some($go_dir_opposite) =>
                    {
                        prev_direction = Some($go_dir);
                        next!(row, col);
                    }
                    _ => {}
                }
            };
        }

        macro_rules! check {
            ($above:expr, $below:expr, $left:expr, $right:expr) => {
                check!($above, $below, $left, $right, None);
            };

            ($above:expr, $below:expr, $left:expr, $right:expr, $prefer:expr) => {
                arm!(
                    $above,
                    connects_south,
                    connects_north,
                    Direction::North,
                    Direction::South
                );

                arm!(
                    $below,
                    connects_north,
                    connects_south,
                    Direction::South,
                    Direction::North
                );

                arm!(
                    $left,
                    connects_east,
                    connects_west,
                    Direction::West,
                    Direction::East
                );

                arm!(
                    $right,
                    connects_west,
                    connects_east,
                    Direction::East,
                    Direction::West
                );

                map[current_row][current_col] = Tile::Ground(true);
                distances = vec![vec![0usize; max_col]; max_row];
                current_row = start_row;
                current_col = start_col;
                seen_start = false;
                tracing::trace!("retry");
                continue;
            };
        }

        match map[current_row][current_col] {
            Tile::Start => {
                if seen_start {
                    break;
                }
                seen_start = true;

                check!(above, below, left, right);
            }

            Tile::Vertical
            | Tile::Horizontal
            | Tile::NorthEast
            | Tile::NorthWest
            | Tile::SouthEast
            | Tile::SouthWest => {
                check!(above, below, left, right);
            }

            Tile::Ground(_) => {
                panic!("fell out of the pipes??");
            }
        }
    }

    (distances, distance / 2)
}

fn part1(data: &str) -> usize {
    let map = parse(data);
    show_map(&map);
    calculate_distances(&map).1
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Fill {
    Wall,
    CountedSpace,
    UncountedSpace,
}

impl Tile {
    fn expand(&self) -> [[Tile; 3]; 3] {
        use Tile::*;

        match self {
            Vertical => [
                [Ground(false), Vertical, Ground(false)],
                [Ground(false), Vertical, Ground(false)],
                [Ground(false), Vertical, Ground(false)],
            ],
            Horizontal => [
                [Ground(false), Ground(false), Ground(false)],
                [Horizontal, Horizontal, Horizontal],
                [Ground(false), Ground(false), Ground(false)],
            ],
            NorthEast => [
                [Ground(false), Vertical, Ground(false)],
                [Ground(false), NorthEast, Horizontal],
                [Ground(false), Ground(false), Ground(false)],
            ],
            NorthWest => [
                [Ground(false), Vertical, Ground(false)],
                [Horizontal, NorthWest, Ground(false)],
                [Ground(false), Ground(false), Ground(false)],
            ],
            SouthEast => [
                [Ground(false), Ground(false), Ground(false)],
                [Ground(false), SouthEast, Horizontal],
                [Ground(false), Vertical, Ground(false)],
            ],
            SouthWest => [
                [Ground(false), Ground(false), Ground(false)],
                [Horizontal, SouthWest, Ground(false)],
                [Ground(false), Vertical, Ground(false)],
            ],
            Start => [
                [Ground(false), Vertical, Ground(false)],
                [Horizontal, Start, Horizontal],
                [Ground(false), Vertical, Ground(false)],
            ],
            Ground(_) => [[Ground(true); 3]; 3],
        }
    }
}

fn expand_map(map: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut new_map = Vec::new();
    for row in map {
        let mut new_row_a = Vec::new();
        let mut new_row_b = Vec::new();
        let mut new_row_c = Vec::new();
        for col in row {
            let expanded = col.expand();
            new_row_a.extend(expanded[0]);
            new_row_b.extend(expanded[1]);
            new_row_c.extend(expanded[2]);
        }
        new_map.push(new_row_a);
        new_map.push(new_row_b);
        new_map.push(new_row_c);
    }
    new_map
}

impl Display for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Fill::Wall => "█",
                Fill::CountedSpace => "▪",
                Fill::UncountedSpace => " ",
            }
        )
    }
}

impl Debug for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn map_to_fill(map: &[Vec<Tile>]) -> Vec<Vec<Fill>> {
    let expanded_map = expand_map(map);
    let (distances, _) = calculate_distances(&expanded_map);

    show_map(&expanded_map);

    let mut fill = vec![vec![Fill::UncountedSpace; expanded_map[0].len()]; expanded_map.len()];

    for (row, tile_row) in expanded_map.iter().enumerate() {
        for (col, &tile_col) in tile_row.iter().enumerate() {
            fill[row][col] = match tile_col {
                Tile::Vertical
                | Tile::Horizontal
                | Tile::NorthEast
                | Tile::NorthWest
                | Tile::SouthEast
                | Tile::SouthWest
                    if distances[row][col] != 0 =>
                {
                    Fill::Wall
                }

                Tile::Vertical
                | Tile::Horizontal
                | Tile::NorthEast
                | Tile::NorthWest
                | Tile::SouthEast
                | Tile::SouthWest
                    if distances[row][col] == 0 =>
                {
                    Fill::CountedSpace
                }

                Tile::Ground(count) => {
                    if count {
                        Fill::CountedSpace
                    } else {
                        Fill::UncountedSpace
                    }
                }

                Tile::Start => Fill::Wall,

                _ => panic!(),
            }
        }
    }

    for three_rows in fill.chunks_mut(3) {
        let [a, b, c] = three_rows else {
            panic!("expanded height not a multiple of 3");
        };

        for ((abc, def), ghi) in a.chunks_mut(3).zip(b.chunks_mut(3)).zip(c.chunks_mut(3)) {
            if abc
                .iter()
                .chain(def.iter())
                .chain(ghi.iter())
                .any(|&fill| fill == Fill::Wall)
            {
                fn modify(fill: &mut Fill) {
                    if *fill == Fill::CountedSpace {
                        *fill = Fill::UncountedSpace;
                    }
                }
                abc.iter_mut().for_each(modify);
                def.iter_mut().for_each(modify);
                ghi.iter_mut().for_each(modify);
                continue;
            }
            abc[0] = Fill::CountedSpace;
            abc[1] = Fill::CountedSpace;
            abc[2] = Fill::CountedSpace;
            def[0] = Fill::CountedSpace;
            def[1] = Fill::CountedSpace;
            def[2] = Fill::CountedSpace;
            ghi[0] = Fill::CountedSpace;
            ghi[1] = Fill::CountedSpace;
            ghi[2] = Fill::CountedSpace;
        }
    }

    fill
}

fn flood_fill(fill: &mut Vec<Vec<Fill>>) {
    let max_row = fill.len();
    let max_col = fill[0].len();
    let mut checked = vec![vec![false; max_col]; max_row];
    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back((0, 0));

    while let Some((row, col)) = queue.pop_front() {
        if checked[row][col] {
            continue;
        }
        checked[row][col] = true;

        fill[row][col] = Fill::UncountedSpace;

        (if row == 0 { 0 } else { -1 }..=if row + 1 >= max_row { 0 } else { 1 }).for_each(
            |row_off: isize| {
                (if col == 0 { 0 } else { -1 }..=if col + 1 >= max_col { 0 } else { 1 }).for_each(
                    |col_off: isize| {
                        let new_row = (row as isize + row_off) as usize;
                        let new_col = (col as isize + col_off) as usize;
                        if !checked[new_row][new_col] && fill[new_row][new_col] != Fill::Wall {
                            queue.push_back((new_row, new_col));
                        }
                    },
                );
            },
        );
    }
}

fn part2(data: &str) -> i64 {
    let map = parse(data);
    let mut fill = map_to_fill(&map);
    flood_fill(&mut fill);
    show_map(&fill);

    let mut num_count = 0;
    for row in fill {
        for col in row {
            if col == Fill::CountedSpace {
                num_count += 1;
            }
        }
    }

    num_count / 9
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day10.1.txt")), 6690);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day10.1.txt")), 525);
}
