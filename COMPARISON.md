# Cross-Year Language Comparison

Observations and insights from solving Advent of Code across multiple languages.

## Language Overview

| Year | Language | Paradigm | Type System | Memory Management | Build Time |
|------|----------|----------|-------------|-------------------|------------|
| 2022 | OCaml | Functional | Strong, static, inferred | Automatic GC | Fast |
| 2023 | Rust | Multi-paradigm | Strong, static, ownership | RAII, no GC | Slow |
| 2024 | C++ | Multi-paradigm | Strong, static, manual | Manual/RAII | Medium |

## Strengths by Language

### OCaml (2022)

**Where it shines:**
- **Pattern matching**: Destructuring complex data structures is elegant
- **Algebraic types**: Sum types make state machines natural
- **Immutability**: Default immutable data prevents bugs
- **Type inference**: Less boilerplate than Rust/C++
- **Recursive functions**: Tail call optimization makes recursion efficient

**Example strength:**
```ocaml
(* Pattern matching on complex structures *)
match parse_instruction line with
| Move (n, from, to) -> process_move n from to
| Rotate dir -> process_rotate dir
| Skip -> state
```

**Best for:**
- Parsing problems with recursive grammars
- State machines with many states
- Problems requiring backtracking
- Tree/graph traversal with immutable structures

### Rust (2023)

**Where it shines:**
- **Type system**: Catches errors at compile time
- **Error handling**: `Result<T, E>` forces handling all cases
- **Iterators**: Expressive and zero-cost
- **No null**: `Option<T>` eliminates null pointer errors
- **Ownership**: Memory safety without GC overhead
- **Cargo ecosystem**: Great tooling and libraries

**Example strength:**
```rust
// Iterator chains are expressive and optimized
numbers
    .iter()
    .filter(|&n| n % 2 == 0)
    .map(|n| n * n)
    .sum()
```

**Best for:**
- Problems requiring complex state management
- Performance-critical solutions
- Grid/array manipulation with bounds checking
- Problems where you want compiler to catch logic errors

### C++ (2024)

**Where it shines:**
- **Performance**: Raw speed when needed
- **STL algorithms**: Rich standard library
- **Templates**: Generic programming
- **Control**: Fine-grained memory and performance control
- **Mature**: Decades of optimization and libraries

**Example strength:**
```cpp
// STL algorithms are composable and fast
std::vector<int> nums = {1, 2, 3, 4, 5};
auto it = std::find_if(nums.begin(), nums.end(),
    [](int n) { return n > 3; });
```

**Best for:**
- Raw performance requirements
- Problems with tight memory constraints
- Numeric computation
- When you need maximum control

## Weaknesses by Language

### OCaml

- Limited libraries compared to mainstream languages
- Steeper learning curve for imperative programmers
- Less community support for AOC-specific tasks
- Can be verbose for simple imperative algorithms
- Module system complexity for small projects

### Rust

- Long compilation times
- Steep learning curve (ownership, lifetimes)
- Can be overly strict for quick prototypes
- Sometimes fights you on valid but complex patterns
- Verbose error types for simple scripts

### C++

- No memory safety by default
- Easy to write undefined behavior
- Verbose compared to modern languages
- Manual memory management overhead
- No built-in package manager (until recently)
- Complex build systems

## Problem Type Analysis

### Parsing Heavy Problems

**Winner: OCaml**
- Pattern matching makes parsing natural
- Parser combinators are elegant
- Algebraic types model grammar rules well

**Example problem:** Day parsing complex instruction sets

### Grid/Map Problems

**Winner: Rust = C++**
- Both offer fast array access
- C++ slightly faster raw access
- Rust offers bounds checking safety
- OCaml can be slower with immutable structures

**Example problem:** Pathfinding on 2D grids

### String Manipulation

**Winner: Depends**
- OCaml: Immutable strings are safe but create copies
- Rust: `String` vs `&str` is powerful but complex
- C++: `std::string` is mutable but can be error-prone

### Recursive Algorithms

**Winner: OCaml**
- Tail call optimization guaranteed
- Natural syntax for recursion
- Immutability prevents mutation bugs

**Example problem:** Tree traversal, backtracking

### Dynamic Programming

**Winner: C++ = Rust**
- Mutable arrays are more natural
- Better performance for large tables
- OCaml's immutability can be limiting

### Graph Algorithms

**Winner: Rust**
- Type system helps model complex state
- Ownership prevents double-free in graph cleanup
- Iterator chains make BFS/DFS elegant

## Development Speed

**Fastest to solution:**
1. OCaml (if you know it well)
2. C++ (if problem is straightforward)
3. Rust (fighting borrow checker takes time)

**Fastest to correct solution:**
1. Rust (compiler catches many bugs)
2. OCaml (type system catches logic errors)
3. C++ (runtime errors more common)

