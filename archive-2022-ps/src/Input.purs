module Input where

import Prelude

import Data.List.NonEmpty (NonEmptyList)
import Data.List.NonEmpty as NonEmptyList
import Data.Maybe (Maybe(..))
import Data.String (Pattern(..))
import Data.String as String
import Effect (Effect)
import Node.Encoding as Encoding
import Node.FS.Sync (readTextFile)
import Partial.Unsafe (unsafeCrashWith)

-- given a number, read the "day1.txt" file from the input dir, replacing the numeber.
readInput :: Int -> Effect (NonEmptyList String)
readInput n =
  readInputFile <#> String.split (Pattern "\n") <#> NonEmptyList.fromFoldable <#> expectNonEmptyInput
  where
  path = "input/day" <> show n <> ".txt"
  readInputFile = readTextFile Encoding.UTF8 path
  expectNonEmptyInput = case _ of
    Just list -> list
    Nothing -> unsafeCrashWith "Empty input file"

main :: Effect Unit
main = pure unit
