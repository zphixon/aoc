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
