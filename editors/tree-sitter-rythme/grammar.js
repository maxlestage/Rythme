/**
 * @file Tree-sitter grammar for Rythme, a simple, purely functional language.
 * @license MIT
 *
 * It follows src/lexer.rs and src/parser.rs. New lines end statements,
 * except inside `()`, `[]`, `{}` and texts, after an operator, or when the
 * next line starts with an operator that continues the expression (`|>`,
 * `.`, `+`, `and`, `else`...): the external scanner (src/scanner.c) only
 * emits a new line where the grammar accepts one, and decides about the
 * continuations by looking at the next line.
 *
 * The scanner also reads the names that end with `?` (`even?`): a name
 * directly followed by `?` is one name when `(` follows directly
 * (`even?(4)`, `fn adult?(p)`) or when it is a built-in one (`xs.empty?`);
 * otherwise the `?` is the operator (`value?`). (Rythme also knows the
 * functions defined with `?` in the file; this grammar does not.)
 *
 * And it reads the name of a call without parentheses (`puts "hi"`,
 * `print x, y`): a name that starts a statement or a result (a branch of
 * `if`, a case of `match`, the body after the `=` of a function), followed
 * on its line by a space and the start of a value. Rythme reads `x -1` and `xs [0]` as a
 * subtraction and an index when the code gives `x` and `xs` values; this
 * grammar always reads them as calls (`rythme fmt` writes the others
 * `x - 1` and `xs[0]`).
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const PREC = {
  lambda: 1,
  or: 2,
  and: 3,
  not: 4,
  compare: 5,
  pipe: 6,
  range: 7,
  add: 8,
  multiply: 9,
  unary: 10,
  power: 11,
  postfix: 12,
};

/** Words that may name a field after `.` or in a record. */
const KEYWORDS = ['fn', 'if', 'then', 'else', 'match', 'end', 'do', 'type', 'test', 'use', 'and', 'or', 'not', 'true', 'false'];

/**
 * `rule, rule, ...` with an optional trailing comma.
 * @param {RuleOrLiteral} rule
 */
function commaSep(rule) {
  return optional(seq(rule, repeat(seq(',', rule)), optional(',')));
}

/**
 * Items separated by new lines, with blank lines anywhere.
 * @param {GrammarSymbols<string>} $
 * @param {RuleOrLiteral} rule
 */
function lines($, rule) {
  return seq(repeat($._newline), optional(seq(rule, repeat(seq(repeat1($._newline), rule)), repeat($._newline))));
}

