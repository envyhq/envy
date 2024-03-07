#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 52
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 42
#define ALIAS_COUNT 0
#define TOKEN_COUNT 20
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_module_keyword = 1,
  aux_sym_module_identifier_token1 = 2,
  anon_sym_LBRACE = 3,
  anon_sym_RBRACE = 4,
  sym_provider_keyword = 5,
  anon_sym_COLON = 6,
  aux_sym_provider_type_identifier_token1 = 7,
  anon_sym_EQ = 8,
  sym_var_keyword = 9,
  sym_var_modifier = 10,
  sym_type_identifier = 11,
  anon_sym_true = 12,
  anon_sym_false = 13,
  sym_nowt = 14,
  sym_float = 15,
  sym_int = 16,
  sym_escape = 17,
  sym_unescaped_string_fragment = 18,
  anon_sym_DQUOTE = 19,
  sym_source_file = 20,
  sym_module_declaration = 21,
  sym_module_identifier = 22,
  sym_module_block = 23,
  sym_provider_declaration = 24,
  sym_provider_type_identifier = 25,
  sym_provider_identifier = 26,
  sym_provider_block = 27,
  sym_attribute = 28,
  sym_attribute_identifier = 29,
  sym_var_declaration = 30,
  sym_var_block = 31,
  sym_var_identifier = 32,
  sym__declaration = 33,
  sym__expression = 34,
  sym__literal = 35,
  sym_bool = 36,
  sym_str = 37,
  aux_sym_source_file_repeat1 = 38,
  aux_sym_module_block_repeat1 = 39,
  aux_sym_provider_block_repeat1 = 40,
  aux_sym_str_repeat1 = 41,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_module_keyword] = "module_keyword",
  [aux_sym_module_identifier_token1] = "module_identifier_token1",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [sym_provider_keyword] = "provider_keyword",
  [anon_sym_COLON] = ":",
  [aux_sym_provider_type_identifier_token1] = "provider_type_identifier_token1",
  [anon_sym_EQ] = "=",
  [sym_var_keyword] = "var_keyword",
  [sym_var_modifier] = "var_modifier",
  [sym_type_identifier] = "type_identifier",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [sym_nowt] = "nowt",
  [sym_float] = "float",
  [sym_int] = "int",
  [sym_escape] = "escape",
  [sym_unescaped_string_fragment] = "unescaped_string_fragment",
  [anon_sym_DQUOTE] = "\"",
  [sym_source_file] = "source_file",
  [sym_module_declaration] = "module_declaration",
  [sym_module_identifier] = "module_identifier",
  [sym_module_block] = "module_block",
  [sym_provider_declaration] = "provider_declaration",
  [sym_provider_type_identifier] = "provider_type_identifier",
  [sym_provider_identifier] = "provider_identifier",
  [sym_provider_block] = "provider_block",
  [sym_attribute] = "attribute",
  [sym_attribute_identifier] = "attribute_identifier",
  [sym_var_declaration] = "var_declaration",
  [sym_var_block] = "var_block",
  [sym_var_identifier] = "var_identifier",
  [sym__declaration] = "_declaration",
  [sym__expression] = "_expression",
  [sym__literal] = "_literal",
  [sym_bool] = "bool",
  [sym_str] = "str",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_module_block_repeat1] = "module_block_repeat1",
  [aux_sym_provider_block_repeat1] = "provider_block_repeat1",
  [aux_sym_str_repeat1] = "str_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_module_keyword] = sym_module_keyword,
  [aux_sym_module_identifier_token1] = aux_sym_module_identifier_token1,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [sym_provider_keyword] = sym_provider_keyword,
  [anon_sym_COLON] = anon_sym_COLON,
  [aux_sym_provider_type_identifier_token1] = aux_sym_provider_type_identifier_token1,
  [anon_sym_EQ] = anon_sym_EQ,
  [sym_var_keyword] = sym_var_keyword,
  [sym_var_modifier] = sym_var_modifier,
  [sym_type_identifier] = sym_type_identifier,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [sym_nowt] = sym_nowt,
  [sym_float] = sym_float,
  [sym_int] = sym_int,
  [sym_escape] = sym_escape,
  [sym_unescaped_string_fragment] = sym_unescaped_string_fragment,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [sym_source_file] = sym_source_file,
  [sym_module_declaration] = sym_module_declaration,
  [sym_module_identifier] = sym_module_identifier,
  [sym_module_block] = sym_module_block,
  [sym_provider_declaration] = sym_provider_declaration,
  [sym_provider_type_identifier] = sym_provider_type_identifier,
  [sym_provider_identifier] = sym_provider_identifier,
  [sym_provider_block] = sym_provider_block,
  [sym_attribute] = sym_attribute,
  [sym_attribute_identifier] = sym_attribute_identifier,
  [sym_var_declaration] = sym_var_declaration,
  [sym_var_block] = sym_var_block,
  [sym_var_identifier] = sym_var_identifier,
  [sym__declaration] = sym__declaration,
  [sym__expression] = sym__expression,
  [sym__literal] = sym__literal,
  [sym_bool] = sym_bool,
  [sym_str] = sym_str,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_module_block_repeat1] = aux_sym_module_block_repeat1,
  [aux_sym_provider_block_repeat1] = aux_sym_provider_block_repeat1,
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
  [aux_sym_module_identifier_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [sym_provider_keyword] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_provider_type_identifier_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [sym_var_keyword] = {
    .visible = true,
    .named = true,
  },
  [sym_var_modifier] = {
    .visible = true,
    .named = true,
  },
  [sym_type_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
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
  [sym_module_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_module_block] = {
    .visible = true,
    .named = true,
  },
  [sym_provider_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym_provider_type_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_provider_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_provider_block] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute_identifier] = {
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
  [sym_var_identifier] = {
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
  [sym__literal] = {
    .visible = false,
    .named = true,
  },
  [sym_bool] = {
    .visible = true,
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
  [aux_sym_provider_block_repeat1] = {
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
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(57);
      if (lookahead == '"') ADVANCE(85);
      if (lookahead == ':') ADVANCE(67);
      if (lookahead == '=') ADVANCE(69);
      if (lookahead == '\\') ADVANCE(40);
      if (lookahead == 'b') ADVANCE(22);
      if (lookahead == 'f') ADVANCE(4);
      if (lookahead == 'i') ADVANCE(27);
      if (lookahead == 'm') ADVANCE(20);
      if (lookahead == 'n') ADVANCE(21);
      if (lookahead == 'p') ADVANCE(25);
      if (lookahead == 's') ADVANCE(26);
      if (lookahead == 't') ADVANCE(23);
      if (lookahead == 'u') ADVANCE(24);
      if (lookahead == 'v') ADVANCE(7);
      if (lookahead == '{') ADVANCE(64);
      if (lookahead == '}') ADVANCE(65);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(56)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(77);
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      if (('A' <= lookahead && lookahead <= 'Z')) ADVANCE(28);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(85);
      if (lookahead == '\\') ADVANCE(40);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(3)
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(83);
      if (lookahead != 0) ADVANCE(84);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(85);
      if (lookahead == 'f') ADVANCE(5);
      if (lookahead == 't') ADVANCE(23);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(77);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 3:
      if (lookahead == '"') ADVANCE(85);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(3)
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(16);
      if (lookahead == 'l') ADVANCE(29);
      if (lookahead == 'n') ADVANCE(32);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(16);
      if (lookahead == 'n') ADVANCE(32);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(38);
      END_STATE();
    case 7:
      if (lookahead == 'a') ADVANCE(35);
      if (lookahead == 'n') ADVANCE(32);
      END_STATE();
    case 8:
      if (lookahead == 'b') ADVANCE(71);
      END_STATE();
    case 9:
      if (lookahead == 'd') ADVANCE(42);
      END_STATE();
    case 10:
      if (lookahead == 'd') ADVANCE(14);
      END_STATE();
    case 11:
      if (lookahead == 'e') ADVANCE(73);
      END_STATE();
    case 12:
      if (lookahead == 'e') ADVANCE(74);
      END_STATE();
    case 13:
      if (lookahead == 'e') ADVANCE(58);
      END_STATE();
    case 14:
      if (lookahead == 'e') ADVANCE(36);
      END_STATE();
    case 15:
      if (lookahead == 'i') ADVANCE(10);
      END_STATE();
    case 16:
      if (lookahead == 'l') ADVANCE(37);
      END_STATE();
    case 17:
      if (lookahead == 'l') ADVANCE(72);
      END_STATE();
    case 18:
      if (lookahead == 'l') ADVANCE(13);
      END_STATE();
    case 19:
      if (lookahead == 'n') ADVANCE(32);
      END_STATE();
    case 20:
      if (lookahead == 'n') ADVANCE(32);
      if (lookahead == 'o') ADVANCE(9);
      END_STATE();
    case 21:
      if (lookahead == 'n') ADVANCE(32);
      if (lookahead == 'o') ADVANCE(44);
      END_STATE();
    case 22:
      if (lookahead == 'n') ADVANCE(32);
      if (lookahead == 'o') ADVANCE(31);
      END_STATE();
    case 23:
      if (lookahead == 'n') ADVANCE(32);
      if (lookahead == 'r') ADVANCE(41);
      END_STATE();
    case 24:
      if (lookahead == 'n') ADVANCE(32);
      if (lookahead == 'r') ADVANCE(17);
      END_STATE();
    case 25:
      if (lookahead == 'n') ADVANCE(32);
      if (lookahead == 'r') ADVANCE(30);
      if (lookahead == 'u') ADVANCE(8);
      END_STATE();
    case 26:
      if (lookahead == 'n') ADVANCE(32);
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 27:
      if (lookahead == 'n') ADVANCE(33);
      END_STATE();
    case 28:
      if (lookahead == 'n') ADVANCE(59);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(63);
      END_STATE();
    case 29:
      if (lookahead == 'o') ADVANCE(6);
      END_STATE();
    case 30:
      if (lookahead == 'o') ADVANCE(43);
      END_STATE();
    case 31:
      if (lookahead == 'o') ADVANCE(17);
      END_STATE();
    case 32:
      if (lookahead == 'o') ADVANCE(45);
      END_STATE();
    case 33:
      if (lookahead == 'o') ADVANCE(45);
      if (lookahead == 't') ADVANCE(72);
      END_STATE();
    case 34:
      if (lookahead == 'r') ADVANCE(72);
      END_STATE();
    case 35:
      if (lookahead == 'r') ADVANCE(70);
      END_STATE();
    case 36:
      if (lookahead == 'r') ADVANCE(66);
      END_STATE();
    case 37:
      if (lookahead == 's') ADVANCE(12);
      END_STATE();
    case 38:
      if (lookahead == 't') ADVANCE(72);
      END_STATE();
    case 39:
      if (lookahead == 't') ADVANCE(55);
      END_STATE();
    case 40:
      if (lookahead == 'u') ADVANCE(46);
      if (lookahead == 'x') ADVANCE(53);
      if (lookahead == '\r' ||
          lookahead == '?') ADVANCE(80);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(82);
      if (lookahead != 0) ADVANCE(79);
      END_STATE();
    case 41:
      if (lookahead == 'u') ADVANCE(11);
      END_STATE();
    case 42:
      if (lookahead == 'u') ADVANCE(18);
      END_STATE();
    case 43:
      if (lookahead == 'v') ADVANCE(15);
      END_STATE();
    case 44:
      if (lookahead == 'w') ADVANCE(38);
      END_STATE();
    case 45:
      if (lookahead == 'w') ADVANCE(39);
      END_STATE();
    case 46:
      if (lookahead == '{') ADVANCE(52);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(50);
      END_STATE();
    case 47:
      if (lookahead == '}') ADVANCE(65);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(47)
      if (('A' <= lookahead && lookahead <= 'Z')) ADVANCE(54);
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(68);
      END_STATE();
    case 48:
      if (lookahead == '}') ADVANCE(79);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(48);
      END_STATE();
    case 49:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(76);
      END_STATE();
    case 50:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(53);
      END_STATE();
    case 51:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(79);
      END_STATE();
    case 52:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(48);
      END_STATE();
    case 53:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(51);
      END_STATE();
    case 54:
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(63);
      END_STATE();
    case 55:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(75);
      END_STATE();
    case 56:
      if (eof) ADVANCE(57);
      if (lookahead == '"') ADVANCE(85);
      if (lookahead == ':') ADVANCE(67);
      if (lookahead == '=') ADVANCE(69);
      if (lookahead == 'b') ADVANCE(22);
      if (lookahead == 'f') ADVANCE(4);
      if (lookahead == 'i') ADVANCE(27);
      if (lookahead == 'm') ADVANCE(20);
      if (lookahead == 'n') ADVANCE(21);
      if (lookahead == 'p') ADVANCE(25);
      if (lookahead == 's') ADVANCE(26);
      if (lookahead == 't') ADVANCE(23);
      if (lookahead == 'u') ADVANCE(24);
      if (lookahead == 'v') ADVANCE(7);
      if (lookahead == '{') ADVANCE(64);
      if (lookahead == '}') ADVANCE(65);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(56)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(77);
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      if (('A' <= lookahead && lookahead <= 'Z')) ADVANCE(28);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(sym_module_keyword);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(aux_sym_module_identifier_token1);
      if (lookahead == 'o') ADVANCE(61);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(63);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(aux_sym_module_identifier_token1);
      if (lookahead == 't') ADVANCE(62);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(63);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(aux_sym_module_identifier_token1);
      if (lookahead == 'w') ADVANCE(60);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(63);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(aux_sym_module_identifier_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(75);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(63);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(aux_sym_module_identifier_token1);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(63);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(sym_provider_keyword);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(aux_sym_provider_type_identifier_token1);
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(68);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(sym_var_keyword);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_var_modifier);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_type_identifier);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_nowt);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_float);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(76);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(sym_int);
      if (lookahead == '.') ADVANCE(49);
      if (lookahead == 'n') ADVANCE(32);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(78);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(sym_int);
      if (lookahead == '.') ADVANCE(49);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(78);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(sym_escape);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(sym_escape);
      if (lookahead == '\n' ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(79);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(sym_escape);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(79);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(sym_escape);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(81);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(sym_unescaped_string_fragment);
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(83);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead) &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(84);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(sym_unescaped_string_fragment);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(84);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 47},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 47},
  [22] = {.lex_state = 47},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 47},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 47},
  [27] = {.lex_state = 1},
  [28] = {.lex_state = 1},
  [29] = {.lex_state = 1},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 47},
  [32] = {.lex_state = 47},
  [33] = {.lex_state = 47},
  [34] = {.lex_state = 47},
  [35] = {.lex_state = 47},
  [36] = {.lex_state = 47},
  [37] = {.lex_state = 47},
  [38] = {.lex_state = 47},
  [39] = {.lex_state = 47},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 0},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_module_keyword] = ACTIONS(1),
    [aux_sym_module_identifier_token1] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [sym_provider_keyword] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [sym_var_keyword] = ACTIONS(1),
    [sym_var_modifier] = ACTIONS(1),
    [sym_type_identifier] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [sym_nowt] = ACTIONS(1),
    [sym_float] = ACTIONS(1),
    [sym_int] = ACTIONS(1),
    [sym_escape] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(45),
    [sym_module_declaration] = STATE(4),
    [sym_provider_declaration] = STATE(4),
    [sym_var_declaration] = STATE(4),
    [sym__declaration] = STATE(4),
    [aux_sym_source_file_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_module_keyword] = ACTIONS(5),
    [sym_provider_keyword] = ACTIONS(7),
    [sym_var_keyword] = ACTIONS(9),
    [sym_var_modifier] = ACTIONS(11),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 6,
    ACTIONS(13), 1,
      ts_builtin_sym_end,
    ACTIONS(15), 1,
      sym_module_keyword,
    ACTIONS(18), 1,
      sym_provider_keyword,
    ACTIONS(21), 1,
      sym_var_keyword,
    ACTIONS(24), 1,
      sym_var_modifier,
    STATE(2), 5,
      sym_module_declaration,
      sym_provider_declaration,
      sym_var_declaration,
      sym__declaration,
      aux_sym_source_file_repeat1,
  [23] = 5,
    ACTIONS(31), 1,
      sym_int,
    ACTIONS(33), 1,
      anon_sym_DQUOTE,
    ACTIONS(27), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(29), 2,
      sym_nowt,
      sym_float,
    STATE(36), 4,
      sym__expression,
      sym__literal,
      sym_bool,
      sym_str,
  [44] = 6,
    ACTIONS(5), 1,
      sym_module_keyword,
    ACTIONS(7), 1,
      sym_provider_keyword,
    ACTIONS(9), 1,
      sym_var_keyword,
    ACTIONS(11), 1,
      sym_var_modifier,
    ACTIONS(35), 1,
      ts_builtin_sym_end,
    STATE(2), 5,
      sym_module_declaration,
      sym_provider_declaration,
      sym_var_declaration,
      sym__declaration,
      aux_sym_source_file_repeat1,
  [67] = 3,
    ACTIONS(39), 1,
      anon_sym_LBRACE,
    STATE(9), 1,
      sym_var_block,
    ACTIONS(37), 6,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [82] = 3,
    ACTIONS(39), 1,
      anon_sym_LBRACE,
    STATE(11), 1,
      sym_var_block,
    ACTIONS(41), 6,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [97] = 3,
    ACTIONS(45), 1,
      anon_sym_LBRACE,
    STATE(14), 1,
      sym_provider_block,
    ACTIONS(43), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [111] = 1,
    ACTIONS(47), 6,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [120] = 1,
    ACTIONS(49), 6,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [129] = 1,
    ACTIONS(51), 6,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [138] = 1,
    ACTIONS(37), 6,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [147] = 1,
    ACTIONS(53), 6,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_LBRACE,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [156] = 1,
    ACTIONS(55), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [164] = 1,
    ACTIONS(57), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [172] = 4,
    ACTIONS(9), 1,
      sym_var_keyword,
    ACTIONS(11), 1,
      sym_var_modifier,
    ACTIONS(59), 1,
      anon_sym_RBRACE,
    STATE(19), 2,
      sym_var_declaration,
      aux_sym_module_block_repeat1,
  [186] = 1,
    ACTIONS(61), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [194] = 4,
    ACTIONS(63), 1,
      anon_sym_RBRACE,
    ACTIONS(65), 1,
      aux_sym_provider_type_identifier_token1,
    STATE(40), 1,
      sym_attribute_identifier,
    STATE(17), 2,
      sym_attribute,
      aux_sym_provider_block_repeat1,
  [208] = 1,
    ACTIONS(68), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [216] = 4,
    ACTIONS(9), 1,
      sym_var_keyword,
    ACTIONS(11), 1,
      sym_var_modifier,
    ACTIONS(70), 1,
      anon_sym_RBRACE,
    STATE(25), 2,
      sym_var_declaration,
      aux_sym_module_block_repeat1,
  [230] = 1,
    ACTIONS(72), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [238] = 4,
    ACTIONS(74), 1,
      anon_sym_RBRACE,
    ACTIONS(76), 1,
      aux_sym_provider_type_identifier_token1,
    STATE(40), 1,
      sym_attribute_identifier,
    STATE(17), 2,
      sym_attribute,
      aux_sym_provider_block_repeat1,
  [252] = 4,
    ACTIONS(76), 1,
      aux_sym_provider_type_identifier_token1,
    ACTIONS(78), 1,
      anon_sym_RBRACE,
    STATE(40), 1,
      sym_attribute_identifier,
    STATE(17), 2,
      sym_attribute,
      aux_sym_provider_block_repeat1,
  [266] = 1,
    ACTIONS(80), 5,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_provider_keyword,
      sym_var_keyword,
      sym_var_modifier,
  [274] = 4,
    ACTIONS(76), 1,
      aux_sym_provider_type_identifier_token1,
    ACTIONS(82), 1,
      anon_sym_RBRACE,
    STATE(40), 1,
      sym_attribute_identifier,
    STATE(21), 2,
      sym_attribute,
      aux_sym_provider_block_repeat1,
  [288] = 4,
    ACTIONS(84), 1,
      anon_sym_RBRACE,
    ACTIONS(86), 1,
      sym_var_keyword,
    ACTIONS(89), 1,
      sym_var_modifier,
    STATE(25), 2,
      sym_var_declaration,
      aux_sym_module_block_repeat1,
  [302] = 4,
    ACTIONS(76), 1,
      aux_sym_provider_type_identifier_token1,
    ACTIONS(92), 1,
      anon_sym_RBRACE,
    STATE(40), 1,
      sym_attribute_identifier,
    STATE(22), 2,
      sym_attribute,
      aux_sym_provider_block_repeat1,
  [316] = 3,
    ACTIONS(97), 1,
      anon_sym_DQUOTE,
    STATE(27), 1,
      aux_sym_str_repeat1,
    ACTIONS(94), 2,
      sym_escape,
      sym_unescaped_string_fragment,
  [327] = 3,
    ACTIONS(101), 1,
      anon_sym_DQUOTE,
    STATE(27), 1,
      aux_sym_str_repeat1,
    ACTIONS(99), 2,
      sym_escape,
      sym_unescaped_string_fragment,
  [338] = 3,
    ACTIONS(105), 1,
      anon_sym_DQUOTE,
    STATE(28), 1,
      aux_sym_str_repeat1,
    ACTIONS(103), 2,
      sym_escape,
      sym_unescaped_string_fragment,
  [349] = 2,
    ACTIONS(107), 1,
      anon_sym_LBRACE,
    STATE(16), 1,
      sym_module_block,
  [356] = 2,
    ACTIONS(109), 1,
      aux_sym_provider_type_identifier_token1,
    STATE(43), 1,
      sym_var_identifier,
  [363] = 2,
    ACTIONS(111), 1,
      aux_sym_provider_type_identifier_token1,
    STATE(7), 1,
      sym_provider_type_identifier,
  [370] = 1,
    ACTIONS(113), 2,
      anon_sym_RBRACE,
      aux_sym_provider_type_identifier_token1,
  [375] = 2,
    ACTIONS(109), 1,
      aux_sym_provider_type_identifier_token1,
    STATE(46), 1,
      sym_var_identifier,
  [382] = 1,
    ACTIONS(115), 2,
      anon_sym_RBRACE,
      aux_sym_provider_type_identifier_token1,
  [387] = 1,
    ACTIONS(117), 2,
      anon_sym_RBRACE,
      aux_sym_provider_type_identifier_token1,
  [392] = 1,
    ACTIONS(119), 2,
      anon_sym_RBRACE,
      aux_sym_provider_type_identifier_token1,
  [397] = 2,
    ACTIONS(121), 1,
      aux_sym_module_identifier_token1,
    STATE(44), 1,
      sym_provider_identifier,
  [404] = 2,
    ACTIONS(123), 1,
      aux_sym_module_identifier_token1,
    STATE(30), 1,
      sym_module_identifier,
  [411] = 1,
    ACTIONS(125), 1,
      anon_sym_EQ,
  [415] = 1,
    ACTIONS(127), 1,
      sym_var_keyword,
  [419] = 1,
    ACTIONS(129), 1,
      anon_sym_COLON,
  [423] = 1,
    ACTIONS(131), 1,
      anon_sym_COLON,
  [427] = 1,
    ACTIONS(133), 1,
      anon_sym_COLON,
  [431] = 1,
    ACTIONS(135), 1,
      ts_builtin_sym_end,
  [435] = 1,
    ACTIONS(137), 1,
      anon_sym_COLON,
  [439] = 1,
    ACTIONS(139), 1,
      sym_type_identifier,
  [443] = 1,
    ACTIONS(141), 1,
      anon_sym_COLON,
  [447] = 1,
    ACTIONS(143), 1,
      sym_type_identifier,
  [451] = 1,
    ACTIONS(145), 1,
      anon_sym_EQ,
  [455] = 1,
    ACTIONS(147), 1,
      anon_sym_LBRACE,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 23,
  [SMALL_STATE(4)] = 44,
  [SMALL_STATE(5)] = 67,
  [SMALL_STATE(6)] = 82,
  [SMALL_STATE(7)] = 97,
  [SMALL_STATE(8)] = 111,
  [SMALL_STATE(9)] = 120,
  [SMALL_STATE(10)] = 129,
  [SMALL_STATE(11)] = 138,
  [SMALL_STATE(12)] = 147,
  [SMALL_STATE(13)] = 156,
  [SMALL_STATE(14)] = 164,
  [SMALL_STATE(15)] = 172,
  [SMALL_STATE(16)] = 186,
  [SMALL_STATE(17)] = 194,
  [SMALL_STATE(18)] = 208,
  [SMALL_STATE(19)] = 216,
  [SMALL_STATE(20)] = 230,
  [SMALL_STATE(21)] = 238,
  [SMALL_STATE(22)] = 252,
  [SMALL_STATE(23)] = 266,
  [SMALL_STATE(24)] = 274,
  [SMALL_STATE(25)] = 288,
  [SMALL_STATE(26)] = 302,
  [SMALL_STATE(27)] = 316,
  [SMALL_STATE(28)] = 327,
  [SMALL_STATE(29)] = 338,
  [SMALL_STATE(30)] = 349,
  [SMALL_STATE(31)] = 356,
  [SMALL_STATE(32)] = 363,
  [SMALL_STATE(33)] = 370,
  [SMALL_STATE(34)] = 375,
  [SMALL_STATE(35)] = 382,
  [SMALL_STATE(36)] = 387,
  [SMALL_STATE(37)] = 392,
  [SMALL_STATE(38)] = 397,
  [SMALL_STATE(39)] = 404,
  [SMALL_STATE(40)] = 411,
  [SMALL_STATE(41)] = 415,
  [SMALL_STATE(42)] = 419,
  [SMALL_STATE(43)] = 423,
  [SMALL_STATE(44)] = 427,
  [SMALL_STATE(45)] = 431,
  [SMALL_STATE(46)] = 435,
  [SMALL_STATE(47)] = 439,
  [SMALL_STATE(48)] = 443,
  [SMALL_STATE(49)] = 447,
  [SMALL_STATE(50)] = 451,
  [SMALL_STATE(51)] = 455,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [15] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(39),
  [18] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(38),
  [21] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(31),
  [24] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(41),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(36),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_declaration, 5),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_declaration, 4),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_provider_declaration, 4),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_block, 3),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_declaration, 6),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_block, 2),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_provider_type_identifier, 1),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_block, 3),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_provider_declaration, 5),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_declaration, 3),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_provider_block_repeat1, 2),
  [65] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_provider_block_repeat1, 2), SHIFT_REPEAT(50),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_block, 2),
  [70] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [72] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_provider_block, 3),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [76] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [78] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [80] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_provider_block, 2),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_module_block_repeat1, 2),
  [86] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_module_block_repeat1, 2), SHIFT_REPEAT(31),
  [89] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_module_block_repeat1, 2), SHIFT_REPEAT(41),
  [92] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [94] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_str_repeat1, 2), SHIFT_REPEAT(27),
  [97] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_str_repeat1, 2),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [101] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [105] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [111] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_str, 3),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_str, 2),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 3),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bool, 1),
  [121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [123] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [127] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [129] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_identifier, 1),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [133] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [135] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [137] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [139] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_provider_identifier, 1),
  [143] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_identifier, 1),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_identifier, 1),
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
