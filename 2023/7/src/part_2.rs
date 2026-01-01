use common::errors::AocError;

use crate::utils;

pub fn solve(filename: &str) -> Result<u32, AocError> {
    let hands = utils::get_hands(filename, true).unwrap();

    let mut sorted_hands: Vec<&(utils::Hand, u32)> = hands.iter().collect();
    sorted_hands.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(42)
}
