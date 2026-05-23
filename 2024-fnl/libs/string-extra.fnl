(fn split [s delim]
  "Split a string into parts using a delimiter."
  ;; Escape any magic characters in the delimiter if needed
  (let [pattern (string.format "[^%s]+" delim)]
    (icollect [chunk (string.gmatch s pattern)]
      chunk)))

;; Example usage
(let [str "apple,banana,cherry"
      parts (split str ",")]
  parts)

{: split}
