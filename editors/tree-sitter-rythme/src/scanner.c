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
//
// It also reads the names that end with `?` (`even?(4)`, `xs.empty?`) and
// the word `unless` when it starts `unless condition then a else b` (it is
// followed, on its line, by the start of a value; `unless(x)` is a call).

#include "tree_sitter/parser.h"

#include <stdbool.h>
#include <stdint.h>

enum TokenType { NEWLINE, STRING_CONTENT, ERROR_SENTINEL, PREDICATE, UNLESS };

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

/// The built-in functions whose name ends with `?` (their Ruby names), without the `?`.
static const char *const PREDICATES[] = {"even", "odd", "include", "start_with", "end_with", "any", "all", "empty", "key"};

/// A name at the lexer's position: `even?` (a name directly followed by `?`,
/// which is a part of it when `(` follows directly or when it is a built-in
/// name), or the keyword `unless`.
static bool scan_name(TSLexer *lexer, const bool *valid_symbols) {
  char name[32];
  unsigned len = 0;
  int32_t c = lexer->lookahead;
  if (!(c == '_' || (c >= 'a' && c <= 'z') || c >= 0x80)) {
    return false;
  }
  while (is_word_char(lexer->lookahead)) {
    if (len < sizeof name - 1) {
      name[len] = lexer->lookahead < 0x80 ? (char)lexer->lookahead : '#';
    }
    len++;
    advance(lexer);
  }
  if (valid_symbols[UNLESS] && len == 6 && name[0] == 'u' && name[1] == 'n' && name[2] == 'l' && name[3] == 'e' && name[4] == 's' &&
      name[5] == 's') {
    lexer->mark_end(lexer);
    // `unless(x)` and `unless[0]`, without a space, use a value named `unless`.
    if (lexer->lookahead != ' ' && lexer->lookahead != '\t') {
      return false;
    }
    while (lexer->lookahead == ' ' || lexer->lookahead == '\t') {
      advance(lexer);
    }
    int32_t n = lexer->lookahead;
    bool value = n == '"' || n == ':' || n == '(' || n == '[' || n == '_' || (n >= 'a' && n <= 'z') || (n >= 'A' && n <= 'Z') ||
                 (n >= '0' && n <= '9') || n >= 0x80;
    if (value) {
      lexer->result_symbol = UNLESS;
    }
    return value;
  }
  if (!valid_symbols[PREDICATE] || lexer->lookahead != '?') {
    return false;
  }
  advance(lexer);
  lexer->mark_end(lexer);
  if (lexer->lookahead == '(') {
    lexer->result_symbol = PREDICATE;
    return true;
  }
  if (len >= sizeof name) {
    return false;
  }
  name[len] = 0;
  for (unsigned i = 0; i < sizeof PREDICATES / sizeof PREDICATES[0]; i++) {
    const char *p = PREDICATES[i];
    unsigned j = 0;
    while (p[j] && p[j] == name[j]) {
      j++;
    }
    if (!p[j] && !name[j]) {
      lexer->result_symbol = PREDICATE;
      return true;
    }
  }
  return false;
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
      if (lexer->lookahead == '#') {
        // `#{` starts a value inserted in the text.
        lexer->mark_end(lexer);
        advance(lexer);
        if (lexer->lookahead == '{') {
          lexer->result_symbol = STRING_CONTENT;
          return any;
        }
        any = true;
        continue;
      }
      advance(lexer);
      any = true;
    }
    lexer->mark_end(lexer);
    lexer->result_symbol = STRING_CONTENT;
    return any;
  }

  if (valid_symbols[PREDICATE] || valid_symbols[UNLESS]) {
    while (is_space(lexer->lookahead)) {
      skip(lexer);
    }
    if (lexer->lookahead != '\n' && lexer->lookahead != ';') {
      return scan_name(lexer, valid_symbols);
    }
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
