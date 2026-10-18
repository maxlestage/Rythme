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

  externals: $ => [$._newline, $.string_content, $._error_sentinel],

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
    [$._expression, $.rest_pattern],
  ],

  supertypes: $ => [$._expression, $._pattern],

  rules: {
    source_file: $ => lines($, $._item),

    _item: $ => choice($.use_declaration, $.type_definition, $.test_definition, $._statement),

    _statement: $ => choice($.function_definition, $.definition, $._expression),

    comment: _ => token(seq('#', /[^\n]*/)),

    // ----- top-level items ------------------------------------------------

    use_declaration: $ => seq('use', field('path', $.string)),

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
        field('name', $.identifier),
        field('parameters', $.parameters),
        optional(field('guard', $.guard)),
        $._body,
      ),

    parameters: $ => seq('(', commaSep($._pattern), ')'),

    guard: $ => seq('if', $._expression),

    _body: $ => choice(seq('=', field('body', $._expression)), seq($._newline, field('body', $.block))),

    block: $ => seq(lines($, $._statement), 'end'),

    definition: $ => seq(field('pattern', $._pattern), '=', field('value', $._expression)),

    // ----- expressions ----------------------------------------------------

    _expression: $ =>
      choice(
        $.identifier,
        $.constructor,
        $.integer,
        $.float,
        $.string,
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
        alias($.constructor, $.field_name),
        ...KEYWORDS.map(k => alias(k, $.field_name)),
      ),

    call: $ => prec(PREC.postfix, seq(field('function', $._expression), field('arguments', $.arguments))),

    arguments: $ => seq('(', commaSep($._expression), ')'),

    index_expression: $ =>
      prec(PREC.postfix, seq(field('value', $._expression), '[', field('index', $._expression), ']')),

    field_expression: $ => prec(PREC.postfix, seq(field('value', $._expression), '.', field('field', $._field_name))),

    /** `value.name(arguments)` calls `name(value, arguments)`. */
    method_call: $ =>
      prec(
        PREC.postfix + 1,
        seq(field('value', $._expression), '.', field('method', $._field_name), field('arguments', $.arguments)),
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
          field('consequence', $._expression),
          'else',
          field('alternative', $._expression),
        ),
      ),

    match_expression: $ =>
      seq('match', field('subject', $._expression), $._newline, lines($, $.match_arm), 'end'),

    match_arm: $ =>
      seq(field('pattern', $._pattern), optional(field('guard', $.guard)), '->', field('body', $._expression)),

    do_expression: $ => seq('do', field('body', $.block)),

    lambda: $ =>
      prec.right(PREC.lambda, seq('fn', field('parameters', $.parameters), $._body)),

    // ----- patterns -------------------------------------------------------

    _pattern: $ =>
      choice(
        $.identifier,
        $.constructor,
        $.constructor_pattern,
        $.integer,
        $.float,
        $.negative_number,
        $.string,
        $.boolean,
        $.unit_pattern,
        $.parenthesized_pattern,
        $.list_pattern,
        $.record_pattern,
        $.or_pattern,
      ),

    constructor_pattern: $ => seq(field('constructor', $.constructor), '(', commaSep($._pattern), ')'),

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

    // ----- tokens ---------------------------------------------------------

    string: $ => seq('"', repeat(choice($.string_content, $.escape_sequence, $.interpolation)), token.immediate('"')),

    escape_sequence: _ => token.immediate(/\\(u\{[0-9a-fA-F]+\}|[ntr0\\"{}])/),

    interpolation: $ => seq(token.immediate('{'), $._expression, '}'),

    integer: _ => token(choice(/0x[0-9a-fA-F_]+/, /0b[01_]+/, /[0-9][0-9_]*/)),

    float: _ => token(choice(/[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9]+)?/, /[0-9][0-9_]*[eE][+-]?[0-9]+/)),

    boolean: _ => choice('true', 'false'),

    identifier: _ => /[\p{Ll}\p{Lm}\p{Lo}_][\p{L}\p{M}\p{N}_]*/,

    constructor: _ => /[\p{Lu}\p{Lt}][\p{L}\p{M}\p{N}_]*/,
  },
});
