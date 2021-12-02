module Test.Main where

import Prelude
import Day1 as Day1
import Effect (Effect)
import Test.Assert as Assert

main :: Effect Unit
main = do
  puzzleInput <- Day1.getPuzzleInput
  let
    solution1 = Day1.getSolution1 puzzleInput

    solution2 = Day1.getSolution2 puzzleInput
  Assert.assertEqual' "day 1, solution 1" { actual: solution1, expected: 1301 } 
  Assert.assertEqual' "day 1, solution 2" { actual: solution2, expected: 1346 }
