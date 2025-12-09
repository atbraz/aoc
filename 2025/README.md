# Advent of Code 2025

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in **Zig**.

## Stack

- **Language**: Zig 0.13.0
- **Build Tool**: zig (native compiler)
- **Task Runner**: Just (justfile for convenience)

## Project Structure

```
2025/
├── 1/
│   ├── main.zig    # Solution implementation
│   ├── input       # Puzzle input
│   └── sample      # Sample input for testing
├── 2/
│   └── ...
└── justfile        # Build and run commands
```

## Commands

```bash
# Show all available commands
just

# Install Zig toolchain
just install-toolchain

# Create a new day solution (generates template)
just new <day>

# Build a specific day
just build <day>

# Build all days
just build-all

# Run with actual input
just run <day>

# Test with sample input
just test <day>

# Format code
just fmt [day]

# Check syntax
just check <day>

# Clean build artifacts
just clean
```

## Development Workflow

1. **Create new day**: `just new 5` creates the directory structure with template
2. **Add inputs**: Copy puzzle input to `5/input` and sample to `5/sample`
3. **Implement**: Edit `5/main.zig` with your solution
4. **Format**: `just fmt 5` to auto-format your code
5. **Test**: `just test 5` to verify with sample input
6. **Run**: `just run 5` to solve with actual input

## Code Structure

Each day's solution follows this pattern:

```zig
const std = @import("std");

fn parseInput(allocator: std.mem.Allocator, filename: []const u8) !std.ArrayList([]const u8) {
    // Read input file line by line
}

fn part1(lines: std.ArrayList([]const u8)) !i64 {
    // Solve part 1
}

fn part2(lines: std.ArrayList([]const u8)) !i64 {
    // Solve part 2
}

pub fn main() !void {
    // Setup allocator, read input, run solutions
}
```

## Memory Management

Zig's explicit allocator pattern:
- Use `GeneralPurposeAllocator` for general use
- Always `defer` cleanup (`deinit`, `free`)
- Memory leaks are caught at runtime in debug mode

```zig
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
defer _ = gpa.deinit();  // Detects leaks in debug builds
const allocator = gpa.allocator();
```

## Error Handling

Zig's explicit error handling:
- Functions return `!Type` for errors
- Use `try` to propagate errors
- Use `catch` to handle errors

```zig
const file = try std.fs.cwd().openFile(filename, .{});
defer file.close();

const result = someFunction() catch |err| {
    std.debug.print("Error: {}\n", .{err});
    return err;
};
```

## Building

### Debug Build
```bash
zig build-exe main.zig
```
- Includes debug symbols
- Runtime safety checks
- Slower execution
- Memory leak detection

### Release Build
```bash
zig build-exe main.zig -O ReleaseFast
```
- Maximum runtime speed
- No safety checks
- Optimized for performance

### Release Safe
```bash
zig build-exe main.zig -O ReleaseSafe
```
- Fast runtime
- Keeps safety checks
- Balance of speed and safety

## Intricacies

### Compilation Model
- **Single-file compilation**: Each day compiles independently
- **Fast compile times**: Zig is designed for fast iteration
- **No build configuration**: Just `zig build-exe main.zig`
- **Static linking**: Binaries are self-contained

### Type System
- **Comptime**: Code that runs at compile-time
- **No hidden control flow**: Everything is explicit
- **No hidden allocations**: All allocations are explicit
- **Null safety**: Use optional types `?T` instead of null

### Arrays and Slices
```zig
// Array - fixed size, known at compile time
var arr: [5]i32 = .{1, 2, 3, 4, 5};

// Slice - runtime-known size
var slice: []i32 = arr[0..3];

// String is just []const u8
const str: []const u8 = "hello";
```

### Allocator Pattern
```zig
// Creating dynamic array
var list = std.ArrayList(i32).init(allocator);
defer list.deinit();

// Allocating memory
const buffer = try allocator.alloc(u8, 100);
defer allocator.free(buffer);

// Duplicating data
const copy = try allocator.dupe(u8, original);
defer allocator.free(copy);
```

### Common Patterns

**Reading input line by line:**
```zig
var buf_reader = std.io.bufferedReader(file.reader());
var in_stream = buf_reader.reader();

var buf: [1024]u8 = undefined;
while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
    // Process line
}
```

**Parsing integers:**
```zig
const num = try std.fmt.parseInt(i32, str, 10);
```

**String formatting:**
```zig
const stdout = std.io.getStdOut().writer();
try stdout.print("Result: {d}\n", .{value});
```

**Splitting strings:**
```zig
var iter = std.mem.split(u8, str, ",");
while (iter.next()) |part| {
    // Process part
}
```

## Why Zig for AoC 2025?

**Strengths:**
- **Fast**: Compiles quickly, runs fast
- **Safe**: Explicit memory management without GC
- **Simple**: No hidden control flow or allocations
- **Readable**: Clear, explicit code
- **Modern**: Great standard library

**Learning opportunities:**
- Manual memory management (with safety)
- Comptime metaprogramming
- Low-level systems concepts
- Error handling patterns

**Challenges:**
- Explicit allocator passing
- More verbose than high-level languages
- Newer ecosystem (fewer libraries)
- Learning curve for error handling

## Resources

- [Zig Language Reference](https://ziglang.org/documentation/master/)
- [Zig Standard Library](https://ziglang.org/documentation/master/std/)
- [Ziglearn](https://ziglearn.org/)
- [Zig by Example](https://zig-by-example.github.io/)

## Tips

1. **Use debug builds while developing** - catches memory errors
2. **Always defer cleanup** - prevents leaks
3. **Use `_` to ignore unused variables** - compiler enforces this
4. **Comptime is powerful** - use it for constants and types
5. **Check the stdlib** - many algorithms already implemented
6. **Profile with release builds** - debug is much slower

## Performance Notes

- Debug builds are **much** slower (10-100x)
- Release builds compete with C/C++ performance
- Compile times are very fast (sub-second for small programs)
- Single-file compilation is instant iteration
