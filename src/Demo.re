let getFuelForModule = mass => mass->(/.)(3.0)->floor->(-.)(2.0);

let () =
  Node.Fs.readFileAsUtf8Sync("./src/input1")
  ->Js.String.split("\n", _)
  ->Belt_Array.keep(str => str !== "")
  ->Belt_Array.map(Js.Float.fromString)
  ->Belt_Array.reduce(0.0, (acc, mass) => acc +. getFuelForModule(mass))
  ->Js.log;
