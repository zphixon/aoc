use std::collections::HashMap;

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
    tracing::info!("day 2 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 2 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl std::str::FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Color, ()> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    phases: Vec<HashMap<Color, usize>>,
}

impl std::str::FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Game, ()> {
        let [game_id, records_str] = s.split(": ").collect::<Vec<&str>>()[..] else {
            tracing::error!("didn't split on colon: {:?}", s);
            return Err(());
        };

        let Ok(id) = game_id["Game ".len()..].parse::<usize>() else {
            tracing::error!("didn't parse game id: {:?}", game_id);
            return Err(());
        };

        let mut phases = Vec::new();
        for phase_str in records_str.split(";") {
            let phase_str = phase_str.trim();
            let mut phase = HashMap::new();
            for draw in phase_str.split(",") {
                let draw = draw.trim();
                let [num_str, color_str] = draw.split(" ").collect::<Vec<_>>()[..] else {
                    tracing::error!(
                        "could not split draw into number and color in game {}: {:?}",
                        id,
                        draw
                    );
                    return Err(());
                };

                let Ok(num) = num_str.parse() else {
                    tracing::error!("not a number in game {}: {:?}", id, num_str);
                    return Err(());
                };

                let Ok(color) = color_str.parse() else {
                    tracing::error!("not a color in game {}: {:?}", id, color_str);
                    return Err(());
                };

                phase.insert(color, num);
            }
            phases.push(phase);
        }

        Ok(Game { id, phases })
    }
}

fn part1(data: &str) -> usize {
    let num_red = 12;
    let num_green = 13;
    let num_blue = 14;

    let mut possible = 0;

    'next_game: for line in data.lines() {
        let game = line.parse::<Game>().unwrap();
        for phase in game.phases {
            if phase
                .get(&Color::Red)
                .map(|&red| red > num_red)
                .unwrap_or(false)
                || phase
                    .get(&Color::Green)
                    .map(|&green| green > num_green)
                    .unwrap_or(false)
                || phase
                    .get(&Color::Blue)
                    .map(|&blue| blue > num_blue)
                    .unwrap_or(false)
            {
                tracing::debug!("game {} impossible: {:?}", game.id, phase);
                continue 'next_game;
            }
        }
        possible += game.id;
        tracing::debug!("game {} possible {}", game.id, possible);
    }

    possible
}

fn part2(data: &str) -> usize {
    let mut sum = 0;

    for line in data.lines() {
        let game = line.parse::<Game>().unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for phase in game.phases {
            if let Some(&red) = phase.get(&Color::Red) {
                max_red = max_red.max(red);
            }
            if let Some(&green) = phase.get(&Color::Green) {
                max_green = max_green.max(green);
            }
            if let Some(&blue) = phase.get(&Color::Blue) {
                max_blue = max_blue.max(blue);
            }
        }

        let power = max_red * max_green * max_blue;
        sum += power;

        tracing::debug!("game {}: {} red, {} green, {} blue => {} power", game.id, max_red, max_green, max_blue, power);
    }

    sum
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day2.1.txt")), 2239);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day2.1.txt")), 83435);
}
