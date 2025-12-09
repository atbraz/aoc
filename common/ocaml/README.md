# aoc-common-ocaml

Shared utilities for Advent of Code solutions in OCaml.

## Overview

This library provides common functionality to reduce boilerplate in daily puzzle solutions. Currently minimal, designed to grow as common patterns emerge.

## Installation

This is a local library managed through dune. It's automatically available to all 2022 solutions that declare it as a dependency.

### In Your Day's `dune` File

```lisp
(executable
 (name main)
 (public_name aoc-2022-5)
 (package aoc-2022)
 (libraries aoc-common-ocaml))
```

The `just new` command automatically adds this for you.

## Usage

### File Reading

The library currently provides a single utility function for reading files:

```ocaml
open Aoc_common_ocaml

let () =
  let filename = "input" in
  let content = read_file filename in
  (* content is a single string with the entire file contents *)
  ...
```

#### Function Signature

```ocaml
val read_file : string -> string
```

Reads the entire file into a single string, preserving all content including newlines.

### Common Patterns

Since the library is minimal, here are common patterns you'll use repeatedly:

#### Reading Lines

```ocaml
let read_lines filename =
  In_channel.with_open_text filename In_channel.input_lines

let () =
  let lines = read_lines "input" in
  List.iter (fun line ->
    (* process each line *)
    Printf.printf "%s\n" line
  ) lines
```

#### Reading as Single String

```ocaml
open Aoc_common_ocaml

let content = read_file "input"
```

#### Parsing Line by Line

```ocaml
let parse_input filename =
  let lines = In_channel.with_open_text filename In_channel.input_lines in
  List.map (fun line ->
    (* parse each line into your data structure *)
    Scanf.sscanf line "%d,%d" (fun x y -> (x, y))
  ) lines
```

#### Reading Paragraphs

Split on blank lines:

```ocaml
let read_paragraphs filename =
  let content = read_file filename in
  String.split_on_char '\n' content
  |> List.fold_left (fun (acc, current) line ->
      if String.trim line = "" then
        (List.rev current :: acc, [])
      else
        (acc, line :: current)
    ) ([], [])
  |> fun (acc, current) -> List.rev (List.rev current :: acc)
  |> List.filter (fun para -> para <> [])
```

#### Reading Character Grid

```ocaml
let read_grid filename =
  let lines = In_channel.with_open_text filename In_channel.input_lines in
  List.map (fun line ->
    String.to_seq line |> List.of_seq
  ) lines

let () =
  let grid = read_grid "input" in
  (* grid : char list list *)

  (* Access by position *)
  let cell = List.nth (List.nth grid y) x in
  ...
```

## Type Safety Patterns

OCaml's type system encourages different patterns than imperative languages:

### Option Types

```ocaml
(* Instead of exceptions, use Option *)
let find_first pred list =
  List.find_opt pred list

let () =
  match find_first (fun x -> x > 10) numbers with
  | Some value -> Printf.printf "Found: %d\n" value
  | None -> Printf.printf "Not found\n"
```

### Result Types

```ocaml
type error = Parse_error of string | File_error of string

let parse_int str =
  try Ok (int_of_string str)
  with Failure _ -> Error (Parse_error str)

let solve filename =
  match parse_input filename with
  | Ok data ->
      let result = compute data in
      Printf.printf "Result: %d\n" result
  | Error (Parse_error msg) ->
      Printf.eprintf "Parse error: %s\n" msg
```

### Pattern Matching

```ocaml
(* Exhaustive matching on custom types *)
type direction = North | South | East | West

let move (x, y) dir =
  match dir with
  | North -> (x, y - 1)
  | South -> (x, y + 1)
  | East -> (x + 1, y)
  | West -> (x - 1, y)

(* Matching on lists *)
let rec process = function
  | [] -> 0
  | x :: xs -> x + process xs

(* Matching with guards *)
let classify n =
  match n with
  | n when n < 0 -> "negative"
  | 0 -> "zero"
  | n when n > 100 -> "large"
  | _ -> "positive"
```

