module Day1 (getPuzzleInput, getSolution1, getSolution2) where

import Prelude
import Data.Array as Array
import Data.Foldable as Foldable
import Data.Int as Int
import Data.String (Pattern(..))
import Data.String as Str
import Data.Tuple (Tuple)
import Data.Tuple as Tuple
import Data.Tuple.Nested ((/\))
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
window n nums =
  nums
    # Array.mapWithIndex makeWindow
    >>> Array.filter getIsValidWindow
  where
  makeWindow :: Int -> a -> Array a
  makeWindow i _ = Array.drop i nums # Array.take n

  getIsValidWindow :: (Array a) -> Boolean
  getIsValidWindow = Array.length >>> eq n

depthChangeFromPair :: Depth -> Depth -> DepthChange
depthChangeFromPair first second = case compare first second of
  GT -> Decrease
  LT -> Increase
  EQ -> NoChange

countMatching :: forall a. Eq a => (a -> Boolean) -> Array a -> Int
countMatching eqFn = Array.filter eqFn >>> Array.length

tupleFromArray :: forall a. Array a -> Tuple a a
tupleFromArray arr = case arr of
  [ a, b ] -> a /\ b
  _ -> unsafeCrashWith "unsafeTupleFromArray can only handle arrays of length two"

getSolution1 :: PuzzleInput -> Int
getSolution1 =
  window 2
    >>> map (tupleFromArray >>> Tuple.uncurry depthChangeFromPair)
    >>> countMatching (eq Increase)

getSolution2 :: PuzzleInput -> Int
getSolution2 =
  window 3
    >>> map Foldable.sum
    >>> window 2
    >>> map (tupleFromArray >>> Tuple.uncurry depthChangeFromPair)
    >>> countMatching (eq Increase)
