use std::{cmp::Ordering, fmt::Debug, str::FromStr};
use unicode_segmentation::UnicodeSegmentation;

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day7.1.txt")
    } else {
        include_str!("../data/day7.1.txt")
    };

    tracing::info!("day 7 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 7 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data)
    );
    tracing::info!("day 7 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 7 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data)
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
}

impl Card {
    fn compare(&self, other: &Card, jokers: bool) -> Ordering {
        if jokers {
            // if jacks are jokers, they are the least valuable
            match (self, other) {
                (&Card::J, &Card::J) => Ordering::Equal,
                (&Card::J, _) => Ordering::Greater,
                (_, &Card::J) => Ordering::Less,
                (_, _) => self.cmp(other),
            }
        } else {
            self.cmp(other)
        }
    }
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            "9" => Ok(Card::_9),
            "8" => Ok(Card::_8),
            "7" => Ok(Card::_7),
            "6" => Ok(Card::_6),
            "5" => Ok(Card::_5),
            "4" => Ok(Card::_4),
            "3" => Ok(Card::_3),
            "2" => Ok(Card::_2),
            _ => Err(()),
        }
    }
}

struct Hand {
    type_: HandType,
    cards: [Card; 5],
    wager: usize,
    jokers: bool,
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {}", self.type_, self.cards, self.wager)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap().is_eq()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.jokers != other.jokers {
            return None;
        }

        if self.type_ != other.type_ {
            return self.type_.partial_cmp(&other.type_);
        }

        for (my_card, their_card) in self.cards.iter().zip(other.cards.iter()) {
            if my_card != their_card {
                // don't use partial_cmp, to allow for jokers rule
                return Some(my_card.compare(their_card, self.jokers));
            }
        }

        Some(Ordering::Equal)
    }
}

impl Eq for Hand {}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    fn from_str(jokers: bool, s: &str) -> Result<Self, ()> {
        let [cards, wager] = s.split(" ").collect::<Vec<_>>()[..] else {
            tracing::error!("did not split on space {:?}", s);
            return Err(());
        };

        let Ok(cards) = cards
            .graphemes(true)
            .map(Card::from_str)
            .collect::<Result<Vec<_>, _>>()
        else {
            tracing::error!("could not parse cards {:?}", cards);
            return Err(());
        };

        if cards.len() != 5 {
            tracing::error!("more than five cards in hand {:?}", cards);
            return Err(());
        }

        let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];

        // initial most frequent is the first non-joker card
        let mut most_freq = cards
            .iter()
            .find(|&card| *card != Card::J)
            .copied()
            .unwrap_or(Card::J);

        // determine frequency of cards
        let mut freq = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for card in cards.iter() {
            freq[*card as usize] += 1;
            if freq[*card as usize] > freq[most_freq as usize] && *card != Card::J {
                most_freq = *card;
            }
        }
        let num_jokers = freq[Card::J as usize];
        if jokers && num_jokers > 0 {
            // add number of jokers to most frequent card, if joker rule is active
            freq[Card::J as usize] = 0;
            freq[most_freq as usize] += num_jokers;
            tracing::trace!("jokers in play - adding {} to {:?}", num_jokers, most_freq);
        }

        // determine the type based on card frequencies
        let mut type_ = HandType::HighCard;

        let mut num_pairs = 0;
        let mut saw_triplet = false;
        for freq in freq {
            if freq == 5 {
                // we could not have seen any pairs or triplets, nothing will
                // happen in the match block
                type_ = HandType::FiveOfAKind;
                break;
            }
            if freq == 4 {
                // ditto
                type_ = HandType::FourOfAKind;
                break;
            }
            if freq == 3 {
                saw_triplet = true;
            }
            if freq == 2 {
                num_pairs += 1;
            }
        }
        match (num_pairs, saw_triplet) {
            (1, false) => type_ = HandType::OnePair,
            (2, false) => type_ = HandType::TwoPair,
            (0, true) => type_ = HandType::ThreeOfAKind,
            (1, true) => type_ = HandType::FullHouse,
            _ => {}
        }

        let Ok(wager) = wager.parse() else {
            tracing::error!("not a number: {:?}", wager);
            return Err(());
        };

        tracing::trace!(
            "{} pairs {} triplet {:?}",
            num_pairs,
            if saw_triplet { "yes" } else { "no" },
            freq,
        );

        Ok(Hand {
            type_,
            cards,
            wager,
            jokers,
        })
    }
}

fn sum_hands(joker: bool, data: &str) -> usize {
    let mut hands = data
        .lines()
        .map(|hand| {
            let hand = Hand::from_str(joker, hand).unwrap();
            tracing::debug!("{:?}", hand);
            hand
        })
        .collect::<Vec<_>>();

    hands.sort_unstable();

    let mut sum = 0;
    for (rank, hand) in hands.iter().rev().enumerate() {
        let rank = rank + 1;
        let score = hand.wager * rank;
        tracing::debug!("rank {} scores {} {:?}", rank, score, hand);
        sum += score;
    }

    sum
}

fn part1(data: &str) -> usize {
    sum_hands(false, data)
}

fn part2(data: &str) -> usize {
    sum_hands(true, data)
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day7.1.txt")), 245794640);
}
#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day7.1.txt")), 247899149);
}

#[test]
fn compares_correctly() {
    assert_eq!(Card::J.compare(&Card::J, false), Ordering::Equal);
    assert_eq!(Card::J.compare(&Card::J, true), Ordering::Equal);
    // yes it looks wrong
    assert_eq!(Card::J.compare(&Card::_2, true), Ordering::Greater);
}
