use aoc_common_rust::errors::AocError;

use crate::utils;

pub fn solve(filename: &str) -> Result<i64, AocError> {
    let all_series = utils::parse_input(filename)?;
    return all_series
        .iter()
        .fold(Ok(0), |acc: Result<i64, AocError>, e| {
            let forecast = utils::backcast(e);
            Ok(acc.unwrap() + forecast)
        });
}
