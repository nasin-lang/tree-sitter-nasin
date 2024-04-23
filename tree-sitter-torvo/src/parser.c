#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 134
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 54
#define ALIAS_COUNT 0
#define TOKEN_COUNT 26
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 20
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 32

enum ts_symbol_identifiers {
  sym__ident = 1,
  anon_sym_fn = 2,
  anon_sym_LPAREN = 3,
  anon_sym_COMMA = 4,
  anon_sym_RPAREN = 5,
  anon_sym_COLON = 6,
  anon_sym_EQ = 7,
  anon_sym_DQUOTE = 8,
  anon_sym_DQUOTE2 = 9,
  sym_string_lit_content = 10,
  anon_sym_LBRACK = 11,
  anon_sym_RBRACK = 12,
  anon_sym_SEMI = 13,
  anon_sym_if = 14,
  anon_sym_then = 15,
  anon_sym_else = 16,
  sym_plus = 17,
  sym_minus = 18,
  sym_star = 19,
  sym_double_star = 20,
  sym_slash = 21,
  sym_percent = 22,
  anon_sym_true = 23,
  anon_sym_false = 24,
  aux_sym_number_token1 = 25,
  sym_root = 26,
  sym__module_stmt = 27,
  sym_fn_decl = 28,
  sym_fn_param = 29,
  sym_global_var_decl = 30,
  sym_var_decl = 31,
  sym__expr = 32,
  sym_bin_op = 33,
  sym_call = 34,
  sym__call_args_list = 35,
  sym_string_lit = 36,
  sym_array_lit = 37,
  sym_block = 38,
  sym__block = 39,
  sym__block_clause = 40,
  sym__block_stmt = 41,
  sym_if = 42,
  sym__type_expr = 43,
  sym_array_type = 44,
  sym__pat = 45,
  sym_ident = 46,
  sym_true = 47,
  sym_false = 48,
  sym_number = 49,
  aux_sym_root_repeat1 = 50,
  aux_sym_fn_decl_repeat1 = 51,
  aux_sym__call_args_list_repeat1 = 52,
  aux_sym_array_lit_repeat1 = 53,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym__ident] = "_ident",
  [anon_sym_fn] = "fn",
  [anon_sym_LPAREN] = "(",
  [anon_sym_COMMA] = ",",
  [anon_sym_RPAREN] = ")",
  [anon_sym_COLON] = ":",
  [anon_sym_EQ] = "=",
  [anon_sym_DQUOTE] = "\"",
  [anon_sym_DQUOTE2] = "\"",
  [sym_string_lit_content] = "string_lit_content",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_SEMI] = ";",
  [anon_sym_if] = "if",
  [anon_sym_then] = "then",
  [anon_sym_else] = "else",
  [sym_plus] = "plus",
  [sym_minus] = "minus",
  [sym_star] = "star",
  [sym_double_star] = "double_star",
  [sym_slash] = "slash",
  [sym_percent] = "percent",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [aux_sym_number_token1] = "number_token1",
  [sym_root] = "root",
  [sym__module_stmt] = "_module_stmt",
  [sym_fn_decl] = "fn_decl",
  [sym_fn_param] = "fn_param",
  [sym_global_var_decl] = "global_var_decl",
  [sym_var_decl] = "var_decl",
  [sym__expr] = "_expr",
  [sym_bin_op] = "bin_op",
  [sym_call] = "call",
  [sym__call_args_list] = "_call_args_list",
  [sym_string_lit] = "string_lit",
  [sym_array_lit] = "array_lit",
  [sym_block] = "block",
  [sym__block] = "_block",
  [sym__block_clause] = "_block_clause",
  [sym__block_stmt] = "_block_stmt",
  [sym_if] = "if",
  [sym__type_expr] = "_type_expr",
  [sym_array_type] = "array_type",
  [sym__pat] = "_pat",
  [sym_ident] = "ident",
  [sym_true] = "true",
  [sym_false] = "false",
  [sym_number] = "number",
  [aux_sym_root_repeat1] = "root_repeat1",
  [aux_sym_fn_decl_repeat1] = "fn_decl_repeat1",
  [aux_sym__call_args_list_repeat1] = "_call_args_list_repeat1",
  [aux_sym_array_lit_repeat1] = "array_lit_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym__ident] = sym__ident,
  [anon_sym_fn] = anon_sym_fn,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [anon_sym_DQUOTE2] = anon_sym_DQUOTE,
  [sym_string_lit_content] = sym_string_lit_content,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_then] = anon_sym_then,
  [anon_sym_else] = anon_sym_else,
  [sym_plus] = sym_plus,
  [sym_minus] = sym_minus,
  [sym_star] = sym_star,
  [sym_double_star] = sym_double_star,
  [sym_slash] = sym_slash,
  [sym_percent] = sym_percent,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [aux_sym_number_token1] = aux_sym_number_token1,
  [sym_root] = sym_root,
  [sym__module_stmt] = sym__module_stmt,
  [sym_fn_decl] = sym_fn_decl,
  [sym_fn_param] = sym_fn_param,
  [sym_global_var_decl] = sym_global_var_decl,
  [sym_var_decl] = sym_var_decl,
  [sym__expr] = sym__expr,
  [sym_bin_op] = sym_bin_op,
  [sym_call] = sym_call,
  [sym__call_args_list] = sym__call_args_list,
  [sym_string_lit] = sym_string_lit,
  [sym_array_lit] = sym_array_lit,
  [sym_block] = sym_block,
  [sym__block] = sym__block,
  [sym__block_clause] = sym__block_clause,
  [sym__block_stmt] = sym__block_stmt,
  [sym_if] = sym_if,
  [sym__type_expr] = sym__type_expr,
  [sym_array_type] = sym_array_type,
  [sym__pat] = sym__pat,
  [sym_ident] = sym_ident,
  [sym_true] = sym_true,
  [sym_false] = sym_false,
  [sym_number] = sym_number,
  [aux_sym_root_repeat1] = aux_sym_root_repeat1,
  [aux_sym_fn_decl_repeat1] = aux_sym_fn_decl_repeat1,
  [aux_sym__call_args_list_repeat1] = aux_sym__call_args_list_repeat1,
  [aux_sym_array_lit_repeat1] = aux_sym_array_lit_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym__ident] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_fn] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DQUOTE2] = {
    .visible = true,
    .named = false,
  },
  [sym_string_lit_content] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_then] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [sym_plus] = {
    .visible = true,
    .named = true,
  },
  [sym_minus] = {
    .visible = true,
    .named = true,
  },
  [sym_star] = {
    .visible = true,
    .named = true,
  },
  [sym_double_star] = {
    .visible = true,
    .named = true,
  },
  [sym_slash] = {
    .visible = true,
    .named = true,
  },
  [sym_percent] = {
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
  [aux_sym_number_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_root] = {
    .visible = true,
    .named = true,
  },
  [sym__module_stmt] = {
    .visible = false,
    .named = true,
  },
  [sym_fn_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_fn_param] = {
    .visible = true,
    .named = true,
  },
  [sym_global_var_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_var_decl] = {
    .visible = true,
    .named = true,
  },
  [sym__expr] = {
    .visible = false,
    .named = true,
  },
  [sym_bin_op] = {
    .visible = true,
    .named = true,
  },
  [sym_call] = {
    .visible = true,
    .named = true,
  },
  [sym__call_args_list] = {
    .visible = false,
    .named = true,
  },
  [sym_string_lit] = {
    .visible = true,
    .named = true,
  },
  [sym_array_lit] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym__block] = {
    .visible = false,
    .named = true,
  },
  [sym__block_clause] = {
    .visible = false,
    .named = true,
  },
  [sym__block_stmt] = {
    .visible = false,
    .named = true,
  },
  [sym_if] = {
    .visible = true,
    .named = true,
  },
  [sym__type_expr] = {
    .visible = false,
    .named = true,
  },
  [sym_array_type] = {
    .visible = true,
    .named = true,
  },
  [sym__pat] = {
    .visible = false,
    .named = true,
  },
  [sym_ident] = {
    .visible = true,
    .named = true,
  },
  [sym_true] = {
    .visible = true,
    .named = true,
  },
  [sym_false] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_root_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_fn_decl_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__call_args_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_array_lit_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_args = 1,
  field_body = 2,
  field_callee = 3,
  field_cond = 4,
  field_content = 5,
  field_else = 6,
  field_item_type = 7,
  field_items = 8,
  field_left = 9,
  field_length = 10,
  field_name = 11,
  field_op = 12,
  field_params = 13,
  field_pat = 14,
  field_ret_type = 15,
  field_return = 16,
  field_right = 17,
  field_then = 18,
  field_type = 19,
  field_value = 20,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_args] = "args",
  [field_body] = "body",
  [field_callee] = "callee",
  [field_cond] = "cond",
  [field_content] = "content",
  [field_else] = "else",
  [field_item_type] = "item_type",
  [field_items] = "items",
  [field_left] = "left",
  [field_length] = "length",
  [field_name] = "name",
  [field_op] = "op",
  [field_params] = "params",
  [field_pat] = "pat",
  [field_ret_type] = "ret_type",
  [field_return] = "return",
  [field_right] = "right",
  [field_then] = "then",
  [field_type] = "type",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 2},
  [3] = {.index = 4, .length = 1},
  [4] = {.index = 5, .length = 1},
  [5] = {.index = 6, .length = 1},
  [6] = {.index = 7, .length = 2},
  [7] = {.index = 9, .length = 3},
  [8] = {.index = 12, .length = 2},
  [9] = {.index = 14, .length = 1},
  [10] = {.index = 15, .length = 3},
  [11] = {.index = 18, .length = 1},
  [12] = {.index = 19, .length = 1},
  [13] = {.index = 20, .length = 2},
  [14] = {.index = 22, .length = 1},
  [15] = {.index = 23, .length = 1},
  [16] = {.index = 24, .length = 1},
  [17] = {.index = 25, .length = 3},
  [18] = {.index = 28, .length = 2},
  [19] = {.index = 30, .length = 3},
  [20] = {.index = 33, .length = 2},
  [21] = {.index = 35, .length = 2},
  [22] = {.index = 37, .length = 2},
  [23] = {.index = 39, .length = 2},
  [24] = {.index = 41, .length = 2},
  [25] = {.index = 43, .length = 2},
  [26] = {.index = 45, .length = 3},
  [27] = {.index = 48, .length = 2},
  [28] = {.index = 50, .length = 3},
  [29] = {.index = 53, .length = 3},
  [30] = {.index = 56, .length = 3},
  [31] = {.index = 59, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 0},
    {field_value, 2},
  [2] =
    {field_body, 0, .inherited = true},
    {field_value, 0, .inherited = true},
  [4] =
    {field_params, 0},
  [5] =
    {field_pat, 0},
  [6] =
    {field_items, 0},
  [7] =
    {field_body, 0},
    {field_value, 1},
  [9] =
    {field_body, 0},
    {field_body, 1, .inherited = true},
    {field_value, 1, .inherited = true},
  [12] =
    {field_params, 0, .inherited = true},
    {field_params, 1, .inherited = true},
  [14] =
    {field_item_type, 1},
  [15] =
    {field_name, 0},
    {field_type, 2},
    {field_value, 4},
  [18] =
    {field_content, 1},
  [19] =
    {field_items, 1, .inherited = true},
  [20] =
    {field_items, 0, .inherited = true},
    {field_items, 1, .inherited = true},
  [22] =
    {field_callee, 0},
  [23] =
    {field_args, 0},
  [24] =
    {field_args, 0, .inherited = true},
  [25] =
    {field_left, 0},
    {field_op, 1},
    {field_right, 2},
  [28] =
    {field_body, 0},
    {field_value, 2},
  [30] =
    {field_body, 0},
    {field_body, 2, .inherited = true},
    {field_value, 2, .inherited = true},
  [33] =
    {field_pat, 0},
    {field_value, 2},
  [35] =
    {field_name, 1},
    {field_return, 5},
  [37] =
    {field_pat, 0},
    {field_type, 2},
  [39] =
    {field_cond, 1},
    {field_then, 3},
  [41] =
    {field_args, 2, .inherited = true},
    {field_callee, 0},
  [43] =
    {field_args, 0, .inherited = true},
    {field_args, 1, .inherited = true},
  [45] =
    {field_name, 1},
    {field_params, 3, .inherited = true},
    {field_return, 6},
  [48] =
    {field_item_type, 1},
    {field_length, 3},
  [50] =
    {field_pat, 0},
    {field_type, 2},
    {field_value, 4},
  [53] =
    {field_name, 1},
    {field_ret_type, 5},
    {field_return, 7},
  [56] =
    {field_cond, 1},
    {field_else, 5},
    {field_then, 3},
  [59] =
    {field_name, 1},
    {field_params, 3, .inherited = true},
    {field_ret_type, 6},
    {field_return, 8},
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
  [3] = 2,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 4,
  [9] = 7,
  [10] = 10,
  [11] = 11,
  [12] = 11,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 16,
  [18] = 18,
  [19] = 14,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 15,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 28,
  [33] = 29,
  [34] = 30,
  [35] = 31,
  [36] = 22,
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
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 37,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 38,
  [65] = 53,
  [66] = 56,
  [67] = 48,
  [68] = 40,
  [69] = 55,
  [70] = 52,
  [71] = 51,
  [72] = 39,
  [73] = 58,
  [74] = 46,
  [75] = 47,
  [76] = 41,
  [77] = 42,
  [78] = 54,
  [79] = 57,
  [80] = 44,
  [81] = 43,
  [82] = 50,
  [83] = 49,
  [84] = 45,
  [85] = 85,
  [86] = 86,
  [87] = 87,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 93,
  [96] = 96,
  [97] = 97,
  [98] = 96,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 123,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 125,
  [131] = 131,
  [132] = 121,
  [133] = 133,
};

