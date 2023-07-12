module Day1Spec where

import Prelude

import Data.List as List
import Data.Maybe (Maybe(..))
import Day1 as Day1
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
  describe "groupCalories" do
    it "groups calories together" do
      input1 <- Input.readInput 1 # liftEffect
      let
        groupedCalories = List.fromFoldable input1 # Day1.groupCalories
        firstGroup = List.head groupedCalories
      firstGroup `shouldEqual` (Just 52551)
      pure unit
