(** https://adventofcode.com/2025/day/1 *)

type dial = Dial of int
type dial_state = { dial : dial; zero_hits : int }
type direction = L | R
type rotation = { direction : direction; distance : int }

let dial_value (Dial n) = n
let normalize n = ((n mod 100) + 100) mod 100
let make_dial n = Dial (normalize n)

let turn dial rotation =
  let current = dial_value dial in
  let distance =
    match rotation.direction with
    | L -> -rotation.distance
    | R -> rotation.distance
  in
  make_dial (current + distance)

let initial_state = { dial = make_dial 50; zero_hits = 0 }

let read_file filename =
  In_channel.with_open_text
    (Printf.sprintf "input/%s.txt" filename)
    In_channel.input_all

let parse_rotation line =
  let direction =
    match String.get line 0 with
    | 'L' -> L
    | 'R' -> R
    | _ -> failwith "Invalid direction"
  in
  let distance = int_of_string (String.sub line 1 (String.length line - 1)) in
  { direction; distance }

let lines input =
  input |> String.split_on_char '\n'
  |> List.filter (fun line -> String.trim line <> "")

let rotations = List.map parse_rotation

let apply_rotation state rotation =
  let dial = turn state.dial rotation in
  let zero_hits = state.zero_hits + if dial_value dial = 0 then 1 else 0 in
  { dial; zero_hits }

let solve1 input =
  let parsed = input |> lines |> rotations in
  let final_state = List.fold_left apply_rotation initial_state parsed in
  final_state.zero_hits
