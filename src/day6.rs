use std::fmt::Debug;

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day6.1.txt")
    } else {
        include_str!("../data/day6.1.txt")
    };

    tracing::info!("day 6 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 6 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data)
    );
    tracing::info!("day 6 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 6 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data)
    );
}

#[derive(Clone, Copy)]
struct Race {
    time_ms: usize,
    record_mm: usize,
}

impl Debug for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mm in {}ms", self.record_mm, self.time_ms)
    }
}

fn races(data: &str) -> Result<Vec<Race>, ()> {
    let [times, records] = data
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line == "" {
                None
            } else {
                Some(line)
            }
        })
        .collect::<Vec<_>>()[..]
    else {
        tracing::error!("more than two lines in input");
        return Err(());
    };

    let Some(times) = times.split("Time:").nth(1) else {
        tracing::error!("didn't split times on 'ime:' {:?}", times);
        return Err(());
    };
    let Some(records) = records.split("Distance:").nth(1) else {
        tracing::error!("didn't split records on 'Distance:' {:?}", records);
        return Err(());
    };

    let Ok(times) = times
        .split_whitespace()
        .map(|time| time.trim().parse())
        .collect::<Result<Vec<_>, _>>()
    else {
        tracing::error!("couldn't parse number in {:?}", times);
        return Err(());
    };
    let Ok(records) = records
        .split_whitespace()
        .map(|record| record.trim().parse())
        .collect::<Result<Vec<_>, _>>()
    else {
        tracing::error!("couldn't parse number in {:?}", records);
        return Err(());
    };

    if times.len() != records.len() {
        tracing::error!("mismatched number of records and times");
        return Err(());
    }

    Ok(times
        .into_iter()
        .zip(records.into_iter())
        .map(|(time_ms, record_mm)| Race { time_ms, record_mm })
        .collect())
}

fn calculate_combos(race: Race) -> usize {
    // larger numbers would  warrant arbitrary-precision floats
    let time = race.time_ms as f64;
    let dist = race.record_mm as f64;

    // button time is the same as the speed, so
    //     distance = button time * (race time - button time)
    // solving for button time
    //     button time = 1/2 (race time +/- sqrt(race time^2 - 4 * distance))
    let low = 0.5 * (time - (time.powi(2) - 4. * dist).sqrt());
    let high = 0.5 * (time + (time.powi(2) - 4. * dist).sqrt());

    // low button time and high button time to get existing record result
    // i.e. distance = low/high * (race time - low/high)
    let low = low.min(high);
    let high = low.max(high);

    // take low to the next highest integer, high to the next lowest integer
    //     e.g. low 5.0 -> 6, high 9.8 -> 9
    // so that old distance < new distance
    //     new distance = new low/high * (race time - new low/high)
    let new_low = (low + 1.).floor() as usize;
    let new_high = (high - 1.).ceil() as usize;

    // find number of integers between them (including low/high)
    let num_hold = new_high - new_low + 1;

    tracing::debug!(
        "winner of {:?} held {} rode {} to go {}, we should hold at least {} ride {} to go {}, {} combos",
        race,
        low,
        high,
        low * (race.time_ms as f64 - low),
        new_low,
        new_high,
        new_low * (race.time_ms - new_low),
        num_hold,
    );

    num_hold
}

fn part1(data: &str) -> usize {
    races(data)
        .unwrap()
        .into_iter()
        .map(calculate_combos)
        .product()
}

fn part2(data: &str) -> usize {
    let races = races(data).unwrap();

    let actual_time = races
        .iter()
        .fold(String::new(), |acc, next| {
            acc + &format!("{}", next.time_ms)
        })
        .parse::<usize>()
        .unwrap();

    let actual_record = races
        .iter()
        .fold(String::new(), |acc, next| {
            acc + &format!("{}", next.record_mm)
        })
        .parse::<usize>()
        .unwrap();

    calculate_combos(Race {
        time_ms: actual_time,
        record_mm: actual_record,
    })
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day6.1.txt")), 2269432);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day6.1.txt")), 35865985);
}
