use crate::utils::{self, get_time_distances, get_winners};
use common::errors::AocError;

pub fn solve(filename: &str) -> Result<u32, AocError> {
    let (times, distances) = utils::parse_input(&filename)?;
    let all_results: Vec<Vec<(u32, u32)>> =
        times.iter().map(|time| get_time_distances(time)).collect();
    let winners: Vec<Vec<u32>> = all_results
        .iter()
        .enumerate()
        .map(|(i, time_distances)| get_winners(time_distances, &distances[i]))
        .collect();

    let winning_count: Vec<u32> = winners
        .iter()
        .map(|winning_times| winning_times.iter().count().try_into().unwrap())
        .collect();
    Ok(winning_count.iter().fold(1, |acc, x| acc * x))
}
