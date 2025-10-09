#!/bin/bash

set -e

if [ -z "$1" ]; then
    echo "Usage: ./new-day.sh <day>"
    echo "Example: ./new-day.sh 9"
    exit 1
fi

DAY=$1
YEAR=2023
PKG_NAME="aoc-$YEAR-$DAY"
DAY_DIR="$DAY"

# Check if directory already exists
if [ -d "$DAY_DIR" ]; then
    echo "Error: Directory $DAY_DIR already exists"
    exit 1
fi

echo "Creating new AOC $YEAR day $DAY package..."

# Use cargo to create the package
cargo new "$DAY_DIR" --name "$PKG_NAME" --vcs none

# Add dependency using cargo
cd "$DAY_DIR"
cargo add aoc-common-rust --path ../../common/rust
cd ..

# Copy all template files, replacing placeholders
TEMPLATE_DIR="../common/rust/template/src"

if [ -d "$TEMPLATE_DIR" ]; then
    for file in "$TEMPLATE_DIR"/*; do
        if [ -f "$file" ]; then
            filename=$(basename "$file")
            sed -e "s/<YEAR>/$YEAR/g" -e "s/<DAY>/$DAY/g" "$file" > "$DAY_DIR/src/$filename"
        fi
    done
fi

# Add to workspace (no cargo command for this AFAIK)
CARGO_TOML="../Cargo.toml"
if ! grep -q "\"$YEAR/$DAY\"" "$CARGO_TOML"; then
    sed -i.bak "s/members = \[/members = [\"$YEAR\/$DAY\", /" "$CARGO_TOML"
    rm "$CARGO_TOML.bak"
    echo "Added $PKG_NAME to workspace"
fi

echo "Successfully created $PKG_NAME at $DAY_DIR/"
echo ""
echo "You can now run:"
echo "  cd $DAY_DIR"
echo "  cargo run -- input.txt"
