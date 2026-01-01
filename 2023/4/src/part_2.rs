use crate::utils::Card;
use common::errors::AocError;
use common::input::InputReader;
use std::str::FromStr;

pub fn solve(filename: &str) -> Result<u32, AocError> {
    let lines = InputReader::as_lines(filename)?;

    let mut cards: Vec<Card> = lines
        .iter()
        .map(|line| Card::from_str(line).unwrap())
        .collect::<Vec<_>>();

    process_cards(&mut cards, 0);

    let total_cards: u32 = cards.iter().map(|card| u32::from(card.copies)).sum();
    Ok(total_cards)
}

fn process_cards(cards: &mut [Card], card_idx: usize) {
    if card_idx >= cards.len() {
        return;
    }

    let matches = cards[card_idx].count_matches();
    let copies = cards[card_idx].copies;
    println!("{:?}", cards[card_idx]);

    for i in 1..=matches as usize {
        if card_idx + i < cards.len() {
            cards[card_idx + i].add_copies(copies);
        }
    }
    process_cards(cards, card_idx + 1);
}
