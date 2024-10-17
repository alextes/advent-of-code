module Day1 where

import Prelude

import Control.Alt ((<|>))
import Data.Array as Array
import Data.Array.NonEmpty (NonEmptyArray)
import Data.Array.NonEmpty as NonEmptyArray
import Data.Either (Either(..))
import Data.Foldable as Foldable
import Data.Maybe (Maybe(..), fromJust)
import Data.Maybe as Maybe
import Data.Newtype (overF)
import Data.String (Pattern(..))
import Data.String as String
import Data.String.CodeUnits as CodeUnits
import Data.String.Regex as Regex
import Data.String.Regex.Flags as RegexFlags
import Data.Tuple (Tuple)
import Data.Tuple as Tuple
import Data.Tuple.Nested ((/\))
import Debug as Debug
import Effect (Effect)
import Input as Input
import Partial (crashWith)
import Partial.Unsafe (unsafeCrashWith, unsafePartial)

-- numberPattern :: String
-- numberPattern = "(\\d|zero|one|two|three|four|five|six|seven|eight|nine)"
--
-- numberRegex :: Regex.Regex
-- numberRegex = case Regex.regex numberPattern RegexFlags.global of
--   Left error -> unsafeCrashWith error
--   Right regex -> regex

intStrings :: Array Pattern
intStrings = map Pattern [ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9" ]

spelledOutIntStrings :: Array Pattern
spelledOutIntStrings = map Pattern [ "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ]

-- -- Parser that handles only string digits
-- parseChar :: String -> Maybe Int
-- parseChar "0" = Just 0
-- parseChar "1" = Just 1
-- parseChar "2" = Just 2
-- parseChar "3" = Just 3
-- parseChar "4" = Just 4
-- parseChar "5" = Just 5
-- parseChar "6" = Just 6
-- parseChar "7" = Just 7
-- parseChar "8" = Just 8
-- parseChar "9" = Just 9
-- parseChar _ = Nothing
--
-- parseSpelledOut :: String -> Maybe Int
-- parseSpelledOut "zero" = Just 0
-- parseSpelledOut "one" = Just 1
-- parseSpelledOut "two" = Just 2
-- parseSpelledOut "three" = Just 3
-- parseSpelledOut "four" = Just 4
-- parseSpelledOut "five" = Just 5
-- parseSpelledOut "six" = Just 6
-- parseSpelledOut "seven" = Just 7
-- parseSpelledOut "eight" = Just 8
-- parseSpelledOut "nine" = Just 9
-- parseSpelledOut _ = Nothing

-- -- Parser that handles string digits and word digits
-- parseCharOrSpelledOut :: String -> Maybe Int
-- parseCharOrSpelledOut str = parseChar str <|> parseSpelledOut str

-- Turn our non-empty and non-empty-maybe array to a pair of first & last integers
findIntPairFromMatches :: NonEmptyArray Int -> Tuple Int Int
findIntPairFromMatches parsedInts =
  let
    _ = Debug.spy "parsedInts" parsedInts
    firstInt = NonEmptyArray.head parsedInts
    lastInt = NonEmptyArray.last parsedInts
  in
    Tuple.Tuple firstInt lastInt

-- turn our pair of ints into a single int by concatenating the two
-- i.e. taking the first int to be the number of tens.
intFromIntPair :: Tuple Int Int -> Int
intFromIntPair (Tuple.Tuple first last) = first * 10 + last

findAllMatches :: Array Pattern -> String -> NonEmptyArray Int
findAllMatches patterns str =
  let
    (maybeMatches :: Array (Maybe (Tuple Int Int))) =
      Array.mapWithIndex
        ( \index pattern -> case String.indexOf pattern str of
            -- the first int is how early in the string the match is found, used to select first and last
            -- the second int is what number the pattern actually represents
            Just matchIndex -> Just $ matchIndex /\ index
            Nothing -> Nothing
        )
        patterns

    matches = Array.catMaybes maybeMatches
    _ = Debug.spy "matches" matches
    (sortedMatchesWithIndicies :: Array (Tuple Int Int)) = Array.sortWith Tuple.fst matches
    sortedMatches = map Tuple.snd sortedMatchesWithIndicies

  in
    unsafePartial fromJust $ NonEmptyArray.fromArray sortedMatches

-- Overall function to parse a line and get a number, or Nothing
intFromLine :: Array Pattern -> String -> Int
intFromLine parser line =
  findAllMatches parser line
    # findIntPairFromMatches
    # intFromIntPair

solutionExample1 :: Effect Int
solutionExample1 = Input.readDayExampleInput 1 1
  <#> NonEmptyArray.fromFoldable1
  <#> map (intFromLine intStrings)
  <#> Foldable.sum

solution1 :: Effect Int
solution1 = Input.readDayInput 1
  <#> NonEmptyArray.fromFoldable1
  <#> map (\line -> intFromLine intStrings line)
  <#> Foldable.sum

solutionExample2 :: Effect Int
solutionExample2 = Input.readDayExampleInput 1 2
  <#> NonEmptyArray.fromFoldable1
  <#> (\lines -> Debug.spy "lines: " lines)
  <#> map (intFromLine intAndSpelledOutIntStrings)
  <#> Foldable.sum

solution2 :: Effect Int
solution2 = Input.readDayInput 1
  <#> NonEmptyArray.fromFoldable1
  <#> map (intFromLine intAndSpelledOutIntStrings)
  <#> Foldable.sum
