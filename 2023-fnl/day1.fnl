(local example-1 "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet")

(local part-1-set [:1 :2 :3 :4 :5 :6 :7 :8 :9])

(fn input-to-lines [input]
  (icollect [line (string.gmatch input "[^\n]+")]
    line))

(local lines (input-to-lines example-1))

(fn extract-and-combine-numbers [line]
  (let [first-num (string.match line "(%d)")
        last-num (string.match (string.reverse line) "(%d)")]
    (tonumber (.. first-num last-num))))

(fn lines-to-nums [lines]
  ;; Map the `extract-and-combine-numbers` function over all lines to get numbers.
  (icollect [_ line (ipairs lines)]
    (extract-and-combine-numbers line)))

(fn sum-nums [nums]
  (accumulate [sum 0 _ n (ipairs nums)]
    (+ sum n)))

(sum-nums (lines-to-nums lines))
; 142
; part 1 

; for part two numbers may show up spelled out
(local example-2 "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen")

(local part-2-set [:1
                   :2
                   :3
                   :4
                   :5
                   :6
                   :7
                   :8
                   :9
                   :one
                   :two
                   :three
                   :four
                   :five
                   :six
                   :seven
                   :eight
                   :nine])

(local match-it (string.gmatch :abcone :one))

(match-it)

; (fn index-of [item line]
;   (
;     (if index
;         index
;         (error "Item not found in set"))))

; (index-of :one :abcone2threexyz)