static inline bool sym__ident_character_set_1(int32_t c) {
  return (c < 6656
    ? (c < 2979
      ? (c < 2308
        ? (c < 1376
          ? (c < 880
            ? (c < 192
              ? (c < 170
                ? (c < '_'
                  ? (c >= 'A' && c <= 'Z')
                  : (c <= '_' || (c >= 'a' && c <= 'z')))
                : (c <= 170 || (c < 186
                  ? c == 181
                  : c <= 186)))
              : (c <= 214 || (c < 736
                ? (c < 248
                  ? (c >= 216 && c <= 246)
                  : (c <= 705 || (c >= 710 && c <= 721)))
                : (c <= 740 || (c < 750
                  ? c == 748
                  : c <= 750)))))
            : (c <= 884 || (c < 910
              ? (c < 902
                ? (c < 890
                  ? (c >= 886 && c <= 887)
                  : (c <= 893 || c == 895))
                : (c <= 902 || (c < 908
                  ? (c >= 904 && c <= 906)
                  : c <= 908)))
              : (c <= 929 || (c < 1162
                ? (c < 1015
                  ? (c >= 931 && c <= 1013)
                  : c <= 1153)
                : (c <= 1327 || (c < 1369
                  ? (c >= 1329 && c <= 1366)
                  : c <= 1369)))))))
          : (c <= 1416 || (c < 1969
            ? (c < 1765
              ? (c < 1646
                ? (c < 1519
                  ? (c >= 1488 && c <= 1514)
                  : (c <= 1522 || (c >= 1568 && c <= 1610)))
                : (c <= 1647 || (c < 1749
                  ? (c >= 1649 && c <= 1747)
                  : c <= 1749)))
              : (c <= 1766 || (c < 1808
                ? (c < 1786
                  ? (c >= 1774 && c <= 1775)
                  : (c <= 1788 || c == 1791))
                : (c <= 1808 || (c < 1869
                  ? (c >= 1810 && c <= 1839)
                  : c <= 1957)))))
            : (c <= 1969 || (c < 2088
              ? (c < 2048
                ? (c < 2036
                  ? (c >= 1994 && c <= 2026)
                  : (c <= 2037 || c == 2042))
                : (c <= 2069 || (c < 2084
                  ? c == 2074
                  : c <= 2084)))
              : (c <= 2088 || (c < 2160
                ? (c < 2144
                  ? (c >= 2112 && c <= 2136)
                  : c <= 2154)
                : (c <= 2183 || (c < 2208
                  ? (c >= 2185 && c <= 2190)
                  : c <= 2249)))))))))
        : (c <= 2361 || (c < 2693
          ? (c < 2527
            ? (c < 2451
              ? (c < 2417
                ? (c < 2384
                  ? c == 2365
                  : (c <= 2384 || (c >= 2392 && c <= 2401)))
                : (c <= 2432 || (c < 2447
                  ? (c >= 2437 && c <= 2444)
                  : c <= 2448)))
              : (c <= 2472 || (c < 2493
                ? (c < 2482
                  ? (c >= 2474 && c <= 2480)
                  : (c <= 2482 || (c >= 2486 && c <= 2489)))
                : (c <= 2493 || (c < 2524
                  ? c == 2510
                  : c <= 2525)))))
            : (c <= 2529 || (c < 2610
              ? (c < 2575
                ? (c < 2556
                  ? (c >= 2544 && c <= 2545)
                  : (c <= 2556 || (c >= 2565 && c <= 2570)))
                : (c <= 2576 || (c < 2602
                  ? (c >= 2579 && c <= 2600)
                  : c <= 2608)))
              : (c <= 2611 || (c < 2649
                ? (c < 2616
                  ? (c >= 2613 && c <= 2614)
                  : c <= 2617)
                : (c <= 2652 || (c < 2674
                  ? c == 2654
                  : c <= 2676)))))))
          : (c <= 2701 || (c < 2866
            ? (c < 2768
              ? (c < 2738
                ? (c < 2707
                  ? (c >= 2703 && c <= 2705)
                  : (c <= 2728 || (c >= 2730 && c <= 2736)))
                : (c <= 2739 || (c < 2749
                  ? (c >= 2741 && c <= 2745)
                  : c <= 2749)))
              : (c <= 2768 || (c < 2831
                ? (c < 2809
                  ? (c >= 2784 && c <= 2785)
                  : (c <= 2809 || (c >= 2821 && c <= 2828)))
                : (c <= 2832 || (c < 2858
                  ? (c >= 2835 && c <= 2856)
                  : c <= 2864)))))
            : (c <= 2867 || (c < 2949
              ? (c < 2911
                ? (c < 2877
                  ? (c >= 2869 && c <= 2873)
                  : (c <= 2877 || (c >= 2908 && c <= 2909)))
                : (c <= 2913 || (c < 2947
                  ? c == 2929
                  : c <= 2947)))
              : (c <= 2954 || (c < 2969
                ? (c < 2962
                  ? (c >= 2958 && c <= 2960)
                  : c <= 2965)
                : (c <= 2970 || (c < 2974
                  ? c == 2972
                  : c <= 2975)))))))))))
      : (c <= 2980 || (c < 4159
        ? (c < 3412
          ? (c < 3214
            ? (c < 3114
              ? (c < 3077
                ? (c < 2990
                  ? (c >= 2984 && c <= 2986)
                  : (c <= 3001 || c == 3024))
                : (c <= 3084 || (c < 3090
                  ? (c >= 3086 && c <= 3088)
                  : c <= 3112)))
              : (c <= 3129 || (c < 3168
                ? (c < 3160
                  ? c == 3133
                  : (c <= 3162 || c == 3165))
                : (c <= 3169 || (c < 3205
                  ? c == 3200
                  : c <= 3212)))))
            : (c <= 3216 || (c < 3313
              ? (c < 3261
                ? (c < 3242
                  ? (c >= 3218 && c <= 3240)
                  : (c <= 3251 || (c >= 3253 && c <= 3257)))
                : (c <= 3261 || (c < 3296
                  ? (c >= 3293 && c <= 3294)
                  : c <= 3297)))
              : (c <= 3314 || (c < 3346
                ? (c < 3342
                  ? (c >= 3332 && c <= 3340)
                  : c <= 3344)
                : (c <= 3386 || (c < 3406
                  ? c == 3389
                  : c <= 3406)))))))
          : (c <= 3414 || (c < 3724
            ? (c < 3520
              ? (c < 3482
                ? (c < 3450
                  ? (c >= 3423 && c <= 3425)
                  : (c <= 3455 || (c >= 3461 && c <= 3478)))
                : (c <= 3505 || (c < 3517
                  ? (c >= 3507 && c <= 3515)
                  : c <= 3517)))
              : (c <= 3526 || (c < 3713
                ? (c < 3634
                  ? (c >= 3585 && c <= 3632)
                  : (c <= 3635 || (c >= 3648 && c <= 3654)))
                : (c <= 3714 || (c < 3718
                  ? c == 3716
                  : c <= 3722)))))
            : (c <= 3747 || (c < 3804
              ? (c < 3773
                ? (c < 3751
                  ? c == 3749
                  : (c <= 3760 || (c >= 3762 && c <= 3763)))
                : (c <= 3773 || (c < 3782
                  ? (c >= 3776 && c <= 3780)
                  : c <= 3782)))
              : (c <= 3807 || (c < 3913
                ? (c < 3904
                  ? c == 3840
                  : c <= 3911)
                : (c <= 3948 || (c < 4096
                  ? (c >= 3976 && c <= 3980)
                  : c <= 4138)))))))))
        : (c <= 4159 || (c < 4888
          ? (c < 4688
            ? (c < 4238
              ? (c < 4197
                ? (c < 4186
                  ? (c >= 4176 && c <= 4181)
                  : (c <= 4189 || c == 4193))
                : (c <= 4198 || (c < 4213
                  ? (c >= 4206 && c <= 4208)
                  : c <= 4225)))
              : (c <= 4238 || (c < 4304
                ? (c < 4295
                  ? (c >= 4256 && c <= 4293)
                  : (c <= 4295 || c == 4301))
                : (c <= 4346 || (c < 4682
                  ? (c >= 4348 && c <= 4680)
                  : c <= 4685)))))
            : (c <= 4694 || (c < 4792
              ? (c < 4746
                ? (c < 4698
                  ? c == 4696
                  : (c <= 4701 || (c >= 4704 && c <= 4744)))
                : (c <= 4749 || (c < 4786
                  ? (c >= 4752 && c <= 4784)
                  : c <= 4789)))
              : (c <= 4798 || (c < 4808
                ? (c < 4802
                  ? c == 4800
                  : c <= 4805)
                : (c <= 4822 || (c < 4882
                  ? (c >= 4824 && c <= 4880)
                  : c <= 4885)))))))
          : (c <= 4954 || (c < 6016
            ? (c < 5792
              ? (c < 5121
                ? (c < 5024
                  ? (c >= 4992 && c <= 5007)
                  : (c <= 5109 || (c >= 5112 && c <= 5117)))
                : (c <= 5740 || (c < 5761
                  ? (c >= 5743 && c <= 5759)
                  : c <= 5786)))
              : (c <= 5866 || (c < 5952
                ? (c < 5888
                  ? (c >= 5873 && c <= 5880)
                  : (c <= 5905 || (c >= 5919 && c <= 5937)))
                : (c <= 5969 || (c < 5998
                  ? (c >= 5984 && c <= 5996)
                  : c <= 6000)))))
            : (c <= 6067 || (c < 6320
              ? (c < 6272
                ? (c < 6108
                  ? c == 6103
                  : (c <= 6108 || (c >= 6176 && c <= 6264)))
                : (c <= 6276 || (c < 6314
                  ? (c >= 6279 && c <= 6312)
                  : c <= 6314)))
              : (c <= 6389 || (c < 6512
                ? (c < 6480
                  ? (c >= 6400 && c <= 6430)
                  : c <= 6509)
                : (c <= 6516 || (c < 6576
                  ? (c >= 6528 && c <= 6571)
                  : c <= 6601)))))))))))))
    : (c <= 6678 || (c < 43259
      ? (c < 8579
        ? (c < 8031
          ? (c < 7401
            ? (c < 7098
              ? (c < 6981
                ? (c < 6823
                  ? (c >= 6688 && c <= 6740)
                  : (c <= 6823 || (c >= 6917 && c <= 6963)))
                : (c <= 6988 || (c < 7086
                  ? (c >= 7043 && c <= 7072)
                  : c <= 7087)))
              : (c <= 7141 || (c < 7296
                ? (c < 7245
                  ? (c >= 7168 && c <= 7203)
                  : (c <= 7247 || (c >= 7258 && c <= 7293)))
                : (c <= 7304 || (c < 7357
                  ? (c >= 7312 && c <= 7354)
                  : c <= 7359)))))
            : (c <= 7404 || (c < 7968
              ? (c < 7424
                ? (c < 7413
                  ? (c >= 7406 && c <= 7411)
                  : (c <= 7414 || c == 7418))
                : (c <= 7615 || (c < 7960
                  ? (c >= 7680 && c <= 7957)
                  : c <= 7965)))
              : (c <= 8005 || (c < 8025
                ? (c < 8016
                  ? (c >= 8008 && c <= 8013)
                  : c <= 8023)
                : (c <= 8025 || (c < 8029
                  ? c == 8027
                  : c <= 8029)))))))
          : (c <= 8061 || (c < 8450
            ? (c < 8150
              ? (c < 8130
                ? (c < 8118
                  ? (c >= 8064 && c <= 8116)
                  : (c <= 8124 || c == 8126))
                : (c <= 8132 || (c < 8144
                  ? (c >= 8134 && c <= 8140)
                  : c <= 8147)))
              : (c <= 8155 || (c < 8305
                ? (c < 8178
                  ? (c >= 8160 && c <= 8172)
                  : (c <= 8180 || (c >= 8182 && c <= 8188)))
                : (c <= 8305 || (c < 8336
                  ? c == 8319
                  : c <= 8348)))))
            : (c <= 8450 || (c < 8488
              ? (c < 8473
                ? (c < 8458
                  ? c == 8455
                  : (c <= 8467 || c == 8469))
                : (c <= 8477 || (c < 8486
                  ? c == 8484
                  : c <= 8486)))
              : (c <= 8488 || (c < 8508
                ? (c < 8495
                  ? (c >= 8490 && c <= 8493)
                  : c <= 8505)
                : (c <= 8511 || (c < 8526
                  ? (c >= 8517 && c <= 8521)
                  : c <= 8526)))))))))
        : (c <= 8580 || (c < 12593
          ? (c < 11712
            ? (c < 11568
              ? (c < 11520
                ? (c < 11499
                  ? (c >= 11264 && c <= 11492)
                  : (c <= 11502 || (c >= 11506 && c <= 11507)))
                : (c <= 11557 || (c < 11565
                  ? c == 11559
                  : c <= 11565)))
              : (c <= 11623 || (c < 11688
                ? (c < 11648
                  ? c == 11631
                  : (c <= 11670 || (c >= 11680 && c <= 11686)))
                : (c <= 11694 || (c < 11704
                  ? (c >= 11696 && c <= 11702)
                  : c <= 11710)))))
            : (c <= 11718 || (c < 12347
              ? (c < 11823
                ? (c < 11728
                  ? (c >= 11720 && c <= 11726)
                  : (c <= 11734 || (c >= 11736 && c <= 11742)))
                : (c <= 11823 || (c < 12337
                  ? (c >= 12293 && c <= 12294)
                  : c <= 12341)))
              : (c <= 12348 || (c < 12449
                ? (c < 12445
                  ? (c >= 12353 && c <= 12438)
                  : c <= 12447)
                : (c <= 12538 || (c < 12549
                  ? (c >= 12540 && c <= 12543)
                  : c <= 12591)))))))
          : (c <= 12686 || (c < 42775
            ? (c < 42192
              ? (c < 19903
                ? (c < 12784
                  ? (c >= 12704 && c <= 12735)
                  : (c <= 12799 || c == 13312))
                : (c <= 19903 || (c < 40959
                  ? c == 19968
                  : c <= 42124)))
              : (c <= 42237 || (c < 42560
                ? (c < 42512
                  ? (c >= 42240 && c <= 42508)
                  : (c <= 42527 || (c >= 42538 && c <= 42539)))
                : (c <= 42606 || (c < 42656
                  ? (c >= 42623 && c <= 42653)
                  : c <= 42725)))))
            : (c <= 42783 || (c < 43011
              ? (c < 42963
                ? (c < 42891
                  ? (c >= 42786 && c <= 42888)
                  : (c <= 42954 || (c >= 42960 && c <= 42961)))
                : (c <= 42963 || (c < 42994
                  ? (c >= 42965 && c <= 42969)
                  : c <= 43009)))
              : (c <= 43013 || (c < 43072
                ? (c < 43020
                  ? (c >= 43015 && c <= 43018)
                  : c <= 43042)
                : (c <= 43123 || (c < 43250
                  ? (c >= 43138 && c <= 43187)
                  : c <= 43255)))))))))))
      : (c <= 43259 || (c < 65313
        ? (c < 43808
          ? (c < 43642
            ? (c < 43488
              ? (c < 43360
                ? (c < 43274
                  ? (c >= 43261 && c <= 43262)
                  : (c <= 43301 || (c >= 43312 && c <= 43334)))
                : (c <= 43388 || (c < 43471
                  ? (c >= 43396 && c <= 43442)
                  : c <= 43471)))
              : (c <= 43492 || (c < 43584
                ? (c < 43514
                  ? (c >= 43494 && c <= 43503)
                  : (c <= 43518 || (c >= 43520 && c <= 43560)))
                : (c <= 43586 || (c < 43616
                  ? (c >= 43588 && c <= 43595)
                  : c <= 43638)))))
            : (c <= 43642 || (c < 43739
              ? (c < 43705
                ? (c < 43697
                  ? (c >= 43646 && c <= 43695)
                  : (c <= 43697 || (c >= 43701 && c <= 43702)))
                : (c <= 43709 || (c < 43714
                  ? c == 43712
                  : c <= 43714)))
              : (c <= 43741 || (c < 43777
                ? (c < 43762
                  ? (c >= 43744 && c <= 43754)
                  : c <= 43764)
                : (c <= 43782 || (c < 43793
                  ? (c >= 43785 && c <= 43790)
                  : c <= 43798)))))))
          : (c <= 43814 || (c < 64287
            ? (c < 55216
              ? (c < 43888
                ? (c < 43824
                  ? (c >= 43816 && c <= 43822)
                  : (c <= 43866 || (c >= 43868 && c <= 43881)))
                : (c <= 44002 || (c < 55203
                  ? c == 44032
                  : c <= 55203)))
              : (c <= 55238 || (c < 64256
                ? (c < 63744
                  ? (c >= 55243 && c <= 55291)
                  : (c <= 64109 || (c >= 64112 && c <= 64217)))
                : (c <= 64262 || (c < 64285
                  ? (c >= 64275 && c <= 64279)
                  : c <= 64285)))))
            : (c <= 64296 || (c < 64467
              ? (c < 64320
                ? (c < 64312
                  ? (c >= 64298 && c <= 64310)
                  : (c <= 64316 || c == 64318))
                : (c <= 64321 || (c < 64326
                  ? (c >= 64323 && c <= 64324)
                  : c <= 64433)))
              : (c <= 64829 || (c < 65008
                ? (c < 64914
                  ? (c >= 64848 && c <= 64911)
                  : c <= 64967)
                : (c <= 65019 || (c < 65142
                  ? (c >= 65136 && c <= 65140)
                  : c <= 65276)))))))))
        : (c <= 65338 || (c < 66864
          ? (c < 66176
            ? (c < 65536
              ? (c < 65482
                ? (c < 65382
                  ? (c >= 65345 && c <= 65370)
                  : (c <= 65470 || (c >= 65474 && c <= 65479)))
                : (c <= 65487 || (c < 65498
                  ? (c >= 65490 && c <= 65495)
                  : c <= 65500)))
              : (c <= 65547 || (c < 65599
                ? (c < 65576
                  ? (c >= 65549 && c <= 65574)
                  : (c <= 65594 || (c >= 65596 && c <= 65597)))
                : (c <= 65613 || (c < 65664
                  ? (c >= 65616 && c <= 65629)
                  : c <= 65786)))))
            : (c <= 66204 || (c < 66464
              ? (c < 66370
                ? (c < 66304
                  ? (c >= 66208 && c <= 66256)
                  : (c <= 66335 || (c >= 66349 && c <= 66368)))
                : (c <= 66377 || (c < 66432
                  ? (c >= 66384 && c <= 66421)
                  : c <= 66461)))
              : (c <= 66499 || (c < 66736
                ? (c < 66560
                  ? (c >= 66504 && c <= 66511)
                  : c <= 66717)
                : (c <= 66771 || (c < 66816
                  ? (c >= 66776 && c <= 66811)
                  : c <= 66855)))))))
          : (c <= 66915 || (c < 67506
            ? (c < 66995
              ? (c < 66964
                ? (c < 66940
                  ? (c >= 66928 && c <= 66938)
                  : (c <= 66954 || (c >= 66956 && c <= 66962)))
                : (c <= 66965 || (c < 66979
                  ? (c >= 66967 && c <= 66977)
                  : c <= 66993)))
              : (c <= 67001 || (c < 67424
                ? (c < 67072
                  ? (c >= 67003 && c <= 67004)
                  : (c <= 67382 || (c >= 67392 && c <= 67413)))
                : (c <= 67431 || (c < 67463
                  ? (c >= 67456 && c <= 67461)
                  : c <= 67504)))))
            : (c <= 67514 || (c < 67680
              ? (c < 67639
                ? (c < 67592
                  ? (c >= 67584 && c <= 67589)
                  : (c <= 67592 || (c >= 67594 && c <= 67637)))
                : (c <= 67640 || (c < 67647
                  ? c == 67644
                  : c <= 67669)))
              : (c <= 67702 || (c < 67828
                ? (c < 67808
                  ? (c >= 67712 && c <= 67742)
                  : c <= 67826)
                : (c <= 67829 || (c < 67872
                  ? (c >= 67840 && c <= 67861)
                  : c <= 67883)))))))))))))));
}

