use aoc_common_rust::errors::AocError;

use crate::utils::{parse_input, CategoryMap};

fn find_lowest_location(seeds: &[u64], maps: &[CategoryMap]) -> u64 {
    let mut lowest_location = u64::MAX;

    for &seed in seeds {
        let mut current = seed;
        for map in maps {
            current = map.convert(current);
        }
        lowest_location = lowest_location.min(current);
    }
    lowest_location
}

pub fn solve(filename: &str) -> Result<u32, AocError> {
    let (seeds, maps) = parse_input(filename)?;
    Ok(u32::try_from(find_lowest_location(&seeds, &maps))
        .ok()
        .expect("Lowest value is larger than u32"))
}
