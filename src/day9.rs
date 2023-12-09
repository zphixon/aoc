pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day9.1.txt")
    } else {
        include_str!("../data/day9.1.txt")
    };

    tracing::info!("day 9 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 9 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data)
    );
    tracing::info!("day 9 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 9 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data)
    );
}

fn parse(data: &str) -> Vec<i64> {
    data.split_whitespace()
        .map(|number| number.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

fn extrap_predict(mut line: Vec<i64>) -> (i64, i64) {
    tracing::debug!("{:?}", line);

    let mut starts = Vec::new();
    let mut ends = Vec::new();

    let mut start = 0;
    while start < line.len() {
        starts.push(line[start]);

        let mut current = start + 1;
        let mut prev = line[start];
        let mut all_zero = true;

        while current < line.len() {
            let diff = line[current] - prev;
            if diff != 0 {
                all_zero = false;
            }
            prev = line[current];
            line[current] = diff;
            current += 1;
        }

        tracing::trace!("{:?}", &line[start + 1..]);

        ends.push(prev);
        start += 1;

        if all_zero {
            break;
        }
    }

    tracing::trace!("starts {:?}", starts);
    tracing::trace!("ends {:?}", ends);

    let mut extrapolated = 0;
    for start in starts.iter().rev() {
        extrapolated = start - extrapolated;
    }

    let mut predicted = 0;
    for end in ends.iter() {
        predicted += end;
    }

    (extrapolated, predicted)
}

fn part1(data: &str) -> i64 {
    let mut sum = 0;

    for line in data.lines() {
        let line = line.trim();
        if line == "" {
            continue;
        }

        let (_, prediction) = extrap_predict(parse(line));
        tracing::trace!("predicted {}", prediction);

        sum += prediction;
    }

    sum
}

fn part2(data: &str) -> i64 {
    let mut sum = 0;

    for line in data.lines() {
        let line = line.trim();
        if line == "" {
            continue;
        }

        let (extrap, _) = extrap_predict(parse(line));
        tracing::trace!("extrapolated {}", extrap);

        sum += extrap;
    }

    sum
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day9.1.txt")), 1972648895);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day9.1.txt")), 919);
}
