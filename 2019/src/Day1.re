exception ParseError(string);
let moduleMasses =
  Node.Fs.readFileAsUtf8Sync("./src/input1")
  ->Js.String.split("\n", _)
  ->Belt_Array.keep(str => str !== "")
  ->Belt_Array.map(strNum => {
      switch (Belt_Float.fromString(strNum)) {
      | None => raise(ParseError("Failed to parse input"))
      | Some(num) => num
      }
    });

/* Part one */
let getFuelForModule = mass => mass->(/.)(3.0)->floor->(-.)(2.0);
let sum = Belt_Array.reduce(_, 0.0, (acc, num) => acc +. num);

let () =
  moduleMasses
  ->Belt_Array.map(getFuelForModule)
  ->sum
  ->Belt_Float.toString
  ->(answer => Js.log("Part 1: " ++ answer));

/* Part two */
let rec getFuelForModule' = mass => {
  let requiredFuel = mass->(/.)(3.0)->floor->(-.)(2.0);

  requiredFuel <= 0.0 ? 0.0 : requiredFuel +. getFuelForModule'(requiredFuel);
};

let () =
  moduleMasses
  ->Belt_Array.map(getFuelForModule')
  ->sum
  ->Belt_Float.toString
  ->(answer => Js.log("Part 2: " ++ answer));
