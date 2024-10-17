module Test.Main where

import Prelude

import Effect (Effect)
import Test.Day1 (day1Spec)
import Test.Spec.Reporter (consoleReporter)
import Test.Spec.Runner.Node (runSpecAndExitProcess)

main :: Effect Unit
main = runSpecAndExitProcess [ consoleReporter ] do
  day1Spec
