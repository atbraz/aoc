use crate::utils::Card;
use aoc_common_rust::errors::AocError;
use aoc_common_rust::input::InputReader;
use std::str::FromStr;

pub fn solve(filename: &str) -> Result<u32, AocError> {
    let mut count = 0;
    let lines = InputReader::as_lines(filename)?;

    for (i, line) in lines.iter().enumerate() {
        count += 1;
        let card: Card = Card::from_str(line)?;

        println!("{}", card.count_matches());
    }

    Ok(count)
}
