use crate::utils::{self, get_time_distances_u64, get_winners_u64};
use common::errors::AocError;

pub fn solve(filename: &str) -> Result<u32, AocError> {
    let (_time, _distance) = utils::parse_input(&filename)?;
    let (time, distance) = (unkern(&_time), unkern(&_distance));
    println!("time: {time:?}\ndistance: {distance:?}");

    let all_results: Vec<Vec<(u64, u64)>> = vec![get_time_distances_u64(&time)];

    let winners: Vec<Vec<u64>> = all_results
        .iter()
        .map(|time_distances| get_winners_u64(time_distances, &distance))
        .collect();

    let winning_count: Vec<u32> = winners
        .iter()
        .map(|winning_times| winning_times.iter().count().try_into().unwrap())
        .collect();
    println!("\nwinning_count: {winning_count:?}");

    Ok(winning_count[0])
}

fn unkern(v: &Vec<u32>) -> u64 {
    return v.iter().fold(0u64, |acc, n| {
        (acc * 10u64.pow(n.checked_ilog10().unwrap_or(0) + 1)) + (*n as u64)
    });
}
