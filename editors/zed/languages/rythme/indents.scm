; Two spaces inside a block, a `match` and brackets; `end` and closing
; brackets go back.

(function_definition
  body: (block
    "end" @end)) @indent

(lambda
  body: (block
    "end" @end)) @indent

(test_definition
  body: (block
    "end" @end)) @indent

(do_expression
  body: (block
    "end" @end)) @indent

(do_block
  body: (block
    "end" @end)) @indent

(match_expression
  "end" @end) @indent

(_
  "(" ")" @end) @indent

(_
  "[" "]" @end) @indent

(_
  "{" "}" @end) @indent
