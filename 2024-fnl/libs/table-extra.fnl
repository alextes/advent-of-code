(fn shallow-copy [tbl]
  "Returns a shallow copy of the table."
  (let [copy []]
    (each [_i v (ipairs tbl)]
      (table.insert copy v))
    copy))

{: shallow-copy}
