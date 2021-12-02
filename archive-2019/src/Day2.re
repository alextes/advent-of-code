let opCodes =
  Node.Fs.readFileAsUtf8Sync("./src/input2")
  ->Js.String.split(",", _)
  ->Belt_List.fromArray
  ->Belt_List.map(strNum => Belt_Int.fromString(strNum)->Belt_Option.getExn);

exception ParseError(string);
type opcode =
  | Add(int, int, int)
  | Multiply(int, int, int)
  | Halt;

let parseOpcodes = rawOpcodes => {
  let setCount = ref(0);
  let break = ref(false);
  let opCodes = [||];
  while (! break^) {
    let instructionCode = Belt_List.getExn(rawOpcodes, setCount^ * 4);
    switch (instructionCode) {
    | 99 =>
      let _ = Js.Array.push(Halt, opCodes);
      break := true;
    | 1 =>
      let inputPositionA = Belt_List.getExn(rawOpcodes, setCount^ * 4 + 1);
      let inputPositionB = Belt_List.getExn(rawOpcodes, setCount^ * 4 + 2);
      let outputPosition = Belt_List.getExn(rawOpcodes, setCount^ * 4 + 3);
      let _ =
        Js.Array.push(
          Add(inputPositionA, inputPositionB, outputPosition),
          opCodes,
        );
      setCount := setCount^ + 4;
    | 2 =>
      let inputPositionA = Belt_List.getExn(rawOpcodes, setCount^ * 4 + 1);
      let inputPositionB = Belt_List.getExn(rawOpcodes, setCount^ * 4 + 2);
      let outputPosition = Belt_List.getExn(rawOpcodes, setCount^ * 4 + 3);
      let _ =
        Js.Array.push(
          Multiply(inputPositionA, inputPositionB, outputPosition),
          opCodes,
        );
      setCount := setCount^ + 4;
    | _ => raise(ParseError("Unknown instruction code"))
    };
  };
  Belt_List.fromArray(opCodes);
};

let rec runOpCodes = (state: array(int), opCodes: list(opcode)) => {
  switch (Belt_List.headExn(opCodes)) {
  | Halt => state
  | Add(inputPositionA, inputPositionB, outputPosition) =>
    let numA = Belt_Array.getExn(state, inputPositionA);
    let numB = Belt_Array.getExn(state, inputPositionB);
    let sum = numA + numB;
    Belt_Array.setExn(state, outputPosition, sum);
    runOpCodes(state, Belt_List.tailExn(opCodes));
  | Multiply(inputPositionA, inputPositionB, outputPosition) =>
    let numA = Belt_Array.getExn(state, inputPositionA);
    let numB = Belt_Array.getExn(state, inputPositionB);
    let sum = numA * numB;
    Belt_Array.setExn(state, outputPosition, sum);
    runOpCodes(state, Belt_List.tailExn(opCodes));
  };
};

/* Part 1 */
let twelveOTwoOpCodes: array(int) = {
  let input = Belt_List.toArray(opCodes);
  Js.log(input);
  let instructions = parseOpcodes(opCodes);
  Js.log(instructions);
  Belt_Array.setExn(input, 1, 12);
  Belt_Array.setExn(input, 2, 2);
  runOpCodes(input, instructions);
};

Js.log(twelveOTwoOpCodes);
