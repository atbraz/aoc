open Aoc_common_ocaml

let parse_input filename =
  let lines = In_channel.with_open_text filename In_channel.input_lines in
  lines

let part1 lines =
  (* TODO: Implement part 1 *)
  ignore lines;
  0

let part2 lines =
  (* TODO: Implement part 2 *)
  ignore lines;
  0

let () =
  if Array.length Sys.argv < 2 then (
    Printf.eprintf "Usage: %s <input_file>\n" Sys.argv.(0);
    exit 1
  );

  let filename = Sys.argv.(1) in
  let lines = parse_input filename in

  Printf.printf "Part 1: %d\n" (part1 lines);
  Printf.printf "Part 2: %d\n" (part2 lines)
