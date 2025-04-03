let read_file filename = In_channel.with_open_bin filename In_channel.input_all

let split_into_groups content =
  let current_group = ref [] in
  let all_groups = ref [] in
    String.split_on_char '\n' content
    |> List.iter (fun line ->
      if line = "" then (
        if List.length !current_group > 0 then (
          all_groups := List.rev !current_group :: !all_groups;
          current_group := []))
      else
        current_group := line :: !current_group);
    if List.length !current_group > 0 then all_groups := List.rev !current_group :: !all_groups;
    List.rev !all_groups


let sum_group group = List.fold_left (fun total item -> total + int_of_string item) 0 group

let () =
  try
    let contents = read_file "sample" in
    let groups = split_into_groups contents in
    let sums = List.map sum_group groups in
      List.iteri (fun i sum -> Printf.printf "Group %d: %d\n" (i + 1) sum) sums;
      let max_sum = List.fold_left max 0 sums in
        Printf.printf "\nMaximum sum: %d\n" max_sum
  with
  | Sys_error msg -> Printf.printf "Error opening file: %s\n" msg
  | Failure msg -> Printf.printf "Parsing error: %s\n" msg