module.exports = grammar({
  name: 'rythme',

  externals: $ => [$._newline, $.string_content, $._error_sentinel, $.predicate, $._unless, $._command_name],

  extras: $ => [/[ \t\r\n\f﻿]/, ';', $.comment],

  word: $ => $.identifier,

  conflicts: $ => [
    [$._expression, $._pattern],
    [$.list, $.list_pattern],
    [$.record, $.record_pattern],
    [$.shorthand_field, $.field_pattern],
    [$.unit, $.unit_pattern],
    [$._expression, $.constructor_pattern],
    [$._expression, $.negative_number],
    [$._expression, $._number_pattern],
    [$._expression, $.prefix_pattern],
    [$._expression, $.rest_pattern],
    [$._expression, $.qualified_constructor],
  ],

  supertypes: $ => [$._expression, $._pattern],

  rules: {
    source_file: $ => lines($, $._item),

    _item: $ => choice($.use_declaration, $.type_definition, $.test_definition, $._statement),

    _statement: $ => choice($.function_definition, $.definition, $.command_call, $._expression),

    /** `print x, y`: a call without parentheses, then a `do` block given to it. */
    command_call: $ =>
      prec.right(
        seq(
          field('function', alias($._command_name, $.identifier)),
          field('arguments', $.command_arguments),
          optional(field('block', $.do_block)),
        ),
      ),

    command_arguments: $ => prec.right(seq($._expression, repeat(seq(',', $._expression)))),

    comment: _ => token(seq('#', /[^\n]*/)),

    // ----- top-level items ------------------------------------------------

    /** `use geometry`, `use shapes/circle as circle`, `use "geometry.ry" as geo`, `use "geometry.ry"`. */
    use_declaration: $ =>
      seq('use', field('path', choice($.string, $.module_path)), optional(seq('as', field('alias', alias($.identifier, $.module_name))))),

    /** `geometry`, `shapes/circle`, `../common/tools`: a file without `.ry`. */
    module_path: $ => seq(repeat(seq(choice(alias($.identifier, $.path_segment), '..'), '/')), alias($.identifier, $.module_name)),

    type_definition: $ =>
      seq(
        'type',
        field('name', alias($.constructor, $.type_identifier)),
        '=',
        optional('|'),
        $.variant,
        repeat(seq('|', $.variant)),
      ),

    variant: $ => seq(field('name', $.constructor), optional($.variant_fields)),

    variant_fields: $ => seq('(', commaSep($.identifier), ')'),

    test_definition: $ => seq('test', field('name', $.string), $._body),

    // ----- definitions ----------------------------------------------------

    function_definition: $ =>
      seq(
        'fn',
        field('name', choice($.identifier, $.predicate)),
        field('parameters', $.parameters),
        optional(field('guard', $.guard)),
        $._body,
      ),

    parameters: $ => seq('(', commaSep($._pattern), ')'),

    guard: $ => seq('if', $._expression),

    _body: $ => choice(seq('=', field('body', $._result)), seq($._newline, field('body', $.block))),

    /**
     * A result: a branch of `if`, a case of `match`, the body after the `=`
     * of a function or a test. A call without parentheses may be written
     * there, as at the start of a statement: `if ok then puts "yes" else
     * puts "no"`.
     */
    _result: $ => choice($.command_call, $._expression),

    block: $ => seq(lines($, $._statement), 'end'),

    definition: $ => seq(field('pattern', $._pattern), '=', field('value', $._expression)),

    // ----- expressions ----------------------------------------------------

    _expression: $ =>
      choice(
        $.identifier,
        $.predicate,
        $.constructor,
        $.integer,
        $.float,
        $.string,
        $.symbol,
        $.boolean,
        $.unit,
        $.list,
        $.record,
        $.parenthesized_expression,
        $.call,
        $.index_expression,
        $.field_expression,
        $.method_call,
        $.try_expression,
        $.unary_expression,
        $.not_expression,
        $.binary_expression,
        $.pipe_expression,
        $.range_expression,
        $.if_expression,
        $.unless_expression,
        $.match_expression,
        $.do_expression,
        $.lambda,
      ),

    parenthesized_expression: $ => seq('(', $._expression, ')'),

    unit: _ => seq('(', ')'),

    list: $ => seq('[', commaSep(choice($._expression, $.spread)), ']'),

    spread: $ => seq('..', $._expression),

    record: $ => seq('{', commaSep(choice($.record_field, $.shorthand_field, $.spread)), '}'),

    record_field: $ => seq(field('key', $._field_key), ':', field('value', $._expression)),

    shorthand_field: $ => field('key', alias($.identifier, $.field_name)),

    _field_key: $ => choice($._field_name, $.string),

    /** A field name: a name, a constructor-like name, or a keyword. */
    _field_name: $ =>
      choice(
        alias($.identifier, $.field_name),
        alias($.predicate, $.field_name),
        alias($.constructor, $.field_name),
        ...KEYWORDS.map(k => alias(k, $.field_name)),
      ),

    // Right associative: a block after `f(x)` belongs to that call.
    call: $ =>
      prec.right(
        PREC.postfix,
        seq(
          field('function', $._expression),
          choice(seq(field('arguments', $.arguments), optional(field('block', $._block))), field('block', $._block)),
        ),
      ),

    /** A block given as the last argument: `do |x| ... end` or `{ |x| ... }`. */
    _block: $ => choice($.do_block, $.brace_block),

    do_block: $ => seq('do', optional(field('parameters', $.block_parameters)), field('body', $.block)),

    brace_block: $ => seq('{', field('parameters', $.block_parameters), lines($, $._statement), '}'),

    block_parameters: $ => choice('||', seq('|', commaSep($._block_parameter), '|')),

    _block_parameter: $ =>
      choice($.identifier, $.constructor_pattern, $.list_pattern, $.record_pattern, $.parenthesized_pattern, $.unit_pattern),

    arguments: $ => seq('(', commaSep($._expression), ')'),

    index_expression: $ =>
      prec(PREC.postfix, seq(field('value', $._expression), '[', field('index', $._expression), ']')),

    field_expression: $ => prec(PREC.postfix, seq(field('value', $._expression), '.', field('field', $._field_name))),

    /** `value.name(arguments)` calls `name(value, arguments)`; a block may follow. */
    method_call: $ =>
      prec.right(
        PREC.postfix + 1,
        seq(
          field('value', $._expression),
          '.',
          field('method', $._field_name),
          choice(seq(field('arguments', $.arguments), optional(field('block', $._block))), field('block', $._block)),
        ),
      ),

    try_expression: $ => prec(PREC.postfix, seq($._expression, '?')),

    unary_expression: $ => prec(PREC.unary, seq(field('operator', '-'), field('operand', $._expression))),

    not_expression: $ => prec(PREC.not, seq(field('operator', 'not'), field('operand', $._expression))),

    binary_expression: $ => {
      /** @type {[number, RuleOrLiteral, 'left' | 'right'][]} */
      const table = [
        [PREC.or, 'or', 'left'],
        [PREC.and, 'and', 'left'],
        [PREC.compare, choice('==', '!=', '<', '<=', '>', '>='), 'left'],
        [PREC.add, choice('+', '-'), 'left'],
        [PREC.multiply, choice('*', '/', '//', '%'), 'left'],
        [PREC.power, '**', 'right'],
      ];
      return choice(
        ...table.map(([p, op, assoc]) =>
          prec[assoc](p, seq(field('left', $._expression), field('operator', op), field('right', $._expression))),
        ),
      );
    },

    pipe_expression: $ => prec.left(PREC.pipe, seq(field('left', $._expression), '|>', field('right', $._expression))),

    range_expression: $ => prec.left(PREC.range, seq(field('start', $._expression), '..', field('end', $._expression))),

    if_expression: $ =>
      prec.right(
        PREC.lambda,
        seq(
          'if',
          field('condition', $._expression),
          'then',
          field('consequence', $._result),
          'else',
          field('alternative', $._result),
        ),
      ),

    /** `unless c then a else b`: the same as `if c then b else a`. */
    unless_expression: $ =>
      prec.right(
        PREC.lambda,
        seq(
          alias($._unless, 'unless'),
          field('condition', $._expression),
          'then',
          field('consequence', $._result),
          'else',
          field('alternative', $._result),
        ),
      ),

    match_expression: $ =>
      seq('match', field('subject', $._expression), $._newline, lines($, $.match_arm), 'end'),

    match_arm: $ =>
      seq(field('pattern', $._pattern), optional(field('guard', $.guard)), '->', field('body', $._result)),

    do_expression: $ => seq('do', field('body', $.block)),

    lambda: $ =>
      prec.right(PREC.lambda, seq('fn', field('parameters', $.parameters), $._body)),

    // ----- patterns -------------------------------------------------------

    _pattern: $ =>
      choice(
        $.identifier,
        $.constructor,
        $.constructor_pattern,
        $.qualified_constructor,
        $.integer,
        $.float,
        $.negative_number,
        $.string,
        $.symbol,
        $.boolean,
        $.unit_pattern,
        $.parenthesized_pattern,
        $.list_pattern,
        $.record_pattern,
        $.or_pattern,
        $.range_pattern,
        $.prefix_pattern,
      ),

    constructor_pattern: $ =>
      seq(field('constructor', choice($.constructor, $.qualified_constructor)), '(', commaSep($._pattern), ')'),

    /** `geo.Circle`: a constructor of the file used as `geo`. */
    qualified_constructor: $ =>
      seq(field('module', alias($.identifier, $.module_name)), '.', field('constructor', $.constructor)),

    negative_number: $ => seq('-', choice($.integer, $.float)),

    unit_pattern: _ => seq('(', ')'),

    parenthesized_pattern: $ => seq('(', $._pattern, ')'),

    list_pattern: $ => seq('[', commaSep(choice($._pattern, $.rest_pattern)), ']'),

    rest_pattern: $ => seq('..', optional($.identifier)),

    record_pattern: $ => seq('{', commaSep(choice($.field_pattern, $.rest_pattern)), '}'),

    field_pattern: $ =>
      choice(
        seq(field('key', $._field_key), ':', field('pattern', $._pattern)),
        field('key', alias($.identifier, $.field_name)),
      ),

    or_pattern: $ => prec.left(seq($._pattern, '|', $._pattern)),

    /** `1..9`, `-0.5..0.5`: a number from the first to the second. */
    range_pattern: $ => seq(field('start', $._number_pattern), '..', field('end', $._number_pattern)),

    _number_pattern: $ => choice($.integer, $.float, $.negative_number),

    /** `"Mr " + name`: a text that starts with this text, and the rest. */
    prefix_pattern: $ => seq(field('prefix', $.string), '+', field('rest', $.identifier)),

    // ----- tokens ---------------------------------------------------------

    string: $ => seq('"', repeat(choice($.string_content, $.escape_sequence, $.interpolation)), token.immediate('"')),

    escape_sequence: _ => token.immediate(/\\(u\{[0-9a-fA-F]+\}|#\{?|[ntr0\\"{}])/),

    /** `{x}`, or `#{x}` as in Ruby. */
    interpolation: $ => seq(choice(token.immediate('{'), token.immediate(prec(1, '#{'))), $._expression, '}'),

    /** `:ok`, `:even?`: a name used as a value. */
    symbol: _ => /:[\p{L}_][\p{L}\p{M}\p{N}_]*\??/,

    integer: _ => token(choice(/0x[0-9a-fA-F_]+/, /0b[01_]+/, /[0-9][0-9_]*/)),

    float: _ => token(choice(/[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9]+)?/, /[0-9][0-9_]*[eE][+-]?[0-9]+/)),

    boolean: _ => choice('true', 'false'),

    identifier: _ => /[\p{Ll}\p{Lm}\p{Lo}_][\p{L}\p{M}\p{N}_]*/,

    constructor: _ => /[\p{Lu}\p{Lt}][\p{L}\p{M}\p{N}_]*/,
  },
});
