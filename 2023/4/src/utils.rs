use std::str::FromStr;

use aoc_common_rust::errors::AocError;

#[must_use]
pub(crate) fn parse_numbers(input: &str) -> Vec<u8> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn vec_to_array<const N: usize>(v: Vec<u8>) -> Result<[u8; N], String> {
    if v.len() != N {
        return Err(format!("Expected {}, got {}", N, v.len()));
    }

    let array: [u8; N] = v
        .try_into()
        .map_err(|_| "Failed to convert vector to array".to_string())?;
    Ok(array)
}

pub(crate) fn parse_to_fixed_array<const N: usize>(input: &str) -> [u8; N] {
    let mut result = [0u8; N];
    let parsed: Vec<u8> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    for (i, &num) in parsed.iter().enumerate().take(N) {
        result[i] = num;
    }
    result
}

#[derive(Debug, Clone)]
pub(crate) struct Card {
    id: u8,
    winning: [u8; 10],
    attempt: [u8; 25],
    pub(crate) copies: u8,
}

impl Card {
    fn new(id: u8, winning: [u8; 10], attempt: [u8; 25]) -> Self {
        Self {
            id,
            winning,
            attempt,
            copies: 1,
        }
    }

    pub fn count_matches(&self) -> u8 {
        self.winning
            .iter()
            .filter(|num| self.attempt.contains(num))
            .count()
            .try_into()
            .unwrap()
    }

    pub fn points(&self) -> u32 {
        let matches = self.count_matches();
        if matches > 0 {
            2_u32.pow(u32::from(matches - 1))
        } else {
            0
        }
    }

    pub(crate) fn add_copies(&mut self, n: u8) {
        self.copies += n;
    }
}

impl FromStr for Card {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');

        let id_part = parts
            .next()
            .ok_or_else(|| AocError::from("Missing ID part"))?;
        let id: u8 = id_part
            .trim()
            .strip_prefix("Card")
            .ok_or_else(|| AocError::from("Invalid card format, missing 'Card' prefix"))?
            .trim()
            .parse()
            .map_err(|_| AocError::from("Failed to parse card ID"))?;

        let number_part = parts
            .next()
            .ok_or_else(|| AocError::from("Missing numbers part"))?;
        let mut number_parts = number_part.split('|');

        let winning_part = number_parts
            .next()
            .ok_or_else(|| AocError::from("Missing winning numbers"))?;
        let attempt_part = number_parts
            .next()
            .ok_or_else(|| AocError::from("Missing player attempt numbers"))?;

        let winning_numbers = parse_to_fixed_array::<10>(winning_part);
        let attempt_numbers = parse_to_fixed_array::<25>(attempt_part);

        Ok(Card::new(id, winning_numbers, attempt_numbers))
    }
}
