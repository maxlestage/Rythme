; The outline of a file: its functions, types, tests and top-level definitions.

(function_definition
  "fn" @context
  name: [
    (identifier)
    (predicate)
  ] @name) @item

(type_definition
  "type" @context
  name: (type_identifier) @name) @item

(test_definition
  "test" @context
  name: (string) @name) @item

(source_file
  (definition
    pattern: (identifier) @name) @item)
