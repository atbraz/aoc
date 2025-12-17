# Advent of Code - Root Justfile
# Usage:
#   just --list                  - Show all available commands
#   just run 2024 1              - Run 2024 day 1 with input
#   just test 2024 1             - Run 2024 day 1 with sample input
#   just new 2024 5              - Create new day 5 solution for 2024
#   just install-toolchain 2023  - Install Rust toolchain for 2023
#   just install-all-toolchains  - Install all toolchains (C++, Rust, OCaml)

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

# Install toolchain for a specific year
[group('setup')]
install-toolchain year:
    @just {{year}}/install-toolchain

# Install all toolchains (Zig, C++, Rust, OCaml)
[group('setup')]
install-all-toolchains:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Installing all toolchains..."
    echo ""
    echo "=== Installing Zig toolchain (2025) ==="
    just 2025/install-toolchain || true
    echo ""
    echo "=== Installing C++ toolchain (2024) ==="
    just 2024/install-toolchain || true
    echo ""
    echo "=== Installing Rust toolchain (2023) ==="
    just 2023/install-toolchain || true
    echo ""
    echo "=== Installing OCaml toolchain (2022) ==="
    just 2022/install-toolchain || true
    echo ""
    echo "All toolchain installations attempted!"
    echo "Run 'just --list' to see available commands."

# Show progress for all years or a specific year
[group('utils')]
progress year="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -n "{{year}}" ]; then
        ./scripts/progress.sh {{year}}
    else
        ./scripts/progress.sh
    fi

# Fetch puzzle input from adventofcode.com
[group('utils')]
fetch year day:
    ./scripts/fetch-input.sh {{year}} {{day}}

# Benchmark a specific solution with hyperfine
[group('bench')]
bench year day iterations="10":
    #!/usr/bin/env bash
    set -euo pipefail

    if ! command -v hyperfine &> /dev/null; then
        echo "Error: hyperfine not installed"
        echo "Install with: brew install hyperfine (macOS) or cargo install hyperfine"
        exit 1
    fi

    echo "Benchmarking {{year}} day {{day}} ({{iterations}} iterations)..."

    case {{year}} in
        2025)
            if [ ! -f "{{year}}/{{day}}/main" ]; then
                just build {{year}} {{day}}
            fi
            hyperfine --warmup 3 --runs {{iterations}} \
                --export-markdown "{{year}}/{{day}}/benchmark.md" \
                '{{year}}/{{day}}/main {{year}}/{{day}}/input'
            ;;
        2024)
            if [ ! -f "{{year}}/build/{{day}}/main" ]; then
                just build {{year}} {{day}}
            fi
            hyperfine --warmup 3 --runs {{iterations}} \
                --export-markdown "{{year}}/{{day}}/benchmark.md" \
                'echo "{{year}}/{{day}}/input" | {{year}}/build/{{day}}/main'
            ;;
        2023)
            hyperfine --warmup 3 --runs {{iterations}} \
                --export-markdown "{{year}}/{{day}}/benchmark.md" \
                'cargo run --release --package aoc-{{year}}-{{day}} -- 1 input'
            ;;
        2022)
            hyperfine --warmup 3 --runs {{iterations}} \
                --export-markdown "{{year}}/{{day}}/benchmark.md" \
                'dune exec aoc-{{year}}-{{day}} {{year}}/{{day}}/input'
            ;;
        *)
            echo "Error: Unknown year {{year}}"
            exit 1
            ;;
    esac

    echo ""
    echo "Benchmark complete! Results saved to {{year}}/{{day}}/benchmark.md"

# Compare performance across all solutions
[group('bench')]
bench-all:
    #!/usr/bin/env bash
    set -euo pipefail

    if ! command -v hyperfine &> /dev/null; then
        echo "Error: hyperfine not installed"
        exit 1
    fi

    echo "Running comprehensive benchmarks..."
    echo ""

    for year in 2022 2023 2024 2025; do
        if [ -d "$year" ]; then
            echo "Benchmarking $year..."
            for day in "$year"/*/; do
                if [ -d "$day" ]; then
                    day_num=$(basename "$day")
                    if [ -f "$day/main.ml" ] || [ -f "$day/src/main.rs" ] || [ -f "$day/main.cpp" ] || [ -f "$day/main.zig" ]; then
                        echo "  Day $day_num..."
                        just bench "$year" "$day_num" 5 2>/dev/null || echo "    Failed"
                    fi
                fi
            done
        fi
    done

    echo ""
    echo "All benchmarks complete!"

# Generate statistics about the codebase
[group('utils')]
stats:
    ./scripts/stats.sh
