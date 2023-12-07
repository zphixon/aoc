//! --- Day 6: Wait For It ---
//!
//! The ferry quickly brings you across Island Island. After asking around, you
//! discover that there is indeed normally a large pile of sand somewhere near
//! here, but you don't see anything besides lots of water and the small island
//! where the ferry has docked.
//!
//! As you try to figure out what to do next, you notice a poster on a wall near
//! the ferry dock. "Boat races! Open to the public! Grand prize is an
//! all-expenses-paid trip to Desert Island!" That must be where the sand comes
//! from! Best of all, the boat races are starting in just a few minutes.
//!
//! You manage to sign up as a competitor in the boat races just in time. The
//! organizer explains that it's not really a traditional race - instead, you
//! will get a fixed amount of time during which your boat has to travel as far
//! as it can, and you win if your boat goes the farthest.
//!
//! As part of signing up, you get a sheet of paper (your puzzle input) that
//! lists the time allowed for each race and also the best distance ever
//! recorded in that race. To guarantee you win the grand prize, you need to
//! make sure you go farther in each race than the current record holder.
//!
//! The organizer brings you over to the area where the boat races are held. The
//! boats are much smaller than you expected - they're actually toy boats, each
//! with a big button on top. Holding down the button charges the boat, and
//! releasing the button allows the boat to move. Boats move faster if their
//! button was held longer, but time spent holding the button counts against the
//! total race time. You can only hold the button at the start of the race, and
//! boats don't move until the button is released.
//!
//! For example:
//!
//! Time:      7  15   30
//! Distance:  9  40  200
//!
//! This document describes three races:
//!
//! - The first race lasts 7 milliseconds. The record distance in this race is
//!   9 millimeters.
//! - The second race lasts 15 milliseconds. The record distance in this race
//!   is 40 millimeters.
//! - The third race lasts 30 milliseconds. The record distance in this race is
//!   200 millimeters.
//!
//! Your toy boat has a starting speed of zero millimeters per millisecond. For
//! each whole millisecond you spend at the beginning of the race holding down
//! the button, the boat's speed increases by one millimeter per millisecond.
//!
//! So, because the first race lasts 7 milliseconds, you only have a few
//! options:
//!
//! - Don't hold the button at all (that is, hold it for 0 milliseconds) at the
//!   start of the race. The boat won't move; it will have traveled 0
//!   millimeters by the end of the race.
//! - Hold the button for 1 millisecond at the start of the race. Then, the
//!   boat will travel at a speed of 1 millimeter per millisecond for 6
//!   milliseconds, reaching a total distance traveled of 6 millimeters.
//! - Hold the button for 2 milliseconds, giving the boat a speed of 2
//!   millimeters per millisecond. It will then get 5 milliseconds to move,
//!   reaching a total distance of 10 millimeters.
//! - Hold the button for 3 milliseconds. After its remaining 4 milliseconds of
//!   travel time, the boat will have gone 12 millimeters.
//! - Hold the button for 4 milliseconds. After its remaining 3 milliseconds of
//!   travel time, the boat will have gone 12 millimeters.
//! - Hold the button for 5 milliseconds, causing the boat to travel a total of
//!   10 millimeters.
//! - Hold the button for 6 milliseconds, causing the boat to travel a total of
//!   6 millimeters.
//! - Hold the button for 7 milliseconds. That's the entire duration of the
//!   race. You never let go of the button. The boat can't move until you let
//!   go of the button. Please make sure you let go of the button so the boat
//!   gets to move. 0 millimeters.
//!
//! Since the current record for this race is 9 millimeters, there are actually
//! 4 different ways you could win: you could hold the button for 2, 3, 4, or 5
//! milliseconds at the start of the race.
//!
//! In the second race, you could hold the button for at least 4 milliseconds
//! and at most 11 milliseconds and beat the record, a total of 8 different ways
//! to win.
//!
//! In the third race, you could hold the button for at least 11 milliseconds
//! and no more than 19 milliseconds and still beat the record, a total of 9
//! ways you could win.
//!
//! To see how much margin of error you have, determine the number of ways you
//! can beat the record in each race; in this example, if you multiply these
//! values together, you get 288 (4 * 8 * 9).
//!
//! Determine the number of ways you could beat the record in each race. What do
//! you get if you multiply these numbers together?
//!
//! --- Part Two ---
//!
//! As the race is about to start, you realize the piece of paper with race
//! times and record distances you got earlier actually just has very bad
//! kerning. There's really only one race - ignore the spaces between the
//! numbers on each line.
//!
//! So, the example from before:
//!
//! Time:      7  15   30
//! Distance:  9  40  200
//!
//! ...now instead means this:
//!
//! Time:      71530
//! Distance:  940200
//!
//! Now, you have to figure out how many ways there are to win this single race.
//! In this example, the race lasts for 71530 milliseconds and the record
//! distance you need to beat is 940200 millimeters. You could hold the button
//! anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503
//! ways!
//!
//! How many ways can you beat the record in this one much longer race?

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
