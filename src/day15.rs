pub fn run(example: bool) {
    let data1 = if example {
        include_str!("../data/example/day15.1.txt")
    } else {
        include_str!("../data/day15.1.txt")
    };

    tracing::info!("day 15 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 15 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data1)
    );
    tracing::info!("day 15 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 15 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data1)
    );
}

fn part1(data: &str) -> usize {
    0
}

fn part2(data: &str) -> usize {
    0
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day15.1.txt")), 113424);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day15.1.txt")), 29846);
}
