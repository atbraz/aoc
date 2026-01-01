# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Multi-year Advent of Code solutions exploring different programming languages and paradigms. Each year uses a different language to explore its unique strengths. The project is organized with year-specific directories (2022-2025) and shared common libraries per language.

## Build System

The project uses [Just](https://github.com/casey/just) as a task runner. All commands work from both the repository root and within year directories.

### Essential Commands

From repository root:
```bash
# Run a specific day with puzzle input
just run <year> <day>

# Test with sample input
just test <year> <day>

# Create a new day solution
just new <year> <day>

# Build all Rust packages (2023)
just build-all

# Run all Rust tests
just test-all

# Run clippy on all Rust code
just check

# Fetch puzzle input (requires AOC session token at ~/.config/aoc-session)
just fetch <year> <day>

# Benchmark a solution
just bench <year> <day>

# Clean build artifacts
just clean <year>
```

From within a year directory (e.g., `cd 2023`):
```bash
# Run day 8 with input
just run 8

# Test day 8 with sample
just test 8

# Test with specific sample file and part
just test 8 2 sample_2  # Rust: part 2 with sample_2
just test 8 sample_2    # C++/OCaml/Zig: sample_2 file

# Create new day
just new 15

# Build specific day
just build 8
```

### Year-Specific Build Details

**2023 (Rust):**
- Cargo workspace with each day as a separate package
- Packages named: `aoc-2023-<day>`
- Use `cargo nextest run` for testing
- Solutions use common library at `2023/common`

**2024 (C++):**
- Standalone C++ files compiled with g++/clang++
- C++17/C++23 standard depending on platform
- Binaries: `build/<day>/main`
- Input provided via stdin: `echo "<day>/input" | ./build/<day>/main`

**2022 (OCaml):**
- Dune build system
- Executables named: `aoc-2022-<day>`
- Run with: `dune exec aoc-2022-<day> <day>/input`

**2025 (Zig):**
- Standalone Zig files compiled with zig build-exe
- Zig ≥ 0.13.0
- Binaries: `<day>/main`
- Run with: `./<day>/main <day>/input`

## Project Architecture

### Directory Structure
```
aoc/
├── 2025/          # Zig solutions (day-by-day directories)
├── 2024/          # C++ solutions (day-by-day directories)
│   └── common/    # C++ common utilities (include/ and src/)
├── 2023/          # Rust solutions (Cargo workspace)
│   └── common/    # aoc-common-rust library
├── 2022/          # OCaml solutions (Dune workspace)
│   └── common/    # aoc-common-ocaml library
├── common/        # Shared utilities per language
│   └── zig/       # Zig common utilities
├── scripts/       # Meta-project utilities (fetch, progress, stats)
├── justfile       # Root task runner (delegates to year justfiles)
└── Cargo.toml     # Rust workspace config (2023/common + days)
```

### Common Library Pattern

Each language has a common library within its year directory to reduce boilerplate:

**Rust (`2023/common`):**
- `cli` module: Automatic CLI parsing, part selection, result formatting
- `input` module: Read as lines, grids, paragraphs, single string
- `errors` module: `AocError` and `InputError` with automatic conversions
- `color` module: ANSI terminal colors
- Solutions import with: `use aoc_common_rust::{cli, input::InputReader, errors::AocError};`
- Main function pattern: `cli::run(part1, part2)` where each part takes `filename: &str`

**C++ (`2024/common`):**
- Header: `include/aoc_common.hpp`
- Implementation: `src/aoc_common.cpp`
- Utilities for reading lines and common operations

**OCaml (`2022/common`):**
- Library: `aoc-common-ocaml`
- File I/O helpers and parsing utilities

### Solution Structure Pattern

Each day follows a consistent pattern:

**Rust (2023):**
```rust
// main.rs - Entry point using cli module
use aoc_2023_8::{part_1, part_2};
use aoc_common_rust::cli;

fn main() {
    cli::run(part_1::solve, part_2::solve);
}

// lib.rs - Solution logic
pub mod part_1 {
    use aoc_common_rust::{errors::AocError, input::InputReader};

    pub fn solve(filename: &str) -> Result<i32, AocError> {
        let lines = InputReader::as_lines(filename)?;
        // Solution implementation
        Ok(result)
    }
}
```

**C++ (2024):**
```cpp
#include <iostream>
#include <fstream>
#include <vector>
#include <string>

// parse_input() - Read input file
// solve_part_1() - Part 1 logic
// solve_part_2() - Part 2 logic
// main() - Reads filename from stdin, calls both parts
```

**OCaml (2022):**
```ocaml
open Aoc_common_ocaml

let parse_input filename = (* ... *)
let part1 lines = (* ... *)
let part2 lines = (* ... *)

let () =
  let filename = Sys.argv.(1) in
  Printf.printf "Part 1: %d\n" (part1 lines);
  Printf.printf "Part 2: %d\n" (part2 lines)
```

**Zig (2025):**
```zig
const std = @import("std");

// parseInput() - Parse input file with allocator
// part1() - Part 1 logic
// part2() - Part 2 logic
// main() - Handles args, allocator lifecycle, calls both parts
```

### Input File Conventions

Each day has a directory with:
- `input` - Actual puzzle input
- `sample` or `sample_1` - Example from problem description
- `sample_2` - Optional second sample (for part 2)

### Creating a New Day

When running `just new <year> <day>`:
1. Creates `<year>/<day>/` directory
2. Copies language-specific template code
3. Creates empty `input` and `sample` files
4. For Rust: adds package to workspace in Cargo.toml
5. For OCaml: creates `dune` build file
6. Prints next steps for implementation

Templates are stored in each year's directory:
- `2022/template/` - OCaml templates (dune, main.ml)
- `2023/template/src/` - Rust templates (main.rs, lib.rs, part_1.rs, part_2.rs, utils.rs)
- `2024/template/` - C++ templates (solution.h, solution.cpp, main.cpp)
- `2025/template/` - Zig templates (main.zig)

## Important Development Notes

### Adding to Workspace (Rust)

When creating a new Rust day, the package must be added to `Cargo.toml`:
```toml
members = ["2023/common", "2023/3", "2023/4", ..., "2023/<new-day>"]
```

The `just new` command does this automatically, but manual additions should follow the pattern.

### Input Fetching

To automatically fetch puzzle inputs:
1. Get session cookie from browser (logged into adventofcode.com)
2. Store in `~/.config/aoc-session`
3. Set permissions: `chmod 600 ~/.config/aoc-session`
4. Run: `just fetch <year> <day>`

Session file location can be overridden with `AOC_SESSION_FILE` environment variable.

### Memory Management (Zig)

Zig solutions use `GeneralPurposeAllocator` with proper defer patterns:
```zig
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
defer _ = gpa.deinit();
const allocator = gpa.allocator();
```

Always free allocated memory in defer blocks immediately after allocation.

### Error Handling Patterns

**Rust:** Use `?` operator with `Result<T, AocError>`. The common library provides automatic conversions from `ParseIntError`, `ParseFloatError`, `&str`, and `String` to `AocError`.

**C++:** Check file streams with `if (!stream)` and return early or exit.

**OCaml:** Pattern match on results or use exception handling for I/O.

**Zig:** Use error unions and `try` keyword. Always handle allocator errors.

## Meta-Project Tools

Located in `scripts/`:
- `fetch-input.sh` - Download puzzle input using session cookie
- `progress.sh` - Visual progress tracking across years
- `stats.sh` - Generate codebase statistics

## Toolchain Requirements

**2025 (Zig):** Zig ≥ 0.13.0 - Install with `just 2025/install-toolchain`

**2024 (C++):** g++ or clang++ with C++17 support - Install with `just 2024/install-toolchain`

**2023 (Rust):** Version specified in `rust-toolchain.toml`, cargo-nextest - Install with `just 2023/install-toolchain`

**2022 (OCaml):** OCaml ≥ 5.3.0, opam, Dune ≥ 3.17.0 - Install with `just 2022/install-toolchain`

Install all at once: `just install-all-toolchains`

## Starting a New Year

See `NEW_YEAR.md` for detailed guide on setting up a new year. Quick summary:
1. Create year directory
2. Copy justfile from similar language's year
3. Update `YEAR` variable in justfile
4. Create templates for `just new` command
5. Test with day 1
6. Set up language-specific workspace files (Cargo.toml, dune-project, etc.)
