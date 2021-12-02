module Day1 (getPuzzleInput, getSolution1, getSolution2) where

import Prelude
import Data.Array as Array
import Data.Foldable as Foldable
import Data.Int as Int
import Data.Maybe (Maybe(..))
import Data.String (Pattern(..))
import Data.String as Str
import Effect (Effect)
import Node.Encoding (Encoding(..))
import Node.FS.Sync (readTextFile)
import Partial.Unsafe (unsafeCrashWith)

type PuzzleInput
  = Array Int

getPuzzleInput :: Effect PuzzleInput
getPuzzleInput =
  readTextFile UTF8 "./input/day-1-input"
    <#> Str.split (Pattern "\n")
    >>> map Int.fromString
    >>> Array.catMaybes

data DepthChange
  = Increase
  | Decrease
  | NoChange

derive instance depthChangeEq :: Eq DepthChange

type Depth
  = Int

window :: ∀ a. Int -> Array a -> Array (Array a)
window n nums = Array.mapWithIndex mapToTuple nums # Array.catMaybes
  where
  mapToTuple :: Int -> a -> Maybe (Array a)
  mapToTuple i _ =
    Array.drop i nums
      # Array.take n
      >>> \inScope -> case (Array.length inScope) == n of
          true -> Just inScope
          false -> Nothing

depthChangeFromPair :: Array Depth -> DepthChange
depthChangeFromPair [ first, second ] = case compare first second of
  GT -> Decrease
  LT -> Increase
  EQ -> NoChange

depthChangeFromPair _ = unsafeCrashWith "depthChangeFromPair can only handle arrays of length 2"

countMatching :: forall a. Eq a => (a -> Boolean) -> Array a -> Int
countMatching eqFn = Array.filter eqFn >>> Array.length

getSolution1 :: PuzzleInput -> Int
getSolution1 =
  window 2
    >>> map depthChangeFromPair
    >>> countMatching (eq Increase)

getSolution2 :: PuzzleInput -> Int
getSolution2 =
  window 3
    >>> map Foldable.sum
    >>> window 2
    >>> map depthChangeFromPair
    >>> countMatching (eq Increase)
