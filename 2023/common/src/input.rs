use crate::errors::InputError;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

/**
A struct that provides different ways to read and parse input files
*/
pub struct InputReader;

impl InputReader {
    /**
    Reads an entire file into a single String, preserving newlines

    This function reads the complete contents of a file at the given path
    and returns it as a String. The content maintains its original line
    endings, making it suitable for processing that requires preserving
    the exact file structure.

    # Arguments
    * `path` - Any type that can be converted into a Path

    # Returns
    A String containing the entire contents of the file if successful

    # Errors
    This function will return an error if:
    * The file does not exist
    * The process lacks permissions to read the file
    * The file content is not valid UTF-8
    * The file is empty

    # Examples

    ```
    use common::input::InputReader;

    assert_eq!(InputReader::as_string(path), );
    ```
    */
    pub fn as_string(path: impl AsRef<Path>) -> Result<String, InputError> {
        let content = fs::read_to_string(path)?;
        if content.is_empty() {
            return Err(InputError::EmptyFile);
        }
        Ok(content)
    }

    /**
    Reads a file into a vector of strings, one per line

    Each line from the file becomes a separate String in the returned vector.
    This is particularly useful for Advent of Code problems where input often
    consists of line-separated data.

    # Arguments
    * `path` - Any type that can be converted into a Path

    # Returns
    A vector of Strings, where each String is a line from the file

    # Errors
    This function will return an error if:
    * The file cannot be opened (doesn't exist or insufficient permissions)
    * Any line in the file contains invalid UTF-8
    * The file is empty
    * IO operations fail while reading the file

    # Examples

    ```
    use common::input::InputReader;

    assert_eq!(InputReader::as_lines(path), );
    ```
    */
    pub fn as_lines(path: impl AsRef<Path>) -> Result<Vec<String>, InputError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;

        if lines.is_empty() {
            return Err(InputError::EmptyFile);
        }
        Ok(lines)
    }

    /**
    Reads a file into a single continuous string by removing all newlines

    This function is useful when working with data that is split across multiple
    lines purely for readability but should logically be treated as one continuous
    string. For example, if your input looks like:
    ```text
    ABCDEF
    GHIJKL
    MNOPQR
    ```
    It will return "ABCDEFGHIJKLMNOPQR" as a single string.

    Unlike `as_string`, which preserves newlines, this function specifically
    removes all line breaks to create a single continuous string. This is
    particularly useful for Advent of Code problems involving long sequences
    that happen to be formatted across multiple lines.

    # Arguments
    * `path` - Any type that can be converted into a Path

    # Returns
    A single String with all newlines removed from the file content

    # Errors
    This function will return an error if:
    * The file cannot be opened (doesn't exist or insufficient permissions)
    * The file contains invalid UTF-8 characters
    * The file is completely empty
    * IO operations fail while reading the file

    # Examples
    ```no_run
    use your_crate::InputReader;

    let content = InputReader::as_single_line("input.txt").unwrap();
    // Now we have a single continuous string to work with
    println!("Total length: {}", content.len());

    // Useful for problems involving long sequences
    if content.contains("AABB") {
        println!("Found pattern AABB in the sequence!");
    }
    ```

    # Examples

    ```
    use common::input::InputReader;

    assert_eq!(InputReader::as_single_line(path), );
    ```
    */
    pub fn as_single_line(path: impl AsRef<Path>) -> Result<String, InputError> {
        let content = Self::as_string(path)?;
        Ok(content.lines().collect::<String>())
    }

    /**
    Reads a file and splits it into paragraphs, where paragraphs are groups of
    non-empty lines separated by blank lines

    This function is particularly useful for Advent of Code problems where input
    data comes in groups separated by empty lines. For example, if your input
    looks like:
    ```text
    1234
    5678

    abcd
    efgh
    ijkl
    ```
    It will return a vector containing two strings: "1234\n5678" and "abcd\nefgh\nijkl"

    # Arguments
    * `path` - Any type that can be converted into a Path

    # Returns
    A vector of Strings, where each String represents a paragraph with its
    internal line breaks preserved

    # Errors
    This function will return an error if:
    * The file cannot be opened (doesn't exist or insufficient permissions)
    * The file contains invalid UTF-8 characters
    * The file is completely empty
    * IO operations fail while reading the file

    # Examples
    ```no_run
    use your_crate::InputReader;

    let paragraphs = InputReader::as_paragraphs("input.txt").unwrap();
    assert!(paragraphs.len() >= 1); // At least one paragraph

    // Each paragraph preserves its internal newlines
    if let Some(first_para) = paragraphs.first() {
        for line in first_para.lines() {
            println!("Line in first paragraph: {}", line);
        }
    }
    ```

    # Examples

    ```
    use common::input::InputReader;

    assert_eq!(InputReader::as_paragraphs(path), );
    ```
    */
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

    /**
    Reads a file into a grid of characters

    Converts the input file into a two-dimensional vector of characters,
    useful for problems involving maps, mazes, or any grid-based challenges.
    Each line in the file becomes a row in the grid.

    # Arguments
    * `path` - Any type that can be converted into a Path

    # Returns
    A vector of vectors of characters, representing the grid

    # Errors
    This function will return an error if:
    * The file cannot be opened
    * The file is empty
    * The file contains invalid UTF-8
    * The grid is not rectangular (lines have different lengths)

    # Examples

    ```
    use common::input::InputReader;

    assert_eq!(InputReader::as_char_grid(path), );
    ```
    */
    pub fn as_char_grid(path: impl AsRef<Path>) -> Result<Vec<Vec<char>>, InputError> {
        let lines = Self::as_lines(path)?;
        let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

        if let Some(first_len) = grid.first().map(std::vec::Vec::len) {
            if grid.iter().any(|row| row.len() != first_len) {
                return Err(InputError::InvalidFormat(
                    "Grid rows have inconsistent lengths".to_string(),
                ));
            }
        }

        Ok(grid)
    }
}
