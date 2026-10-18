; Colors for Rythme in Zed (a copy of editors/tree-sitter-rythme/queries/highlights.scm
; with the capture names of Zed's themes).
; The patterns never capture the same node twice, so that their order does
; not matter (editors disagree on which of two captures wins).

; Keywords

"fn" @keyword

[
  "if"
  "then"
  "else"
  "match"
] @keyword

[
  "do"
  "end"
  "test"
] @keyword

"type" @keyword

[
  "use"
  "as"
] @keyword

; `use shapes/circle as c`, `c.Circle(r)` in a pattern
[
  (module_name)
  (path_segment)
] @variable.special

[
  "and"
  "or"
  "not"
] @keyword

; Definitions and calls

(function_definition
  name: (identifier) @function)

(definition
  pattern: (identifier) @function
  value: (lambda))

(call
  function: (identifier) @function)

(pipe_expression
  right: (identifier) @function)

(method_call
  method: (field_name) @function.method)

(parameters
  (identifier) @variable.parameter)

; Types and constructors

(type_identifier) @type

(constructor) @constructor

; Fields

(field_expression
  field: (field_name) @property)

(record_field
  key: (field_name) @property)

(shorthand_field
  (field_name) @property)

(field_pattern
  key: (field_name) @property)

; Literals

(integer) @number

(float) @number

(boolean) @boolean

(string
  "\"" @string)

(string_content) @string

(escape_sequence) @string.escape

(interpolation
  [
    "{"
    "}"
  ] @punctuation.special)

((comment) @comment
  (#not-match? @comment "^##"))

((comment) @comment.doc
  (#match? @comment.doc "^##"))

; Operators and punctuation

[
  "+"
  "-"
  "*"
  "/"
  "//"
  "%"
  "**"
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
  "|>"
  ".."
  "->"
  "="
  "?"
  "|"
] @operator

[
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

(record
  [
    "{"
    "}"
  ] @punctuation.bracket)

(record_pattern
  [
    "{"
    "}"
  ] @punctuation.bracket)

[
  ","
  ":"
  "."
] @punctuation.delimiter
