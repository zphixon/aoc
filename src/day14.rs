use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
};
use unicode_segmentation::UnicodeSegmentation;

pub fn run(example: bool) {
    let data1 = if example {
        include_str!("../data/example/day14.1.txt")
    } else {
        include_str!("../data/day14.1.txt")
    };

    tracing::info!("day 14 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 14 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data1)
    );
    tracing::info!("day 14 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 14 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data1)
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Round,
    Cube,
    Space,
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O" => Ok(Tile::Round),
            "#" => Ok(Tile::Cube),
            "." => Ok(Tile::Space),
            _ => Err(()),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Round => write!(f, "O"),
            Tile::Cube => write!(f, "#"),
            Tile::Space => write!(f, "."),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

fn show_platform(platform: &[Vec<Tile>]) {
    let mut msg = String::from("\n");
    platform.iter().for_each(|row| {
        row.iter().for_each(|col| msg += &format!("{}", col));
        msg += "\n";
    });
    tracing::debug!("{}", msg);
}

fn tilt_north(platform: &mut [Vec<Tile>]) {
    for col in 0..platform[0].len() {
        for row in 1..platform.len() {
            let mut back = 1;
            while row >= back
                && row - back + 1 < platform.len()
                && platform[row - back][col] == Tile::Space
                && platform[row - back + 1][col] == Tile::Round
            {
                platform[row - back][col] = Tile::Round;
                platform[row - back + 1][col] = Tile::Space;
                back += 1;
            }
        }
    }
}

fn load_north(platform: &[Vec<Tile>]) -> usize {
    let mut load = 0;
    for (row_i, row) in platform.iter().enumerate() {
        let row_load = platform.len() - row_i;
        for &col in row.iter() {
            if col == Tile::Round {
                load += row_load;
            }
        }
    }
    load
}

fn part1(data: &str) -> usize {
    let mut platform = parse(data);
    show_platform(&platform);
    tilt_north(&mut platform);
    show_platform(&platform);
    load_north(&platform)
}

fn tilt_west(platform: &mut [Vec<Tile>]) {
    for col in 0..platform[0].len() {
        for row in 0..platform.len() {
            let mut back = 1;
            while col >= back
                && col - back + 1 < platform[0].len()
                && platform[row][col - back] == Tile::Space
                && platform[row][col - back + 1] == Tile::Round
            {
                platform[row][col - back] = Tile::Round;
                platform[row][col - back + 1] = Tile::Space;
                back += 1;
            }
        }
    }
}

fn tilt_south(platform: &mut [Vec<Tile>]) {
    for col in 0..platform[0].len() {
        for row in 1..platform.len() {
            let mut back = 1;
            while row >= back
                && row - back + 1 < platform.len()
                && platform[row - back][col] == Tile::Round
                && platform[row - back + 1][col] == Tile::Space
            {
                platform[row - back][col] = Tile::Space;
                platform[row - back + 1][col] = Tile::Round;
                back += 1;
            }
        }
    }
}

fn tilt_east(platform: &mut [Vec<Tile>]) {
    for col in 0..platform[0].len() {
        for row in 0..platform.len() {
            let mut back = 1;
            while col >= back
                && col - back + 1 < platform[0].len()
                && platform[row][col - back] == Tile::Round
                && platform[row][col - back + 1] == Tile::Space
            {
                platform[row][col - back] = Tile::Space;
                platform[row][col - back + 1] = Tile::Round;
                back += 1;
            }
        }
    }
}

fn part2(data: &str) -> usize {
    let mut platform = parse(data);
    show_platform(&platform);

    fn cycle(platform: &mut [Vec<Tile>]) {
        tilt_north(platform);
        tilt_west(platform);
        tilt_south(platform);
        tilt_east(platform);
    }

    let (mut first, mut second) = (None, None);
    let mut records = HashMap::new();

    for i in 0..1_000_000_000 {
        tracing::trace!("\ni={}\nload={}", i, load_north(&platform));
        show_platform(&platform);

        if let Some(first_i) = records.get(&platform).copied() {
            tracing::trace!("first seen at {}, again at {}", first_i, i);
            first = Some(first_i);
            second = Some(i);
            break;
        } else {
            records.insert(platform.clone(), i);
        }

        cycle(&mut platform);
    }

    let first = first.unwrap();
    let second = second.unwrap();
    let cycle_length = second - first; // no +1 because inclusive
    let nth_in_cycle = (1_000_000_000 - first) % cycle_length;
    let get_load_of = nth_in_cycle + first;

    let load = records
        .iter()
        .find_map(|(platform, &found_at)| {
            if found_at == get_load_of {
                Some(load_north(platform))
            } else {
                None
            }
        })
        .unwrap();

    tracing::debug!("first={first} second={second} cycle_length={cycle_length} nth_in_cycle={nth_in_cycle} get_load_of={get_load_of} load={load}");

load
}

#[test]
fn hash() {
    let a = parse(
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#..OO#..OO",
    );
    let b = parse(
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
    );

    assert_ne!(a, b);

    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    a.hash(&mut hasher);
    let a_hash = hasher.finish();
    b.hash(&mut hasher);
    let b_hash = hasher.finish();
    assert_ne!(a_hash, b_hash);

    let mut map = HashMap::new();
    assert!(map.insert(a.clone(), 1).is_none());
    assert!(map.insert(b.clone(), 2).is_none());
    assert!(map.get(&a).copied() == Some(1));
    assert!(map.get(&b).copied() == Some(2));
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day14.1.txt")), 113424);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day14.1.txt")), 29846);
}
