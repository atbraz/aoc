use aoc_common_rust::{errors::AocError, input::InputReader};

pub fn parse_input(filename: &str) -> Result<(Vec<u32>, Vec<u32>), AocError> {
    let lines = InputReader::as_lines(&filename)?;
    let times = parse_line(&lines[0], "Time")?;
    let distances = parse_line(&lines[1], "Distance")?;
    Ok((times, distances))
}

fn parse_line(line: &str, prefix: &str) -> Result<Vec<u32>, AocError> {
    return line
        .strip_prefix(&format!("{}: ", prefix))
        .ok_or_else(|| AocError::from(format!("Invalid '{prefix}' Section")))?
        .split_whitespace()
        .map(|n| n.parse().map_err(AocError::from))
        .collect();
}

fn calc_distance(held_time: &u32, max_time: &u32) -> u32 {
    return (max_time - held_time) * held_time;
}

pub(crate) fn get_time_distances(max_time: &u32) -> Vec<(u32, u32)> {
    return (0..=*max_time)
        .map(|held_time| (held_time, calc_distance(&held_time, &max_time)))
        .collect();
}

pub(crate) fn get_winners(time_distances: &Vec<(u32, u32)>, distance_to_beat: &u32) -> Vec<u32> {
    let mut winners: Vec<u32> = Vec::new();
    for (time, distance) in time_distances {
        if distance > distance_to_beat {
            winners.push(*time);
        }
    }
    return winners;
}

fn calc_distance_u64(held_time: &u64, max_time: &u64) -> u64 {
    return (max_time - held_time) * held_time;
}

pub(crate) fn get_time_distances_u64(max_time: &u64) -> Vec<(u64, u64)> {
    return (0..=*max_time)
        .map(|held_time| (held_time, calc_distance_u64(&held_time, &max_time)))
        .collect();
}

pub(crate) fn get_winners_u64(time_distances: &Vec<(u64, u64)>, distance_to_beat: &u64) -> Vec<u64> {
    let mut winners: Vec<u64> = Vec::new();
    for (time, distance) in time_distances {
        if distance > distance_to_beat {
            winners.push(*time);
        }
    }
    return winners;
}
