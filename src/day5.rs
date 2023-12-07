//! --- Day 5: If You Give A Seed A Fertilizer ---
//!
//! You take the boat and find the gardener right where you were told he would
//! be: managing a giant "garden" that looks more to you like a farm.
//!
//! "A water source? Island Island is the water source!" You point out that Snow
//! Island isn't receiving any water.
//!
//! "Oh, we had to stop the water because we ran out of sand to filter it with!
//! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand
//! soon; we only turned off the water a few days... weeks... oh no." His face
//! sinks into a look of horrified realization.
//!
//! "I've been so busy making sure everyone here has food that I completely
//! forgot to check why we stopped getting more sand! There's a ferry leaving
//! soon that is headed over in that direction - it's much faster than your
//! boat. Could you please go check it out?"
//!
//! You barely have time to agree to this request when he brings up another.
//! "While you wait for the ferry, maybe you can help us with our food
//! production problem. The latest Island Island Almanac just arrived and we're
//! having trouble making sense of it."
//!
//! The almanac (your puzzle input) lists all of the seeds that need to be
//! planted. It also lists what type of soil to use with each kind of seed, what
//! type of fertilizer to use with each kind of soil, what type of water to use
//! with each kind of fertilizer, and so on. Every type of seed, soil,
//! fertilizer and so on is identified with a number, but numbers are reused by
//! each category - that is, soil 123 and fertilizer 123 aren't necessarily
//! related to each other.
//!
//! For example:
//!
//! seeds: 79 14 55 13
//!
//! seed-to-soil map:
//! 50 98 2
//! 52 50 48
//!
//! soil-to-fertilizer map:
//! 0 15 37
//! 37 52 2
//! 39 0 15
//!
//! fertilizer-to-water map:
//! 49 53 8
//! 0 11 42
//! 42 0 7
//! 57 7 4
//!
//! water-to-light map:
//! 88 18 7
//! 18 25 70
//!
//! light-to-temperature map:
//! 45 77 23
//! 81 45 19
//! 68 64 13
//!
//! temperature-to-humidity map:
//! 0 69 1
//! 1 0 69
//!
//! humidity-to-location map:
//! 60 56 37
//! 56 93 4
//!
//! The almanac starts by listing which seeds need to be planted: seeds 79, 14,
//! 55, and 13.
//!
//! The rest of the almanac contains a list of maps which describe how to
//! convert numbers from a source category into numbers in a destination
//! category. That is, the section that starts with seed-to-soil map: describes
//! how to convert a seed number (the source) to a soil number (the
//! destination). This lets the gardener and his team know which soil to use
//! with which seeds, which water to use with which fertilizer, and so on.
//!
//! Rather than list every source number and its corresponding destination
//! number one by one, the maps describe entire ranges of numbers that can be
//! converted. Each line within a map contains three numbers: the destination
//! range start, the source range start, and the range length.
//!
//! Consider again the example seed-to-soil map:
//!
//! 50 98 2
//! 52 50 48
//!
//! The first line has a destination range start of 50, a source range start of
//! 98, and a range length of 2. This line means that the source range starts at
//! 98 and contains two values: 98 and 99. The destination range is the same
//! length, but it starts at 50, so its two values are 50 and 51. With this
//! information, you know that seed number 98 corresponds to soil number 50 and
//! that seed number 99 corresponds to soil number 51.
//!
//! The second line means that the source range starts at 50 and contains 48
//! values: 50, 51, ..., 96, 97. This corresponds to a destination range
//! starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed
//! number 53 corresponds to soil number 55.
//!
//! Any source numbers that aren't mapped correspond to the same destination
//! number. So, seed number 10 corresponds to soil number 10.
//!
//! So, the entire list of seed numbers and their corresponding soil numbers
//! looks like this:
//!
//! seed  soil
//! 0     0
//! 1     1
//! ...   ...
//! 48    48
//! 49    49
//! 50    52
//! 51    53
//! ...   ...
//! 96    98
//! 97    99
//! 98    50
//! 99    51
//!
//! With this map, you can look up the soil number required for each initial
//! seed number:
//!
//!     Seed number 79 corresponds to soil number 81.
//!     Seed number 14 corresponds to soil number 14.
//!     Seed number 55 corresponds to soil number 57.
//!     Seed number 13 corresponds to soil number 13.
//!
//! The gardener and his team want to get started as soon as possible, so they'd
//! like to know the closest location that needs a seed. Using these maps, find
//! the lowest location number that corresponds to any of the initial seeds. To
//! do this, you'll need to convert each seed number through other categories
//! until you can find its corresponding location number. In this example, the
//! corresponding types are:
//!
//!
//! - Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78,
//!   humidity 78, location 82.
//! - Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42,
//!   humidity 43, location 43.
//! - Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82,
//!   humidity 82, location 86.
//! - Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34,
//!   humidity 35, location 35.
//!
//! So, the lowest location number in this example is 35.
//!
//! What is the lowest location number that corresponds to any of the initial
//! seed numbers?
//!
//!
//! --- Part Two ---
//!
//! Everyone will starve if you only plant such a small number of seeds.
//! Re-reading the almanac, it looks like the seeds: line actually describes
//! ranges of seed numbers.
//!
//! The values on the initial seeds: line come in pairs. Within each pair, the
//! first value is the start of the range and the second value is the length of
//! the range. So, in the first line of the example above:
//!
//! seeds: 79 14 55 13
//!
//! This line describes two ranges of seed numbers to be planted in the garden.
//! The first range starts with seed number 79 and contains 14 values: 79, 80,
//! ..., 91, 92. The second range starts with seed number 55 and contains 13
//! values: 55, 56, ..., 66, 67.
//!
//! Now, rather than considering four seed numbers, you need to consider a total
//! of 27 seed numbers.
//!
//! In the above example, the lowest location number can be obtained from seed
//! number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77,
//! temperature 45, humidity 46, and location 46. So, the lowest location number
//! is 46.
//!
//! Consider all of the initial seed numbers listed in the ranges on the first
//! line of the almanac. What is the lowest location number that corresponds to
//! any of the initial seed numbers?

