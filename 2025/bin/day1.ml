(** https://adventofcode.com/2025/day/1 *)

let () =
  let file = Aoc2025.Day1.read_file "day1" in
  Printf.printf "Part 1: %d\n" (Aoc2025.Day1.solve1 file);
  Printf.printf "Part 2: %d\n" (Aoc2025.Day1.solve2 file)
