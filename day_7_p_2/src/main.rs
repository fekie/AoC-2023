#![feature(is_sorted)]

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use strum_macros::EnumIter;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                // We compare the cards in order and choose whether
                // it's greater or less than when the cards are different.
                for (original_card, other_card) in self.cards.iter().zip(&other.cards) {
                    match original_card.cmp(other_card) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {}
                    }
                }

                // If that all fails (which it shouldn't with this input),
                // we just return [`Ordering::Equal`]
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(line: &str) -> Self {
        let split = line.split_whitespace().collect::<Vec<&str>>();

        // We parse the cards from the line and then clone them into an array
        // so that we know the size.
        let mut cards = [Card::A; 5];
        cards.clone_from_slice(
            &split[0]
                .chars()
                .filter_map(|character| Card::try_new(character).ok())
                .collect::<Vec<Card>>(),
        );

        let bid = split[1].parse().unwrap();

        let hand_type = HandType::calculate(&cards);

        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn calculate(cards: &[Card; 5]) -> Self {
        // A hashmap that stores the amount of times we have seen a card in this set.
        let mut cards_found: HashMap<Card, u64> = HashMap::new();

        for card in cards {
            let updated_amount = cards_found.get(card).unwrap_or(&0) + 1;
            cards_found.insert(*card, updated_amount);
        }

        // We go and take out the Jokers, and count them towards the highest
        // card amount that we currently have. This works because even something like
        // AQQQJ would rather go to a 4-of-a-kind vs a full house, and a 77J45 would rather
        // be a 3-of-a-kind than a two pair.
        let joker_count = cards_found.remove(&Card::J).unwrap_or(0);

        // We check for the next cases in order of importance. We also add
        // the joker count to the highest match, as it will act towards the
        // most common card. It does not matter if the card is the highest or
        // not as we are just calculating what the hand type is. Ties are broken
        // in the `Ord` impl. We `unwrap_or_default()`` here so that we can handle
        // the case of 5 Jokers.
        let highest_match_amount =
            cards_found.values().max().cloned().unwrap_or_default() + joker_count;
        let second_highest_match_amount = cards_found
            .values()
            .cloned()
            .sorted()
            .rev()
            .nth(1)
            .unwrap_or_default();

        if highest_match_amount == 5 {
            Self::FiveOfAKind
        } else if highest_match_amount == 4 {
            Self::FourOfAKind
        } else if highest_match_amount == 3 && second_highest_match_amount == 2 {
            Self::FullHouse
        } else if highest_match_amount == 3 {
            Self::ThreeOfAKind
        } else if highest_match_amount == 2 && second_highest_match_amount == 2 {
            Self::TwoPair
        } else if highest_match_amount == 2 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

// Deriving Ord means that the order is defined by how high the variant is listed.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Q,
    K,
    A,
}

impl Card {
    fn try_new(character: char) -> Result<Self, ()> {
        match character {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(()),
        }
    }
}

fn main() {
    // We parse and sort the hands by strength.
    let hands = INPUT.lines().map(Hand::new).sorted().collect::<Vec<Hand>>();

    let sum_of_hand_scores = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as u64)
        .sum::<u64>();

    println!("Sum of Hand Scores: {sum_of_hand_scores}");
}
