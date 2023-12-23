use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    hand_rank: HandRank,
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

        let hand_rank = HandRank::calculate(&cards);

        Self {
            cards,
            bid,
            hand_rank,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandRank {
    fn calculate(cards: &[Card; 5]) -> Self {
        // A hashmap that stores the amount of times we have seen a card in this set.
        let mut cards_found: HashMap<Card, u64> = HashMap::new();

        for card in cards {
            let updated_amount = cards_found.get(card).unwrap_or(&0) + 1;
            cards_found.insert(*card, updated_amount);
        }

        // We check for the next cases in order of importance.
        let highest_match_amount = *cards_found.values().max().unwrap();
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
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn try_new(character: char) -> Result<Self, ()> {
        match character {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
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
    let hands = INPUT.lines().map(Hand::new).collect::<Vec<Hand>>();

    dbg!(hands);
}