static inline bool sym__ident_character_set_2(int32_t c) {
  return (c < 6400
    ? (c < 2984
      ? (c < 2384
        ? (c < 1488
          ? (c < 880
            ? (c < 192
              ? (c < 'a'
                ? (c < 'A'
                  ? (c >= '0' && c <= '9')
                  : (c <= 'Z' || c == '_'))
                : (c <= 'z' || (c < 181
                  ? c == 170
                  : (c <= 181 || c == 186))))
              : (c <= 214 || (c < 736
                ? (c < 248
                  ? (c >= 216 && c <= 246)
                  : (c <= 705 || (c >= 710 && c <= 721)))
                : (c <= 740 || (c < 750
                  ? c == 748
                  : c <= 750)))))
            : (c <= 884 || (c < 910
              ? (c < 902
                ? (c < 890
                  ? (c >= 886 && c <= 887)
                  : (c <= 893 || c == 895))
                : (c <= 902 || (c < 908
                  ? (c >= 904 && c <= 906)
                  : c <= 908)))
              : (c <= 929 || (c < 1329
                ? (c < 1015
                  ? (c >= 931 && c <= 1013)
                  : (c <= 1153 || (c >= 1162 && c <= 1327)))
                : (c <= 1366 || (c < 1376
                  ? c == 1369
                  : c <= 1416)))))))
          : (c <= 1514 || (c < 1984
            ? (c < 1765
              ? (c < 1646
                ? (c < 1568
                  ? (c >= 1519 && c <= 1522)
                  : (c <= 1610 || (c >= 1632 && c <= 1641)))
                : (c <= 1647 || (c < 1749
                  ? (c >= 1649 && c <= 1747)
                  : c <= 1749)))
              : (c <= 1766 || (c < 1810
                ? (c < 1791
                  ? (c >= 1774 && c <= 1788)
                  : (c <= 1791 || c == 1808))
                : (c <= 1839 || (c < 1969
                  ? (c >= 1869 && c <= 1957)
                  : c <= 1969)))))
            : (c <= 2026 || (c < 2112
              ? (c < 2074
                ? (c < 2042
                  ? (c >= 2036 && c <= 2037)
                  : (c <= 2042 || (c >= 2048 && c <= 2069)))
                : (c <= 2074 || (c < 2088
                  ? c == 2084
                  : c <= 2088)))
              : (c <= 2136 || (c < 2208
                ? (c < 2160
                  ? (c >= 2144 && c <= 2154)
                  : (c <= 2183 || (c >= 2185 && c <= 2190)))
                : (c <= 2249 || (c < 2365
                  ? (c >= 2308 && c <= 2361)
                  : c <= 2365)))))))))
        : (c <= 2384 || (c < 2707
          ? (c < 2556
            ? (c < 2482
              ? (c < 2437
                ? (c < 2406
                  ? (c >= 2392 && c <= 2401)
                  : (c <= 2415 || (c >= 2417 && c <= 2432)))
                : (c <= 2444 || (c < 2451
                  ? (c >= 2447 && c <= 2448)
                  : (c <= 2472 || (c >= 2474 && c <= 2480)))))
              : (c <= 2482 || (c < 2524
                ? (c < 2493
                  ? (c >= 2486 && c <= 2489)
                  : (c <= 2493 || c == 2510))
                : (c <= 2525 || (c < 2534
                  ? (c >= 2527 && c <= 2529)
                  : c <= 2545)))))
            : (c <= 2556 || (c < 2616
              ? (c < 2602
                ? (c < 2575
                  ? (c >= 2565 && c <= 2570)
                  : (c <= 2576 || (c >= 2579 && c <= 2600)))
                : (c <= 2608 || (c < 2613
                  ? (c >= 2610 && c <= 2611)
                  : c <= 2614)))
              : (c <= 2617 || (c < 2674
                ? (c < 2654
                  ? (c >= 2649 && c <= 2652)
                  : (c <= 2654 || (c >= 2662 && c <= 2671)))
                : (c <= 2676 || (c < 2703
                  ? (c >= 2693 && c <= 2701)
                  : c <= 2705)))))))
          : (c <= 2728 || (c < 2869
            ? (c < 2790
              ? (c < 2749
                ? (c < 2738
                  ? (c >= 2730 && c <= 2736)
                  : (c <= 2739 || (c >= 2741 && c <= 2745)))
                : (c <= 2749 || (c < 2784
                  ? c == 2768
                  : c <= 2785)))
              : (c <= 2799 || (c < 2835
                ? (c < 2821
                  ? c == 2809
                  : (c <= 2828 || (c >= 2831 && c <= 2832)))
                : (c <= 2856 || (c < 2866
                  ? (c >= 2858 && c <= 2864)
                  : c <= 2867)))))
            : (c <= 2873 || (c < 2949
              ? (c < 2918
                ? (c < 2908
                  ? c == 2877
                  : (c <= 2909 || (c >= 2911 && c <= 2913)))
                : (c <= 2927 || (c < 2947
                  ? c == 2929
                  : c <= 2947)))
              : (c <= 2954 || (c < 2972
                ? (c < 2962
                  ? (c >= 2958 && c <= 2960)
                  : (c <= 2965 || (c >= 2969 && c <= 2970)))
                : (c <= 2972 || (c < 2979
                  ? (c >= 2974 && c <= 2975)
                  : c <= 2980)))))))))))
      : (c <= 2986 || (c < 3904
        ? (c < 3412
          ? (c < 3214
            ? (c < 3133
              ? (c < 3077
                ? (c < 3024
                  ? (c >= 2990 && c <= 3001)
                  : (c <= 3024 || (c >= 3046 && c <= 3055)))
                : (c <= 3084 || (c < 3090
                  ? (c >= 3086 && c <= 3088)
                  : (c <= 3112 || (c >= 3114 && c <= 3129)))))
              : (c <= 3133 || (c < 3174
                ? (c < 3165
                  ? (c >= 3160 && c <= 3162)
                  : (c <= 3165 || (c >= 3168 && c <= 3169)))
                : (c <= 3183 || (c < 3205
                  ? c == 3200
                  : c <= 3212)))))
            : (c <= 3216 || (c < 3302
              ? (c < 3261
                ? (c < 3242
                  ? (c >= 3218 && c <= 3240)
                  : (c <= 3251 || (c >= 3253 && c <= 3257)))
                : (c <= 3261 || (c < 3296
                  ? (c >= 3293 && c <= 3294)
                  : c <= 3297)))
              : (c <= 3311 || (c < 3346
                ? (c < 3332
                  ? (c >= 3313 && c <= 3314)
                  : (c <= 3340 || (c >= 3342 && c <= 3344)))
                : (c <= 3386 || (c < 3406
                  ? c == 3389
                  : c <= 3406)))))))
          : (c <= 3414 || (c < 3713
            ? (c < 3517
              ? (c < 3461
                ? (c < 3430
                  ? (c >= 3423 && c <= 3425)
                  : (c <= 3439 || (c >= 3450 && c <= 3455)))
                : (c <= 3478 || (c < 3507
                  ? (c >= 3482 && c <= 3505)
                  : c <= 3515)))
              : (c <= 3517 || (c < 3634
                ? (c < 3558
                  ? (c >= 3520 && c <= 3526)
                  : (c <= 3567 || (c >= 3585 && c <= 3632)))
                : (c <= 3635 || (c < 3664
                  ? (c >= 3648 && c <= 3654)
                  : c <= 3673)))))
            : (c <= 3714 || (c < 3773
              ? (c < 3749
                ? (c < 3718
                  ? c == 3716
                  : (c <= 3722 || (c >= 3724 && c <= 3747)))
                : (c <= 3749 || (c < 3762
                  ? (c >= 3751 && c <= 3760)
                  : c <= 3763)))
              : (c <= 3773 || (c < 3804
                ? (c < 3782
                  ? (c >= 3776 && c <= 3780)
                  : (c <= 3782 || (c >= 3792 && c <= 3801)))
                : (c <= 3807 || (c < 3872
                  ? c == 3840
                  : c <= 3881)))))))))
        : (c <= 3911 || (c < 4802
          ? (c < 4295
            ? (c < 4193
              ? (c < 4159
                ? (c < 3976
                  ? (c >= 3913 && c <= 3948)
                  : (c <= 3980 || (c >= 4096 && c <= 4138)))
                : (c <= 4169 || (c < 4186
                  ? (c >= 4176 && c <= 4181)
                  : c <= 4189)))
              : (c <= 4193 || (c < 4238
                ? (c < 4206
                  ? (c >= 4197 && c <= 4198)
                  : (c <= 4208 || (c >= 4213 && c <= 4225)))
                : (c <= 4238 || (c < 4256
                  ? (c >= 4240 && c <= 4249)
                  : c <= 4293)))))
            : (c <= 4295 || (c < 4698
              ? (c < 4682
                ? (c < 4304
                  ? c == 4301
                  : (c <= 4346 || (c >= 4348 && c <= 4680)))
                : (c <= 4685 || (c < 4696
                  ? (c >= 4688 && c <= 4694)
                  : c <= 4696)))
              : (c <= 4701 || (c < 4786
                ? (c < 4746
                  ? (c >= 4704 && c <= 4744)
                  : (c <= 4749 || (c >= 4752 && c <= 4784)))
                : (c <= 4789 || (c < 4800
                  ? (c >= 4792 && c <= 4798)
                  : c <= 4800)))))))
          : (c <= 4805 || (c < 5919
            ? (c < 5112
              ? (c < 4888
                ? (c < 4824
                  ? (c >= 4808 && c <= 4822)
                  : (c <= 4880 || (c >= 4882 && c <= 4885)))
                : (c <= 4954 || (c < 5024
                  ? (c >= 4992 && c <= 5007)
                  : c <= 5109)))
              : (c <= 5117 || (c < 5792
                ? (c < 5743
                  ? (c >= 5121 && c <= 5740)
                  : (c <= 5759 || (c >= 5761 && c <= 5786)))
                : (c <= 5866 || (c < 5888
                  ? (c >= 5873 && c <= 5880)
                  : c <= 5905)))))
            : (c <= 5937 || (c < 6112
              ? (c < 6016
                ? (c < 5984
                  ? (c >= 5952 && c <= 5969)
                  : (c <= 5996 || (c >= 5998 && c <= 6000)))
                : (c <= 6067 || (c < 6108
                  ? c == 6103
                  : c <= 6108)))
              : (c <= 6121 || (c < 6279
                ? (c < 6176
                  ? (c >= 6160 && c <= 6169)
                  : (c <= 6264 || (c >= 6272 && c <= 6276)))
                : (c <= 6312 || (c < 6320
                  ? c == 6314
                  : c <= 6389)))))))))))))
    : (c <= 6430 || (c < 43216
      ? (c < 8490
        ? (c < 7968
          ? (c < 7086
            ? (c < 6784
              ? (c < 6576
                ? (c < 6512
                  ? (c >= 6470 && c <= 6509)
                  : (c <= 6516 || (c >= 6528 && c <= 6571)))
                : (c <= 6601 || (c < 6656
                  ? (c >= 6608 && c <= 6617)
                  : (c <= 6678 || (c >= 6688 && c <= 6740)))))
              : (c <= 6793 || (c < 6981
                ? (c < 6823
                  ? (c >= 6800 && c <= 6809)
                  : (c <= 6823 || (c >= 6917 && c <= 6963)))
                : (c <= 6988 || (c < 7043
                  ? (c >= 6992 && c <= 7001)
                  : c <= 7072)))))
            : (c <= 7141 || (c < 7401
              ? (c < 7296
                ? (c < 7232
                  ? (c >= 7168 && c <= 7203)
                  : (c <= 7241 || (c >= 7245 && c <= 7293)))
                : (c <= 7304 || (c < 7357
                  ? (c >= 7312 && c <= 7354)
                  : c <= 7359)))
              : (c <= 7404 || (c < 7424
                ? (c < 7413
                  ? (c >= 7406 && c <= 7411)
                  : (c <= 7414 || c == 7418))
                : (c <= 7615 || (c < 7960
                  ? (c >= 7680 && c <= 7957)
                  : c <= 7965)))))))
          : (c <= 8005 || (c < 8160
            ? (c < 8064
              ? (c < 8027
                ? (c < 8016
                  ? (c >= 8008 && c <= 8013)
                  : (c <= 8023 || c == 8025))
                : (c <= 8027 || (c < 8031
                  ? c == 8029
                  : c <= 8061)))
              : (c <= 8116 || (c < 8134
                ? (c < 8126
                  ? (c >= 8118 && c <= 8124)
                  : (c <= 8126 || (c >= 8130 && c <= 8132)))
                : (c <= 8140 || (c < 8150
                  ? (c >= 8144 && c <= 8147)
                  : c <= 8155)))))
            : (c <= 8172 || (c < 8455
              ? (c < 8319
                ? (c < 8182
                  ? (c >= 8178 && c <= 8180)
                  : (c <= 8188 || c == 8305))
                : (c <= 8319 || (c < 8450
                  ? (c >= 8336 && c <= 8348)
                  : c <= 8450)))
              : (c <= 8455 || (c < 8484
                ? (c < 8469
                  ? (c >= 8458 && c <= 8467)
                  : (c <= 8469 || (c >= 8473 && c <= 8477)))
                : (c <= 8484 || (c < 8488
                  ? c == 8486
                  : c <= 8488)))))))))
        : (c <= 8493 || (c < 12449
          ? (c < 11680
            ? (c < 11506
              ? (c < 8526
                ? (c < 8508
                  ? (c >= 8495 && c <= 8505)
                  : (c <= 8511 || (c >= 8517 && c <= 8521)))
                : (c <= 8526 || (c < 11264
                  ? (c >= 8579 && c <= 8580)
                  : (c <= 11492 || (c >= 11499 && c <= 11502)))))
              : (c <= 11507 || (c < 11568
                ? (c < 11559
                  ? (c >= 11520 && c <= 11557)
                  : (c <= 11559 || c == 11565))
                : (c <= 11623 || (c < 11648
                  ? c == 11631
                  : c <= 11670)))))
            : (c <= 11686 || (c < 11736
              ? (c < 11712
                ? (c < 11696
                  ? (c >= 11688 && c <= 11694)
                  : (c <= 11702 || (c >= 11704 && c <= 11710)))
                : (c <= 11718 || (c < 11728
                  ? (c >= 11720 && c <= 11726)
                  : c <= 11734)))
              : (c <= 11742 || (c < 12347
                ? (c < 12293
                  ? c == 11823
                  : (c <= 12294 || (c >= 12337 && c <= 12341)))
                : (c <= 12348 || (c < 12445
                  ? (c >= 12353 && c <= 12438)
                  : c <= 12447)))))))
          : (c <= 12538 || (c < 42623
            ? (c < 19903
              ? (c < 12704
                ? (c < 12549
                  ? (c >= 12540 && c <= 12543)
                  : (c <= 12591 || (c >= 12593 && c <= 12686)))
                : (c <= 12735 || (c < 13312
                  ? (c >= 12784 && c <= 12799)
                  : c <= 13312)))
              : (c <= 19903 || (c < 42240
                ? (c < 40959
                  ? c == 19968
                  : (c <= 42124 || (c >= 42192 && c <= 42237)))
                : (c <= 42508 || (c < 42560
                  ? (c >= 42512 && c <= 42539)
                  : c <= 42606)))))
            : (c <= 42653 || (c < 42965
              ? (c < 42891
                ? (c < 42775
                  ? (c >= 42656 && c <= 42725)
                  : (c <= 42783 || (c >= 42786 && c <= 42888)))
                : (c <= 42954 || (c < 42963
                  ? (c >= 42960 && c <= 42961)
                  : c <= 42963)))
              : (c <= 42969 || (c < 43020
                ? (c < 43011
                  ? (c >= 42994 && c <= 43009)
                  : (c <= 43013 || (c >= 43015 && c <= 43018)))
                : (c <= 43042 || (c < 43138
                  ? (c >= 43072 && c <= 43123)
                  : c <= 43187)))))))))))
      : (c <= 43225 || (c < 65296
        ? (c < 43808
          ? (c < 43616
            ? (c < 43471
              ? (c < 43264
                ? (c < 43259
                  ? (c >= 43250 && c <= 43255)
                  : (c <= 43259 || (c >= 43261 && c <= 43262)))
                : (c <= 43301 || (c < 43360
                  ? (c >= 43312 && c <= 43334)
                  : (c <= 43388 || (c >= 43396 && c <= 43442)))))
              : (c <= 43481 || (c < 43584
                ? (c < 43494
                  ? (c >= 43488 && c <= 43492)
                  : (c <= 43518 || (c >= 43520 && c <= 43560)))
                : (c <= 43586 || (c < 43600
                  ? (c >= 43588 && c <= 43595)
                  : c <= 43609)))))
            : (c <= 43638 || (c < 43714
              ? (c < 43701
                ? (c < 43646
                  ? c == 43642
                  : (c <= 43695 || c == 43697))
                : (c <= 43702 || (c < 43712
                  ? (c >= 43705 && c <= 43709)
                  : c <= 43712)))
              : (c <= 43714 || (c < 43777
                ? (c < 43744
                  ? (c >= 43739 && c <= 43741)
                  : (c <= 43754 || (c >= 43762 && c <= 43764)))
                : (c <= 43782 || (c < 43793
                  ? (c >= 43785 && c <= 43790)
                  : c <= 43798)))))))
          : (c <= 43814 || (c < 64285
            ? (c < 55203
              ? (c < 43888
                ? (c < 43824
                  ? (c >= 43816 && c <= 43822)
                  : (c <= 43866 || (c >= 43868 && c <= 43881)))
                : (c <= 44002 || (c < 44032
                  ? (c >= 44016 && c <= 44025)
                  : c <= 44032)))
              : (c <= 55203 || (c < 64112
                ? (c < 55243
                  ? (c >= 55216 && c <= 55238)
                  : (c <= 55291 || (c >= 63744 && c <= 64109)))
                : (c <= 64217 || (c < 64275
                  ? (c >= 64256 && c <= 64262)
                  : c <= 64279)))))
            : (c <= 64285 || (c < 64326
              ? (c < 64318
                ? (c < 64298
                  ? (c >= 64287 && c <= 64296)
                  : (c <= 64310 || (c >= 64312 && c <= 64316)))
                : (c <= 64318 || (c < 64323
                  ? (c >= 64320 && c <= 64321)
                  : c <= 64324)))
              : (c <= 64433 || (c < 65008
                ? (c < 64848
                  ? (c >= 64467 && c <= 64829)
                  : (c <= 64911 || (c >= 64914 && c <= 64967)))
                : (c <= 65019 || (c < 65142
                  ? (c >= 65136 && c <= 65140)
                  : c <= 65276)))))))))
        : (c <= 65305 || (c < 66816
          ? (c < 65664
            ? (c < 65498
              ? (c < 65474
                ? (c < 65345
                  ? (c >= 65313 && c <= 65338)
                  : (c <= 65370 || (c >= 65382 && c <= 65470)))
                : (c <= 65479 || (c < 65490
                  ? (c >= 65482 && c <= 65487)
                  : c <= 65495)))
              : (c <= 65500 || (c < 65596
                ? (c < 65549
                  ? (c >= 65536 && c <= 65547)
                  : (c <= 65574 || (c >= 65576 && c <= 65594)))
                : (c <= 65597 || (c < 65616
                  ? (c >= 65599 && c <= 65613)
                  : c <= 65629)))))
            : (c <= 65786 || (c < 66432
              ? (c < 66349
                ? (c < 66208
                  ? (c >= 66176 && c <= 66204)
                  : (c <= 66256 || (c >= 66304 && c <= 66335)))
                : (c <= 66368 || (c < 66384
                  ? (c >= 66370 && c <= 66377)
                  : c <= 66421)))
              : (c <= 66461 || (c < 66720
                ? (c < 66504
                  ? (c >= 66464 && c <= 66499)
                  : (c <= 66511 || (c >= 66560 && c <= 66717)))
                : (c <= 66729 || (c < 66776
                  ? (c >= 66736 && c <= 66771)
                  : c <= 66811)))))))
          : (c <= 66855 || (c < 67463
            ? (c < 66979
              ? (c < 66956
                ? (c < 66928
                  ? (c >= 66864 && c <= 66915)
                  : (c <= 66938 || (c >= 66940 && c <= 66954)))
                : (c <= 66962 || (c < 66967
                  ? (c >= 66964 && c <= 66965)
                  : c <= 66977)))
              : (c <= 66993 || (c < 67392
                ? (c < 67003
                  ? (c >= 66995 && c <= 67001)
                  : (c <= 67004 || (c >= 67072 && c <= 67382)))
                : (c <= 67413 || (c < 67456
                  ? (c >= 67424 && c <= 67431)
                  : c <= 67461)))))
            : (c <= 67504 || (c < 67647
              ? (c < 67594
                ? (c < 67584
                  ? (c >= 67506 && c <= 67514)
                  : (c <= 67589 || c == 67592))
                : (c <= 67637 || (c < 67644
                  ? (c >= 67639 && c <= 67640)
                  : c <= 67644)))
              : (c <= 67669 || (c < 67828
                ? (c < 67712
                  ? (c >= 67680 && c <= 67702)
                  : (c <= 67742 || (c >= 67808 && c <= 67826)))
                : (c <= 67829 || (c < 67872
                  ? (c >= 67840 && c <= 67861)
                  : c <= 67883)))))))))))))));
}

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(6);
      if (lookahead == '"') ADVANCE(13);
      if (lookahead == '%') ADVANCE(25);
      if (lookahead == '(') ADVANCE(7);
      if (lookahead == ')') ADVANCE(9);
      if (lookahead == '*') ADVANCE(22);
      if (lookahead == '+') ADVANCE(20);
      if (lookahead == ',') ADVANCE(8);
      if (lookahead == '-') ADVANCE(21);
      if (lookahead == '.') ADVANCE(4);
      if (lookahead == '/') ADVANCE(24);
      if (lookahead == ':') ADVANCE(10);
      if (lookahead == ';') ADVANCE(19);
      if (lookahead == '=') ADVANCE(11);
      if (lookahead == '[') ADVANCE(17);
      if (lookahead == ']') ADVANCE(18);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      if (sym__ident_character_set_1(lookahead)) ADVANCE(26);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(13);
      if (lookahead == '\\') ADVANCE(15);
      if (lookahead != 0) ADVANCE(16);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(12);
      if (lookahead == '%') ADVANCE(25);
      if (lookahead == '(') ADVANCE(7);
      if (lookahead == ')') ADVANCE(9);
      if (lookahead == '*') ADVANCE(22);
      if (lookahead == '+') ADVANCE(20);
      if (lookahead == ',') ADVANCE(8);
      if (lookahead == '-') ADVANCE(21);
      if (lookahead == '.') ADVANCE(4);
      if (lookahead == '/') ADVANCE(24);
      if (lookahead == ':') ADVANCE(10);
      if (lookahead == ';') ADVANCE(19);
      if (lookahead == '=') ADVANCE(11);
      if (lookahead == '[') ADVANCE(17);
      if (lookahead == ']') ADVANCE(18);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      if (sym__ident_character_set_1(lookahead)) ADVANCE(26);
      END_STATE();
    case 3:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      END_STATE();
    case 4:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(28);
      END_STATE();
    case 5:
      if (eof) ADVANCE(6);
      if (lookahead == '"') ADVANCE(12);
      if (lookahead == '%') ADVANCE(25);
      if (lookahead == '(') ADVANCE(7);
      if (lookahead == ')') ADVANCE(9);
      if (lookahead == '*') ADVANCE(22);
      if (lookahead == '+') ADVANCE(20);
      if (lookahead == ',') ADVANCE(8);
      if (lookahead == '-') ADVANCE(21);
      if (lookahead == '.') ADVANCE(4);
      if (lookahead == '/') ADVANCE(24);
      if (lookahead == ':') ADVANCE(10);
      if (lookahead == ';') ADVANCE(19);
      if (lookahead == '=') ADVANCE(11);
      if (lookahead == '[') ADVANCE(17);
      if (lookahead == ']') ADVANCE(18);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      if (sym__ident_character_set_1(lookahead)) ADVANCE(26);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_DQUOTE2);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_string_lit_content);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_string_lit_content);
      if (lookahead == '"') ADVANCE(14);
      if (lookahead != 0) ADVANCE(16);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym_string_lit_content);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(16);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(sym_plus);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(sym_minus);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_star);
      if (lookahead == '*') ADVANCE(23);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_double_star);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_slash);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_percent);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym__ident);
      if (sym__ident_character_set_2(lookahead)) ADVANCE(26);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '.') ADVANCE(4);
      if (lookahead == '_') ADVANCE(3);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '_') ADVANCE(4);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(28);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (lookahead == 'e') ADVANCE(1);
      if (lookahead == 'f') ADVANCE(2);
      if (lookahead == 'i') ADVANCE(3);
      if (lookahead == 't') ADVANCE(4);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == 'l') ADVANCE(5);
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(6);
      if (lookahead == 'n') ADVANCE(7);
      END_STATE();
    case 3:
      if (lookahead == 'f') ADVANCE(8);
      END_STATE();
    case 4:
      if (lookahead == 'h') ADVANCE(9);
      if (lookahead == 'r') ADVANCE(10);
      END_STATE();
    case 5:
      if (lookahead == 's') ADVANCE(11);
      END_STATE();
    case 6:
      if (lookahead == 'l') ADVANCE(12);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_fn);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 9:
      if (lookahead == 'e') ADVANCE(13);
      END_STATE();
    case 10:
      if (lookahead == 'u') ADVANCE(14);
      END_STATE();
    case 11:
      if (lookahead == 'e') ADVANCE(15);
      END_STATE();
    case 12:
      if (lookahead == 's') ADVANCE(16);
      END_STATE();
    case 13:
      if (lookahead == 'n') ADVANCE(17);
      END_STATE();
    case 14:
      if (lookahead == 'e') ADVANCE(18);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 16:
      if (lookahead == 'e') ADVANCE(19);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_then);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 2},
  [6] = {.lex_state = 2},
  [7] = {.lex_state = 2},
  [8] = {.lex_state = 2},
  [9] = {.lex_state = 2},
  [10] = {.lex_state = 2},
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 2},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 2},
  [16] = {.lex_state = 2},
  [17] = {.lex_state = 2},
  [18] = {.lex_state = 2},
  [19] = {.lex_state = 2},
  [20] = {.lex_state = 2},
  [21] = {.lex_state = 2},
  [22] = {.lex_state = 2},
  [23] = {.lex_state = 2},
  [24] = {.lex_state = 2},
  [25] = {.lex_state = 2},
  [26] = {.lex_state = 2},
  [27] = {.lex_state = 2},
  [28] = {.lex_state = 2},
  [29] = {.lex_state = 2},
  [30] = {.lex_state = 2},
  [31] = {.lex_state = 2},
  [32] = {.lex_state = 2},
  [33] = {.lex_state = 2},
  [34] = {.lex_state = 2},
  [35] = {.lex_state = 2},
  [36] = {.lex_state = 2},
  [37] = {.lex_state = 2},
  [38] = {.lex_state = 2},
  [39] = {.lex_state = 2},
  [40] = {.lex_state = 2},
  [41] = {.lex_state = 2},
  [42] = {.lex_state = 2},
  [43] = {.lex_state = 2},
  [44] = {.lex_state = 2},
  [45] = {.lex_state = 2},
  [46] = {.lex_state = 2},
  [47] = {.lex_state = 2},
  [48] = {.lex_state = 2},
  [49] = {.lex_state = 2},
  [50] = {.lex_state = 2},
  [51] = {.lex_state = 2},
  [52] = {.lex_state = 2},
  [53] = {.lex_state = 2},
  [54] = {.lex_state = 2},
  [55] = {.lex_state = 2},
  [56] = {.lex_state = 2},
  [57] = {.lex_state = 2},
  [58] = {.lex_state = 2},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 2},
  [61] = {.lex_state = 2},
  [62] = {.lex_state = 2},
  [63] = {.lex_state = 2},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 0},
  [66] = {.lex_state = 0},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 0},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 0},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 0},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 0},
  [77] = {.lex_state = 0},
  [78] = {.lex_state = 0},
  [79] = {.lex_state = 0},
  [80] = {.lex_state = 0},
  [81] = {.lex_state = 0},
  [82] = {.lex_state = 0},
  [83] = {.lex_state = 0},
  [84] = {.lex_state = 0},
  [85] = {.lex_state = 0},
  [86] = {.lex_state = 0},
  [87] = {.lex_state = 0},
  [88] = {.lex_state = 0},
  [89] = {.lex_state = 0},
  [90] = {.lex_state = 0},
  [91] = {.lex_state = 2},
  [92] = {.lex_state = 2},
  [93] = {.lex_state = 0},
  [94] = {.lex_state = 0},
  [95] = {.lex_state = 0},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 0},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 0},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 1},
  [122] = {.lex_state = 0},
  [123] = {.lex_state = 1},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 0},
  [126] = {.lex_state = 1},
  [127] = {.lex_state = 0},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 0},
  [130] = {.lex_state = 0},
  [131] = {.lex_state = 0},
  [132] = {.lex_state = 1},
  [133] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym__ident] = ACTIONS(1),
    [anon_sym_fn] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE2] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_then] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [sym_plus] = ACTIONS(1),
    [sym_minus] = ACTIONS(1),
    [sym_star] = ACTIONS(1),
    [sym_double_star] = ACTIONS(1),
    [sym_slash] = ACTIONS(1),
    [sym_percent] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [aux_sym_number_token1] = ACTIONS(1),
  },
  [1] = {
    [sym_root] = STATE(133),
    [sym__module_stmt] = STATE(94),
    [sym_fn_decl] = STATE(94),
    [sym_global_var_decl] = STATE(94),
    [sym_ident] = STATE(114),
    [aux_sym_root_repeat1] = STATE(94),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym__ident] = ACTIONS(5),
    [anon_sym_fn] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 17,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(13), 1,
      anon_sym_RPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    STATE(10), 1,
      aux_sym__call_args_list_repeat1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(130), 1,
      sym__call_args_list,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(61), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [62] = 17,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    ACTIONS(27), 1,
      anon_sym_RPAREN,
    STATE(10), 1,
      aux_sym__call_args_list_repeat1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(125), 1,
      sym__call_args_list,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(61), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [124] = 16,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    ACTIONS(29), 1,
      anon_sym_RBRACK,
    STATE(9), 1,
      aux_sym_array_lit_repeat1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(60), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [183] = 16,
    ACTIONS(31), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      anon_sym_DQUOTE,
    ACTIONS(40), 1,
      anon_sym_LBRACK,
    ACTIONS(43), 1,
      anon_sym_RBRACK,
    ACTIONS(45), 1,
      anon_sym_if,
    ACTIONS(48), 1,
      anon_sym_true,
    ACTIONS(51), 1,
      anon_sym_false,
    ACTIONS(54), 1,
      aux_sym_number_token1,
    STATE(5), 1,
      aux_sym_array_lit_repeat1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(60), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [242] = 16,
    ACTIONS(57), 1,
      sym__ident,
    ACTIONS(60), 1,
      anon_sym_LPAREN,
    ACTIONS(63), 1,
      anon_sym_RPAREN,
    ACTIONS(65), 1,
      anon_sym_DQUOTE,
    ACTIONS(68), 1,
      anon_sym_LBRACK,
    ACTIONS(71), 1,
      anon_sym_if,
    ACTIONS(74), 1,
      anon_sym_true,
    ACTIONS(77), 1,
      anon_sym_false,
    ACTIONS(80), 1,
      aux_sym_number_token1,
    STATE(6), 1,
      aux_sym__call_args_list_repeat1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(61), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [301] = 16,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    ACTIONS(83), 1,
      anon_sym_RBRACK,
    STATE(5), 1,
      aux_sym_array_lit_repeat1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(60), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [360] = 16,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    ACTIONS(85), 1,
      anon_sym_RBRACK,
    STATE(7), 1,
      aux_sym_array_lit_repeat1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(60), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [419] = 16,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    ACTIONS(87), 1,
      anon_sym_RBRACK,
    STATE(5), 1,
      aux_sym_array_lit_repeat1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(60), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [478] = 16,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    ACTIONS(89), 1,
      anon_sym_RPAREN,
    STATE(6), 1,
      aux_sym__call_args_list_repeat1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(61), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [537] = 15,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    ACTIONS(91), 1,
      anon_sym_SEMI,
    STATE(38), 1,
      sym_ident,
    STATE(42), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(41), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [593] = 15,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(99), 1,
      anon_sym_SEMI,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(77), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(76), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [649] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(86), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [702] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(95), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [755] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(80), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [808] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(96), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [861] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(98), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [914] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(85), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [967] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(93), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1020] = 14,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(62), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1073] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(87), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1126] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(79), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1179] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(88), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1232] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(90), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1285] = 14,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(44), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1338] = 14,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(63), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1391] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(89), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1444] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(71), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(70), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1497] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(69), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1550] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(68), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1603] = 14,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(93), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      anon_sym_DQUOTE,
    ACTIONS(97), 1,
      anon_sym_LBRACK,
    ACTIONS(101), 1,
      anon_sym_if,
    ACTIONS(103), 1,
      anon_sym_true,
    ACTIONS(105), 1,
      anon_sym_false,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(64), 1,
      sym_ident,
    STATE(67), 1,
      sym__block,
    STATE(74), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(66), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1656] = 14,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    STATE(38), 1,
      sym_ident,
    STATE(48), 1,
      sym__block,
    STATE(51), 1,
      sym__block_clause,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(52), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1709] = 14,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(55), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1762] = 14,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(40), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1815] = 14,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(56), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1868] = 14,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_LBRACK,
    ACTIONS(19), 1,
      anon_sym_if,
    ACTIONS(21), 1,
      anon_sym_true,
    ACTIONS(23), 1,
      anon_sym_false,
    ACTIONS(25), 1,
      aux_sym_number_token1,
    STATE(38), 1,
      sym_ident,
    STATE(46), 1,
      sym__block_clause,
    STATE(48), 1,
      sym__block,
    STATE(118), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(57), 10,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_string_lit,
      sym_array_lit,
      sym_block,
      sym_if,
      sym_true,
      sym_false,
      sym_number,
  [1921] = 2,
    ACTIONS(109), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(111), 15,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [1947] = 3,
    ACTIONS(117), 2,
      anon_sym_COLON,
      anon_sym_EQ,
    ACTIONS(113), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(115), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [1975] = 2,
    ACTIONS(119), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(121), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [1999] = 4,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(129), 1,
      sym_double_star,
    ACTIONS(123), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 11,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2027] = 7,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(129), 1,
      sym_double_star,
    ACTIONS(137), 1,
      sym_star,
    ACTIONS(135), 2,
      sym_plus,
      sym_minus,
    ACTIONS(139), 2,
      sym_slash,
      sym_percent,
    ACTIONS(131), 5,
      anon_sym_if,
      anon_sym_else,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(133), 7,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [2061] = 2,
    ACTIONS(141), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(143), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2085] = 2,
    ACTIONS(145), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(147), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2109] = 8,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(129), 1,
      sym_double_star,
    ACTIONS(137), 1,
      sym_star,
    ACTIONS(153), 1,
      anon_sym_else,
    ACTIONS(135), 2,
      sym_plus,
      sym_minus,
    ACTIONS(139), 2,
      sym_slash,
      sym_percent,
    ACTIONS(149), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(151), 7,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [2145] = 2,
    ACTIONS(155), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(157), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2169] = 2,
    ACTIONS(159), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(161), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2193] = 2,
    ACTIONS(163), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(165), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2217] = 2,
    ACTIONS(167), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(169), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2241] = 2,
    ACTIONS(171), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(173), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2265] = 2,
    ACTIONS(175), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(177), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2289] = 2,
    ACTIONS(179), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(181), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2313] = 7,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(129), 1,
      sym_double_star,
    ACTIONS(137), 1,
      sym_star,
    ACTIONS(135), 2,
      sym_plus,
      sym_minus,
    ACTIONS(139), 2,
      sym_slash,
      sym_percent,
    ACTIONS(183), 5,
      anon_sym_if,
      anon_sym_else,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(185), 7,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [2347] = 2,
    ACTIONS(187), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(189), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2371] = 2,
    ACTIONS(191), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(193), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2395] = 3,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(123), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 12,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2421] = 6,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(129), 1,
      sym_double_star,
    ACTIONS(137), 1,
      sym_star,
    ACTIONS(139), 2,
      sym_slash,
      sym_percent,
    ACTIONS(123), 5,
      anon_sym_if,
      anon_sym_else,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 9,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      aux_sym_number_token1,
  [2453] = 7,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(129), 1,
      sym_double_star,
    ACTIONS(137), 1,
      sym_star,
    ACTIONS(135), 2,
      sym_plus,
      sym_minus,
    ACTIONS(139), 2,
      sym_slash,
      sym_percent,
    ACTIONS(195), 5,
      anon_sym_if,
      anon_sym_else,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(197), 7,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [2487] = 2,
    ACTIONS(199), 6,
      anon_sym_if,
      anon_sym_else,
      sym_star,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(201), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      aux_sym_number_token1,
  [2511] = 2,
    ACTIONS(109), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(111), 12,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2533] = 8,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(129), 1,
      sym_double_star,
    ACTIONS(137), 1,
      sym_star,
    ACTIONS(205), 1,
      anon_sym_COMMA,
    ACTIONS(135), 2,
      sym_plus,
      sym_minus,
    ACTIONS(139), 2,
      sym_slash,
      sym_percent,
    ACTIONS(203), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(207), 4,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      aux_sym_number_token1,
  [2566] = 8,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(129), 1,
      sym_double_star,
    ACTIONS(137), 1,
      sym_star,
    ACTIONS(211), 1,
      anon_sym_COMMA,
    ACTIONS(135), 2,
      sym_plus,
      sym_minus,
    ACTIONS(139), 2,
      sym_slash,
      sym_percent,
    ACTIONS(209), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(213), 4,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      aux_sym_number_token1,
  [2599] = 7,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(129), 1,
      sym_double_star,
    ACTIONS(137), 1,
      sym_star,
    ACTIONS(135), 2,
      sym_plus,
      sym_minus,
    ACTIONS(139), 2,
      sym_slash,
      sym_percent,
    ACTIONS(215), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(217), 4,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [2629] = 7,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    ACTIONS(129), 1,
      sym_double_star,
    ACTIONS(137), 1,
      sym_star,
    ACTIONS(135), 2,
      sym_plus,
      sym_minus,
    ACTIONS(139), 2,
      sym_slash,
      sym_percent,
    ACTIONS(219), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(221), 4,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [2659] = 3,
    ACTIONS(117), 2,
      anon_sym_COLON,
      anon_sym_EQ,
    ACTIONS(113), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(115), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2681] = 2,
    ACTIONS(187), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(189), 9,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2700] = 6,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(123), 4,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym__ident,
    ACTIONS(127), 4,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
  [2726] = 2,
    ACTIONS(167), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(169), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2744] = 4,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(123), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(127), 6,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_slash,
      sym_percent,
  [2766] = 3,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(123), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(127), 7,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2786] = 7,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(185), 2,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
    ACTIONS(183), 4,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym__ident,
  [2814] = 2,
    ACTIONS(179), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(181), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2832] = 2,
    ACTIONS(119), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(121), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2850] = 2,
    ACTIONS(199), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(201), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2868] = 2,
    ACTIONS(159), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(161), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2886] = 2,
    ACTIONS(163), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(165), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2904] = 7,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(133), 2,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
    ACTIONS(131), 4,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym__ident,
  [2932] = 2,
    ACTIONS(141), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(143), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2950] = 2,
    ACTIONS(191), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(193), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [2968] = 7,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(197), 2,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
    ACTIONS(195), 4,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym__ident,
  [2996] = 8,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(233), 1,
      anon_sym_else,
    ACTIONS(151), 2,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
    ACTIONS(149), 3,
      anon_sym_fn,
      anon_sym_then,
      sym__ident,
  [3026] = 2,
    ACTIONS(145), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(147), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [3044] = 2,
    ACTIONS(175), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(177), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [3062] = 2,
    ACTIONS(171), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(173), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [3080] = 2,
    ACTIONS(155), 5,
      anon_sym_fn,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym__ident,
    ACTIONS(157), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [3098] = 7,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(235), 1,
      ts_builtin_sym_end,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
    ACTIONS(237), 2,
      anon_sym_fn,
      sym__ident,
  [3123] = 7,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(239), 1,
      ts_builtin_sym_end,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
    ACTIONS(241), 2,
      anon_sym_fn,
      sym__ident,
  [3148] = 7,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(243), 1,
      ts_builtin_sym_end,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
    ACTIONS(245), 2,
      anon_sym_fn,
      sym__ident,
  [3173] = 7,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(247), 1,
      ts_builtin_sym_end,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
    ACTIONS(249), 2,
      anon_sym_fn,
      sym__ident,
  [3198] = 7,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(251), 1,
      ts_builtin_sym_end,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
    ACTIONS(253), 2,
      anon_sym_fn,
      sym__ident,
  [3223] = 7,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(255), 1,
      ts_builtin_sym_end,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
    ACTIONS(257), 2,
      anon_sym_fn,
      sym__ident,
  [3248] = 2,
    ACTIONS(259), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(261), 5,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      aux_sym_number_token1,
  [3262] = 2,
    ACTIONS(263), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(265), 5,
      anon_sym_LPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      aux_sym_number_token1,
  [3276] = 6,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(267), 1,
      anon_sym_then,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
  [3297] = 5,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(7), 1,
      anon_sym_fn,
    ACTIONS(269), 1,
      ts_builtin_sym_end,
    STATE(114), 1,
      sym_ident,
    STATE(97), 4,
      sym__module_stmt,
      sym_fn_decl,
      sym_global_var_decl,
      aux_sym_root_repeat1,
  [3316] = 6,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(271), 1,
      anon_sym_then,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
  [3337] = 6,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(273), 1,
      anon_sym_RPAREN,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
  [3358] = 5,
    ACTIONS(275), 1,
      ts_builtin_sym_end,
    ACTIONS(277), 1,
      sym__ident,
    ACTIONS(280), 1,
      anon_sym_fn,
    STATE(114), 1,
      sym_ident,
    STATE(97), 4,
      sym__module_stmt,
      sym_fn_decl,
      sym_global_var_decl,
      aux_sym_root_repeat1,
  [3377] = 6,
    ACTIONS(223), 1,
      anon_sym_LPAREN,
    ACTIONS(225), 1,
      sym_star,
    ACTIONS(227), 1,
      sym_double_star,
    ACTIONS(283), 1,
      anon_sym_RPAREN,
    ACTIONS(229), 2,
      sym_slash,
      sym_percent,
    ACTIONS(231), 2,
      sym_plus,
      sym_minus,
  [3398] = 1,
    ACTIONS(285), 6,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym__ident,
  [3407] = 1,
    ACTIONS(287), 6,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym__ident,
  [3416] = 5,
    ACTIONS(289), 1,
      sym__ident,
    ACTIONS(292), 1,
      anon_sym_RPAREN,
    STATE(101), 1,
      aux_sym_fn_decl_repeat1,
    STATE(112), 1,
      sym_fn_param,
    STATE(110), 2,
      sym__pat,
      sym_ident,
  [3433] = 5,
    ACTIONS(294), 1,
      sym__ident,
    ACTIONS(296), 1,
      anon_sym_RPAREN,
    STATE(103), 1,
      aux_sym_fn_decl_repeat1,
    STATE(112), 1,
      sym_fn_param,
    STATE(110), 2,
      sym__pat,
      sym_ident,
  [3450] = 5,
    ACTIONS(294), 1,
      sym__ident,
    ACTIONS(298), 1,
      anon_sym_RPAREN,
    STATE(101), 1,
      aux_sym_fn_decl_repeat1,
    STATE(112), 1,
      sym_fn_param,
    STATE(110), 2,
      sym__pat,
      sym_ident,
  [3467] = 3,
    ACTIONS(300), 1,
      sym__ident,
    ACTIONS(302), 1,
      anon_sym_LBRACK,
    STATE(115), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [3479] = 3,
    ACTIONS(300), 1,
      sym__ident,
    ACTIONS(302), 1,
      anon_sym_LBRACK,
    STATE(122), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [3491] = 3,
    ACTIONS(300), 1,
      sym__ident,
    ACTIONS(302), 1,
      anon_sym_LBRACK,
    STATE(124), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [3503] = 3,
    ACTIONS(294), 1,
      sym__ident,
    ACTIONS(302), 1,
      anon_sym_LBRACK,
    STATE(111), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [3515] = 3,
    ACTIONS(300), 1,
      sym__ident,
    ACTIONS(302), 1,
      anon_sym_LBRACK,
    STATE(131), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [3527] = 3,
    ACTIONS(300), 1,
      sym__ident,
    ACTIONS(302), 1,
      anon_sym_LBRACK,
    STATE(129), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [3539] = 2,
    ACTIONS(306), 1,
      anon_sym_COLON,
    ACTIONS(304), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      sym__ident,
  [3548] = 1,
    ACTIONS(308), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      sym__ident,
  [3554] = 2,
    ACTIONS(312), 1,
      anon_sym_COMMA,
    ACTIONS(310), 2,
      anon_sym_RPAREN,
      sym__ident,
  [3562] = 1,
    ACTIONS(314), 2,
      anon_sym_RPAREN,
      sym__ident,
  [3567] = 2,
    ACTIONS(316), 1,
      anon_sym_COLON,
    ACTIONS(318), 1,
      anon_sym_EQ,
  [3574] = 2,
    ACTIONS(320), 1,
      anon_sym_RBRACK,
    ACTIONS(322), 1,
      anon_sym_SEMI,
  [3581] = 2,
    ACTIONS(107), 1,
      aux_sym_number_token1,
    STATE(128), 1,
      sym_number,
  [3588] = 2,
    ACTIONS(324), 1,
      anon_sym_COLON,
    ACTIONS(326), 1,
      anon_sym_EQ,
  [3595] = 2,
    ACTIONS(328), 1,
      anon_sym_COLON,
    ACTIONS(330), 1,
      anon_sym_EQ,
  [3602] = 2,
    ACTIONS(332), 1,
      anon_sym_COLON,
    ACTIONS(334), 1,
      anon_sym_EQ,
  [3609] = 2,
    ACTIONS(300), 1,
      sym__ident,
    STATE(127), 1,
      sym_ident,
  [3616] = 1,
    ACTIONS(336), 1,
      sym_string_lit_content,
  [3620] = 1,
    ACTIONS(338), 1,
      anon_sym_EQ,
  [3624] = 1,
    ACTIONS(340), 1,
      anon_sym_DQUOTE2,
  [3628] = 1,
    ACTIONS(342), 1,
      anon_sym_EQ,
  [3632] = 1,
    ACTIONS(344), 1,
      anon_sym_RPAREN,
  [3636] = 1,
    ACTIONS(346), 1,
      anon_sym_DQUOTE2,
  [3640] = 1,
    ACTIONS(348), 1,
      anon_sym_LPAREN,
  [3644] = 1,
    ACTIONS(350), 1,
      anon_sym_RBRACK,
  [3648] = 1,
    ACTIONS(352), 1,
      anon_sym_EQ,
  [3652] = 1,
    ACTIONS(354), 1,
      anon_sym_RPAREN,
  [3656] = 1,
    ACTIONS(356), 1,
      anon_sym_EQ,
  [3660] = 1,
    ACTIONS(358), 1,
      sym_string_lit_content,
  [3664] = 1,
    ACTIONS(360), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 62,
  [SMALL_STATE(4)] = 124,
  [SMALL_STATE(5)] = 183,
  [SMALL_STATE(6)] = 242,
  [SMALL_STATE(7)] = 301,
  [SMALL_STATE(8)] = 360,
  [SMALL_STATE(9)] = 419,
  [SMALL_STATE(10)] = 478,
  [SMALL_STATE(11)] = 537,
  [SMALL_STATE(12)] = 593,
  [SMALL_STATE(13)] = 649,
  [SMALL_STATE(14)] = 702,
  [SMALL_STATE(15)] = 755,
  [SMALL_STATE(16)] = 808,
  [SMALL_STATE(17)] = 861,
  [SMALL_STATE(18)] = 914,
  [SMALL_STATE(19)] = 967,
  [SMALL_STATE(20)] = 1020,
  [SMALL_STATE(21)] = 1073,
  [SMALL_STATE(22)] = 1126,
  [SMALL_STATE(23)] = 1179,
  [SMALL_STATE(24)] = 1232,
  [SMALL_STATE(25)] = 1285,
  [SMALL_STATE(26)] = 1338,
  [SMALL_STATE(27)] = 1391,
  [SMALL_STATE(28)] = 1444,
  [SMALL_STATE(29)] = 1497,
  [SMALL_STATE(30)] = 1550,
  [SMALL_STATE(31)] = 1603,
  [SMALL_STATE(32)] = 1656,
  [SMALL_STATE(33)] = 1709,
  [SMALL_STATE(34)] = 1762,
  [SMALL_STATE(35)] = 1815,
  [SMALL_STATE(36)] = 1868,
  [SMALL_STATE(37)] = 1921,
  [SMALL_STATE(38)] = 1947,
  [SMALL_STATE(39)] = 1975,
  [SMALL_STATE(40)] = 1999,
  [SMALL_STATE(41)] = 2027,
  [SMALL_STATE(42)] = 2061,
  [SMALL_STATE(43)] = 2085,
  [SMALL_STATE(44)] = 2109,
  [SMALL_STATE(45)] = 2145,
  [SMALL_STATE(46)] = 2169,
  [SMALL_STATE(47)] = 2193,
  [SMALL_STATE(48)] = 2217,
  [SMALL_STATE(49)] = 2241,
  [SMALL_STATE(50)] = 2265,
  [SMALL_STATE(51)] = 2289,
  [SMALL_STATE(52)] = 2313,
  [SMALL_STATE(53)] = 2347,
  [SMALL_STATE(54)] = 2371,
  [SMALL_STATE(55)] = 2395,
  [SMALL_STATE(56)] = 2421,
  [SMALL_STATE(57)] = 2453,
  [SMALL_STATE(58)] = 2487,
  [SMALL_STATE(59)] = 2511,
  [SMALL_STATE(60)] = 2533,
  [SMALL_STATE(61)] = 2566,
  [SMALL_STATE(62)] = 2599,
  [SMALL_STATE(63)] = 2629,
  [SMALL_STATE(64)] = 2659,
  [SMALL_STATE(65)] = 2681,
  [SMALL_STATE(66)] = 2700,
  [SMALL_STATE(67)] = 2726,
  [SMALL_STATE(68)] = 2744,
  [SMALL_STATE(69)] = 2766,
  [SMALL_STATE(70)] = 2786,
  [SMALL_STATE(71)] = 2814,
  [SMALL_STATE(72)] = 2832,
  [SMALL_STATE(73)] = 2850,
  [SMALL_STATE(74)] = 2868,
  [SMALL_STATE(75)] = 2886,
  [SMALL_STATE(76)] = 2904,
  [SMALL_STATE(77)] = 2932,
  [SMALL_STATE(78)] = 2950,
  [SMALL_STATE(79)] = 2968,
  [SMALL_STATE(80)] = 2996,
  [SMALL_STATE(81)] = 3026,
  [SMALL_STATE(82)] = 3044,
  [SMALL_STATE(83)] = 3062,
  [SMALL_STATE(84)] = 3080,
  [SMALL_STATE(85)] = 3098,
  [SMALL_STATE(86)] = 3123,
  [SMALL_STATE(87)] = 3148,
  [SMALL_STATE(88)] = 3173,
  [SMALL_STATE(89)] = 3198,
  [SMALL_STATE(90)] = 3223,
  [SMALL_STATE(91)] = 3248,
  [SMALL_STATE(92)] = 3262,
  [SMALL_STATE(93)] = 3276,
  [SMALL_STATE(94)] = 3297,
  [SMALL_STATE(95)] = 3316,
  [SMALL_STATE(96)] = 3337,
  [SMALL_STATE(97)] = 3358,
  [SMALL_STATE(98)] = 3377,
  [SMALL_STATE(99)] = 3398,
  [SMALL_STATE(100)] = 3407,
  [SMALL_STATE(101)] = 3416,
  [SMALL_STATE(102)] = 3433,
  [SMALL_STATE(103)] = 3450,
  [SMALL_STATE(104)] = 3467,
  [SMALL_STATE(105)] = 3479,
  [SMALL_STATE(106)] = 3491,
  [SMALL_STATE(107)] = 3503,
  [SMALL_STATE(108)] = 3515,
  [SMALL_STATE(109)] = 3527,
  [SMALL_STATE(110)] = 3539,
  [SMALL_STATE(111)] = 3548,
  [SMALL_STATE(112)] = 3554,
  [SMALL_STATE(113)] = 3562,
  [SMALL_STATE(114)] = 3567,
  [SMALL_STATE(115)] = 3574,
  [SMALL_STATE(116)] = 3581,
  [SMALL_STATE(117)] = 3588,
  [SMALL_STATE(118)] = 3595,
  [SMALL_STATE(119)] = 3602,
  [SMALL_STATE(120)] = 3609,
  [SMALL_STATE(121)] = 3616,
  [SMALL_STATE(122)] = 3620,
  [SMALL_STATE(123)] = 3624,
  [SMALL_STATE(124)] = 3628,
  [SMALL_STATE(125)] = 3632,
  [SMALL_STATE(126)] = 3636,
  [SMALL_STATE(127)] = 3640,
  [SMALL_STATE(128)] = 3644,
  [SMALL_STATE(129)] = 3648,
  [SMALL_STATE(130)] = 3652,
  [SMALL_STATE(131)] = 3656,
  [SMALL_STATE(132)] = 3660,
  [SMALL_STATE(133)] = 3664,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_root, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(59),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(120),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(39),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(58),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [31] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 13), SHIFT_REPEAT(37),
  [34] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 13), SHIFT_REPEAT(16),
  [37] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 13), SHIFT_REPEAT(132),
  [40] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 13), SHIFT_REPEAT(8),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 13),
  [45] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 13), SHIFT_REPEAT(14),
  [48] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 13), SHIFT_REPEAT(39),
  [51] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 13), SHIFT_REPEAT(58),
  [54] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 13), SHIFT_REPEAT(53),
  [57] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 25), SHIFT_REPEAT(37),
  [60] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 25), SHIFT_REPEAT(16),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 25),
  [65] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 25), SHIFT_REPEAT(132),
  [68] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 25), SHIFT_REPEAT(8),
  [71] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 25), SHIFT_REPEAT(14),
  [74] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 25), SHIFT_REPEAT(39),
  [77] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 25), SHIFT_REPEAT(58),
  [80] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 25), SHIFT_REPEAT(53),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__call_args_list, 1, .production_id = 16),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [93] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [97] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [101] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [103] = {.entry = {.count = 1, .reusable = false}}, SHIFT(72),
  [105] = {.entry = {.count = 1, .reusable = false}}, SHIFT(73),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [109] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ident, 1),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ident, 1),
  [113] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 1),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 1),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__pat, 1),
  [119] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_true, 1),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_true, 1),
  [123] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bin_op, 3, .production_id = 17),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bin_op, 3, .production_id = 17),
  [129] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [131] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 2, .production_id = 6),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 2, .production_id = 6),
  [135] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [137] = {.entry = {.count = 1, .reusable = false}}, SHIFT(34),
  [139] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [141] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 2, .production_id = 7),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 2, .production_id = 7),
  [145] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 3),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 3),
  [149] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if, 4, .production_id = 23),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if, 4, .production_id = 23),
  [153] = {.entry = {.count = 1, .reusable = false}}, SHIFT(36),
  [155] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_lit, 3, .production_id = 11),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_lit, 3, .production_id = 11),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block, 1, .production_id = 2),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block, 1, .production_id = 2),
  [163] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_lit, 2),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_lit, 2),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 1, .production_id = 2),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 1, .production_id = 2),
  [171] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_lit, 3, .production_id = 12),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_lit, 3, .production_id = 12),
  [175] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 4, .production_id = 24),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 4, .production_id = 24),
  [179] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 3, .production_id = 19),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 3, .production_id = 19),
  [183] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 3, .production_id = 18),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 3, .production_id = 18),
  [187] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 1),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1),
  [191] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 3, .production_id = 14),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 3, .production_id = 14),
  [195] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if, 6, .production_id = 30),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if, 6, .production_id = 30),
  [199] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_false, 1),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_false, 1),
  [203] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 1, .production_id = 5),
  [205] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 1, .production_id = 5),
  [209] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 1, .production_id = 15),
  [211] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [213] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 1, .production_id = 15),
  [215] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_var_decl, 5, .production_id = 28),
  [217] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_decl, 5, .production_id = 28),
  [219] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_var_decl, 3, .production_id = 20),
  [221] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_decl, 3, .production_id = 20),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [225] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [227] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [229] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [233] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 7, .production_id = 26),
  [237] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 7, .production_id = 26),
  [239] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_var_decl, 5, .production_id = 10),
  [241] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_var_decl, 5, .production_id = 10),
  [243] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 8, .production_id = 29),
  [245] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 8, .production_id = 29),
  [247] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 9, .production_id = 31),
  [249] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 9, .production_id = 31),
  [251] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_var_decl, 3, .production_id = 1),
  [253] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_var_decl, 3, .production_id = 1),
  [255] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 6, .production_id = 21),
  [257] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 6, .production_id = 21),
  [259] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 15),
  [261] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 15),
  [263] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 5),
  [265] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 5),
  [267] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [269] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_root, 1),
  [271] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [273] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [275] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_root_repeat1, 2),
  [277] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_root_repeat1, 2), SHIFT_REPEAT(59),
  [280] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_root_repeat1, 2), SHIFT_REPEAT(120),
  [283] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [285] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 5, .production_id = 27),
  [287] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 3, .production_id = 9),
  [289] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 2, .production_id = 8), SHIFT_REPEAT(37),
  [292] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 2, .production_id = 8),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [298] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [300] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [302] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [304] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_param, 1, .production_id = 4),
  [306] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [308] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_param, 3, .production_id = 22),
  [310] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 1, .production_id = 3),
  [312] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [314] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 2, .production_id = 3),
  [316] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [318] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [320] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [322] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [324] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [326] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [328] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [330] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [332] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [334] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [340] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [344] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [346] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [348] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [350] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [352] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [354] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [356] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [358] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [360] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_torvo(void) {
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
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym__ident,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
