module Test.Day1 where

import Prelude

import Day1 as Day1
import Effect.Class as Aff
import Test.Spec (Spec, it, itOnly)
import Test.Spec.Assertions (shouldEqual)

day1Spec :: Spec Unit
day1Spec = do
  it "day 1 example 1 solution" do
    exampleSolution <- Aff.liftEffect Day1.solutionExample1
    exampleSolution `shouldEqual` 142
    pure unit

  it "day 1 solution 1" do
    solution1 <- Aff.liftEffect Day1.solution1
    solution1 `shouldEqual` 53080
    pure unit

  itOnly "day 1 example 2 solution" do
    exampleSolution <- Aff.liftEffect Day1.solutionExample2
    exampleSolution `shouldEqual` 281
    pure unit

  it "day 1 solution 2" do
    solution2 <- Aff.liftEffect Day1.solution2
    solution2 `shouldEqual` 53255
    pure unit
