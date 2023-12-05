//! --- Day 1: Trebuchet?! ---
//!
//! Something is wrong with global snow production, and you've been selected to
//! take a look. The Elves have even given you a map; on it, they've used stars
//! to mark the top fifty locations that are likely to be having problems.
//!
//! You've been doing this long enough to know that to restore snow operations,
//! you need to check all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each
//! day in the Advent calendar; the second puzzle is unlocked when you complete
//! the first. Each puzzle grants one star. Good luck!
//!
//! You try to ask why they can't just use a weather machine ("not powerful
//! enough") and where they're even sending you ("the sky") and why your map
//! looks mostly blank ("you sure ask a lot of questions") and hang on did you
//! just say the sky ("of course, where do you think snow comes from") when you
//! realize that the Elves are already loading you into a trebuchet ("please
//! hold still, we need to strap you in").
//!
//! As they're making the final adjustments, they discover that their
//! calibration document (your puzzle input) has been amended by a very young
//! Elf who was apparently just excited to show off her art skills.
//! Consequently, the Elves are having trouble reading the values on the
//! document.
//!
//! The newly-improved calibration document consists of lines of text; each line
//! originally contained a specific calibration value that the Elves now need to
//! recover. On each line, the calibration value can be found by combining the
//! first digit and the last digit (in that order) to form a single two-digit
//! number.
//!
//! For example:
//!
//! 1abc2
//! pqr3stu8vwx
//! a1b2c3d4e5f
//! treb7uchet
//!
//! In this example, the calibration values of these four lines are 12, 38, 15,
//! and 77. Adding these together produces 142.
//!
//! Consider your entire calibration document. What is the sum of all of the
//! calibration values?
//!
//!
//! --- Part Two ---
//!
//! Your calculation isn't quite right. It looks like some of the digits are
//! actually spelled out with letters: one, two, three, four, five, six, seven,
//! eight, and nine also count as valid "digits".
//!
//! Equipped with this new information, you now need to find the real first and
//! last digit on each line. For example:
//!
//! two1nine
//! eightwothree
//! abcone2threexyz
//! xtwone3four
//! 4nineeightseven2
//! zoneight234
//! 7pqrstsixteen
//!
//! In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
//! Adding these together produces 281.
//!
//! What is the sum of all of the calibration values?

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub fn run(example: bool) {
    let part1_data = if example {
        include_str!("../data/example/day1.1.txt")
    } else {
        include_str!("../data/day1.1.txt")
    };
    let part2_data = if example {
        include_str!("../data/example/day1.2.txt")
    } else {
        // same data
        include_str!("../data/day1.1.txt")
    };

    tracing::info!("day 1 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 1 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(part1_data)
    );
    tracing::info!("day 1 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 1 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(part2_data)
    );
}

fn part1(data: &str) -> i64 {
    let mut sum = 0;

    for line in data.lines() {
        let digits = line
            .graphemes(true)
            .filter_map(|char| {
                if char.len() != 1 {
                    None
                } else {
                    char.parse::<i64>().ok()
                }
            })
            .collect::<Vec<i64>>();

        let term = match &digits[..] {
            [tens, .., ones] => (tens * 10) + ones,
            [tens_ones] => (tens_ones * 10) + tens_ones,
            _ => 0,
        };

        tracing::debug!("{} + {:?} {} = {} {}", sum, digits, term, sum + term, line);
        sum += term;
    }

    sum
}

fn part2(data: &str) -> i64 {
    let mut sum = 0;

    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let word_values = words
        .iter()
        .enumerate()
        .map(|(i, &word)| (word, i + 1))
        .collect::<HashMap<_, _>>();

    for line in data.lines() {
        let mut ptr = &line[..];
        let mut tens = None::<i64>;
        let mut ones = None::<i64>;

        while ptr != "" {
            tracing::trace!("ptr={}", ptr);

            if let Some((_, &value)) = word_values.iter().find(|(&word, _)| ptr.starts_with(word)) {
                if tens.is_none() {
                    tens = Some(value as i64);
                } else {
                    ones = Some(value as i64);
                }
            } else if let Some(Some(number)) = ptr.graphemes(true).take(1).next().map(|char| {
                if char.len() != 1 {
                    None
                } else {
                    char.parse::<i64>().ok()
                }
            }) {
                if tens.is_none() {
                    tens = Some(number);
                } else {
                    ones = Some(number);
                }
            }

            // don't advance by the full length of the word to handle cases
            // like "sevenine", which should count as 7 and 9
            ptr = &ptr[1..];

            tracing::trace!("tens={:?} ones={:?}", tens, ones);
        }

        let term = match (tens, ones) {
            (Some(tens), Some(ones)) => (tens * 10) + ones,
            (Some(tens_ones), None) => (tens_ones * 10) + tens_ones,
            _ => 0,
        } as i64;

        tracing::debug!("{} + {} = {} {}", sum, term, sum + term, line);

        sum += term;
    }

    sum
}
