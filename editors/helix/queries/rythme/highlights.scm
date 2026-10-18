; Colors for Rythme in Helix (a copy of editors/tree-sitter-rythme/queries/highlights.scm
; with the capture names of Helix's themes).
; The patterns never capture the same node twice, so that their order does
; not matter (editors disagree on which of two captures wins).

; Keywords

"fn" @keyword.function

[
  "if"
  "then"
  "else"
  "match"
] @keyword.control.conditional

[
  "do"
  "end"
  "test"
] @keyword

"type" @keyword.storage.type

"use" @keyword.control.import

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
  field: (field_name) @variable.other.member)

(record_field
  key: (field_name) @variable.other.member)

(shorthand_field
  (field_name) @variable.other.member)

(field_pattern
  key: (field_name) @variable.other.member)

; Literals

(integer) @constant.numeric.integer

(float) @constant.numeric.float

(boolean) @constant.builtin.boolean

(string
  "\"" @string)

(string_content) @string

(escape_sequence) @constant.character.escape

(interpolation
  [
    "{"
    "}"
  ] @punctuation.special)

((comment) @comment.line
  (#not-match? @comment.line "^##"))

((comment) @comment.line.documentation
  (#match? @comment.line.documentation "^##"))

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
