# Advent of Code Solutions

Multi-year Advent of Code solutions exploring different programming languages and paradigms.

## Project Structure

```
aoc/
├── 2024/          # C++ solutions
├── 2023/          # Rust solutions
├── 2022/          # OCaml solutions
├── common/        # Shared utilities per language
│   ├── rust/      # Rust common library
│   ├── ocaml/     # OCaml common library
│   └── cpp/       # C++ common library (coming soon)
├── justfile       # Root task runner
└── scripts/       # Meta-project utilities
```

## Quick Start

```bash
# Run a specific day
just run <year> <day>

# Test with sample input
just test <year> <day>

# Create a new day solution
just new <year> <day>

# Show all available commands
just --list
```

## Languages by Year

| Year | Language | Build Tool | Paradigm |
|------|----------|------------|----------|
| 2024 | C++17 | g++ | Imperative, OOP |
| 2023 | Rust | Cargo | Systems, functional-ish |
| 2022 | OCaml | Dune/opam | Functional |

Each year explores different strengths:
- **C++**: Performance, low-level control, STL algorithms
- **Rust**: Memory safety, zero-cost abstractions, strong type system
- **OCaml**: Pattern matching, immutability, algebraic types

## Year-Specific Documentation

Each year has its own README with detailed setup and workflow:

- [2024 README](2024/README.md) - C++ setup and commands
- [2023 README](2023/README.md) - Rust workspace structure
- [2022 README](2022/README.md) - OCaml/Dune configuration

## Common Libraries

Shared utilities to reduce boilerplate across solutions:

- **[common/rust](common/rust/README.md)** - CLI parsing, input handling, colored output
- **[common/ocaml](common/ocaml/README.md)** - File I/O helpers
- **common/cpp** - Coming soon

## Project Commands

### Running Solutions

```bash
# From project root
just run 2023 8        # Run 2023 day 8 with input
just test 2024 1       # Test 2024 day 1 with sample

# From year directory (e.g., cd 2023)
just run 8             # Run day 8
just test 8 2          # Test day 8 part 2
```

### Creating New Solutions

```bash
# From root
just new 2024 15       # Create day 15 for 2024

# From year directory
just new 15            # Create day 15 for current year
```

The `new` command creates:
- Day directory with template code
- Empty `input` and `sample` files
- Language-specific build configuration

### Building and Testing

```bash
# Build specific day
just 2023/build 8

# Run all Rust tests (2023 workspace)
just test-all

# Run clippy on all Rust code
just check

# Clean build artifacts
just clean 2023        # Clean specific year
```

### Utilities

```bash
# Show progress for a year
just progress 2023

# Fetch puzzle input (requires AOC session token)
just fetch 2024 10

# Benchmark a solution
just bench 2023 8

# Generate statistics
just stats
```

## Development Workflow

1. **Start new day**: `just new 2024 5`
2. **Copy input**: Add puzzle input to `2024/5/input`
3. **Add sample**: Add example from problem to `2024/5/sample_1`
4. **Implement**: Edit solution file (`main.cpp`, `main.rs`, or `main.ml`)
5. **Test**: `just test 2024 5` to verify with sample
6. **Run**: `just run 2024 5` to solve with actual input
7. **Iterate**: Refine for part 2

## Starting a New Year

See [NEW_YEAR.md](NEW_YEAR.md) for a guide on setting up a new year (2025, etc.).

## Meta-Project Tools

- **Progress Tracking**: Visual dashboard of completion status
- **Input Fetching**: Automatic puzzle input download
- **Benchmarking**: Performance comparison across solutions
- **Stats Generation**: Lines of code, completion rates, language metrics

## Repository Structure

- **Cargo.toml** - Rust workspace configuration (2023 + common)
- **dune-workspace** - OCaml workspace configuration (2022)
- **justfile** - Root task runner, delegates to year-specific justfiles
- **.editorconfig** - Consistent formatting across editors
- **rust-toolchain.toml** - Rust version specification

## Requirements

### All Years
- [Just](https://github.com/casey/just) - Task runner
- Git - Version control

### 2024 (C++)
- g++ with C++17 support
- Installed via Homebrew on macOS: `brew install gcc`

### 2023 (Rust)
- Rust toolchain (specified in rust-toolchain.toml)
- cargo-nextest: `cargo install cargo-nextest`

### 2022 (OCaml)
- OCaml ≥ 5.3.0
- opam - Package manager
- Dune ≥ 3.17.0: `opam install dune`

## Configuration

### AOC Session Token
For automatic input fetching:

```bash
# Store your session cookie
echo "your-session-token" > ~/.config/aoc-session
chmod 600 ~/.config/aoc-session
```

Get your session token from browser dev tools when logged into adventofcode.com.

## Tips

- **Language Choice**: Pick the language that best fits the problem type
  - Parsing heavy? OCaml's pattern matching shines
  - Performance critical? C++ or Rust
  - Complex state? Rust's type system helps
- **Common Libraries**: Extract repeated patterns into common libraries
- **Templates**: Year-specific templates encode common solution structure
- **Sample Tests**: Always test with sample input before running on actual input
- **Incremental**: Start simple, refactor as patterns emerge

## Contributing

This is a personal learning project, but feel free to:
- Open issues for bugs or suggestions
- Share interesting alternative solutions
- Suggest additional meta-project tooling

## License

MIT - See [LICENSE.md](LICENSE.md)
