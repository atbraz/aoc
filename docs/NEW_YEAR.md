# Setting Up a New Year

Guide for starting Advent of Code solutions for a new year (e.g., 2025).

## Table of Contents

1. [Setup by Language](#setup-by-language)
2. [Creating the Justfile](#creating-the-justfile)
3. [Creating a README](#creating-a-readme)
4. [Testing Your Setup](#testing-your-setup)
5. [Optional: Common Library](#optional-common-library)

## Quick Start

For a language you've already used in a previous year:

```bash
# 1. Create year directory
mkdir 2025
cd 2025

# 2. Copy justfile from previous year
cp ../2023/justfile .

# 3. Update YEAR variable in justfile
sed -i '' 's/YEAR := "2023"/YEAR := "2025"/' justfile

# 4. Create README (copy and modify from previous year)
cp ../2023/README.md .
# Edit README.md to update year and any specific changes

# 5. Test with first day
just new 1
```

## Setup by Language

### Rust Setup

```bash
# 1. Create year directory
mkdir 2025
cd 2025

# 2. Initialize Cargo workspace (if needed, or use root workspace)
# Edit ../Cargo.toml to add members = ["2025/1", "2025/2", ...]

# 3. Copy justfile
cp ../2023/justfile .
sed -i '' 's/YEAR := "2023"/YEAR := "2025"/' justfile

# 4. Test with day 1
just new 1
just test 1
```

**Required files:**
- `justfile` - Task runner
- Days will have: `Cargo.toml`, `src/main.rs`

**Dependencies:**
- Rust toolchain (see `rust-toolchain.toml` in repo root)
- cargo-nextest: `cargo install cargo-nextest`

### OCaml Setup

```bash
# 1. Create year directory
mkdir 2025
cd 2025

# 2. Create dune-project
cat > dune-project << 'EOF'
(lang dune 3.17)
(name aoc-2025)
(version 1.0.0)

(package
 (name aoc-2025)
 (synopsis "Advent of Code 2025 solutions")
 (depends ocaml dune))
EOF

# 3. Create opam file
cat > aoc-2025.opam << 'EOF'
opam-version: "2.0"
name: "aoc-2025"
version: "1.0.0"
synopsis: "Advent of Code 2025 solutions"
maintainer: "Your Name <your@email.com>"
authors: "Your Name <your@email.com>"
license: "MIT"
depends: [
  "ocaml" {>= "5.3.0"}
  "dune" {>= "3.17.0"}
]
build: [
  ["dune" "build" "-p" name "-j" jobs]
]
EOF

# 4. Copy .ocamlformat (optional but recommended)
cp ../2022/.ocamlformat .

# 5. Copy justfile
cp ../2022/justfile .
sed -i '' 's/YEAR := "2022"/YEAR := "2025"/' justfile

# 6. Test with day 1
just new 1
just test 1
```

**Required files:**
- `dune-project` - Dune configuration
- `aoc-2025.opam` - Package definition
- `justfile` - Task runner
- `.ocamlformat` - Code formatting rules
- Days will have: `dune`, `main.ml`

### C++ Setup

```bash
# 1. Create year directory
mkdir 2025
cd 2025

# 2. Copy justfile
cp ../2024/justfile .

# 3. Update days list (optional, for build-all)
# Edit DAYS variable in justfile

# 4. Test with day 1
just new 1
just build 1
just test 1
```

**Required files:**
- `justfile` - Task runner
- Days will have: `main.cpp`, `input`, `sample_1`

**Requirements:**
- g++ with C++17 support
- On macOS: `brew install gcc`

### Other Languages

For a new language not used before:

1. Create the year directory
2. Define your solution structure (one executable per day? shared library?)
3. Create a justfile with these commands:
   - `new <day>` - Create day template
   - `build <day>` - Build/compile if needed
   - `run <day>` - Run with input file
   - `test <day>` - Run with sample file
   - `clean` - Remove build artifacts
4. Create template files that `new` command will copy
5. Add README documenting your setup

## Creating the Justfile

The justfile should follow this pattern:

```just
# Advent of Code 2025 - Justfile
YEAR := "2025"

default:
    @just --list

# Run a specific day with input file
[group('run')]
run day:
    # Your run command here
    # Example: cargo run --package aoc-2025-{{day}} -- 1 input

# Test with sample input
[group('test')]
test day:
    # Your test command here
    # Example: cargo run --package aoc-2025-{{day}} -- 1 sample

# Create a new day solution
[group('setup')]
new day:
    #!/usr/bin/env bash
    # Create directory structure
    # Copy template files
    # Initialize build configuration

# Clean build artifacts
[group('clean')]
clean:
    # Your clean command
    # Example: cargo clean
```

Copy from the most similar previous year and adapt.

## Creating a README

Your year's README should include:

1. **Header**: Year and language
2. **Stack**: Language version, build tools, dependencies
3. **Project Structure**: Directory layout
4. **Commands**: Common just commands with examples
5. **Development Workflow**: Step-by-step process
6. **Code Structure**: Expected file structure per day
7. **Intricacies**: Language/tooling-specific quirks

See existing year READMEs for reference:
- [2024/README.md](2024/README.md) - C++
- [2023/README.md](2023/README.md) - Rust
- [2022/README.md](2022/README.md) - OCaml

## Testing Your Setup

After setup, verify everything works:

```bash
# 1. Create day 1
just new 1

# 2. Check files were created
ls 1/

# 3. Add a simple test
echo "42" > 1/sample

# 4. Write minimal solution
# Edit main file to read input and print "42"

# 5. Test it
just test 1

# 6. Should print: 42

# 7. Clean up
just clean
```

## Optional: Common Library

If you'll write utility functions, create a common library:

### Rust Common Library

```bash
mkdir -p 2023/common  # Replace 2023 with your year
cd 2023/common

# Create Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "aoc-common-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF

# Create lib.rs
mkdir src
cat > src/lib.rs << 'EOF'
pub mod utils;
EOF

# Create utils module
cat > src/utils.rs << 'EOF'
// Your common utilities here
EOF
```

### OCaml Common Library

```bash
mkdir -p 2022/common  # Replace 2022 with your year
cd 2022/common

# Create dune
cat > dune << 'EOF'
(library
 (name aoc_common_ocaml)
 (public_name aoc-common-ocaml)
 (modules aoc_common_ocaml))
EOF

# Create library file
cat > aoc_common_ocaml.ml << 'EOF'
(* Your common utilities here *)
let read_file filename =
  In_channel.with_open_bin filename In_channel.input_all
EOF
```

### C++ Common Library

```bash
mkdir -p common/cpp/include
mkdir -p common/cpp/src

# Create header
cat > common/cpp/include/aoc_common.hpp << 'EOF'
#pragma once
#include <string>
#include <vector>

namespace aoc {
    std::vector<std::string> read_lines(const std::string& filename);
}
EOF

# Create implementation
cat > common/cpp/src/aoc_common.cpp << 'EOF'
#include "aoc_common.hpp"
#include <fstream>

namespace aoc {
    std::vector<std::string> read_lines(const std::string& filename) {
        std::ifstream file(filename);
        std::vector<std::string> lines;
        std::string line;
        while (std::getline(file, line)) {
            lines.push_back(line);
        }
        return lines;
    }
}
EOF
```

Then reference it in your day's build commands.

## Checklist

- [ ] Create year directory
- [ ] Set up language-specific configuration files
- [ ] Create justfile with YEAR variable updated
- [ ] Copy and update README.md
- [ ] Test `just new 1` command
- [ ] Test `just test 1` with a simple solution
- [ ] Create common library (optional)
- [ ] Update root workspace files if needed
- [ ] Add year to root README.md

