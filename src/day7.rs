//! --- Day 7: Camel Cards ---
//!
//! Your all-expenses-paid trip turns out to be a one-way, five-minute ride in
//! an airship. (At least it's a cool airship!) It drops you off at the edge of
//! a vast desert and descends back to Island Island.
//!
//! "Did you bring the parts?"
//!
//! You turn around to see an Elf completely covered in white clothing, wearing
//! goggles, and riding a large camel.
//!
//! "Did you bring the parts?" she asks again, louder this time. You aren't sure
//! what parts she's looking for; you're here to figure out why the sand
//! stopped.
//!
//! "The parts! For the sand, yes! Come with me; I will show you." She beckons
//! you onto the camel.
//!
//! After riding a bit across the sands of Desert Island, you can see what look
//! like very large rocks covering half of the horizon. The Elf explains that
//! the rocks are all along the part of Desert Island that is directly above
//! Island Island, making it hard to even get there. Normally, they use big
//! machines to move the rocks and filter the sand, but the machines have broken
//! down because Desert Island recently stopped receiving the parts they need to
//! fix the machines.
//!
//! You've already assumed it'll be your job to figure out why the parts stopped
//! when she asks if you can help. You agree automatically.
//!
//! Because the journey will take a few days, she offers to teach you the game
//! of Camel Cards. Camel Cards is sort of similar to poker except it's designed
//! to be easier to play while riding a camel.
//!
//! In Camel Cards, you get a list of hands, and your goal is to order them
//! based on the strength of each hand. A hand consists of five cards labeled
//! one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of
//! each card follows this order, where A is the highest and 2 is the lowest.
//!
//! Every hand is exactly one type. From strongest to weakest, they are:
//!
//! - Five of a kind, where all five cards have the same label: AAAAA
//! - Four of a kind, where four cards have the same label and one card has a
//!   different label: AA8AA
//! - Full house, where three cards have the same label, and the remaining two
//!   cards share a different label: 23332
//! - Three of a kind, where three cards have the same label, and the remaining
//!   two cards are each different from any other card in the hand: TTT98
//! - Two pair, where two cards share one label, two other cards share a second
//!   label, and the remaining card has a third label: 23432
//! - One pair, where two cards share one label, and the other three cards have
//!   a different label from the pair and each other: A23A4
//! - High card, where all cards' labels are distinct: 23456
//!
//! Hands are primarily ordered based on type; for example, every full house is
//! stronger than any three of a kind.
//!
//! If two hands have the same type, a second ordering rule takes effect. Start
//! by comparing the first card in each hand. If these cards are different, the
//! hand with the stronger first card is considered stronger. If the first card
//! in each hand have the same label, however, then move on to considering the
//! second card in each hand. If they differ, the hand with the higher second
//! card wins; otherwise, continue with the third card in each hand, then the
//! fourth, then the fifth.
//!
//! So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger
//! because its first card is stronger. Similarly, 77888 and 77788 are both a
//! full house, but 77888 is stronger because its third card is stronger (and
//! both hands have the same first and second card).
//!
//! To play Camel Cards, you are given a list of hands and their corresponding
//! bid (your puzzle input). For example:
//!
//! 32T3K 765
//! T55J5 684
//! KK677 28
//! KTJJT 220
//! QQQJA 483
//!
//! This example shows five hands; each hand is followed by its bid amount. Each
//! hand wins an amount equal to its bid multiplied by its rank, where the
//! weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up
//! to the strongest hand. Because there are five hands in this example, the
//! strongest hand will have rank 5 and its bid will be multiplied by 5.
//!
//! So, the first step is to put the hands in order of strength:
//!
//! - 32T3K is the only one pair and the other hands are all a stronger type,
//!   so it gets rank 1.
//! - KK677 and KTJJT are both two pair. Their first cards both have the same
//!   label, but the second card of KK677 is stronger (K vs T), so KTJJT gets
//!   rank 2 and KK677 gets rank 3.
//! - T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first
//!   card, so it gets rank 5 and T55J5 gets rank 4.
//!
//! Now, you can determine the total winnings of this set of hands by adding up
//! the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 +
//! 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.
//!
//! Find the rank of every hand in your set. What are the total winnings?
//!
//!
//! --- Part Two ---
//!
//! To make things a little more interesting, the Elf introduces one additional
//! rule. Now, J cards are jokers - wildcards that can act like whatever card
//! would make the hand the strongest type possible.
//!
//! To balance this, J cards are now the weakest individual cards, weaker even
//! than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5,
//! 4, 3, 2, J.
//!
//! J cards can pretend to be whatever card is best for the purpose of
//! determining hand type; for example, QJJQ2 is now considered four of a kind.
//! However, for the purpose of breaking ties between two hands of the same
//! type, J is always treated as J, not the card it's pretending to be: JKKK2 is
//! weaker than QQQQ2 because J is weaker than Q.
//!
//! Now, the above example goes very differently:
//!
//! 32T3K 765
//! T55J5 684
//! KK677 28
//! KTJJT 220
//! QQQJA 483
//!
//! - 32T3K is still the only one pair; it doesn't contain any jokers, so its
//!   strength doesn't increase.
//! - KK677 is now the only two pair, making it the second-weakest hand.
//! - T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3,
//!   QQQJA gets rank 4, and KTJJT gets rank 5.
//!
//! With the new joker rule, the total winnings in this example are 5905.
//!
//! Using the new joker rule, find the rank of every hand in your set. What are
//! the new total winnings?

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
