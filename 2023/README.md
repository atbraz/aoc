# AOC 2023 - Rust Boogaloo

## Initializing a new day

```bash
./new-day.sh <day>
```

Example: `./new-day.sh 9`

This will:
- Create a new package using `cargo new`
- Add `aoc-common-rust` as a dependency
- Copy all template files from `common/rust/template`
- Add the package to the workspace

## Running a day

```bash
aoc/2023/{day} $ cargo run {part} [input_file: optional, defaults to `input`]
```
