use std::{fmt::Debug, hash::Hash, str::FromStr};

pub fn run(example: bool) {
    // same data
    let part1_data = if example {
        include_str!("../data/example/day4.1.txt")
    } else {
        include_str!("../data/day4.1.txt")
    };

    tracing::info!("day 4 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 4 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(part1_data),
    );
    tracing::info!("day 4 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 4 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(part1_data),
    );
}

struct Card {
    number: usize,
    winners: Vec<usize>,
    scratched: Vec<usize>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [card_number, numbers] = s.split(": ").collect::<Vec<_>>()[..] else {
            tracing::error!("didn't split card on ':' {}", s);
            return Err(());
        };

        if !card_number.starts_with("Card ") {
            tracing::error!("couldn't get number from {}", card_number);
            return Err(());
        }

        let Ok(number) = card_number["Card ".len()..].trim().parse() else {
            tracing::error!(
                "couldn't parse card number {:?}",
                &card_number["Card ".len()..]
            );
            return Err(());
        };

        let [winning_numbers, my_numbers] = numbers.split(" | ").collect::<Vec<_>>()[..] else {
            tracing::error!("couldn't split on '|' {}", numbers);
            return Err(());
        };

        let Ok(winners) = winning_numbers
            .trim()
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
        else {
            tracing::error!("couldn't parse winners {:?}", winning_numbers);
            return Err(());
        };
        let Ok(scratched) = my_numbers
            .trim()
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
        else {
            tracing::error!("couldn't parse scratched {:?}", my_numbers);
            return Err(());
        };

        Ok(Card {
            number,
            winners,
            scratched,
        })
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{} win={:?} got={:?}",
            self.number, self.winners, self.scratched
        )
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for Card {}

impl Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

fn part1(data: &str) -> usize {
    let mut sum = 0;

    for line in data.lines() {
        let card: Card = line.parse().unwrap();
        let mut score = 0;
        for scratched in card.scratched.iter() {
            if card.winners.contains(scratched) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
                tracing::trace!("{:?} {} -> {}", card, scratched, score);
            }
        }
        tracing::debug!("{:?} -> {} + {} = {}", card, score, sum, sum + score);
        sum += score;
    }

    sum
}

fn part2(data: &str) -> usize {
    let cards = data
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut card_nums = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let mut score = 0;
        for scratched in card.scratched.iter() {
            if card.winners.contains(&scratched) {
                score += 1;
                tracing::trace!("{:?} {} -> {}", card, scratched, score);
            }
        }

        tracing::debug!("{:?} scored {}", card, score);

        if score == 0 {
            continue;
        }

        let range = card.number..(card.number + score).clamp(0, cards.len());
        let my_num = card_nums[i];

        tracing::debug!("add {} to {:?}", my_num, range);
        for card_num in &mut card_nums[range.clone()] {
            *card_num += my_num;
        }
        tracing::trace!("nums: {:?}", card_nums);
    }

    card_nums.iter().sum()
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day4.1.txt")), 22897);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day4.1.txt")), 5095824);
}
