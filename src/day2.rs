pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day2.1.txt")
    } else {
        include_str!("../data/day2.1.txt")
    };

    tracing::info!("day 2 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 2 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 2 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 2 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

fn lines_numbers(data: &str) -> Option<Vec<Vec<u64>>> {
    let mut lines = Vec::new();

    for (line_num, line) in data.lines().enumerate() {
        let mut words = Vec::new();

        for word in line.split_whitespace() {
            let Ok(num) = word.parse() else {
                tracing::error!("not a number on line {}: {:?}", line_num, word);
                return None;
            };

            words.push(num);
        }

        lines.push(words);
    }

    Some(lines)
}

fn part1(data: &str) -> u64 {
    let Some(reports) = lines_numbers(data) else {
        return 0;
    };

    reports
        .iter()
        .filter(|report| {
            tracing::debug!("{:?}", report);
            is_safe(report, None)
        })
        .count() as u64
}

fn is_safe(report: &[u64], skip: Option<usize>) -> bool {
    let mut increasing: Option<bool> = None;
    let mut prev: Option<u64> = None;
    for (i, &level) in report.iter().enumerate() {
        if Some(i) == skip {
            continue;
        }

        if let Some(prev_level) = prev {
            let diff = prev_level.abs_diff(level);
            tracing::trace!("prev={:?} level={}", prev_level, level);
            if (1..=3).contains(&diff) {
                tracing::trace!("good difference");
                if prev_level > level {
                    tracing::debug!("decreasing");
                    if increasing.is_none() {
                        increasing = Some(false);
                    }
                    if increasing != Some(false) {
                        tracing::debug!("not safe - was increasing");
                        return false;
                    }
                } else {
                    tracing::debug!("increasing");
                    if increasing.is_none() {
                        increasing = Some(true);
                    }
                    if increasing != Some(true) {
                        tracing::debug!("not safe - was decreasing");
                        return false;
                    }
                }
            } else {
                tracing::trace!("bad difference");
                return false;
            }
        } else {
            tracing::debug!("first iteration");
        }

        prev = Some(level);
    }

    tracing::debug!("safe");
    true
}

fn part2(data: &str) -> u64 {
    let Some(reports) = lines_numbers(data) else {
        return 0;
    };

    let mut safe = 0;
    for report in reports {
        if is_safe(&report, None) {
            safe += 1;
        } else {
            'skips: for i in 0..report.len() {
                if is_safe(&report, Some(i)) {
                    safe += 1;
                    break 'skips;
                }
            }
        }
    }

    safe
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day2.1.txt")), 246);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day2.1.txt")), 318);
}
