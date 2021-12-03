module Day2 (getPuzzleInput, getSolution1, getSolution2) where

import Prelude
import Data.Array as Array
import Data.Foldable as Foldable
import Data.Int as Int
import Data.Maybe as Maybe
import Data.Newtype (class Newtype)
import Data.Newtype as Newtype
import Data.String as String
import Data.Tuple (Tuple)
import Data.Tuple as Tuple
import Data.Tuple.Nested (type (/\), (/\))
import Data.Tuple.Nested as Nested
import Effect (Effect)
import Node.Encoding as Encoding
import Node.FS.Sync as Sync
import Partial.Unsafe (unsafeCrashWith, unsafePartial)
import Unsafe.Coerce (unsafeCoerce)

type PuzzleInput
  = Array String

getPuzzleInput :: Effect PuzzleInput
getPuzzleInput =
  Sync.readTextFile Encoding.UTF8 "./input/day-2-input"
    <#> String.split (String.Pattern "\n")
    <#> Array.dropEnd 1

unsafeIntFromString :: String -> Int
unsafeIntFromString = Int.fromString >>> unsafePartial Maybe.fromJust

instructionFromString :: String -> PositionChange1
instructionFromString =
  String.split (String.Pattern " ")
    >>> case _ of
        [ instruction, amountStr ] -> unsafeIntFromString amountStr # fromRawInstruction instruction
        strs ->
          unsafeCrashWith
            $ "instruction: "
            <> String.joinWith "," strs
            <> " not in format 'word 0'"
    >>> PositionChange1
  where
  fromRawInstruction :: String -> Int -> Tuple Int Int
  fromRawInstruction instruction amount = case instruction of
    "forward" -> 0 /\ amount
    "up" -> -1 * amount /\ 0
    "down" -> amount /\ 0
    str -> unsafeCrashWith $ "unexpected instruction: " <> str

positionChange2FromPositionChange1 :: PositionChange1 -> PositionChange2
positionChange2FromPositionChange1 (PositionChange1 (d /\ h)) =  d /\ h /\ 0 # Newtype.wrap

type DepthChange
  = Int

type HorizontalChange
  = Int

type Aim
  = Int

newtype PositionChange1
  = PositionChange1 (DepthChange /\ HorizontalChange)

newtype PositionChange2
  = PositionChange2 (DepthChange /\ HorizontalChange /\ Aim)

derive instance positionChange1Newtype :: Newtype PositionChange1 _

derive instance positionChange2Newtype :: Newtype PositionChange2 _

instance positionChange1Semigroup :: Semiring PositionChange1 where
  add (PositionChange1 t1) (PositionChange1 t2) = add t1 t2 # PositionChange1
  zero = PositionChange1 $ 0 /\ 0
  mul = unsafeCoerce unit
  one = unsafeCoerce unit

instance positionChange2Semigroup :: Semiring PositionChange2 where
  add (PositionChange2 (d1 /\ h1 /\ a1)) (PositionChange2 (d2 /\ h2 /\ _)) =
    let
      newAim = a1 + d2
    in
      PositionChange2
        $ (d1 + (h2 * newAim))
        /\ (h1 + h2)
        /\ newAim
  zero = PositionChange2 $ 0 /\ 0 /\ 0
  mul = unsafeCoerce unit
  one = unsafeCoerce unit

productFromPosition1 :: PositionChange1 -> Int
productFromPosition1 = Newtype.unwrap >>> Tuple.uncurry mul

productFromPosition2 :: PositionChange2 -> Int
productFromPosition2 = Newtype.unwrap >>> Nested.uncurry2 mul

getSolution1 :: PuzzleInput -> Int
getSolution1 =
  map instructionFromString
    >>> Foldable.sum
    >>> productFromPosition1

getSolution2 :: PuzzleInput -> Int
getSolution2 =
  map instructionFromString
    >>> map positionChange2FromPositionChange1
    >>> Foldable.sum
    >>> productFromPosition2
