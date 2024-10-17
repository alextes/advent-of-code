open Jest;

open Day1;

open Expect;

test("Single pair", () =>
  expect(getPair([|1, 2|], 0)) |> toEqual((1, 2))
);

test("Last pair", () =>
  expect(getPair([|1, 2|], 1)) |> toEqual((2, 1))
);

test("Empty array", () =>
  expect(() =>
    getPair([||], 0)
  ) |> toThrow
);

test("Out of bounds", () =>
  expect(() =>
    getPair([||], -1)
  ) |> toThrow
);
