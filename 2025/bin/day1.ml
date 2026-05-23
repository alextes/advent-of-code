(** https://adventofcode.com/2025/day/1 *)

let () =
  let file = Aoc2025.Day1.read_file "day1" in
  Printf.printf "%d\n" (Aoc2025.Day1.solve1 file)
