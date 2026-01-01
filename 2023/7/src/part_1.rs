use crate::utils;
use common::errors::AocError;

pub fn solve(filename: &str) -> Result<u32, AocError> {
    let hands = utils::get_hands(filename, false).unwrap();

    let mut sorted_hands: Vec<&(utils::Hand, u32)> = hands.iter().collect();
    sorted_hands.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(sorted_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + bid * (i as u32 + 1)))
}
