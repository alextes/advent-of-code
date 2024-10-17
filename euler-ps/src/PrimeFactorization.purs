-- Problem 3
module PrimeFactorization (isPrime, primeFactors, largestPrimeFactor) where

import Prelude

import Data.List (List(..), (..), (:))
import Data.List as List
import Data.List.NonEmpty (NonEmptyList)
import Data.List.NonEmpty as NonEmptyList
import Data.Maybe (Maybe)
import Data.Maybe as Maybe
import JS.BigInt (BigInt)
import JS.BigInt as BigInt
import Partial.Unsafe (unsafePartial)

isDivisible :: Int -> Int -> Boolean
isDivisible n m = mod n m # eq 0

isPrime :: Int -> Boolean
isPrime 0 = false
isPrime 1 = false
isPrime 2 = true
isPrime 3 = true
isPrime n = not $ List.any (isDivisible n) (2 .. (n - 1))

-- Primes up to 10k
primes :: NonEmptyList Int
primes = List.filter isPrime (List.range 2 10000) # NonEmptyList.fromList # unsafePartial Maybe.fromJust

primesBigInt :: NonEmptyList BigInt
primesBigInt = map BigInt.fromInt primes

-- Prime factors of a number
primeFactors :: BigInt -> List BigInt
primeFactors n = helper n (NonEmptyList.toList primesBigInt)
  where
  helper :: BigInt -> List BigInt -> List BigInt
  helper _ Nil = Nil
  helper num _ | num == BigInt.fromInt 1 = Nil
  helper num (prime : rest)
    | num `mod` prime == BigInt.fromInt 0 =
        prime : helper (num / prime) nextPrimes
        where
        nextPrimes =
          if (num / prime) `mod` prime == BigInt.fromInt 0 then prime : rest
          else rest
    | otherwise = helper num rest

largestPrimeFactor :: String -> Maybe BigInt
largestPrimeFactor n = do
  let number = BigInt.fromString n # unsafePartial Maybe.fromJust
  primeFactors number # List.last
