use crate::util;

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day1.1.txt")
    } else {
        include_str!("../data/day1.1.txt")
    };

    tracing::info!("day 1 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 1 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 1 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 1 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

fn left_right(data: &str) -> Option<(Vec<u64>, Vec<u64>)> {

    let mut left = Vec::new();
    let mut right = Vec::new();

    for (line_num, line) in data.lines().enumerate() {
        let [left_str, right_str] = &line.split_whitespace().collect::<Vec<&str>>()[..] else {
            tracing::error!("more than two words on line {}: {:?}", line_num, line);
            return None;
        };

        let Ok(left_num) = left_str.parse::<u64>() else {
            tracing::error!("not a number on line {}: {:?}", line_num, left_str);
            return None;
        };
        let Ok(right_num) = right_str.parse::<u64>() else {
            tracing::error!("not a number on line {}: {:?}", line_num, right_str);
            return None;
        };

        left.push(left_num);
        right.push(right_num);
    }

    Some((left, right))
}

fn part1(data: &str) -> u64 {
    let Some((mut left, mut right)) = left_right(data) else {
        return 0;
    };

    left.sort();
    right.sort();

    let mut diff_sum = 0;
    for (l, r) in left.into_iter().zip(right.into_iter()) {
        diff_sum += l.max(r) - l.min(r);
    }

    diff_sum
}

fn part2(data: &str) -> u64 {
    let Some((left, right)) = left_right(data) else {
        return 0;
    };

    let mut similarity = 0;
    let right_freq = util::frequency(right.iter());

    for num in left.iter() {
        similarity += *num * right_freq.get(num).unwrap_or(&0);
    }

    similarity
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day1.1.txt")), 2176849);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day1.1.txt")), 23384288);
}
