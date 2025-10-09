use std::collections::HashMap;

use aoc_common_rust::{errors::AocError, input::InputReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub(crate) enum Card {
    Two = b'2',
    Three = b'3',
    Four = b'4',
    Five = b'5',
    Six = b'6',
    Seven = b'7',
    Eight = b'8',
    Nine = b'9',
    Ten = b'T',
    Jack = b'J',
    Queen = b'Q',
    King = b'K',
    Ace = b'A',
    Unknown = 0,
}

impl Card {
    fn index(&self) -> u8 {
        match *self {
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::Ten => 9,
            Card::Jack => 10,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13,
            Card::Unknown => 0,
        }
    }
    fn index_joker(&self) -> u8 {
        match *self {
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::Ten => 9,
            Card::Jack => 0,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13,
            Card::Unknown => 0,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => Card::Unknown,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index().cmp(&other.index())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn index(&self) -> u8 {
        match *self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index().cmp(&other.index())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Hand {
    cards: [Card; 5],
    joker: bool,
}

impl Hand {
    pub(crate) fn hand_type(&self) -> HandType {
        let mut counts: HashMap<Card, usize> = HashMap::new();
        for &card in self.cards.iter() {
            *counts.entry(card).or_insert(0) += 1
        }

        let mut sorted_counts: Vec<(&Card, &usize)> = counts.iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(a.1).then(b.0.cmp(a.0)));

        let max = sorted_counts[0].1;
        let second_max = if sorted_counts.len() > 1 {
            sorted_counts[1].1
        } else {
            &0_usize
        };

        if self.joker && self.cards.contains(&Card::Jack) {
            println!("found joker on hand {:#?}", self.cards);
            // fuck it we brute force
        }

        return match *max {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if *second_max == 2 => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 if *second_max == 2 => HandType::TwoPair,
            2 => HandType::OnePair,
            _ => HandType::HighCard,
        };
    }

    fn calc_joker_hand(&self) {}
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type() == other.hand_type() {
            let card_it = self.cards.iter().clone().enumerate();
            for (i, card) in card_it {
                if *card != other.cards[i] {
                    return card.cmp(&other.cards[i]);
                }
            }
            std::cmp::Ordering::Equal
        } else {
            self.hand_type().cmp(&other.hand_type())
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub(crate) fn get_hands(filename: &str, joker: bool) -> Result<Vec<(Hand, u32)>, AocError> {
    let lines = InputReader::as_lines(&filename)?;
    let hands: Vec<(Hand, u32)> = lines
        .iter()
        .map(|line| parse_line(line, joker).unwrap())
        .collect();
    Ok(hands)
}

fn parse_line(line: &str, joker: bool) -> Result<(Hand, u32), AocError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let cards: Vec<Card> = parts[0].chars().map(|char| Card::from(char)).collect();
    let hand = Hand {
        cards: cards.try_into().unwrap(),
        joker,
    };
    let bid: u32 = parts[1].parse().unwrap();

    Ok((hand, bid))
}
