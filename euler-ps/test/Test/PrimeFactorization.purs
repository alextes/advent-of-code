module Test.PrimeFactorization where

import Prelude

import Data.List ((:))
import Data.List as List
import Data.Maybe (Maybe(..))
import JS.BigInt as BigInt
import PrimeFactorization (isPrime, largestPrimeFactor, primeFactors)
import Test.Spec (Spec, describe, it)
import Test.Spec.Assertions (shouldEqual)

tests :: Spec Unit
tests =
  describe "PrimeFactorization" do
    it "isPrime - checks for prime numbers" do
      isPrime 1 `shouldEqual` false
      isPrime 2 `shouldEqual` true
      isPrime 3 `shouldEqual` true
      isPrime 4 `shouldEqual` false
      isPrime 13 `shouldEqual` true
      isPrime 29 `shouldEqual` true

    it "primeFactors - finds the prime factors" do
      primeFactors (BigInt.fromInt 1) `shouldEqual` List.Nil
      primeFactors (BigInt.fromInt 12) `shouldEqual` ((BigInt.fromInt 2) : (BigInt.fromInt 2) : (BigInt.fromInt 3) : List.Nil)
      primeFactors (BigInt.fromInt 20) `shouldEqual` ((BigInt.fromInt 2) : (BigInt.fromInt 2) : (BigInt.fromInt 5) : List.Nil)
      primeFactors (BigInt.fromInt 100) `shouldEqual` ((BigInt.fromInt 2) : (BigInt.fromInt 2) : (BigInt.fromInt 5) : (BigInt.fromInt 5) : List.Nil)
      primeFactors (BigInt.fromInt 13195) `shouldEqual` ((BigInt.fromInt 5) : (BigInt.fromInt 7) : (BigInt.fromInt 13) : (BigInt.fromInt 29) : List.Nil)

    it "largestPrimeFactor - finds the largest prime factor" do
      largestPrimeFactor "1" `shouldEqual` Nothing
      largestPrimeFactor "12" `shouldEqual` Just (BigInt.fromInt 3)
      largestPrimeFactor "20" `shouldEqual` Just (BigInt.fromInt 5)
      largestPrimeFactor "100" `shouldEqual` Just (BigInt.fromInt 5)
      largestPrimeFactor "13195" `shouldEqual` Just (BigInt.fromInt 29)
