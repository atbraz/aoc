use aoc_common_rust::{errors::AocError, input::InputReader};

pub fn parse_input(filename: &str) -> Result<Vec<u32>, AocError> {
    let lines = InputReader::as_lines(&filename)?;
    let times = parse_line(&lines[0], "Times");
    let distances = parse_line(&lines[1], "Distances");
    return times;
}

fn parse_line(line: &str, prefix: &str) -> Result<Vec<u32>, AocError> {
    return line
        .strip_prefix(format!("{prefix}: "))
        .ok_or(AocError::from(format!("Invalid '{prefix}' Section")))?
        .split_whitespace()
        .map(|n| n.parse())
        .collect();
}
