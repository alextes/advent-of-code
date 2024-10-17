module Day2 where

import Prelude
import Data.Array (updateAt) as Array
import Data.Int as Int
import Data.Maybe (fromJust) as Maybe
import Data.String (Pattern(..))
import Data.String as String
import Effect (Effect)
import Effect.Aff (Aff, runAff_)
import Effect.Class (liftEffect)
import Effect.Console (logShow) as Console
import Node.Buffer (toString)
import Node.Encoding as Encoding
import Node.FS.Aff (readFile)
import Partial.Unsafe (unsafePartial)

readPuzzleInput :: Aff (Array Int)
readPuzzleInput =
  readFile "../src/input2"
    >>= toString Encoding.UTF8
    >>> liftEffect
    >>= String.trim
    >>> String.split (Pattern ",")
    >>> map Int.fromString
    >>> map (unsafePartial Maybe.fromJust)
    >>> pure

main :: Effect Unit
main =
  runAff_ Console.logShow do
    puzzleInput <- readPuzzleInput
    let
      twelveOTwoOpCodes = puzzleInput # Array.updateAt 1 12 >>= Array.updateAt 2 2
    pure twelveOTwoOpCodes
