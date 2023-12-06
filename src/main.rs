macro_rules! main {
    ($($day_nums:tt),*) => { paste::paste! {
        $(mod [< day $day_nums >];)*

        fn main() {
            tracing_subscriber::fmt::init();

            let args = std::env::args().collect::<Vec<_>>();
            let example_arg = args.iter().any(|arg| arg == "--example");
            let day_args = args.iter().filter_map(|arg| arg.parse::<u8>().ok()).collect::<Vec<_>>();
            let day_nums = vec![$(stringify!($day_nums).parse::<u8>().unwrap()),*];

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

            $(if day_args.contains(&$day_nums) {
                [< day $day_nums >]::run(example_arg);
            })*

            if day_args.is_empty() {
                $([< day $day_nums >]::run(example_arg);)*
            }
        }
    } };
}

main!(1, 2, 3, 4, 5);
