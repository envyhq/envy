#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 19
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 23
#define ALIAS_COUNT 0
#define TOKEN_COUNT 14
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 4
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_module_keyword = 1,
  anon_sym_LBRACE = 2,
  anon_sym_RBRACE = 3,
  sym_var_keyword = 4,
  anon_sym_COLON = 5,
  anon_sym_EQ = 6,
  aux_sym_var_identifier_token1 = 7,
  sym_type_identifier = 8,
  sym_bool = 9,
  sym_nowt = 10,
  sym_str = 11,
  sym_float = 12,
  sym_int = 13,
  sym_source_file = 14,
  sym_module_declaration = 15,
  sym_module_block = 16,
  sym_var_declaration = 17,
  sym__declaration = 18,
  sym_var_identifier = 19,
  sym_module_identifier = 20,
  aux_sym_source_file_repeat1 = 21,
  aux_sym_module_block_repeat1 = 22,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_module_keyword] = "module_keyword",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [sym_var_keyword] = "var_keyword",
  [anon_sym_COLON] = ":",
  [anon_sym_EQ] = "=",
  [aux_sym_var_identifier_token1] = "var_identifier_token1",
  [sym_type_identifier] = "type_identifier",
  [sym_bool] = "bool",
  [sym_nowt] = "nowt",
  [sym_str] = "str",
  [sym_float] = "float",
  [sym_int] = "int",
  [sym_source_file] = "source_file",
  [sym_module_declaration] = "module_declaration",
  [sym_module_block] = "module_block",
  [sym_var_declaration] = "var_declaration",
  [sym__declaration] = "_declaration",
  [sym_var_identifier] = "var_identifier",
  [sym_module_identifier] = "module_identifier",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_module_block_repeat1] = "module_block_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_module_keyword] = sym_module_keyword,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [sym_var_keyword] = sym_var_keyword,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_EQ] = anon_sym_EQ,
  [aux_sym_var_identifier_token1] = aux_sym_var_identifier_token1,
  [sym_type_identifier] = sym_type_identifier,
  [sym_bool] = sym_bool,
  [sym_nowt] = sym_nowt,
  [sym_str] = sym_str,
  [sym_float] = sym_float,
  [sym_int] = sym_int,
  [sym_source_file] = sym_source_file,
  [sym_module_declaration] = sym_module_declaration,
  [sym_module_block] = sym_module_block,
  [sym_var_declaration] = sym_var_declaration,
  [sym__declaration] = sym__declaration,
  [sym_var_identifier] = sym_var_identifier,
  [sym_module_identifier] = sym_module_identifier,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_module_block_repeat1] = aux_sym_module_block_repeat1,
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
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
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
  [sym_str] = {
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
  [sym__declaration] = {
    .visible = false,
    .named = true,
  },
  [sym_var_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_module_identifier] = {
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(45);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == ':') ADVANCE(50);
      if (lookahead == '=') ADVANCE(51);
      if (lookahead == 'b') ADVANCE(12);
      if (lookahead == 'f') ADVANCE(9);
      if (lookahead == 'i') ADVANCE(16);
      if (lookahead == 'm') ADVANCE(10);
      if (lookahead == 'n') ADVANCE(11);
      if (lookahead == 's') ADVANCE(15);
      if (lookahead == 'u') ADVANCE(13);
      if (lookahead == 'v') ADVANCE(4);
      if (lookahead == '{') ADVANCE(47);
      if (lookahead == '}') ADVANCE(48);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(59);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(14);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(57);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(34);
      END_STATE();
    case 3:
      if (lookahead == 'a') ADVANCE(18);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(32);
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(26);
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 5:
      if (lookahead == 'b') ADVANCE(25);
      if (lookahead == 'f') ADVANCE(20);
      if (lookahead == 'i') ADVANCE(21);
      if (lookahead == 'n') ADVANCE(22);
      if (lookahead == 's') ADVANCE(36);
      if (lookahead == 'u') ADVANCE(28);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5)
      END_STATE();
    case 6:
      if (lookahead == 'd') ADVANCE(38);
      END_STATE();
    case 7:
      if (lookahead == 'e') ADVANCE(46);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(44);
      END_STATE();
    case 9:
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'l') ADVANCE(24);
      if (lookahead == 'n') ADVANCE(26);
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 10:
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(26);
      if (lookahead == 'o') ADVANCE(6);
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 11:
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(26);
      if (lookahead == 'o') ADVANCE(39);
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 12:
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(26);
      if (lookahead == 'o') ADVANCE(23);
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 13:
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(26);
      if (lookahead == 'r') ADVANCE(17);
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 14:
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(26);
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 15:
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(26);
      if (lookahead == 't') ADVANCE(31);
      END_STATE();
    case 16:
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(27);
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 17:
      if (lookahead == 'l') ADVANCE(53);
      END_STATE();
    case 18:
      if (lookahead == 'l') ADVANCE(33);
      END_STATE();
    case 19:
      if (lookahead == 'l') ADVANCE(7);
      END_STATE();
    case 20:
      if (lookahead == 'l') ADVANCE(24);
      END_STATE();
    case 21:
      if (lookahead == 'n') ADVANCE(34);
      END_STATE();
    case 22:
      if (lookahead == 'o') ADVANCE(39);
      END_STATE();
    case 23:
      if (lookahead == 'o') ADVANCE(17);
      END_STATE();
    case 24:
      if (lookahead == 'o') ADVANCE(2);
      END_STATE();
    case 25:
      if (lookahead == 'o') ADVANCE(23);
      END_STATE();
    case 26:
      if (lookahead == 'o') ADVANCE(40);
      END_STATE();
    case 27:
      if (lookahead == 'o') ADVANCE(40);
      if (lookahead == 't') ADVANCE(53);
      END_STATE();
    case 28:
      if (lookahead == 'r') ADVANCE(17);
      END_STATE();
    case 29:
      if (lookahead == 'r') ADVANCE(37);
      END_STATE();
    case 30:
      if (lookahead == 'r') ADVANCE(53);
      END_STATE();
    case 31:
      if (lookahead == 'r') ADVANCE(54);
      END_STATE();
    case 32:
      if (lookahead == 'r') ADVANCE(49);
      END_STATE();
    case 33:
      if (lookahead == 's') ADVANCE(8);
      END_STATE();
    case 34:
      if (lookahead == 't') ADVANCE(53);
      END_STATE();
    case 35:
      if (lookahead == 't') ADVANCE(43);
      END_STATE();
    case 36:
      if (lookahead == 't') ADVANCE(30);
      END_STATE();
    case 37:
      if (lookahead == 'u') ADVANCE(8);
      END_STATE();
    case 38:
      if (lookahead == 'u') ADVANCE(19);
      END_STATE();
    case 39:
      if (lookahead == 'w') ADVANCE(34);
      END_STATE();
    case 40:
      if (lookahead == 'w') ADVANCE(35);
      END_STATE();
    case 41:
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(41)
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(52);
      END_STATE();
    case 42:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(58);
      END_STATE();
    case 43:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(56);
      END_STATE();
    case 44:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(55);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_module_keyword);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_var_keyword);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(aux_sym_var_identifier_token1);
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(52);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_type_identifier);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_type_identifier);
      if (lookahead == 'u') ADVANCE(8);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_bool);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(sym_nowt);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(sym_str);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(sym_float);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(58);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(sym_int);
      if (lookahead == '.') ADVANCE(42);
      if (lookahead == 'f') ADVANCE(3);
      if (lookahead == 'n') ADVANCE(26);
      if (lookahead == 't') ADVANCE(29);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(60);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_int);
      if (lookahead == '.') ADVANCE(42);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(60);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 41},
  [12] = {.lex_state = 41},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 5},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_module_keyword] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [sym_var_keyword] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [sym_type_identifier] = ACTIONS(1),
    [sym_bool] = ACTIONS(1),
    [sym_nowt] = ACTIONS(1),
    [sym_str] = ACTIONS(1),
    [sym_float] = ACTIONS(1),
    [sym_int] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(14),
    [sym_module_declaration] = STATE(2),
    [sym_var_declaration] = STATE(2),
    [sym__declaration] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_module_keyword] = ACTIONS(5),
    [sym_var_keyword] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(5), 1,
      sym_module_keyword,
    ACTIONS(7), 1,
      sym_var_keyword,
    ACTIONS(9), 1,
      ts_builtin_sym_end,
    STATE(3), 4,
      sym_module_declaration,
      sym_var_declaration,
      sym__declaration,
      aux_sym_source_file_repeat1,
  [16] = 4,
    ACTIONS(11), 1,
      ts_builtin_sym_end,
    ACTIONS(13), 1,
      sym_module_keyword,
    ACTIONS(16), 1,
      sym_var_keyword,
    STATE(3), 4,
      sym_module_declaration,
      sym_var_declaration,
      sym__declaration,
      aux_sym_source_file_repeat1,
  [32] = 3,
    ACTIONS(7), 1,
      sym_var_keyword,
    ACTIONS(19), 1,
      anon_sym_RBRACE,
    STATE(5), 2,
      sym_var_declaration,
      aux_sym_module_block_repeat1,
  [43] = 3,
    ACTIONS(7), 1,
      sym_var_keyword,
    ACTIONS(21), 1,
      anon_sym_RBRACE,
    STATE(7), 2,
      sym_var_declaration,
      aux_sym_module_block_repeat1,
  [54] = 1,
    ACTIONS(23), 4,
      ts_builtin_sym_end,
      sym_module_keyword,
      anon_sym_RBRACE,
      sym_var_keyword,
  [61] = 3,
    ACTIONS(25), 1,
      anon_sym_RBRACE,
    ACTIONS(27), 1,
      sym_var_keyword,
    STATE(7), 2,
      sym_var_declaration,
      aux_sym_module_block_repeat1,
  [72] = 1,
    ACTIONS(30), 3,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_var_keyword,
  [78] = 1,
    ACTIONS(32), 3,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_var_keyword,
  [84] = 1,
    ACTIONS(34), 3,
      ts_builtin_sym_end,
      sym_module_keyword,
      sym_var_keyword,
  [90] = 2,
    ACTIONS(36), 1,
      aux_sym_var_identifier_token1,
    STATE(13), 1,
      sym_module_identifier,
  [97] = 2,
    ACTIONS(38), 1,
      aux_sym_var_identifier_token1,
    STATE(17), 1,
      sym_var_identifier,
  [104] = 2,
    ACTIONS(40), 1,
      anon_sym_LBRACE,
    STATE(8), 1,
      sym_module_block,
  [111] = 1,
    ACTIONS(42), 1,
      ts_builtin_sym_end,
  [115] = 1,
    ACTIONS(44), 1,
      anon_sym_LBRACE,
  [119] = 1,
    ACTIONS(46), 1,
      anon_sym_COLON,
  [123] = 1,
    ACTIONS(48), 1,
      anon_sym_COLON,
  [127] = 1,
    ACTIONS(50), 1,
      sym_type_identifier,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 16,
  [SMALL_STATE(4)] = 32,
  [SMALL_STATE(5)] = 43,
  [SMALL_STATE(6)] = 54,
  [SMALL_STATE(7)] = 61,
  [SMALL_STATE(8)] = 72,
  [SMALL_STATE(9)] = 78,
  [SMALL_STATE(10)] = 84,
  [SMALL_STATE(11)] = 90,
  [SMALL_STATE(12)] = 97,
  [SMALL_STATE(13)] = 104,
  [SMALL_STATE(14)] = 111,
  [SMALL_STATE(15)] = 115,
  [SMALL_STATE(16)] = 119,
  [SMALL_STATE(17)] = 123,
  [SMALL_STATE(18)] = 127,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [13] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(11),
  [16] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(12),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_declaration, 4),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_module_block_repeat1, 2),
  [27] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_module_block_repeat1, 2), SHIFT_REPEAT(12),
  [30] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_declaration, 3),
  [32] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_block, 2),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_block, 3),
  [36] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [40] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [42] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_module_identifier, 1),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_identifier, 1),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
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
