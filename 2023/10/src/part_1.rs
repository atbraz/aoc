use common::errors::AocError;

use crate::utils;

pub fn solve(filename: &str) -> Result<u64, AocError> {
    let grid = utils::parse_input(filename);
    println!("{grid:?}");
    Ok(42)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let result = solve("sample_1").unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sample_2() {
        let result = solve("sample_2").unwrap();
        assert_eq!(result, 8);
    }
}
