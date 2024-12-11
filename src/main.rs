use std::io::prelude::*;

pub mod util;

const TEMPLATE: &str = r#"
pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day{day}.1.txt")
    } else {
        include_str!("../data/day{day}.1.txt")
    };

    tracing::info!("day {day} part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day {day} part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day {day} part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day {day} part 2{} result: {}",
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
    assert_eq!(part1(include_str!("../data/day{day}.1.txt")), 0);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day{day}.1.txt")), 0);
}
"#;

macro_rules! main {
    ($($day_nums:tt),* $(,)?) => { paste::paste! {
        $(mod [< day $day_nums >];)*

        fn main() {
            tracing_subscriber::fmt::init();

            let args = std::env::args().collect::<Vec<_>>();
            let example_arg = args.iter().any(|arg| arg == "--example");
            let new_day_arg = args.iter().any(|arg| arg == "--new-day");

            let day_args = args.iter().filter_map(|arg| arg.parse::<u8>().ok()).collect::<Vec<_>>();
            let day_nums = vec![$(stringify!($day_nums).parse::<u8>().unwrap()),*];

            if new_day_arg {
                for day in day_args {
                    std::fs::write(format!("data/example/day{}.1.txt", day), "").unwrap();
                    std::fs::write(format!("src/day{}.rs", day), TEMPLATE.replace("{day}", &format!("{}", day))).unwrap();
                    let mut main_rs = std::fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(false)
                        .open("src/main.rs")
                        .unwrap();
                    let mut contents = String::new();
                    main_rs.read_to_string(&mut contents).unwrap();
                    main_rs.set_len(0).unwrap();
                    main_rs.seek(std::io::SeekFrom::Start(0)).unwrap();
                    let lol = "// new ";
                    let xd = "days here";
                    let lolxd = format!("{}{}", lol, xd);
                    main_rs.write_all(contents.replace(&lolxd, &format!("{},\n    {}", day, lolxd)).as_bytes()).unwrap();
                }
                return;
            }

            let mut not_a_day = false;
            for day_arg in day_args.iter() {
                if !day_nums.contains(day_arg) {
                    not_a_day = true;
                    tracing::error!("not a day: {}", day_arg);
                }
            }
            if not_a_day {
                std::process::exit(1);
            }

            if example_arg {
                tracing::warn!("using example data");
            } else {
                tracing::info!("using real data");
            }

            $(if day_args.contains(&$day_nums) {
                [< day $day_nums >]::run(example_arg);
                if example_arg {
                    tracing::warn!("used example data");
                }
            })*

            if day_args.is_empty() {
                $(
                    [< day $day_nums >]::run(example_arg);
                    if example_arg {
                        tracing::warn!("used example data");
                    }
                )*
            }
        }
    } };
}

main!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    11,
    // new days here
);
