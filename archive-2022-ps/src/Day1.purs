module Day1 (biggestSum, biggestThreeSums, groupCalories, solution1, solution2) where

import Prelude

import Data.Foldable (sum)
import Data.Int as Int
import Data.List (List)
import Data.List as List
import Data.List.NonEmpty as NonEmptyList
import Data.Maybe (Maybe, isJust)
import Data.Ord as Ord
import Data.Ordering (invert)
import Effect (Effect)
import Effect.Console (logShow)
import Input as Input

groupMaybes :: List (Maybe Int) -> List (List Int)
groupMaybes = List.groupBy (\_ y -> isJust y)
  -- We now have a list of lists of Maybes, the nothings need to be filtered out
  >>> map (NonEmptyList.catMaybes)
  -- Now we may have entirely empty lists, which we need to filter out
  >>> List.filter (List.null >>> not)

-- Sum into calorie groups
-- Groups are delimited by empty strings
groupCalories :: List String -> List Int
groupCalories = map Int.fromString >>> groupMaybes >>> map (sum)

descOrdering :: Int -> Int -> Ordering
descOrdering a b = Ord.compare a b # invert

-- Get the biggest sum
-- Searches the list of sums for the biggest one
biggestSum :: List Int -> Int
biggestSum = NonEmptyList.foldl Ord.max 0

-- Get biggest three sum
biggestThreeSums :: List Int -> Int
biggestThreeSums = List.sortBy descOrdering >>> List.take 3 >>> sum

solution1 :: Effect Unit
solution1 =
  Input.readInput 1
    <#> List.fromFoldable
    <#> groupCalories
    <#> biggestSum
    >>= logShow

solution2 :: Effect Unit
solution2 =
  Input.readInput 1
    <#> List.fromFoldable
    <#> groupCalories
    <#> biggestThreeSums
    >>= logShow
