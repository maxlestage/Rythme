// The external scanner of the Rythme grammar: significant new lines and the
// literal parts of texts.
//
// A new line (or a `;`) ends a statement only where the grammar accepts the
// end of a statement: inside brackets the parser never asks for one, and the
// new line is then skipped like a space. It does not end anything either
// when the next line (past blank lines and comments) starts with an operator
// that continues the expression, as in src/parser.rs (`is_continuation`),
// or with `then`, `else` or `|` (an `if` or a `type` written on several
// lines).

#include "tree_sitter/parser.h"

#include <stdbool.h>
#include <stdint.h>

enum TokenType { NEWLINE, STRING_CONTENT, ERROR_SENTINEL };

void *tree_sitter_rythme_external_scanner_create(void) { return NULL; }

void tree_sitter_rythme_external_scanner_destroy(void *payload) { (void)payload; }

unsigned tree_sitter_rythme_external_scanner_serialize(void *payload, char *buffer) {
  (void)payload;
  (void)buffer;
  return 0;
}

void tree_sitter_rythme_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
  (void)payload;
  (void)buffer;
  (void)length;
}

static inline void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

static inline void skip(TSLexer *lexer) { lexer->advance(lexer, true); }

static bool is_space(int32_t c) { return c == ' ' || c == '\t' || c == '\r' || c == '\f' || c == 0xFEFF; }

static bool is_word_char(int32_t c) {
  return c == '_' || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c >= 0x80;
}

/// Consumes `word` if the text continues with it as a whole word.
static bool is_word(TSLexer *lexer, const char *word) {
  for (const char *w = word; *w; w++) {
    if (lexer->lookahead != *w) {
      return false;
    }
    advance(lexer);
  }
  return !is_word_char(lexer->lookahead);
}

/// Does the code at the lexer's position continue the previous line?
/// (The lexer is moved; the token end was marked before.)
static bool starts_continuation(TSLexer *lexer) {
  switch (lexer->lookahead) {
    case '|':  // `|>`, and `|` between the variants of a type
    case '.':  // `.field`, `.method()`, `..`
    case '+':
    case '*':
    case '/':
    case '%':
    case '<':
    case '>':
      return true;
    case '=':
    case '!':
      advance(lexer);
      return lexer->lookahead == '=';
    case 'a':
      return is_word(lexer, "and");
    case 'o':
      return is_word(lexer, "or");
    case 't':
      return is_word(lexer, "then");
    case 'e':
      return is_word(lexer, "else");
    default:
      return false;
  }
}

bool tree_sitter_rythme_external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) {
  (void)payload;
  // During error recovery every token is valid: let the other rules decide.
  if (valid_symbols[ERROR_SENTINEL]) {
    return false;
  }

  if (valid_symbols[STRING_CONTENT] && !valid_symbols[NEWLINE]) {
    bool any = false;
    while (lexer->lookahead != '"' && lexer->lookahead != '\\' && lexer->lookahead != '{' && !lexer->eof(lexer)) {
      advance(lexer);
      any = true;
    }
    lexer->result_symbol = STRING_CONTENT;
    return any;
  }

  if (!valid_symbols[NEWLINE]) {
    return false;
  }
  while (is_space(lexer->lookahead)) {
    skip(lexer);
  }
  if (lexer->lookahead == ';') {
    advance(lexer);
    lexer->mark_end(lexer);
    lexer->result_symbol = NEWLINE;
    return true;
  }
  if (lexer->lookahead != '\n') {
    return false;
  }
  advance(lexer);
  lexer->mark_end(lexer);
  // Look at the next line that holds code.
  for (;;) {
    if (is_space(lexer->lookahead) || lexer->lookahead == '\n') {
      advance(lexer);
    } else if (lexer->lookahead == '#') {
      while (lexer->lookahead != '\n' && !lexer->eof(lexer)) {
        advance(lexer);
      }
    } else {
      break;
    }
  }
  if (!lexer->eof(lexer) && starts_continuation(lexer)) {
    return false;
  }
  lexer->result_symbol = NEWLINE;
  return true;
}
