# Advent of Code 2023

Solutions for [Advent of Code 2023](https://adventofcode.com/2023) written in **Rust**.

## Stack

- **Language**: Rust
- **Build Tool**: Cargo (Rust's package manager)
- **Task Runner**: Just (justfile for convenience)
- **Test Framework**: cargo-nextest
- **Workspace**: Cargo workspace with per-day packages

## Project Structure

```
2023/
├── 1/
│   ├── src/
│   │   └── main.rs    # Solution implementation
│   ├── Cargo.toml     # Package manifest
│   ├── input          # Puzzle input
│   └── sample_1       # Sample input for testing
├── Cargo.toml         # Workspace manifest
└── justfile           # Build and run commands
```

## Commands

```bash
# Show all available commands
just

# Create a new day solution (generates template)
just new <day>

# Run with actual input
just run <day> [part]

# Test with sample input
just test <day> [part] [sample_file]

# Build a specific day
just build <day>

# Run unit tests
just test-unit <day>

# Run clippy linter
just check <day>

# Clean build artifacts
just clean
```

## Development Workflow

1. **Create new day**: `just new 8` creates package and adds to workspace
2. **Add inputs**: Copy puzzle input to `8/input` and samples to `8/sample_1`
3. **Implement**: Edit `8/src/main.rs` with your solution
4. **Test**: `just test 8` to verify with sample input
5. **Run**: `just run 8` to solve with actual input

## Code Structure

Each day's solution expects command line arguments:

```bash
cargo run -- <part> <input_file>
```

- `part`: "1" or "2" for which part to solve
- `input_file`: Path to input file (relative to day directory)

## Intricacies

- **Cargo Workspace**: All days are in a single workspace, sharing dependencies
- **Common Library**: Uses `aoc-common-rust` for shared utilities
- **Flexible Testing**: Can specify different sample files per part
  - `just test 8 2 sample_2` runs part 2 with alternate sample
- **Template System**: The `new` command copies from `../common/rust/template/`
- **Auto Workspace Add**: New days are automatically added to root `Cargo.toml`
- **Quiet Runs**: Uses `--quiet` flag to reduce cargo output noise
