use crate::utils::parse_numbers;
use aoc_common_rust::input::{InputError, InputReader};

pub fn solve(filename: &str) -> Result<i32, InputError> {
    let lines = InputReader::as_lines(filename)?;

    let mut sum = 0;

    for line in lines {
        let numbers_part = line.split(':').nth(1).expect("Invalid card format");

        let (winning_part, given_part) = numbers_part
            .split_once('|')
            .expect("Missing separator between number sets");

        let (winning_numbers, given_numbers): (Vec<i32>, Vec<i32>) =
            (parse_numbers(winning_part), parse_numbers(given_part));

        let matches = winning_numbers
            .iter()
            .filter(|num| given_numbers.contains(num))
            .count();

        if matches > 0 {
            sum += 2_i32.pow(matches as u32 - 1);
        }
    }
    Ok(sum)
}
