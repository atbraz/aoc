let read_file filename = In_channel.with_open_bin filename In_channel.input_all

let () =
    try
      let contents = read_file "sample" in
          Printf.printf "File contents: %s\n" contents
    with Sys_error msg -> Printf.printf "Error opening file: %s\n" msg
