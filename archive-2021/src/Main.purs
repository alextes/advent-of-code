module Main where

import Prelude

import Day1 as Day1
import Effect (Effect)
import Effect.Class.Console (logShow)

main :: Effect Unit
main = do
  puzzleInput <- Day1.getPuzzleInput
  logShow $ Day1.getSolution1 puzzleInput
  logShow $ Day1.getSolution2 puzzleInput
