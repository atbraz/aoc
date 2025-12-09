# Advent of Code - Root Justfile
# Usage:
#   just --list              - Show all available commands
#   just 2024 1              - Run 2024 day 1 with input
#   just test 2024 1         - Run 2024 day 1 with sample input
#   just 2023 8              - Run 2023 day 8 with input
#   just new 2024 5          - Create new day 5 solution for 2024

# Default recipe - show help
default:
    @just --list

# Run a specific year/day with input file
[group('run')]
run year day *args:
    @just {{year}}/run {{day}} {{args}}


# Test a specific year/day with sample input
[group('test')]
test year day *args:
    @just {{year}}/test {{day}} {{args}}

# Create a new day solution
[group('setup')]
new year day:
    @just {{year}}/new {{day}}

# Clean build artifacts for a year or specific day
[group('clean')]
clean year day="":
    @just {{year}}/clean {{day}}

# Run all tests in workspace (Rust years)
[group('test')]
test-all:
    cargo nextest run --workspace

# Build all Rust packages
[group('build')]
build-all:
    cargo build --workspace --release

# Check Rust code
[group('build')]
check:
    cargo clippy --workspace -- -D warnings

# Update dependencies for a specific year
[group('update')]
update year:
    @just {{year}}/update

# Update Rust toolchain
[group('update')]
update-rust:
    @just 2023/update-toolchain
