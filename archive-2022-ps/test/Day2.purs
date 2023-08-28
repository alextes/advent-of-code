module Day2Spec where

import Prelude

import Day2 as Day2
import Effect (Effect)
import Effect.Aff as Aff
import Effect.Class (liftEffect)
import Input as Input
import Test.Spec (describe, it)
import Test.Spec.Assertions (shouldEqual)
import Test.Spec.Reporter.Console (consoleReporter)
import Test.Spec.Runner (runSpec)

main :: Effect Unit
main = Aff.launchAff_ $ runSpec [ consoleReporter ] do
  describe "rockPaperScissors" do
    it "should simulate the score outcome" do
      input2 <- Input.readInput 2 # liftEffect
      let
        scoreSum = Day2.scoreFromInput input2
      scoreSum `shouldEqual` 11449
      pure unit
