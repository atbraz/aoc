use aoc_common_rust::errors::AocError;

use crate::utils;

pub fn solve(filename: &str) -> Result<u64, AocError> {
    let (instructions, graph) = utils::parse_input(filename)?;
    Ok(utils::get_n_steps_2_spooky_boogaloo(instructions, graph))
}
