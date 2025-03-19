use crate::utils;
use aoc_common_rust::errors::AocError;

pub fn solve(filename: &str) -> Result<u32, AocError> {
    utils::parse_input(&filename);
    Ok(42)
}
