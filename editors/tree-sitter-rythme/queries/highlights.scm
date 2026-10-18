; Colors for Rythme (capture names of Neovim and of `tree-sitter highlight`).
; The patterns never capture the same node twice, so that their order does
; not matter (editors disagree on which of two captures wins).

; Keywords

"fn" @keyword.function

[
  "if"
  "then"
  "else"
  "match"
] @keyword.conditional

[
  "do"
  "end"
  "test"
] @keyword

"type" @keyword.type

"use" @keyword.import

[
  "and"
  "or"
  "not"
] @keyword.operator

; Definitions and calls

(function_definition
  name: (identifier) @function)

(definition
  pattern: (identifier) @function
  value: (lambda))

(call
  function: (identifier) @function.call)

(pipe_expression
  right: (identifier) @function.call)

(method_call
  method: (field_name) @function.method.call)

(parameters
  (identifier) @variable.parameter)

; Types and constructors

(type_identifier) @type.definition

(constructor) @constructor

; Fields

(field_expression
  field: (field_name) @variable.member)

(record_field
  key: (field_name) @property)

(shorthand_field
  (field_name) @property)

(field_pattern
  key: (field_name) @property)

; Literals

(integer) @number

(float) @number.float

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

((comment) @comment.documentation
  (#match? @comment.documentation "^##"))

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