use std::{collections::HashMap, fmt::Debug, str::FromStr};

pub fn run(example: bool) {
    // same data
    let part1_data = if example {
        include_str!("../data/example/day5.1.txt")
    } else {
        include_str!("../data/day5.1.txt")
    };

    tracing::info!("day 5 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 5 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(part1_data),
    );
    tracing::info!("day 5 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 5 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(part1_data),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Id {
    Seed(usize),
    Soil(usize),
    Fertilizer(usize),
    Water(usize),
    Light(usize),
    Temperature(usize),
    Humidity(usize),
    Location(usize),
}

impl Id {
    #[inline]
    fn num(&self) -> usize {
        match *self {
            Id::Seed(num)
            | Id::Soil(num)
            | Id::Fertilizer(num)
            | Id::Water(num)
            | Id::Light(num)
            | Id::Temperature(num)
            | Id::Humidity(num)
            | Id::Location(num) => num,
        }
    }

    fn next(&self, almanac: &Almanac) -> Option<Id> {
        let mapped = match *self {
            Id::Seed(id) => Id::Soil(
                almanac
                    .seed_soil
                    .iter()
                    .find_map(|map| map.map(id))
                    .unwrap_or(id),
            ),
            Id::Soil(id) => Id::Fertilizer(
                almanac
                    .soil_fertilizer
                    .iter()
                    .find_map(|map| map.map(id))
                    .unwrap_or(id),
            ),
            Id::Fertilizer(id) => Id::Water(
                almanac
                    .fertilizer_water
                    .iter()
                    .find_map(|map| map.map(id))
                    .unwrap_or(id),
            ),
            Id::Water(id) => Id::Light(
                almanac
                    .water_light
                    .iter()
                    .find_map(|map| map.map(id))
                    .unwrap_or(id),
            ),
            Id::Light(id) => Id::Temperature(
                almanac
                    .light_temperature
                    .iter()
                    .find_map(|map| map.map(id))
                    .unwrap_or(id),
            ),
            Id::Temperature(id) => Id::Humidity(
                almanac
                    .temperature_humidity
                    .iter()
                    .find_map(|map| map.map(id))
                    .unwrap_or(id),
            ),
            Id::Humidity(id) => Id::Location(
                almanac
                    .humidity_location
                    .iter()
                    .find_map(|map| map.map(id))
                    .unwrap_or(id),
            ),
            Id::Location(id) => {
                tracing::trace!("at the end: {}", id);
                return None;
            }
        };
        tracing::trace!("{:?} => {:?}", self, mapped);
        Some(mapped)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Map {
    dest: usize,
    source: usize,
    len: usize,
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} to {} -> {} to {}",
            self.source,
            self.source + self.len - 1,
            self.dest,
            self.dest + self.len - 1,
        )
    }
}

impl Map {
    #[inline]
    fn maps(&self, id: usize) -> bool {
        self.source <= id && id <= self.source + self.len - 1
    }

