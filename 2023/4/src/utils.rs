#[must_use]
pub(crate) fn parse_numbers(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
