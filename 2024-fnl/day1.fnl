(local string-extra (require (.. :libs/ :.string-extra)))
(local table-extra (require (.. :libs/ :.table-extra)))

;; Takes input string and splits it into lines.
(fn split-lines [input-string]
  (local lines [])
  (each [line (string.gmatch input-string "[^\n]+")]
    (table.insert lines line))
  lines)

;; Takes an array of lines and splits them into two numeric columns.
(fn lines-to-cols [lines]
  (local col-1-nums [])
  (local col-2-nums [])
  (each [_ line (ipairs lines)]
    (let [nums (string-extra.split line "   ")]
      (table.insert col-1-nums (tonumber (table.remove nums 1)))
      (table.insert col-2-nums (tonumber (table.remove nums 1)))))
  [col-1-nums col-2-nums])

;; Given two arrays, calculates absolute distance pairwise.
(fn calc-distances [nums1 nums2]
  (local distances [])
  (for [i 1 (length nums1)]
    (table.insert distances (math.abs (- (. nums1 i) (. nums2 i)))))
  distances)

;; Core function that takes a raw multi-line string and returns summed distances.
(fn sum-distances-from-string [input-string]
  ;; Split text into lines
  (local lines (split-lines input-string))
  ;; Convert lines into two columns
  (let [[col-1 col-2] (lines-to-cols lines)
        col-1-sorted (table-extra.shallow-copy col-1)
        col-2-sorted (table-extra.shallow-copy col-2)]
    (table.sort col-1-sorted)
    (table.sort col-2-sorted)
    ;; Calculate the distances and sum them
    (var sum 0)
    (each [_ distance (ipairs (calc-distances col-1-sorted col-2-sorted))]
      (set sum (+ sum distance)))
    sum))

;; Reads a file and then reuses `sum-distances-from-string`.
(fn sum-distances-from-file [path]
  (let [file (io.open path :r)
        content (file:read :*a)]
    (file:close)
    (sum-distances-from-string content)))

;; Example usage:
(local example-1 "3   4\n4   3\n2   5\n1   3\n3   9\n3   3")

;; Run the inline string through the steps:
(sum-distances-from-string example-1)
; 11
; 

;; Uncomment to run via file:
(sum-distances-from-file :day1.txt)
; 1666427
; 

(fn input-to-cols [input-string]
  (local lines (split-lines input-string))
  (let [[col-1 col-2]] (lines-to-cols lines)
    print
    col-1)
  (local memo {})
  ;; Convert lines into two columns
  (let [[col-1 col-2] (lines-to-cols lines)
        col-1-sorted (table-extra.shallow-copy col-1)
        col-2-sorted (table-extra.shallow-copy col-2)]
    (table.sort col-1-sorted)
    (table.sort col-2-sorted)
    [col-1-sorted col-2-sorted]))
