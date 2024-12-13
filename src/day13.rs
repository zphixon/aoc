use crate::util;
use itertools::Itertools;
use std::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day13.1.txt")
    } else {
        include_str!("../data/day13.1.txt")
    };

    tracing::info!("day 13 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 13 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 13 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 13 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TaxiCoord {
    x: u64,
    y: u64,
}

impl TaxiCoord {
    fn zero() -> Self {
        TaxiCoord { x: 0, y: 0 }
    }

    fn slope(&self) -> f64 {
        self.y as f64 / self.x as f64
    }
}

impl Add for TaxiCoord {
    type Output = TaxiCoord;

    fn add(self, rhs: Self) -> Self::Output {
        TaxiCoord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for TaxiCoord {
    type Output = TaxiCoord;

    fn sub(self, rhs: Self) -> Self::Output {
        TaxiCoord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<u64> for TaxiCoord {
    type Output = TaxiCoord;

    fn mul(self, rhs: u64) -> Self::Output {
        TaxiCoord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug)]
struct Game {
    a: TaxiCoord,
    b: TaxiCoord,
    prize: TaxiCoord,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [a_line, b_line, prize_line] = s.lines().collect::<Vec<_>>()[..] else {
            return Err(String::from("more/less than 3 lines"));
        };

        let Some(a_coords) = a_line.split("A: ").nth(1) else {
            return Err(String::from("no coords for a"));
        };
        let Some(b_coords) = b_line.split("B: ").nth(1) else {
            return Err(String::from("no coords for b"));
        };
        let Some(prize_coords) = prize_line.split(": ").nth(1) else {
            return Err(String::from("no coords for prize"));
        };

        let [a_plus_x, a_plus_y] = a_coords.split(", ").collect::<Vec<_>>()[..] else {
            return Err(String::from("no x/y for a"));
        };
        let [b_plus_x, b_plus_y] = b_coords.split(", ").collect::<Vec<_>>()[..] else {
            return Err(String::from("no x/y for b"));
        };
        let [prize_x, prize_y] = prize_coords.split(", ").collect::<Vec<_>>()[..] else {
            return Err(String::from("no x/y for prize"));
        };

        let Some(a_x_str) = a_plus_x.split("+").nth(1) else {
            return Err(String::from("no num for a +x"));
        };
        let Some(a_y_str) = a_plus_y.split("+").nth(1) else {
            return Err(String::from("no num for a +y"));
        };
        let Some(b_x_str) = b_plus_x.split("+").nth(1) else {
            return Err(String::from("no num for b +x"));
        };
        let Some(b_y_str) = b_plus_y.split("+").nth(1) else {
            return Err(String::from("no num for b +y"));
        };
        let Some(prize_x_str) = prize_x.split("=").nth(1) else {
            return Err(String::from("no num for prize x"));
        };
        let Some(prize_y_str) = prize_y.split("=").nth(1) else {
            return Err(String::from("no num for prize y"));
        };

        let Ok(a_x) = a_x_str.parse() else {
            return Err(String::from("a x not a number"));
        };
        let Ok(a_y) = a_y_str.parse() else {
            return Err(String::from("a y not a number"));
        };
        let Ok(b_x) = b_x_str.parse() else {
            return Err(String::from("b x not a number"));
        };
        let Ok(b_y) = b_y_str.parse() else {
            return Err(String::from("b y not a number"));
        };
        let Ok(prize_x) = prize_x_str.parse() else {
            return Err(String::from("prize x not a number"));
        };
        let Ok(prize_y) = prize_y_str.parse() else {
            return Err(String::from("prize y not a number"));
        };

        Ok(Game {
            a: TaxiCoord { x: a_x, y: a_y },
            b: TaxiCoord { x: b_x, y: b_y },
            prize: TaxiCoord {
                x: prize_x,
                y: prize_y,
            },
        })
    }
}

fn parse(data: &str) -> Vec<Game> {
    data.split("\n\n")
        .map(Game::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn part1(data: &str) -> u64 {
    let games = parse(data);

    let mut tokens = 0;
    for game in games.iter() {
        tracing::debug!("{:?}", game);
        for (a_presses, b_presses) in (0..=100).cartesian_product(0..=100) {
            if game.a * a_presses + game.b * b_presses == game.prize {
                tracing::debug!("solution: {} a, {} b", a_presses, b_presses);
                tokens += a_presses * 3 + b_presses;
            }
        }
    }

    tokens
}

fn part2(data: &str) -> u64 {
    let mut games = parse(data);

    let mut tokens = 0;
    for game in games.iter_mut() {
        game.prize.x += 10_000_000_000_000;
        game.prize.y += 10_000_000_000_000;
        tracing::debug!("game={:?}", game);

        let (steep, shallow) = if game.a.slope() > game.b.slope() {
            (game.a, game.b)
        } else {
            (game.b, game.a)
        };
        let most_steep_presses = (game.prize.x / steep.x).min(game.prize.y / steep.y);

        tracing::trace!(
            "most_steep={} steep={:?} shallow={:?}",
            most_steep_presses,
            steep,
            shallow
        );

        let mut div = 2;
        let mut steep_presses = most_steep_presses;
        loop {
            tracing::trace!("---------------");
            tracing::trace!("tip_steep={:?}", steep * steep_presses);

            // does the prize lie on the line formed by steep*steep_presses + X*shallow_presses?
            let new_prize = game.prize - steep * steep_presses;
            tracing::trace!("new_prize={:?}", new_prize);

            if new_prize.x % shallow.x == 0
                && new_prize.y % shallow.y == 0
                && new_prize.x / shallow.x == new_prize.y / shallow.y
            {
                let shallow_presses = new_prize.y / shallow.y;
                tracing::debug!(
                    "solution: {} steep, {} shallow",
                    steep_presses,
                    shallow_presses
                );
                assert_eq!(
                    steep * steep_presses + shallow * shallow_presses,
                    game.prize
                );
                break;
            }

            if steep_presses == 0 {
                let shallow_presses = new_prize.y / shallow.y;
                tracing::debug!("no solution at {} steep, {} shallow", steep_presses, shallow_presses);
                break;
            }

            tracing::trace!("have y={}", new_prize.y);
            tracing::trace!("want y=mx={}", shallow.slope() * new_prize.x as f64);

            // y = mx + b
            if (new_prize.y as f64) < (shallow.slope() * new_prize.x as f64) {
                tracing::trace!("half");
                steep_presses /= 2;
            } else {
                if steep_presses / div > 0 {
                    tracing::trace!("plus 1/{}", div);
                    steep_presses += steep_presses / div;
                } else {
                    tracing::trace!("plus 1 (div too small)");
                }
            }
            if let Some(new_div) = div.checked_mul(2) {
                // div should be too small by now
                div = new_div;
            }
        }
    }

    tokens
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day13.1.txt")), 0);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day13.1.txt")), 0);
}