## Code Verbosity

**Most concise:**
1. OCaml (type inference, pattern matching)
2. Rust (modern syntax, but verbose error handling)
3. C++ (lots of boilerplate)

**Example - Reading lines:**
```ocaml
(* OCaml: 1 line *)
let lines = In_channel.with_open_text filename In_channel.input_lines
```

```rust
// Rust: 3-4 lines
let file = File::open(filename)?;
let reader = BufReader::new(file);
let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
```

```cpp
// C++: 6-8 lines
std::ifstream file(filename);
std::vector<std::string> lines;
std::string line;
while (std::getline(file, line)) {
    lines.push_back(line);
}
```

## Error Handling

**Most explicit:**
1. Rust (`Result<T, E>` everywhere)
2. OCaml (`option` and `result` types)
3. C++ (exceptions or error codes)

**Most ergonomic:**
1. OCaml (pattern matching on results)
2. Rust (`?` operator is nice)
3. C++ (exceptions can be invisible)

## Compile Times

**Fastest:**
1. OCaml (~seconds for full build)
2. C++ (~seconds per day)
3. Rust (minutes for full workspace)

## Runtime Performance

For AOC problems specifically:

**Typical ranking:**
1. C++ (when optimized)
2. Rust (close to C++)
3. OCaml (still very fast, ~2-3x slower)

**Note:** For AOC, the difference is usually negligible. Most solutions run in <1s regardless of language.

## Memory Usage

**Most efficient:**
1. C++ (full control)
2. Rust (no GC, ownership model)
3. OCaml (GC overhead, but still good)

## Tooling Experience

### Best Build System
**Cargo (Rust)** - Clear winner
- Dependency management
- Built-in testing
- Standardized structure
- Excellent documentation

### Best Formatter
1. rustfmt (Rust) - Opinionated, consistent
2. ocamlformat (OCaml) - Good but requires config
3. clang-format (C++) - Highly configurable but complex

### Best Language Server
1. rust-analyzer (Rust) - Fast, accurate
2. clangd (C++) - Good but slower
3. ocamllsp (OCaml) - Works but less mature

## When to Use Each Language

### Use OCaml when:
- Learning functional programming
- Problem involves complex parsing
- You want elegant, concise solutions
- Performance isn't the top priority
- You enjoy pattern matching and algebraic types

### Use Rust when:
- You want the compiler to catch bugs
- Performance matters
- Learning systems programming
- You need complex state management
- You want confidence in correctness

### Use C++ when:
- Maximum performance required
- You're comfortable with manual memory management
- Using existing C++ libraries
- Learning low-level concepts
- Time-critical numeric computation

## Interesting Observations

### Productivity Differences
- **Day 1 in new language:** C++ fastest, Rust slowest
- **Day 10 in same language:** All similar productivity
- **Complex problems:** OCaml often fastest to solution

### Bug Patterns
- **C++:** Segfaults, off-by-one, memory leaks
- **Rust:** Ownership errors, lifetime issues (compile-time)
- **OCaml:** Type mismatches, incomplete pattern matches (compile-time)

### Refactoring Confidence
1. **Rust** - Compiler catches everything
2. **OCaml** - Type system catches most issues
3. **C++** - Runtime testing required

### Learning Curve
- **OCaml:** Hard to start, smooth after initial hurdle
- **Rust:** Very hard at first, gradually easier
- **C++:** Easy to start, hard to master

## Language Evolution

### Modern Features Used

**C++ (C++17):**
- Structured bindings
- `std::optional`
- `if constexpr`
- Class template argument deduction

**Rust (2021 edition):**
- `?` operator
- `impl Trait`
- Pattern matching improvements
- Better error messages

**OCaml (5.x):**
- Effects (not yet used in AOC)
- Multicore domains
- Improved type inference

## Recommendations

### For Learning
**Start with:** OCaml (teaches functional thinking) or Rust (teaches ownership)
**Avoid:** C++ (too easy to write bad code while learning)

### For Speed Solving
**Best choice:** Language you know best
**Runner up:** C++ (if you're experienced)

### For Correctness
**Best choice:** Rust (compiler helps)
**Runner up:** OCaml (type system helps)

### For Fun
**Best choice:** Try a new language each year!

## Conclusion

There's no "best" language for Advent of Code:

- **OCaml** excels at elegance and parsing
- **Rust** excels at safety and correctness
- **C++** excels at raw performance and control

The best language is:
1. The one you're learning
2. The one you enjoy using
3. The one that fits the problem

## Future Years

**Languages to consider:**
- **Go:** Simplicity, great for concurrent problems
- **Haskell:** Pure functional, lazy evaluation
- **Zig:** Modern C alternative
- **Python:** Rapid prototyping, rich libraries
- **Julia:** Scientific computing focus

Each would bring unique insights and trade-offs!
