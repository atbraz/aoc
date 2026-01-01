use common::errors::AocError;

use crate::utils;

pub fn solve(filename: &str) -> Result<u64, AocError> {
    let (instructions, graph) = utils::parse_input(filename)?;
    Ok(utils::get_n_steps(instructions, graph, "AAA", "ZZZ").into())
}
