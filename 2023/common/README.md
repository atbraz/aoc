# common

Shared utilities for Advent of Code solutions in Rust.

## Overview

This library provides common functionality to reduce boilerplate in daily puzzle solutions:

- **CLI Handling**: Automatic command-line parsing and result formatting
- **Input Reading**: Multiple ways to parse input files (lines, paragraphs, grids, etc.)
- **Error Handling**: Ergonomic error types with automatic conversions
- **Terminal Colors**: ANSI color codes for pretty output

## Installation

This is a local workspace crate. Add it to your day's `Cargo.toml`:

```toml
[dependencies]
common = { path = "../common" }
```

Or use `cargo add`:

```bash
cd 2023/5
cargo add common --path ../common
```

## Usage

### CLI Module

The CLI module handles command-line argument parsing and result formatting automatically.

#### Basic Usage

```rust
use common::cli;
use common::errors::AocError;

fn part1(filename: &str) -> Result<i32, AocError> {
    // Your solution here
    Ok(42)
}

fn part2(filename: &str) -> Result<i32, AocError> {
    // Your solution here
    Ok(24)
}

fn main() {
    cli::run(part1, part2);
}
```

#### What It Does

- Parses command-line args: `./program <part> [input_file]`
- Defaults to `input` file if not specified
- Validates part number (1 or 2)
- Prints colorized results
- Displays error chains on failure
- Exits with appropriate status codes

#### Example Invocations

```bash
cargo run -- 1         # Run part 1 with "input"
cargo run -- 2 sample  # Run part 2 with "sample"
```

### Input Module

The `InputReader` struct provides multiple methods for reading and parsing input files.

#### Read as Lines

```rust
use common::input::InputReader;

fn part1(filename: &str) -> Result<i32, AocError> {
    let lines = InputReader::as_lines(filename)?;

    for line in lines {
        // Process each line
    }

    Ok(42)
}
```

#### Read as Single String

Preserves newlines:

```rust
let content = InputReader::as_string(filename)?;
// Process the entire file as one string
```

#### Read as Single Line

Removes all newlines, useful for long sequences:

```rust
let sequence = InputReader::as_single_line(filename)?;
// "ABCD\nEFGH" becomes "ABCDEFGH"
```

#### Read as Paragraphs

Splits on blank lines:

```rust
let paragraphs = InputReader::as_paragraphs(filename)?;
// Each element is a group of lines separated by blank lines

for paragraph in paragraphs {
    for line in paragraph.lines() {
        // Process each line in the paragraph
    }
}
```

#### Read as Character Grid

For map/maze problems:

```rust
let grid = InputReader::as_char_grid(filename)?;
// Vec<Vec<char>> - 2D grid of characters

for row in &grid {
    for &ch in row {
        match ch {
            '#' => { /* wall */ }
            '.' => { /* empty */ }
            _ => { /* other */ }
        }
    }
}

// Access specific positions
let cell = grid[y][x];
```

Validates that all rows have the same length.

### Error Module

Provides two error types with automatic conversions.

#### AocError

General-purpose error for solution functions:

```rust
use common::errors::AocError;

fn solve(filename: &str) -> Result<i32, AocError> {
    let lines = InputReader::as_lines(filename)?;  // Converts InputError

    let num: i32 = lines[0].parse()?;  // Converts ParseIntError

    if num < 0 {
        return Err("Negative numbers not allowed".into());  // From &str
    }

    Ok(num)
}
```

**Variants:**
- `Input(InputError)` - File I/O errors
- `Parse(String)` - General parsing errors
- `ParseInt(ParseIntError)` - Integer parsing errors
- `ParseFloat(ParseFloatError)` - Float parsing errors
- `Custom(String)` - Custom error messages

**Automatic Conversions:**
- `InputError` → `AocError`
- `ParseIntError` → `AocError`
- `ParseFloatError` → `AocError`
- `String` → `AocError`
- `&str` → `AocError`

#### InputError

Lower-level errors for input reading:

**Variants:**
- `IoError(io::Error)` - File system errors
- `EmptyFile` - File exists but is empty
- `InvalidFormat(String)` - Content doesn't match expected format

#### Custom Errors

```rust
use common::errors::custom_error;

fn validate(value: i32) -> Result<(), AocError> {
    if value < 0 {
        return Err(custom_error("Value must be positive"));
    }
    Ok(())
}
```

### Color Module

Terminal color and styling using ANSI escape codes.

#### Basic Colors

```rust
use common::color::Color;

println!("{}", Color::Red.wrap("Error!"));
println!("{}", Color::Green.wrap("Success"));
println!("{}", Color::Blue.wrap("42"));
```

#### Available Colors

**Foreground:**
- `Black`, `Red`, `Green`, `Yellow`, `Blue`, `Magenta`, `Cyan`, `White`
- `Bright*` variants: `BrightRed`, `BrightGreen`, etc.

**Background:**
- `OnBlack`, `OnRed`, `OnGreen`, `OnYellow`, `OnBlue`, `OnMagenta`, `OnCyan`, `OnWhite`
- `OnBright*` variants

**Styles:**
- `Bold`, `Dim`, `Italic`, `Underline`, `Blink`, `Reverse`

#### Combining Styles

```rust
let text = Color::combine(
    &[Color::Bold, Color::Red, Color::OnWhite],
    "Important!"
);
println!("{}", text);
```

## Complete Example

```rust
use common::{cli, color::Color, errors::AocError, input::InputReader};

fn part1(filename: &str) -> Result<i32, AocError> {
    let lines = InputReader::as_lines(filename)?;

    let mut sum = 0;
    for line in lines {
        let num: i32 = line.trim().parse()?;
        sum += num;
    }

    println!("{}", Color::Green.wrap(&format!("Processed {} lines", sum)));
    Ok(sum)
}

fn part2(filename: &str) -> Result<i32, AocError> {
    let grid = InputReader::as_char_grid(filename)?;

    let height = grid.len();
    let width = grid[0].len();

    println!("Grid: {}x{}", width, height);

    // Count '#' symbols
    let count = grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&ch| ch == '#')
        .count();

    Ok(count as i32)
}

fn main() {
    cli::run(part1, part2);
}
```

## Design Philosophy

- **Ergonomic**: Reduce boilerplate without hiding complexity
- **Composable**: Mix and match utilities as needed
- **Explicit**: Clear function names and error types
- **Fail-Fast**: Validate input early, propagate errors cleanly
- **Zero Dependencies**: Only uses Rust standard library

## Adding New Utilities

When you find yourself writing the same code across multiple days:

1. Add the function to the appropriate module (`input.rs`, `cli.rs`, etc.)
2. Document with rustdoc comments
3. Add error handling using existing error types
4. Use it in your solutions to verify it works

Common patterns to extract:
- Parsing specific formats (coordinates, ranges, etc.)
- Grid traversal (neighbors, paths)
- Algorithm helpers (BFS, DFS, dynamic programming)
- Math utilities (GCD, LCM, combinations)
