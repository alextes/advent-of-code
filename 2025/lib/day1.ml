(** https://adventofcode.com/2025/day/1 *)

type dial = Dial of int
type dial_state = { dial : dial; crossings : int }
type direction = L | R
type rotation = { direction : direction; distance : int }

let make_dial n =
  if n < 0 || n > 99 then failwith "Dial value must be between 0 and 99"
  else Dial n

let initial_state = { dial = make_dial 50; crossings = 0 }

let read_file filename =
  In_channel.with_open_text
    (Printf.sprintf "input/%s.txt" filename)
    In_channel.input_all

let parse_rotation line = 
  let direction = match String.get line 0 with
    | 'L' -> L
    | 'R' -> R
    | _ -> failwith "Invalid direction" in
  let distance = int_of_string (String.sub line 1 (String.length line - 1)) in
  { direction; distance }

let lines input =  input
  |> String.split_on_char '\n' 
  |> List.filter (fun line -> String.trim line <> "")

let rotations = List.map parse_rotation

let print_rotation r =
  Printf.sprintf "%s%d" (match r.direction with L -> "L" | R -> "R") r.distance

let solve1 input =
  let instruction1 = 
    lines input
    |> List.map parse_rotation
    |> (fun parsed -> List.nth parsed 0)
  in
  Printf.printf "First instruction: %s" (print_rotation instruction1);
  0