    #[inline]
    fn map(&self, id: usize) -> Option<usize> {
        if self.maps(id) {
            let new_id = id - self.source + self.dest;
            //tracing::trace!("{:?}: {} to {}", self, id, new_id);
            Some(new_id)
        } else {
            //tracing::trace!("{:?}: no change for {}", self, id);
            None
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_soil: Vec<Map>,
    soil_fertilizer: Vec<Map>,
    fertilizer_water: Vec<Map>,
    water_light: Vec<Map>,
    light_temperature: Vec<Map>,
    temperature_humidity: Vec<Map>,
    humidity_location: Vec<Map>,
}

unsafe impl Send for Almanac {}
unsafe impl Sync for Almanac {}

impl FromStr for Almanac {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_map = None;
        let mut maps = HashMap::<String, Vec<Map>>::new();
        let mut seeds = None;
        'next_line: for line in s.lines() {
            if line.trim() == "" {
                continue;
            }

            if line.starts_with("seeds") {
                seeds = line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .map(|num| num.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
                    .ok();
                tracing::trace!("got seeds {:?}", seeds);
                continue 'next_line;
            }

            for map in [
                "seed-to-soil",
                "soil-to-fertilizer",
                "fertilizer-to-water",
                "water-to-light",
                "light-to-temperature",
                "temperature-to-humidity",
                "humidity-to-location",
            ] {
                if line.starts_with(map) {
                    tracing::trace!("mapping {}", map);
                    current_map = Some(map);
                    continue 'next_line;
                }
            }

            let Some(current_map) = current_map else {
                tracing::error!("no current map");
                return Err(());
            };

            let Ok(map) = line
                .split_whitespace()
                .map(|num| num.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
            else {
                tracing::error!("couldn't parse a num in {}", line);
                return Err(());
            };

            let [dest, source, len] = map[..] else {
                tracing::error!("too many nums in {:?}", map);
                return Err(());
            };

            let map = Map { dest, source, len };
            tracing::trace!("{}: {:?}", line, map);

            maps.entry(current_map.into())
                .or_insert(Vec::new())
                .push(map);
        }

        let Some(seeds) = seeds else {
            tracing::error!("no seeds");
            return Err(());
        };

        Ok(Almanac {
            seeds,
            seed_soil: maps.remove("seed-to-soil").unwrap(),
            soil_fertilizer: maps.remove("soil-to-fertilizer").unwrap(),
            fertilizer_water: maps.remove("fertilizer-to-water").unwrap(),
            water_light: maps.remove("water-to-light").unwrap(),
            light_temperature: maps.remove("light-to-temperature").unwrap(),
            temperature_humidity: maps.remove("temperature-to-humidity").unwrap(),
            humidity_location: maps.remove("humidity-to-location").unwrap(),
        })
    }
}

fn part1(data: &str) -> usize {
    let almanac = data.parse::<Almanac>().unwrap();
    tracing::trace!("{:#?}", almanac);

    let mut lowest = usize::MAX;

    for seed in almanac.seeds.iter().copied() {
        let mut id = Id::Seed(seed);
        while let Some(next) = id.next(&almanac) {
            id = next;
        }
        if !matches!(id, Id::Location(_)) {
            tracing::error!("not a location? {:?}", id);
            return 0;
        }
        if id.num() < lowest {
            lowest = id.num();
        }
        tracing::debug!(
            "seed {} maps to location {}, lowest is {}",
            seed,
            id.num(),
            lowest
        );
    }

    lowest
}

fn part2(data: &str) -> usize {
    let almanac = data.parse::<Almanac>().unwrap();
    tracing::trace!("{:#?}", almanac);

    std::thread::scope(|spawner| {
        let mut handles = Vec::new();

        for ranges in almanac.seeds.chunks(2) {
            handles.push(spawner.spawn(|| {
                let [start, len] = ranges[..] else {
                    tracing::error!("non-even number of seed ranges?");
                    return 0;
                };

                tracing::info!("range {} to {:?}", start, start + len);
                let mut lowest = usize::MAX;

                for seed in start..start + len {
                    let mut id = Id::Seed(seed);
                    while let Some(next) = id.next(&almanac) {
                        id = next;
                    }
                    // comment this out cause it's so slow :/
                    //if !matches!(id, Id::Location(_)) {
                    //    tracing::error!("not a location? {:?}", id);
                    //    return 0;
                    //}
                    if id.num() < lowest {
                        lowest = id.num();
                    }
                    //tracing::debug!(
                    //    "seed {} maps to location {}, lowest is {}",
                    //    seed,
                    //    id.num(),
                    //    lowest
                    //);
                }

                tracing::info!("range {} to {:?} finished: {}", start, start + len, lowest);
                lowest
            }));
        }

        let mut lowest = usize::MAX;
        for thread_result in handles.into_iter().map(|thread| thread.join().unwrap()) {
            if thread_result < lowest {
                lowest = thread_result;
            }
        }

        lowest
    })
}

#[test]
fn maps_correctly() {
    let map = Map {
        dest: 50,
        source: 98,
        len: 5,
    };
    assert_eq!(
        (95..=105)
            .filter_map(|num| map.map(num))
            .collect::<Vec<_>>(),
        vec![50, 51, 52, 53, 54]
    );
    assert!(matches!(
        (79..79 + 14).collect::<Vec<usize>>()[..],
        [79, .., 92]
    ));
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day5.1.txt")), 173706076);
}
#[allow(unreachable_code)]
#[test]
fn test2() {
    panic!("this thing is stupid slow. if you really wanna, remove this panic and use something like \"sudo nice -n -20 cargo test\"");
    assert_eq!(part2(include_str!("../data/day5.1.txt")), 11611182);
}
