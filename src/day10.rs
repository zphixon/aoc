
pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day10.1.txt")
    } else {
        include_str!("../data/day10.1.txt")
    };

    tracing::info!("day 10 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 10 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 10 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 10 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

fn part1(data: &str) -> u64 {
    0
}

fn part2(data: &str) -> u64 {
    0
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day10.1.txt")), 0);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day10.1.txt")), 0);
}
