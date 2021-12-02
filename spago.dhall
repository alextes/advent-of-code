{ name = "my-project"
, dependencies =
  [ "arrays"
  , "assert"
  , "console"
  , "effect"
  , "foldable-traversable"
  , "integers"
  , "node-buffer"
  , "node-fs"
  , "partial"
  , "prelude"
  , "psci-support"
  , "strings"
  ]
, packages = ./packages.dhall
, sources = [ "src/**/*.purs", "test/**/*.purs" ]
}
