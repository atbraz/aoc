use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Represents errors that can occur when reading and parsing input files
#[derive(Debug)]
pub enum InputError {
    IoError(io::Error),
    EmptyFile,
    InvalidFormat(String),
}

impl From<io::Error> for InputError {
    fn from(error: io::Error) -> Self {
        InputError::IoError(error)
    }
}

/// A struct that provides different ways to read and parse input files
pub struct InputReader;

impl InputReader {
    /// Reads an entire file into a single String, preserving newlines
    ///
    /// This is useful when you need to process the input as one chunk,
    /// like when dealing with multi-line patterns or complex formats.
    pub fn as_string(path: impl AsRef<Path>) -> Result<String, InputError> {
        let content = fs::read_to_string(path)?;
        if content.is_empty() {
            return Err(InputError::EmptyFile);
        }
        Ok(content)
    }

    /// Reads a file into a vector of strings, one per line
    ///
    /// This is the most common way to read AoC inputs, where each line
    /// represents a distinct piece of data.
    pub fn as_lines(path: impl AsRef<Path>) -> Result<Vec<String>, InputError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;

        if lines.is_empty() {
            return Err(InputError::EmptyFile);
        }
        Ok(lines)
    }

    /// Reads a file into a single string, removing all newlines
    ///
    /// This is useful for inputs that represent one continuous piece of data
    /// that happens to be split across lines for readability.
    pub fn as_single_line(path: impl AsRef<Path>) -> Result<String, InputError> {
        let content = Self::as_string(path)?;
        Ok(content.lines().collect::<String>())
    }

    /// Reads a file and splits it into paragraphs (groups of lines separated by blank lines)
    ///
    /// This is common in AoC where inputs sometimes contain groups of related data
    /// separated by empty lines.
    pub fn as_paragraphs(path: impl AsRef<Path>) -> Result<Vec<String>, InputError> {
        let content = Self::as_string(path)?;
        let paragraphs: Vec<String> = content
            .split("\n\n")
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();

        if paragraphs.is_empty() {
            return Err(InputError::EmptyFile);
        }
        Ok(paragraphs)
    }

    /// Reads a file into a grid of characters
    ///
    /// This is useful for problems involving 2D grids, mazes, or maps.
    pub fn as_char_grid(path: impl AsRef<Path>) -> Result<Vec<Vec<char>>, InputError> {
        let lines = Self::as_lines(path)?;
        let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

        // Validate that all rows have the same length
        if let Some(first_len) = grid.first().map(|row| row.len()) {
            if grid.iter().any(|row| row.len() != first_len) {
                return Err(InputError::InvalidFormat(
                    "Grid rows have inconsistent lengths".to_string(),
                ));
            }
        }

        Ok(grid)
    }
}
