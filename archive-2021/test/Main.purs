module Test.Main where

import Prelude
import Day1 as Day1
import Day2 as Day2
import Effect (Effect)
import Test.Assert as Assert

day1 :: Effect Unit
day1 = do
  puzzleInput <- Day1.getPuzzleInput
  let
    solution1 = Day1.getSolution1 puzzleInput

    solution2 = Day1.getSolution2 puzzleInput
  Assert.assertEqual' "day 1, solution 1" { actual: solution1, expected: 1301 }
  Assert.assertEqual' "day 1, solution 2" { actual: solution2, expected: 1346 }

day2 :: Effect Unit
day2 = do
  puzzleInput <- Day2.getPuzzleInput
  let
    solution1 = Day2.getSolution1 puzzleInput

    solution2 = Day2.getSolution2 puzzleInput
  Assert.assertEqual' "day 2, solution 1" { actual: solution1, expected: 2091984 }
  Assert.assertEqual' "day 2, solution 2" { actual: solution2, expected: 2086261056}

main :: Effect Unit
main = do
  day1
  day2
