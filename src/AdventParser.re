/* input example */
let line = "488   393   393   3939";

/* Every parser should return a parseResult */
type parseResult = {
  rest: string,
  value: string,
};

type tokens =
  | Number
  | Whitespace;

type parser = string => option(parseResult);

let stringAfter = (s, n) => String.sub(s, n, String.length(s) - n);

let pChar = (char, input) => {
  let a = String.sub(input, 0, 1);
  let rest = stringAfter(input, 1);
  a == char ? Some({rest, value: a}) : None;
};

let pSpace = pChar(" ");

let isCharNum = char =>
  switch (int_of_string(char)) {
  | exception (Failure(_)) => false
  | _ => true
  };

let getNext = input =>
  String.length(input) == 0 ? None : Some(String.sub(input, 0, 1));

let rec pNumber = (~nums="", input) => {
  let isParsingNums = String.length(nums) != 0;
  let next = getNext(input) |. Belt.Option.getWithDefault("");
  let isNextNum = isCharNum(next);
  switch (isParsingNums, isNextNum) {
  | (false, false) => None
  | (true, false) => Some({rest: input, value: nums})
  | (_, true) => pNumber(~nums=nums ++ next, stringAfter(input, 1))
  };
};

let rec any = (parsers, input) =>
  switch (parsers) {
  | [] => None
  | [parser, ...rest] =>
    let parsed = parser(input);
    switch (parsed) {
    | None => any(rest, input)
    | _ => parsed
    };
  };

/* define list of parsers */
let parsers: list(parser) = [pSpace, pNumber];

let rec parse = (parsers, input) =>
  /* run input through all parsers */
  switch (any(parsers, input)) {
  | Some(a) =>
    /* if any is successful, store the result */
    /* continue with the rest  */
    String.length(a.rest) === 0 ?
      Js.log("Successfully parsed") : parse(parsers, a.rest)
  /* if none is successful, we can't parse the result, explode */
  | _ => failwith("Could not parse input")
  };

/* run input through parser list */
/* if any successful, return successful result */
/* read input */
/* assume newline delimited data */
/* run a single line through the parser */
/* run data through solver */
let _ =
  Node.Fs.readFileAsUtf8Sync("src/input-1")
  |> Js.String.split("\n")
  |> Belt_List.fromArray;

let nums = parse(parsers, line);

Js.log(nums);
