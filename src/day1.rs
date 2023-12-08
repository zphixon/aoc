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

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day1.1.txt")), 54573);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day1.1.txt")), 54591);
}
