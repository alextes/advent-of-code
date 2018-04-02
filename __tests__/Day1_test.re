open Jest;

open Day1;

open Expect;

test("Single pair", () =>
  expect(Day1.getPair([|1, 2|], 0)) |> toEqual((1, 2))
);

test("Last pair", () =>
  expect(Day1.getPair([|1, 2|], 1)) |> toEqual((2, 1))
);

test("Empty array", () =>
  expect(() =>
    Day1.getPair([||], 0)
  ) |> toThrow
);

test("Out of bounds", () =>
  expect(() =>
    Day1.getPair([||], -1)
  ) |> toThrow
);