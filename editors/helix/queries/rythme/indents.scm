; Indentation of Rythme in Helix: two spaces inside a function, a block, a
; `match`, brackets and the continuation lines of a definition; `end` and
; closing brackets go back.

[
  (function_definition)
  (lambda)
  (test_definition)
  (do_expression)
  (do_block)
  (brace_block)
  (match_expression)
  (type_definition)
  (definition)
  (list)
  (record)
  (arguments)
  (parameters)
  (list_pattern)
  (record_pattern)
] @indent

[
  "end"
  ")"
  "]"
  "}"
] @outdent
