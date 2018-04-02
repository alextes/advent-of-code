let lastIndex = (nums, i) => Array.length(nums) - 1 == i;

let getPair = (nums, i) => {
  let len = Array.length(nums);
  (nums[i], nums[(i + 1) mod len]);
};

let getPair2 = (nums, i) => {
  let len = Array.length(nums);
  (nums[i], nums[(i + len / 2) mod len]);
};

let sumPairs = pairs => Array.map(((a, b)) => a == b ? a : 0, pairs);

let sumNums = nums => Array.fold_left((a, b) => a + b, 0, nums);

let getFirst = arr => arr[0];

/* solution 1 */
let () =
  Node.Fs.readFileAsUtf8Sync("src/input-1")
  |> Js.String.split("\n")
  |> getFirst
  |> Js.String.split("")
  |> Array.map(int_of_string)
  |> (
    nums =>
      Array.mapi((i, _) => getPair(nums, i), nums)
      |> sumPairs
      |> sumNums
      |> Js.log
  );

/* solution 2 */
let () =
  Node.Fs.readFileAsUtf8Sync("src/input-1")
  |> Js.String.split("\n")
  |> getFirst
  |> Js.String.split("")
  |> Array.map(int_of_string)
  |> (
    nums =>
      Array.mapi((i, _) => getPair2(nums, i), nums)
      |> sumPairs
      |> sumNums
      |> Js.log
  );