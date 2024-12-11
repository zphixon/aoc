use itertools::Itertools;
use tracing::Level;

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day9.1.txt")
    } else {
        include_str!("../data/really_evil_input.txt")
    };

    tracing::info!("day 9 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 9 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 9 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 9 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

#[derive(Debug, Clone, Copy)]
struct SizeSpace {
    id: u32,
    size: u32,
    space: u32,
}

fn size_space(data: &str) -> Vec<SizeSpace> {
    let mut tups = data
        .chars()
        .filter(|c| c.is_numeric())
        .flat_map(|c| c.to_digit(10))
        .tuples();
    let mut size_space = tups
        .by_ref()
        .enumerate()
        .map(|(id, (size, space))| SizeSpace {
            id: id as u32,
            size,
            space,
        })
        .collect::<Vec<_>>();
    if let Some(last) = tups
        .into_buffer()
        .map(|last| SizeSpace {
            id: size_space.last().unwrap().id + 1,
            size: last,
            space: 0,
        })
        .next()
    {
        // :(
        size_space.push(last);
    }

    size_space
}

fn vis_size_space(size_space: &[SizeSpace], level: Level) {
    if tracing::enabled!(Level::TRACE) || tracing::enabled!(Level::DEBUG) {
        let mut s = String::new();
        for SizeSpace { id, size, space } in size_space {
            if *size >= 10 {
                s.push_str(&format!("({})", id).repeat(*size as usize));
            } else {
                s.push_str(&format!("{}", id).repeat(*size as usize));
            }
            s.push_str(&".".repeat(*space as usize));
        }
        if level == Level::DEBUG {
            tracing::debug!("{}", s);
        }
        if level == Level::TRACE {
            tracing::trace!("{}", s);
        }
    }
}

fn blocks(size_space: &[SizeSpace]) -> (Vec<Option<usize>>, usize) {
    let mut num_blocks = 0;
    let mut map = Vec::new();
    for SizeSpace { id, size, space } in size_space {
        num_blocks += *size as usize;
        map.extend(std::iter::repeat(Some(*id as usize)).take(*size as usize));
        map.extend(std::iter::repeat(None).take(*space as usize));
    }

    (map, num_blocks)
}

fn vis_blocks(map: &[Option<usize>], level: Level) {
    if tracing::enabled!(Level::TRACE) || tracing::enabled!(Level::DEBUG) {
        let mut s = String::new();
        for space in map {
            match space {
                Some(id) => {
                    if *id >= 10 {
                        s.push_str(&format!("({})", id))
                    } else {
                        s.push_str(&format!("{}", id))
                    }
                }
                None => s.push('.'),
            }
        }
        if level == Level::DEBUG {
            tracing::debug!("{}", s);
        }
        if level == Level::TRACE {
            tracing::trace!("{}", s);
        }
    }
}

fn find_free_forward(map: &[Option<usize>], from: usize) -> Option<usize> {
    let mut i = from;
    while i < map.len() {
        if map[i].is_none() {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_occupied_backward(map: &[Option<usize>], from: usize) -> Option<usize> {
    let mut i = from;
    while i > 0 {
        if map[i].is_some() {
            return Some(i);
        }
        i -= 1;
    }
    None
}

fn part1(data: &str) -> usize {
    let size_space = size_space(data);
    let (mut map, mut num_blocks) = blocks(&size_space);
    vis_blocks(&map, Level::DEBUG);

    let mut free_i = find_free_forward(&map, 0).expect("no space left on device");
    let mut prev_block_i = map.len();
    num_blocks -= free_i;
    tracing::trace!(
        "{} left, skipped {}, {} to {}",
        num_blocks,
        free_i,
        0,
        free_i
    );

    while let Some(block_i) = find_occupied_backward(&map, prev_block_i - 1) {
        map.swap(free_i, block_i);
        vis_blocks(&map, Level::TRACE);

        let next_free_i = find_free_forward(&map, free_i).expect("no space left on device");
        let skipped = next_free_i - free_i;
        num_blocks -= skipped;
        tracing::trace!(
            "{} left, skipped {} ({} to {}), swap {:?} and {:?}",
            num_blocks,
            skipped,
            free_i,
            next_free_i,
            map[block_i],
            map[free_i]
        );
        if num_blocks == 0 {
            break;
        }
        free_i = next_free_i;

        prev_block_i = block_i;
    }

    vis_blocks(&map, Level::DEBUG);

    let mut checksum = 0;
    for (pos, id) in map.iter().enumerate() {
        if id.is_none() {
            break;
        }
        checksum += pos * id.unwrap();
    }

    checksum
}

fn part2(data: &str) -> usize {
    let mut map = size_space(data);
    vis_size_space(&map, Level::DEBUG);

    // if space at elt for last, then
    //     move last
    //     new elt is moved last
    //     set space of new elt
    //     new last is now the actual last
    // new elt is next elt

    let mut last = map.len() - 1;

    'next_last: while last > 0 {
        tracing::trace!("trying to place {}", map[last].id);
        let mut first = 0;
        while first < last {
            tracing::trace!("trying to place {} after {}", map[last].id, map[first].id);
            if map[first].space >= map[last].size {
                map[last - 1].space += map[last].size + map[last].space;
                map[last].space = map[first].space - map[last].size;
                map[first].space = 0;
                let last_item = map.remove(last);
                map.insert(first + 1, last_item);
                vis_size_space(&map, Level::TRACE);
                continue 'next_last;
            }
            first += 1;
        }
        last -= 1;
    }

    let (map, _) = blocks(&map);
    vis_blocks(&map, Level::DEBUG);

    let mut checksum = 0;
    for (pos, id) in map.iter().enumerate() {
        if id.is_none() {
            continue;
        }
        checksum += pos * id.unwrap();
    }

    checksum
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day9.1.txt")), 6242766523059);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day9.1.txt")), 6272188244509);
}
