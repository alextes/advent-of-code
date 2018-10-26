// Generated by BUCKLESCRIPT VERSION 4.0.6, PLEASE EDIT WITH CARE
'use strict';

var Jest = require("@glennsl/bs-jest/src/jest.js");
var Day1$AdventOfCode = require("../src/Day1.bs.js");

Jest.test("Single pair", (function () {
        return Jest.Expect[/* toEqual */12](/* tuple */[
                    1,
                    2
                  ], Jest.Expect[/* expect */0](Day1$AdventOfCode.getPair(/* array */[
                            1,
                            2
                          ], 0)));
      }));

Jest.test("Last pair", (function () {
        return Jest.Expect[/* toEqual */12](/* tuple */[
                    2,
                    1
                  ], Jest.Expect[/* expect */0](Day1$AdventOfCode.getPair(/* array */[
                            1,
                            2
                          ], 1)));
      }));

Jest.test("Empty array", (function () {
        return Jest.Expect[/* toThrow */18](Jest.Expect[/* expect */0]((function () {
                          return Day1$AdventOfCode.getPair(/* array */[], 0);
                        })));
      }));

Jest.test("Out of bounds", (function () {
        return Jest.Expect[/* toThrow */18](Jest.Expect[/* expect */0]((function () {
                          return Day1$AdventOfCode.getPair(/* array */[], -1);
                        })));
      }));

/*  Not a pure module */
