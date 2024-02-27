#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 39
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 34
#define ALIAS_COUNT 0
#define TOKEN_COUNT 18
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_module_keyword = 1,
  anon_sym_LBRACE = 2,
  anon_sym_RBRACE = 3,
  sym_var_keyword = 4,
  anon_sym_COLON = 5,
  sym_var_modifier = 6,
  anon_sym_EQ = 7,
  sym_module_identifier = 8,
  aux_sym_var_identifier_token1 = 9,
  sym_type_identifier = 10,
  sym_bool = 11,
  sym_nowt = 12,
  sym_float = 13,
  sym_int = 14,
  sym_escape = 15,
  sym_unescaped_string_fragment = 16,
  anon_sym_DQUOTE = 17,
  sym_source_file = 18,
  sym_module_declaration = 19,
  sym_module_block = 20,
  sym_var_declaration = 21,
  sym_var_block = 22,
  sym_var_attribute = 23,
  sym__declaration = 24,
  sym__expression = 25,
  sym_var_identifier = 26,
  sym_attribute_identifier = 27,
  sym__literal = 28,
  sym_str = 29,
  aux_sym_source_file_repeat1 = 30,
  aux_sym_module_block_repeat1 = 31,
  aux_sym_var_block_repeat1 = 32,
  aux_sym_str_repeat1 = 33,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_module_keyword] = "module_keyword",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [sym_var_keyword] = "var_keyword",
  [anon_sym_COLON] = ":",
  [sym_var_modifier] = "var_modifier",
  [anon_sym_EQ] = "=",
  [sym_module_identifier] = "module_identifier",
  [aux_sym_var_identifier_token1] = "var_identifier_token1",
  [sym_type_identifier] = "type_identifier",
  [sym_bool] = "bool",
  [sym_nowt] = "nowt",
  [sym_float] = "float",
  [sym_int] = "int",
  [sym_escape] = "escape",
  [sym_unescaped_string_fragment] = "unescaped_string_fragment",
  [anon_sym_DQUOTE] = "\"",
  [sym_source_file] = "source_file",
  [sym_module_declaration] = "module_declaration",
  [sym_module_block] = "module_block",
  [sym_var_declaration] = "var_declaration",
  [sym_var_block] = "var_block",
  [sym_var_attribute] = "var_attribute",
  [sym__declaration] = "_declaration",
  [sym__expression] = "_expression",
  [sym_var_identifier] = "var_identifier",
  [sym_attribute_identifier] = "attribute_identifier",
  [sym__literal] = "_literal",
  [sym_str] = "str",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_module_block_repeat1] = "module_block_repeat1",
  [aux_sym_var_block_repeat1] = "var_block_repeat1",
  [aux_sym_str_repeat1] = "str_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_module_keyword] = sym_module_keyword,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [sym_var_keyword] = sym_var_keyword,
  [anon_sym_COLON] = anon_sym_COLON,
  [sym_var_modifier] = sym_var_modifier,
  [anon_sym_EQ] = anon_sym_EQ,
  [sym_module_identifier] = sym_module_identifier,
  [aux_sym_var_identifier_token1] = aux_sym_var_identifier_token1,
  [sym_type_identifier] = sym_type_identifier,
  [sym_bool] = sym_bool,
  [sym_nowt] = sym_nowt,
  [sym_float] = sym_float,
  [sym_int] = sym_int,
  [sym_escape] = sym_escape,
  [sym_unescaped_string_fragment] = sym_unescaped_string_fragment,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [sym_source_file] = sym_source_file,
  [sym_module_declaration] = sym_module_declaration,
  [sym_module_block] = sym_module_block,
  [sym_var_declaration] = sym_var_declaration,
  [sym_var_block] = sym_var_block,
  [sym_var_attribute] = sym_var_attribute,
  [sym__declaration] = sym__declaration,
  [sym__expression] = sym__expression,
  [sym_var_identifier] = sym_var_identifier,
  [sym_attribute_identifier] = sym_attribute_identifier,
  [sym__literal] = sym__literal,
  [sym_str] = sym_str,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_module_block_repeat1] = aux_sym_module_block_repeat1,
  [aux_sym_var_block_repeat1] = aux_sym_var_block_repeat1,
  [aux_sym_str_repeat1] = aux_sym_str_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_module_keyword] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [sym_var_keyword] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [sym_var_modifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [sym_module_identifier] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_var_identifier_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_type_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_bool] = {
    .visible = true,
    .named = true,
  },
  [sym_nowt] = {
    .visible = true,
    .named = true,
  },
  [sym_float] = {
    .visible = true,
    .named = true,
  },
  [sym_int] = {
    .visible = true,
    .named = true,
  },
  [sym_escape] = {
    .visible = true,
    .named = true,
  },
  [sym_unescaped_string_fragment] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_module_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym_module_block] = {
    .visible = true,
    .named = true,
  },
  [sym_var_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym_var_block] = {
    .visible = true,
    .named = true,
  },
  [sym_var_attribute] = {
    .visible = true,
    .named = true,
  },
  [sym__declaration] = {
    .visible = false,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [sym_var_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym__literal] = {
    .visible = false,
    .named = true,
  },
  [sym_str] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_module_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_var_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_str_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(59);
      if (lookahead == '"') ADVANCE(93);
      if (lookahead == ':') ADVANCE(64);
      if (lookahead == '=') ADVANCE(66);
      if (lookahead == '\\') ADVANCE(42);
      if (lookahead == 'b') ADVANCE(15);
      if (lookahead == 'f') ADVANCE(12);
      if (lookahead == 'i') ADVANCE(20);
      if (lookahead == 'm') ADVANCE(13);
      if (lookahead == 'n') ADVANCE(14);
      if (lookahead == 'p') ADVANCE(18);
      if (lookahead == 's') ADVANCE(19);
      if (lookahead == 'u') ADVANCE(16);
      if (lookahead == 'v') ADVANCE(6);
      if (lookahead == '{') ADVANCE(61);
      if (lookahead == '}') ADVANCE(62);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(58)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(85);
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(17);
      if (('A' <= lookahead && lookahead <= 'Z')) ADVANCE(21);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(93);
      if (lookahead == '\\') ADVANCE(42);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(3)
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(91);
      if (lookahead != 0) ADVANCE(92);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(93);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(85);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(17);
      END_STATE();
    case 3:
      if (lookahead == '"') ADVANCE(93);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(3)
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(39);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(23);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(37);
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(31);
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 7:
      if (lookahead == 'b') ADVANCE(65);
      END_STATE();
    case 8:
      if (lookahead == 'b') ADVANCE(30);
      if (lookahead == 'f') ADVANCE(25);
      if (lookahead == 'i') ADVANCE(26);
      if (lookahead == 'n') ADVANCE(27);
      if (lookahead == 's') ADVANCE(41);
      if (lookahead == 'u') ADVANCE(33);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(8)
      END_STATE();
    case 9:
      if (lookahead == 'd') ADVANCE(44);
      END_STATE();
    case 10:
      if (lookahead == 'e') ADVANCE(60);
      END_STATE();
    case 11:
      if (lookahead == 'e') ADVANCE(57);
      END_STATE();
    case 12:
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'l') ADVANCE(29);
      if (lookahead == 'n') ADVANCE(31);
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 13:
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(31);
      if (lookahead == 'o') ADVANCE(9);
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 14:
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(31);
      if (lookahead == 'o') ADVANCE(45);
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 15:
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(31);
      if (lookahead == 'o') ADVANCE(28);
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 16:
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(31);
      if (lookahead == 'r') ADVANCE(22);
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 17:
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(31);
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 18:
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(31);
      if (lookahead == 't') ADVANCE(34);
      if (lookahead == 'u') ADVANCE(7);
      END_STATE();
    case 19:
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(31);
      if (lookahead == 't') ADVANCE(36);
      END_STATE();
    case 20:
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(32);
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 21:
      if (lookahead == 'f') ADVANCE(67);
      if (lookahead == 'n') ADVANCE(70);
      if (lookahead == 't') ADVANCE(71);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 22:
      if (lookahead == 'l') ADVANCE(80);
      END_STATE();
    case 23:
      if (lookahead == 'l') ADVANCE(38);
      END_STATE();
    case 24:
      if (lookahead == 'l') ADVANCE(10);
      END_STATE();
    case 25:
      if (lookahead == 'l') ADVANCE(29);
      END_STATE();
    case 26:
      if (lookahead == 'n') ADVANCE(39);
      END_STATE();
    case 27:
      if (lookahead == 'o') ADVANCE(45);
      END_STATE();
    case 28:
      if (lookahead == 'o') ADVANCE(22);
      END_STATE();
    case 29:
      if (lookahead == 'o') ADVANCE(4);
      END_STATE();
    case 30:
      if (lookahead == 'o') ADVANCE(28);
      END_STATE();
    case 31:
      if (lookahead == 'o') ADVANCE(46);
      END_STATE();
    case 32:
      if (lookahead == 'o') ADVANCE(46);
      if (lookahead == 't') ADVANCE(80);
      END_STATE();
    case 33:
      if (lookahead == 'r') ADVANCE(22);
      END_STATE();
    case 34:
      if (lookahead == 'r') ADVANCE(43);
      END_STATE();
    case 35:
      if (lookahead == 'r') ADVANCE(80);
      END_STATE();
    case 36:
      if (lookahead == 'r') ADVANCE(81);
      END_STATE();
    case 37:
      if (lookahead == 'r') ADVANCE(63);
      END_STATE();
    case 38:
      if (lookahead == 's') ADVANCE(11);
      END_STATE();
    case 39:
      if (lookahead == 't') ADVANCE(80);
      END_STATE();
    case 40:
      if (lookahead == 't') ADVANCE(56);
      END_STATE();
    case 41:
      if (lookahead == 't') ADVANCE(35);
      END_STATE();
    case 42:
      if (lookahead == 'u') ADVANCE(47);
      if (lookahead == 'x') ADVANCE(54);
      if (lookahead == '\r' ||
          lookahead == '?') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(90);
      if (lookahead != 0) ADVANCE(87);
      END_STATE();
    case 43:
      if (lookahead == 'u') ADVANCE(11);
      END_STATE();
    case 44:
      if (lookahead == 'u') ADVANCE(24);
      END_STATE();
    case 45:
      if (lookahead == 'w') ADVANCE(39);
      END_STATE();
    case 46:
      if (lookahead == 'w') ADVANCE(40);
      END_STATE();
    case 47:
      if (lookahead == '{') ADVANCE(53);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(51);
      END_STATE();
    case 48:
      if (lookahead == '}') ADVANCE(62);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(48)
      if (('A' <= lookahead && lookahead <= 'Z')) ADVANCE(55);
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(79);
      END_STATE();
    case 49:
      if (lookahead == '}') ADVANCE(87);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(49);
      END_STATE();
    case 50:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(84);
      END_STATE();
    case 51:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(54);
      END_STATE();
    case 52:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(87);
      END_STATE();
    case 53:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(49);
      END_STATE();
    case 54:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(52);
      END_STATE();
    case 55:
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 56:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(83);
      END_STATE();
    case 57:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(82);
      END_STATE();
    case 58:
      if (eof) ADVANCE(59);
      if (lookahead == '"') ADVANCE(93);
      if (lookahead == ':') ADVANCE(64);
      if (lookahead == '=') ADVANCE(66);
      if (lookahead == 'b') ADVANCE(15);
      if (lookahead == 'f') ADVANCE(12);
      if (lookahead == 'i') ADVANCE(20);
      if (lookahead == 'm') ADVANCE(13);
      if (lookahead == 'n') ADVANCE(14);
      if (lookahead == 'p') ADVANCE(18);
      if (lookahead == 's') ADVANCE(19);
      if (lookahead == 'u') ADVANCE(16);
      if (lookahead == 'v') ADVANCE(6);
      if (lookahead == '{') ADVANCE(61);
      if (lookahead == '}') ADVANCE(62);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(58)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(85);
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(17);
      if (('A' <= lookahead && lookahead <= 'Z')) ADVANCE(21);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_module_keyword);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(sym_var_keyword);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(sym_var_modifier);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(sym_module_identifier);
      if (lookahead == 'a') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_module_identifier);
      if (lookahead == 'e') ADVANCE(77);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(sym_module_identifier);
      if (lookahead == 'l') ADVANCE(72);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(sym_module_identifier);
      if (lookahead == 'o') ADVANCE(75);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_module_identifier);
      if (lookahead == 'r') ADVANCE(74);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_module_identifier);
      if (lookahead == 's') ADVANCE(68);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(sym_module_identifier);
      if (lookahead == 't') ADVANCE(76);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_module_identifier);
      if (lookahead == 'u') ADVANCE(68);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_module_identifier);
      if (lookahead == 'w') ADVANCE(73);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_module_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(83);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(sym_module_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(82);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(sym_module_identifier);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(78);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(aux_sym_var_identifier_token1);
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(79);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(sym_type_identifier);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(sym_type_identifier);
      if (lookahead == 'u') ADVANCE(11);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(sym_bool);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(sym_nowt);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(sym_float);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(84);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(sym_int);
      if (lookahead == '.') ADVANCE(50);
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(31);
      if (lookahead == 't') ADVANCE(34);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(86);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(sym_int);
      if (lookahead == '.') ADVANCE(50);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(86);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(sym_escape);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(sym_escape);
      if (lookahead == '\n' ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(87);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(sym_escape);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(87);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(sym_escape);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(89);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(sym_unescaped_string_fragment);
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(91);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead) &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(92);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(sym_unescaped_string_fragment);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(92);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 48},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 48},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 48},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 1},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 1},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 48},
  [25] = {.lex_state = 48},
  [26] = {.lex_state = 48},
  [27] = {.lex_state = 48},
  [28] = {.lex_state = 48},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 48},
  [33] = {.lex_state = 8},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 8},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_module_keyword] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [sym_var_keyword] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [sym_var_modifier] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [sym_module_identifier] = ACTIONS(1),
    [sym_type_identifier] = ACTIONS(1),
    [sym_bool] = ACTIONS(1),
    [sym_nowt] = ACTIONS(1),
    [sym_float] = ACTIONS(1),
    [sym_int] = ACTIONS(1),
    [sym_escape] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(37),
    [sym_module_declaration] = STATE(3),
    [sym_var_declaration] = STATE(3),
    [sym__declaration] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_module_keyword] = ACTIONS(5),
    [sym_var_keyword] = ACTIONS(7),
    [sym_var_modifier] = ACTIONS(9),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(13), 1,
      sym_int,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(11), 3,
      sym_bool,
      sym_nowt,
      sym_float,
    STATE(25), 3,
      sym__expression,
      sym__literal,
      sym_str,
  [17] = 5,
    ACTIONS(5), 1,
      sym_module_keyword,
    ACTIONS(7), 1,
      sym_var_keyword,
    ACTIONS(9), 1,
      sym_var_modifier,
    ACTIONS(17), 1,
      ts_builtin_sym_end,
    STATE(4), 4,
      sym_module_declaration,
      sym_var_declaration,
      sym__declaration,
      aux_sym_source_file_repeat1,
  [36] = 5,
    ACTIONS(19), 1,
      ts_builtin_sym_end,
    ACTIONS(21), 1,
      sym_module_keyword,
    ACTIONS(24), 1,
      sym_var_keyword,
    ACTIONS(27), 1,
      sym_var_modifier,
    STATE(4), 4,
      sym_module_declaration,
      sym_var_declaration,
      sym__declaration,
      aux_sym_source_file_repeat1,
  [55] = 3,
    ACTIONS(32), 1,
      anon_sym_LBRACE,
    STATE(9), 1,
      sym_var_block,
    ACTIONS(30), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_var_keyword,
      sym_var_modifier,
  [69] = 3,
    ACTIONS(32), 1,
      anon_sym_LBRACE,
    STATE(10), 1,
      sym_var_block,
    ACTIONS(34), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_var_keyword,
      sym_var_modifier,
  [83] = 4,
    ACTIONS(36), 1,
      anon_sym_RBRACE,
    ACTIONS(38), 1,
      sym_var_keyword,
    ACTIONS(41), 1,
      sym_var_modifier,
    STATE(7), 2,
      sym_var_declaration,
      aux_sym_module_block_repeat1,
  [97] = 4,
    ACTIONS(44), 1,
      anon_sym_RBRACE,
    ACTIONS(46), 1,
      aux_sym_var_identifier_token1,
    STATE(31), 1,
      sym_attribute_identifier,
    STATE(12), 2,
      sym_var_attribute,
      aux_sym_var_block_repeat1,
  [111] = 1,
    ACTIONS(48), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_var_keyword,
      sym_var_modifier,
  [119] = 1,
    ACTIONS(30), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_var_keyword,
      sym_var_modifier,
  [127] = 1,
    ACTIONS(50), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_var_keyword,
      sym_var_modifier,
  [135] = 4,
    ACTIONS(46), 1,
      aux_sym_var_identifier_token1,
    ACTIONS(52), 1,
      anon_sym_RBRACE,
    STATE(31), 1,
      sym_attribute_identifier,
    STATE(16), 2,
      sym_var_attribute,
      aux_sym_var_block_repeat1,
  [149] = 1,
    ACTIONS(54), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_var_keyword,
      sym_var_modifier,
  [157] = 4,
    ACTIONS(7), 1,
      sym_var_keyword,
    ACTIONS(9), 1,
      sym_var_modifier,
    ACTIONS(56), 1,
      anon_sym_RBRACE,
    STATE(15), 2,
      sym_var_declaration,
      aux_sym_module_block_repeat1,
  [171] = 4,
    ACTIONS(7), 1,
      sym_var_keyword,
    ACTIONS(9), 1,
      sym_var_modifier,
    ACTIONS(58), 1,
      anon_sym_RBRACE,
    STATE(7), 2,
      sym_var_declaration,
      aux_sym_module_block_repeat1,
  [185] = 4,
    ACTIONS(60), 1,
      anon_sym_RBRACE,
    ACTIONS(62), 1,
      aux_sym_var_identifier_token1,
    STATE(31), 1,
      sym_attribute_identifier,
    STATE(16), 2,
      sym_var_attribute,
      aux_sym_var_block_repeat1,
  [199] = 3,
    ACTIONS(67), 1,
      anon_sym_DQUOTE,
    STATE(22), 1,
      aux_sym_str_repeat1,
    ACTIONS(65), 2,
      sym_escape,
      sym_unescaped_string_fragment,
  [210] = 3,
    ACTIONS(71), 1,
      anon_sym_DQUOTE,
    STATE(17), 1,
      aux_sym_str_repeat1,
    ACTIONS(69), 2,
      sym_escape,
      sym_unescaped_string_fragment,
  [221] = 1,
    ACTIONS(73), 4,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [228] = 1,
    ACTIONS(75), 4,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [235] = 1,
    ACTIONS(77), 4,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [242] = 3,
    ACTIONS(82), 1,
      anon_sym_DQUOTE,
    STATE(22), 1,
      aux_sym_str_repeat1,
    ACTIONS(79), 2,
      sym_escape,
      sym_unescaped_string_fragment,
  [253] = 2,
    ACTIONS(84), 1,
      anon_sym_LBRACE,
    STATE(21), 1,
      sym_module_block,
  [260] = 2,
    ACTIONS(86), 1,
      aux_sym_var_identifier_token1,
    STATE(29), 1,
      sym_var_identifier,
  [267] = 1,
    ACTIONS(88), 2,
      anon_sym_RBRACE,
      aux_sym_var_identifier_token1,
  [272] = 2,
    ACTIONS(86), 1,
      aux_sym_var_identifier_token1,
    STATE(34), 1,
      sym_var_identifier,
  [279] = 1,
    ACTIONS(90), 2,
      anon_sym_RBRACE,
      aux_sym_var_identifier_token1,
  [284] = 1,
    ACTIONS(92), 2,
      anon_sym_RBRACE,
      aux_sym_var_identifier_token1,
  [289] = 1,
    ACTIONS(94), 1,
      anon_sym_COLON,
  [293] = 1,
    ACTIONS(96), 1,
      anon_sym_EQ,
  [297] = 1,
    ACTIONS(98), 1,
      anon_sym_EQ,
  [301] = 1,
    ACTIONS(100), 1,
      sym_module_identifier,
  [305] = 1,
    ACTIONS(102), 1,
      sym_type_identifier,
  [309] = 1,
    ACTIONS(104), 1,
      anon_sym_COLON,
  [313] = 1,
    ACTIONS(106), 1,
      sym_type_identifier,
  [317] = 1,
    ACTIONS(108), 1,
      anon_sym_COLON,
  [321] = 1,
    ACTIONS(110), 1,
      ts_builtin_sym_end,
  [325] = 1,
    ACTIONS(112), 1,
      sym_var_keyword,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 17,
  [SMALL_STATE(4)] = 36,
  [SMALL_STATE(5)] = 55,
  [SMALL_STATE(6)] = 69,
  [SMALL_STATE(7)] = 83,
  [SMALL_STATE(8)] = 97,
  [SMALL_STATE(9)] = 111,
  [SMALL_STATE(10)] = 119,
  [SMALL_STATE(11)] = 127,
  [SMALL_STATE(12)] = 135,
  [SMALL_STATE(13)] = 149,
  [SMALL_STATE(14)] = 157,
  [SMALL_STATE(15)] = 171,
  [SMALL_STATE(16)] = 185,
  [SMALL_STATE(17)] = 199,
  [SMALL_STATE(18)] = 210,
  [SMALL_STATE(19)] = 221,
  [SMALL_STATE(20)] = 228,
  [SMALL_STATE(21)] = 235,
  [SMALL_STATE(22)] = 242,
  [SMALL_STATE(23)] = 253,
  [SMALL_STATE(24)] = 260,
  [SMALL_STATE(25)] = 267,
  [SMALL_STATE(26)] = 272,
  [SMALL_STATE(27)] = 279,
  [SMALL_STATE(28)] = 284,
  [SMALL_STATE(29)] = 289,
  [SMALL_STATE(30)] = 293,
  [SMALL_STATE(31)] = 297,
  [SMALL_STATE(32)] = 301,
  [SMALL_STATE(33)] = 305,
  [SMALL_STATE(34)] = 309,
  [SMALL_STATE(35)] = 313,
  [SMALL_STATE(36)] = 317,
  [SMALL_STATE(37)] = 321,
  [SMALL_STATE(38)] = 325,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [21] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(32),
  [24] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(24),
  [27] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(38),
  [30] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_declaration, 5),
  [32] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_declaration, 4),
  [36] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_module_block_repeat1, 2),
  [38] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_module_block_repeat1, 2), SHIFT_REPEAT(24),
  [41] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_module_block_repeat1, 2), SHIFT_REPEAT(38),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_declaration, 6),
  [50] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_block, 3),
  [52] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_block, 2),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [58] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_var_block_repeat1, 2),
  [62] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_var_block_repeat1, 2), SHIFT_REPEAT(30),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [71] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_block, 2),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_block, 3),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_declaration, 3),
  [79] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_str_repeat1, 2), SHIFT_REPEAT(22),
  [82] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_str_repeat1, 2),
  [84] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [86] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_attribute, 3),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_str, 2),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_str, 3),
  [94] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [96] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_identifier, 1),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [102] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [106] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_identifier, 1),
  [110] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [112] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_nv(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