## Complete Example

```ocaml
open Aoc_common_ocaml

let parse_input filename =
  In_channel.with_open_text filename In_channel.input_lines
  |> List.map (fun line ->
      Scanf.sscanf line "%d" (fun n -> n)
    )

let part1 numbers =
  List.fold_left (+) 0 numbers

let part2 numbers =
  numbers
  |> List.filter (fun n -> n mod 2 = 0)
  |> List.fold_left (+) 0

let () =
  if Array.length Sys.argv < 2 then (
    Printf.eprintf "Usage: %s <input_file>\n" Sys.argv.(0);
    exit 1
  );

  let filename = Sys.argv.(1) in
  let numbers = parse_input filename in

  Printf.printf "Part 1: %d\n" (part1 numbers);
  Printf.printf "Part 2: %d\n" (part2 numbers)
```

## Useful Standard Library Modules

OCaml's standard library has many useful modules:

### List Module

```ocaml
List.map       (* transform each element *)
List.filter    (* keep elements matching predicate *)
List.fold_left (* reduce from left *)
List.fold_right (* reduce from right *)
List.iter      (* side effects on each element *)
List.length    (* get length *)
List.nth       (* get nth element (0-indexed) *)
List.find_opt  (* find first matching element *)
List.sort      (* sort with comparator *)
```

### String Module

```ocaml
String.length       (* string length *)
String.sub          (* substring *)
String.split_on_char (* split string *)
String.trim         (* remove whitespace *)
String.starts_with  (* check prefix *)
String.ends_with    (* check suffix *)
String.contains     (* check if contains char *)
```

### Array Module

```ocaml
Array.make         (* create array with default value *)
Array.init         (* create array with function *)
Array.map          (* transform elements *)
Array.fold_left    (* reduce array *)
Array.iter         (* iterate with side effects *)
Array.sort         (* in-place sort *)
```

### Hashtbl Module

```ocaml
let table = Hashtbl.create 100 in
Hashtbl.add table "key" 42;
Hashtbl.find_opt table "key"  (* Some 42 *)
```

### Seq Module

For lazy sequences:

```ocaml
String.to_seq "hello"
|> Seq.filter (fun c -> c <> 'l')
|> Seq.map Char.uppercase_ascii
|> String.of_seq  (* "HEO" *)
```

## Design Philosophy

- **Minimal**: Only extract truly common patterns
- **Functional**: Prefer immutability and pure functions
- **Explicit**: No magic, clear function signatures
- **Type-Safe**: Leverage OCaml's type system
- **Standard-First**: Use standard library when possible

## Adding New Utilities

When you find yourself writing the same code across multiple days:

1. Add the function to `aoc_common_ocaml.ml`
2. Add the signature to `aoc_common_ocaml.mli` (if it exists)
3. Run `dune build` to ensure it compiles
4. Update this README with examples
5. Use it in your solutions to verify it works

Common candidates for extraction:
- Parsing helpers (grids, coordinates, patterns)
- Common algorithms (BFS, DFS, dijkstra)
- Data structure utilities (priority queues, graphs)
- Math functions (GCD, LCM, modular arithmetic)

## OCaml 5.x Features

This project uses OCaml 5.3+, which includes:

- **Effects**: Algebraic effects and handlers
- **Multicore**: Parallel programming with domains
- **Improved Performance**: Better runtime optimizations

Most AOC problems won't need these features, but they're available if needed:

```ocaml
(* Domains for parallelism *)
let d1 = Domain.spawn (fun () -> compute_part1 data) in
let d2 = Domain.spawn (fun () -> compute_part2 data) in
let r1 = Domain.join d1 in
let r2 = Domain.join d2 in
Printf.printf "Part 1: %d\nPart 2: %d\n" r1 r2
```

## Resources

- [OCaml Manual](https://ocaml.org/manual/)
- [OCaml Standard Library](https://ocaml.org/api/)
- [Real World OCaml](https://dev.realworldocaml.org/)
- [Dune Documentation](https://dune.readthedocs.io/)
