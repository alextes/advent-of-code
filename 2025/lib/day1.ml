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
let parse_input input = input |> lines |> rotations

let distance_to_zero dial direction =
  let current = dial_value dial in
  match direction with
  | L -> if current = 0 then 100 else current
  | R -> if current = 0 then 100 else 100 - current

let zero_hits_during_rotation dial rotation =
  let first_hit = distance_to_zero dial rotation.direction in
  if rotation.distance < first_hit then 0
  else 1 + ((rotation.distance - first_hit) / 100)

let apply_rotation_part1 state rotation =
  let dial = turn state.dial rotation in
  let zero_hits = state.zero_hits + if dial_value dial = 0 then 1 else 0 in
  { dial; zero_hits }

let apply_rotation_part2 state rotation =
  let dial = turn state.dial rotation in
  let zero_hits =
    state.zero_hits + zero_hits_during_rotation state.dial rotation
  in
  { dial; zero_hits }

let solve1 input =
  let parsed = parse_input input in
  let final_state = List.fold_left apply_rotation_part1 initial_state parsed in
  final_state.zero_hits

let solve2 input =
  let parsed = parse_input input in
  let final_state = List.fold_left apply_rotation_part2 initial_state parsed in
  final_state.zero_hits
