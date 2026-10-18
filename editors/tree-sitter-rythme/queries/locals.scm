; Scopes, definitions and references of Rythme (Neovim's capture names).

; Scopes

[
  (source_file)
  (block)
  (lambda)
  (do_block)
  (brace_block)
  (match_arm)
  (test_definition)
] @local.scope

((function_definition) @local.scope
  (#set! definition.function.scope "parent"))

; Definitions

(function_definition
  name: (identifier) @local.definition.function)

(parameters
  (identifier) @local.definition.parameter)

(block_parameters
  (identifier) @local.definition.parameter)

(definition
  pattern: (identifier) @local.definition.var)

(match_arm
  pattern: (identifier) @local.definition.var)

(constructor_pattern
  (identifier) @local.definition.var)

(list_pattern
  (identifier) @local.definition.var)

(rest_pattern
  (identifier) @local.definition.var)

(or_pattern
  (identifier) @local.definition.var)

(parenthesized_pattern
  (identifier) @local.definition.var)

(field_pattern
  pattern: (identifier) @local.definition.var)

(field_pattern
  key: (field_name) @local.definition.var
  !pattern)

(type_definition
  name: (type_identifier) @local.definition.type)

(variant
  name: (constructor) @local.definition.constructor)

; References

(identifier) @local.reference
