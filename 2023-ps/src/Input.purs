module Input where

import Prelude

import Data.Array as Array
import Data.List.NonEmpty (NonEmptyList)
import Data.List.NonEmpty as NonEmptyList
import Data.Maybe (Maybe(..))
import Data.String (Pattern(..))
import Data.String as String
import Effect (Effect)
import Node.Encoding as Encoding
import Node.FS.Sync (readTextFile)
import Partial.Unsafe (unsafeCrashWith)

-- Given a number n, read the puzzle input of day n.
readInput :: String -> Effect (NonEmptyList String)
readInput n =
  readInputFile
    <#> String.split (Pattern "\n")
    <#> Array.takeWhile (eq "" >>> not)
    <#> NonEmptyList.fromFoldable
    <#> expectNonEmptyInput
  where
  path = "input/day" <> n <> ".txt"
  readInputFile = readTextFile Encoding.UTF8 path
  expectNonEmptyInput = case _ of
    Just list -> list
    Nothing -> unsafeCrashWith "empty input file"

readDayInput :: Int -> Effect (NonEmptyList String)
readDayInput dayN = readInput $ show dayN

readDayExampleInput :: Int -> Int -> Effect (NonEmptyList String)
readDayExampleInput n m = readInput $ show n <> "-example" <> show m
