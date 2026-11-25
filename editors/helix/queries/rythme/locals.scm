; Scopes and definitions of Rythme in Helix: the uses of a parameter get
; the color of the parameter.

[
  (source_file)
  (function_definition)
  (block)
  (lambda)
  (do_block)
  (brace_block)
  (match_arm)
  (test_definition)
] @local.scope

(parameters
  (identifier) @local.definition.variable.parameter)

(block_parameters
  (identifier) @local.definition.variable.parameter)

(identifier) @local.reference
