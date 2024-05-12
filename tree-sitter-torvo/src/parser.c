#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 95
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 59
#define ALIAS_COUNT 0
#define TOKEN_COUNT 31
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 20
#define MAX_ALIAS_SEQUENCE_LENGTH 8
#define PRODUCTION_ID_COUNT 32

enum ts_symbol_identifiers {
  sym__ident = 1,
  anon_sym_LPAREN = 2,
  anon_sym_COMMA = 3,
  anon_sym_RPAREN = 4,
  anon_sym_COLON = 5,
  anon_sym_EQ = 6,
  anon_sym_DQUOTE = 7,
  anon_sym_DQUOTE2 = 8,
  sym_string_lit_content = 9,
  anon_sym_LBRACK = 10,
  anon_sym_RBRACK = 11,
  anon_sym_SEMI = 12,
  anon_sym_if = 13,
  anon_sym_then = 14,
  anon_sym_else = 15,
  sym_plus = 16,
  sym_minus = 17,
  sym_star = 18,
  sym_double_star = 19,
  sym_slash = 20,
  sym_percent = 21,
  sym_double_eq = 22,
  sym_not_eq = 23,
  sym_gt = 24,
  sym_lt = 25,
  sym_gt_eq = 26,
  sym_lt_eq = 27,
  anon_sym_true = 28,
  anon_sym_false = 29,
  aux_sym_number_token1 = 30,
  sym_root = 31,
  sym__module_stmt = 32,
  sym_func_decl = 33,
  sym_func_param = 34,
  sym_global_var_decl = 35,
  sym_var_decl = 36,
  sym__expr = 37,
  sym_bin_op = 38,
  sym_call = 39,
  sym__call_args_list = 40,
  sym_string_lit = 41,
  sym_array_lit = 42,
  sym_block = 43,
  sym__block = 44,
  sym__block_clause = 45,
  sym__block_stmt = 46,
  sym_if = 47,
  sym__type_expr = 48,
  sym_array_type = 49,
  sym__pat = 50,
  sym_ident = 51,
  sym_true = 52,
  sym_false = 53,
  sym_number = 54,
  aux_sym_root_repeat1 = 55,
  aux_sym_func_decl_repeat1 = 56,
  aux_sym__call_args_list_repeat1 = 57,
  aux_sym_array_lit_repeat1 = 58,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym__ident] = "_ident",
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
  [sym_double_eq] = "double_eq",
  [sym_not_eq] = "not_eq",
  [sym_gt] = "gt",
  [sym_lt] = "lt",
  [sym_gt_eq] = "gt_eq",
  [sym_lt_eq] = "lt_eq",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [aux_sym_number_token1] = "number_token1",
  [sym_root] = "root",
  [sym__module_stmt] = "_module_stmt",
  [sym_func_decl] = "func_decl",
  [sym_func_param] = "func_param",
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
  [aux_sym_func_decl_repeat1] = "func_decl_repeat1",
  [aux_sym__call_args_list_repeat1] = "_call_args_list_repeat1",
  [aux_sym_array_lit_repeat1] = "array_lit_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym__ident] = sym__ident,
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
  [sym_double_eq] = sym_double_eq,
  [sym_not_eq] = sym_not_eq,
  [sym_gt] = sym_gt,
  [sym_lt] = sym_lt,
  [sym_gt_eq] = sym_gt_eq,
  [sym_lt_eq] = sym_lt_eq,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [aux_sym_number_token1] = aux_sym_number_token1,
  [sym_root] = sym_root,
  [sym__module_stmt] = sym__module_stmt,
  [sym_func_decl] = sym_func_decl,
  [sym_func_param] = sym_func_param,
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
  [aux_sym_func_decl_repeat1] = aux_sym_func_decl_repeat1,
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
  [sym_double_eq] = {
    .visible = true,
    .named = true,
  },
  [sym_not_eq] = {
    .visible = true,
    .named = true,
  },
  [sym_gt] = {
    .visible = true,
    .named = true,
  },
  [sym_lt] = {
    .visible = true,
    .named = true,
  },
  [sym_gt_eq] = {
    .visible = true,
    .named = true,
  },
  [sym_lt_eq] = {
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
  [sym_func_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_func_param] = {
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
  [aux_sym_func_decl_repeat1] = {
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
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 2},
  [4] = {.index = 4, .length = 2},
  [5] = {.index = 6, .length = 2},
  [6] = {.index = 8, .length = 1},
  [7] = {.index = 9, .length = 2},
  [8] = {.index = 11, .length = 3},
  [9] = {.index = 14, .length = 2},
  [10] = {.index = 16, .length = 2},
  [11] = {.index = 18, .length = 1},
  [12] = {.index = 19, .length = 3},
  [13] = {.index = 22, .length = 1},
  [14] = {.index = 23, .length = 1},
  [15] = {.index = 24, .length = 2},
  [16] = {.index = 26, .length = 1},
  [17] = {.index = 27, .length = 1},
  [18] = {.index = 28, .length = 1},
  [19] = {.index = 29, .length = 3},
  [20] = {.index = 32, .length = 2},
  [21] = {.index = 34, .length = 3},
  [22] = {.index = 37, .length = 2},
  [23] = {.index = 39, .length = 3},
  [24] = {.index = 42, .length = 2},
  [25] = {.index = 44, .length = 2},
  [26] = {.index = 46, .length = 2},
  [27] = {.index = 48, .length = 3},
  [28] = {.index = 51, .length = 2},
  [29] = {.index = 53, .length = 3},
  [30] = {.index = 56, .length = 4},
  [31] = {.index = 60, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_params, 0},
  [1] =
    {field_pat, 0},
  [2] =
    {field_name, 0},
    {field_value, 2},
  [4] =
    {field_body, 0, .inherited = true},
    {field_value, 0, .inherited = true},
  [6] =
    {field_params, 0, .inherited = true},
    {field_params, 1, .inherited = true},
  [8] =
    {field_items, 0},
  [9] =
    {field_body, 0},
    {field_value, 1},
  [11] =
    {field_body, 0},
    {field_body, 1, .inherited = true},
    {field_value, 1, .inherited = true},
  [14] =
    {field_name, 0},
    {field_return, 4},
  [16] =
    {field_pat, 0},
    {field_type, 2},
  [18] =
    {field_item_type, 1},
  [19] =
    {field_name, 0},
    {field_type, 2},
    {field_value, 4},
  [22] =
    {field_content, 1},
  [23] =
    {field_items, 1, .inherited = true},
  [24] =
    {field_items, 0, .inherited = true},
    {field_items, 1, .inherited = true},
  [26] =
    {field_callee, 0},
  [27] =
    {field_args, 0},
  [28] =
    {field_args, 0, .inherited = true},
  [29] =
    {field_left, 0},
    {field_op, 1},
    {field_right, 2},
  [32] =
    {field_body, 0},
    {field_value, 2},
  [34] =
    {field_body, 0},
    {field_body, 2, .inherited = true},
    {field_value, 2, .inherited = true},
  [37] =
    {field_pat, 0},
    {field_value, 2},
  [39] =
    {field_name, 0},
    {field_params, 2, .inherited = true},
    {field_return, 5},
  [42] =
    {field_cond, 1},
    {field_then, 3},
  [44] =
    {field_args, 2, .inherited = true},
    {field_callee, 0},
  [46] =
    {field_args, 0, .inherited = true},
    {field_args, 1, .inherited = true},
  [48] =
    {field_name, 0},
    {field_ret_type, 4},
    {field_return, 6},
  [51] =
    {field_item_type, 1},
    {field_length, 3},
  [53] =
    {field_pat, 0},
    {field_type, 2},
    {field_value, 4},
  [56] =
    {field_name, 0},
    {field_params, 2, .inherited = true},
    {field_ret_type, 5},
    {field_return, 7},
  [60] =
    {field_cond, 1},
    {field_else, 5},
    {field_then, 3},
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
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 84,
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
      if (lookahead == '!') ADVANCE(2);
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
      if (lookahead == '<') ADVANCE(29);
      if (lookahead == '=') ADVANCE(11);
      if (lookahead == '>') ADVANCE(28);
      if (lookahead == '[') ADVANCE(17);
      if (lookahead == ']') ADVANCE(18);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(33);
      if (sym__ident_character_set_1(lookahead)) ADVANCE(32);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(13);
      if (lookahead == '\\') ADVANCE(15);
      if (lookahead != 0) ADVANCE(16);
      END_STATE();
    case 2:
      if (lookahead == '=') ADVANCE(27);
      END_STATE();
    case 3:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(33);
      END_STATE();
    case 4:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(34);
      END_STATE();
    case 5:
      if (eof) ADVANCE(6);
      if (lookahead == '!') ADVANCE(2);
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
      if (lookahead == '<') ADVANCE(29);
      if (lookahead == '=') ADVANCE(11);
      if (lookahead == '>') ADVANCE(28);
      if (lookahead == '[') ADVANCE(17);
      if (lookahead == ']') ADVANCE(18);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(33);
      if (sym__ident_character_set_1(lookahead)) ADVANCE(32);
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
      if (lookahead == '=') ADVANCE(26);
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
      ACCEPT_TOKEN(sym_double_eq);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_not_eq);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_gt);
      if (lookahead == '=') ADVANCE(30);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_lt);
      if (lookahead == '=') ADVANCE(31);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_gt_eq);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_lt_eq);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym__ident);
      if (sym__ident_character_set_2(lookahead)) ADVANCE(32);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '.') ADVANCE(4);
      if (lookahead == '_') ADVANCE(3);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(33);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '_') ADVANCE(4);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(34);
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
      END_STATE();
    case 3:
      if (lookahead == 'f') ADVANCE(7);
      END_STATE();
    case 4:
      if (lookahead == 'h') ADVANCE(8);
      if (lookahead == 'r') ADVANCE(9);
      END_STATE();
    case 5:
      if (lookahead == 's') ADVANCE(10);
      END_STATE();
    case 6:
      if (lookahead == 'l') ADVANCE(11);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(12);
      END_STATE();
    case 9:
      if (lookahead == 'u') ADVANCE(13);
      END_STATE();
    case 10:
      if (lookahead == 'e') ADVANCE(14);
      END_STATE();
    case 11:
      if (lookahead == 's') ADVANCE(15);
      END_STATE();
    case 12:
      if (lookahead == 'n') ADVANCE(16);
      END_STATE();
    case 13:
      if (lookahead == 'e') ADVANCE(17);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 15:
      if (lookahead == 'e') ADVANCE(18);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_then);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 5},
  [3] = {.lex_state = 5},
  [4] = {.lex_state = 5},
  [5] = {.lex_state = 5},
  [6] = {.lex_state = 5},
  [7] = {.lex_state = 5},
  [8] = {.lex_state = 5},
  [9] = {.lex_state = 5},
  [10] = {.lex_state = 5},
  [11] = {.lex_state = 5},
  [12] = {.lex_state = 5},
  [13] = {.lex_state = 5},
  [14] = {.lex_state = 5},
  [15] = {.lex_state = 5},
  [16] = {.lex_state = 5},
  [17] = {.lex_state = 5},
  [18] = {.lex_state = 5},
  [19] = {.lex_state = 5},
  [20] = {.lex_state = 5},
  [21] = {.lex_state = 5},
  [22] = {.lex_state = 5},
  [23] = {.lex_state = 5},
  [24] = {.lex_state = 5},
  [25] = {.lex_state = 5},
  [26] = {.lex_state = 5},
  [27] = {.lex_state = 5},
  [28] = {.lex_state = 5},
  [29] = {.lex_state = 5},
  [30] = {.lex_state = 5},
  [31] = {.lex_state = 5},
  [32] = {.lex_state = 5},
  [33] = {.lex_state = 5},
  [34] = {.lex_state = 5},
  [35] = {.lex_state = 5},
  [36] = {.lex_state = 5},
  [37] = {.lex_state = 5},
  [38] = {.lex_state = 5},
  [39] = {.lex_state = 5},
  [40] = {.lex_state = 5},
  [41] = {.lex_state = 5},
  [42] = {.lex_state = 5},
  [43] = {.lex_state = 5},
  [44] = {.lex_state = 5},
  [45] = {.lex_state = 5},
  [46] = {.lex_state = 5},
  [47] = {.lex_state = 5},
  [48] = {.lex_state = 5},
  [49] = {.lex_state = 5},
  [50] = {.lex_state = 5},
  [51] = {.lex_state = 5},
  [52] = {.lex_state = 5},
  [53] = {.lex_state = 0},
  [54] = {.lex_state = 0},
  [55] = {.lex_state = 0},
  [56] = {.lex_state = 0},
  [57] = {.lex_state = 0},
  [58] = {.lex_state = 0},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 5},
  [62] = {.lex_state = 5},
  [63] = {.lex_state = 0},
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
  [91] = {.lex_state = 1},
  [92] = {.lex_state = 0},
  [93] = {.lex_state = 1},
  [94] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym__ident] = ACTIONS(1),
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
    [sym_double_eq] = ACTIONS(1),
    [sym_not_eq] = ACTIONS(1),
    [sym_gt] = ACTIONS(1),
    [sym_lt] = ACTIONS(1),
    [sym_gt_eq] = ACTIONS(1),
    [sym_lt_eq] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [aux_sym_number_token1] = ACTIONS(1),
  },
  [1] = {
    [sym_root] = STATE(89),
    [sym__module_stmt] = STATE(64),
    [sym_func_decl] = STATE(64),
    [sym_global_var_decl] = STATE(64),
    [sym_ident] = STATE(79),
    [aux_sym_root_repeat1] = STATE(64),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym__ident] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(11), 1,
      anon_sym_COLON,
    ACTIONS(13), 1,
      anon_sym_EQ,
    ACTIONS(9), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(7), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [38] = 2,
    ACTIONS(17), 10,
      anon_sym_EQ,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(15), 19,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_COLON,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [72] = 2,
    ACTIONS(21), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(19), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [104] = 2,
    ACTIONS(25), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(23), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [136] = 10,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(33), 1,
      anon_sym_else,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
    ACTIONS(29), 5,
      anon_sym_if,
      anon_sym_then,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [184] = 2,
    ACTIONS(49), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(47), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [216] = 9,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
    ACTIONS(53), 6,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(51), 8,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [262] = 7,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(57), 8,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(55), 12,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [304] = 3,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(57), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(55), 17,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [338] = 4,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(57), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(55), 16,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [374] = 6,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(57), 8,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(55), 14,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [414] = 2,
    ACTIONS(61), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(59), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [446] = 2,
    ACTIONS(65), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(63), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [478] = 9,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
    ACTIONS(69), 6,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(67), 8,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [524] = 17,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(75), 1,
      anon_sym_RPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(30), 1,
      aux_sym__call_args_list_repeat1,
    STATE(82), 1,
      sym__pat,
    STATE(90), 1,
      sym__call_args_list,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(49), 10,
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
  [586] = 2,
    ACTIONS(91), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(89), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [618] = 9,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
    ACTIONS(95), 6,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(93), 8,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [664] = 2,
    ACTIONS(99), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(97), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [696] = 2,
    ACTIONS(103), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(101), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [728] = 2,
    ACTIONS(107), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(105), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [760] = 2,
    ACTIONS(111), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(109), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [792] = 2,
    ACTIONS(115), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(113), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [824] = 2,
    ACTIONS(119), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(117), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [856] = 2,
    ACTIONS(123), 9,
      anon_sym_if,
      anon_sym_then,
      anon_sym_else,
      sym_star,
      sym_gt,
      sym_lt,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(121), 18,
      ts_builtin_sym_end,
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
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
      aux_sym_number_token1,
  [888] = 16,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    ACTIONS(125), 1,
      anon_sym_RBRACK,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(28), 1,
      aux_sym_array_lit_repeat1,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(50), 10,
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
  [947] = 16,
    ACTIONS(127), 1,
      sym__ident,
    ACTIONS(130), 1,
      anon_sym_LPAREN,
    ACTIONS(133), 1,
      anon_sym_DQUOTE,
    ACTIONS(136), 1,
      anon_sym_LBRACK,
    ACTIONS(139), 1,
      anon_sym_RBRACK,
    ACTIONS(141), 1,
      anon_sym_if,
    ACTIONS(144), 1,
      anon_sym_true,
    ACTIONS(147), 1,
      anon_sym_false,
    ACTIONS(150), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(27), 1,
      aux_sym_array_lit_repeat1,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(50), 10,
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
  [1006] = 16,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    ACTIONS(153), 1,
      anon_sym_RBRACK,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(27), 1,
      aux_sym_array_lit_repeat1,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(50), 10,
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
  [1065] = 16,
    ACTIONS(155), 1,
      sym__ident,
    ACTIONS(158), 1,
      anon_sym_LPAREN,
    ACTIONS(161), 1,
      anon_sym_RPAREN,
    ACTIONS(163), 1,
      anon_sym_DQUOTE,
    ACTIONS(166), 1,
      anon_sym_LBRACK,
    ACTIONS(169), 1,
      anon_sym_if,
    ACTIONS(172), 1,
      anon_sym_true,
    ACTIONS(175), 1,
      anon_sym_false,
    ACTIONS(178), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(29), 1,
      aux_sym__call_args_list_repeat1,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(49), 10,
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
  [1124] = 16,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    ACTIONS(181), 1,
      anon_sym_RPAREN,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(29), 1,
      aux_sym__call_args_list_repeat1,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(49), 10,
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
  [1183] = 15,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    ACTIONS(183), 1,
      anon_sym_SEMI,
    STATE(2), 1,
      sym_ident,
    STATE(23), 1,
      sym__block_clause,
    STATE(24), 1,
      sym__block,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(18), 10,
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
  [1239] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(10), 10,
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
  [1292] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
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
  [1345] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(54), 10,
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
  [1398] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(15), 10,
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
  [1451] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(53), 10,
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
  [1504] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
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
  [1557] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(58), 10,
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
  [1610] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
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
  [1663] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
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
  [1716] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(6), 10,
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
  [1769] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(59), 10,
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
  [1822] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(12), 10,
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
  [1875] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(11), 10,
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
  [1928] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(9), 10,
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
  [1981] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(7), 1,
      sym__block_clause,
    STATE(24), 1,
      sym__block,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(8), 10,
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
  [2034] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(51), 10,
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
  [2087] = 14,
    ACTIONS(71), 1,
      sym__ident,
    ACTIONS(73), 1,
      anon_sym_LPAREN,
    ACTIONS(77), 1,
      anon_sym_DQUOTE,
    ACTIONS(79), 1,
      anon_sym_LBRACK,
    ACTIONS(81), 1,
      anon_sym_if,
    ACTIONS(83), 1,
      anon_sym_true,
    ACTIONS(85), 1,
      anon_sym_false,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(2), 1,
      sym_ident,
    STATE(24), 1,
      sym__block,
    STATE(25), 1,
      sym__block_clause,
    STATE(82), 1,
      sym__pat,
    STATE(31), 2,
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
  [2140] = 10,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(187), 1,
      anon_sym_COMMA,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
    ACTIONS(185), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(189), 4,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      aux_sym_number_token1,
  [2183] = 10,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(193), 1,
      anon_sym_COMMA,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
    ACTIONS(191), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(195), 4,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      aux_sym_number_token1,
  [2226] = 9,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
    ACTIONS(197), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(199), 4,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [2266] = 9,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
    ACTIONS(201), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(203), 4,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      aux_sym_number_token1,
  [2306] = 8,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(205), 2,
      ts_builtin_sym_end,
      sym__ident,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
  [2338] = 8,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(207), 2,
      ts_builtin_sym_end,
      sym__ident,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
  [2370] = 8,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(209), 2,
      ts_builtin_sym_end,
      sym__ident,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
  [2402] = 8,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(211), 2,
      ts_builtin_sym_end,
      sym__ident,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
  [2434] = 8,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(213), 2,
      ts_builtin_sym_end,
      sym__ident,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
  [2466] = 8,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(215), 2,
      ts_builtin_sym_end,
      sym__ident,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
  [2498] = 8,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(217), 1,
      anon_sym_RPAREN,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
  [2529] = 8,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(37), 1,
      sym_star,
    ACTIONS(39), 1,
      sym_double_star,
    ACTIONS(219), 1,
      anon_sym_then,
    ACTIONS(35), 2,
      sym_plus,
      sym_minus,
    ACTIONS(41), 2,
      sym_slash,
      sym_percent,
    ACTIONS(45), 2,
      sym_gt,
      sym_lt,
    ACTIONS(43), 4,
      sym_double_eq,
      sym_not_eq,
      sym_gt_eq,
      sym_lt_eq,
  [2560] = 2,
    ACTIONS(221), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(223), 5,
      anon_sym_LPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      aux_sym_number_token1,
  [2574] = 2,
    ACTIONS(225), 4,
      anon_sym_if,
      sym__ident,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(227), 5,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_LBRACK,
      aux_sym_number_token1,
  [2588] = 4,
    ACTIONS(229), 1,
      ts_builtin_sym_end,
    ACTIONS(231), 1,
      sym__ident,
    STATE(79), 1,
      sym_ident,
    STATE(63), 4,
      sym__module_stmt,
      sym_func_decl,
      sym_global_var_decl,
      aux_sym_root_repeat1,
  [2604] = 4,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(234), 1,
      ts_builtin_sym_end,
    STATE(79), 1,
      sym_ident,
    STATE(63), 4,
      sym__module_stmt,
      sym_func_decl,
      sym_global_var_decl,
      aux_sym_root_repeat1,
  [2620] = 5,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(236), 1,
      anon_sym_RPAREN,
    STATE(68), 1,
      aux_sym_func_decl_repeat1,
    STATE(77), 1,
      sym_func_param,
    STATE(76), 2,
      sym__pat,
      sym_ident,
  [2637] = 1,
    ACTIONS(238), 6,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym__ident,
  [2646] = 1,
    ACTIONS(240), 6,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym__ident,
  [2655] = 5,
    ACTIONS(242), 1,
      sym__ident,
    ACTIONS(245), 1,
      anon_sym_RPAREN,
    STATE(68), 1,
      aux_sym_func_decl_repeat1,
    STATE(77), 1,
      sym_func_param,
    STATE(76), 2,
      sym__pat,
      sym_ident,
  [2672] = 5,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(247), 1,
      anon_sym_RPAREN,
    STATE(65), 1,
      aux_sym_func_decl_repeat1,
    STATE(77), 1,
      sym_func_param,
    STATE(76), 2,
      sym__pat,
      sym_ident,
  [2689] = 3,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(249), 1,
      anon_sym_LBRACK,
    STATE(87), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2701] = 3,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(249), 1,
      anon_sym_LBRACK,
    STATE(80), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2713] = 3,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(249), 1,
      anon_sym_LBRACK,
    STATE(92), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2725] = 3,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(249), 1,
      anon_sym_LBRACK,
    STATE(86), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2737] = 3,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(249), 1,
      anon_sym_LBRACK,
    STATE(78), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2749] = 3,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(249), 1,
      anon_sym_LBRACK,
    STATE(94), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2761] = 2,
    ACTIONS(253), 1,
      anon_sym_COLON,
    ACTIONS(251), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      sym__ident,
  [2770] = 2,
    ACTIONS(257), 1,
      anon_sym_COMMA,
    ACTIONS(255), 2,
      anon_sym_RPAREN,
      sym__ident,
  [2778] = 1,
    ACTIONS(259), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      sym__ident,
  [2784] = 3,
    ACTIONS(261), 1,
      anon_sym_LPAREN,
    ACTIONS(263), 1,
      anon_sym_COLON,
    ACTIONS(265), 1,
      anon_sym_EQ,
  [2794] = 2,
    ACTIONS(267), 1,
      anon_sym_RBRACK,
    ACTIONS(269), 1,
      anon_sym_SEMI,
  [2801] = 2,
    ACTIONS(271), 1,
      anon_sym_COLON,
    ACTIONS(273), 1,
      anon_sym_EQ,
  [2808] = 2,
    ACTIONS(275), 1,
      anon_sym_COLON,
    ACTIONS(277), 1,
      anon_sym_EQ,
  [2815] = 1,
    ACTIONS(279), 2,
      anon_sym_RPAREN,
      sym__ident,
  [2820] = 2,
    ACTIONS(281), 1,
      anon_sym_COLON,
    ACTIONS(283), 1,
      anon_sym_EQ,
  [2827] = 2,
    ACTIONS(87), 1,
      aux_sym_number_token1,
    STATE(88), 1,
      sym_number,
  [2834] = 1,
    ACTIONS(285), 1,
      anon_sym_EQ,
  [2838] = 1,
    ACTIONS(287), 1,
      anon_sym_EQ,
  [2842] = 1,
    ACTIONS(289), 1,
      anon_sym_RBRACK,
  [2846] = 1,
    ACTIONS(291), 1,
      ts_builtin_sym_end,
  [2850] = 1,
    ACTIONS(293), 1,
      anon_sym_RPAREN,
  [2854] = 1,
    ACTIONS(295), 1,
      anon_sym_DQUOTE2,
  [2858] = 1,
    ACTIONS(297), 1,
      anon_sym_EQ,
  [2862] = 1,
    ACTIONS(299), 1,
      sym_string_lit_content,
  [2866] = 1,
    ACTIONS(301), 1,
      anon_sym_EQ,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 38,
  [SMALL_STATE(4)] = 72,
  [SMALL_STATE(5)] = 104,
  [SMALL_STATE(6)] = 136,
  [SMALL_STATE(7)] = 184,
  [SMALL_STATE(8)] = 216,
  [SMALL_STATE(9)] = 262,
  [SMALL_STATE(10)] = 304,
  [SMALL_STATE(11)] = 338,
  [SMALL_STATE(12)] = 374,
  [SMALL_STATE(13)] = 414,
  [SMALL_STATE(14)] = 446,
  [SMALL_STATE(15)] = 478,
  [SMALL_STATE(16)] = 524,
  [SMALL_STATE(17)] = 586,
  [SMALL_STATE(18)] = 618,
  [SMALL_STATE(19)] = 664,
  [SMALL_STATE(20)] = 696,
  [SMALL_STATE(21)] = 728,
  [SMALL_STATE(22)] = 760,
  [SMALL_STATE(23)] = 792,
  [SMALL_STATE(24)] = 824,
  [SMALL_STATE(25)] = 856,
  [SMALL_STATE(26)] = 888,
  [SMALL_STATE(27)] = 947,
  [SMALL_STATE(28)] = 1006,
  [SMALL_STATE(29)] = 1065,
  [SMALL_STATE(30)] = 1124,
  [SMALL_STATE(31)] = 1183,
  [SMALL_STATE(32)] = 1239,
  [SMALL_STATE(33)] = 1292,
  [SMALL_STATE(34)] = 1345,
  [SMALL_STATE(35)] = 1398,
  [SMALL_STATE(36)] = 1451,
  [SMALL_STATE(37)] = 1504,
  [SMALL_STATE(38)] = 1557,
  [SMALL_STATE(39)] = 1610,
  [SMALL_STATE(40)] = 1663,
  [SMALL_STATE(41)] = 1716,
  [SMALL_STATE(42)] = 1769,
  [SMALL_STATE(43)] = 1822,
  [SMALL_STATE(44)] = 1875,
  [SMALL_STATE(45)] = 1928,
  [SMALL_STATE(46)] = 1981,
  [SMALL_STATE(47)] = 2034,
  [SMALL_STATE(48)] = 2087,
  [SMALL_STATE(49)] = 2140,
  [SMALL_STATE(50)] = 2183,
  [SMALL_STATE(51)] = 2226,
  [SMALL_STATE(52)] = 2266,
  [SMALL_STATE(53)] = 2306,
  [SMALL_STATE(54)] = 2338,
  [SMALL_STATE(55)] = 2370,
  [SMALL_STATE(56)] = 2402,
  [SMALL_STATE(57)] = 2434,
  [SMALL_STATE(58)] = 2466,
  [SMALL_STATE(59)] = 2498,
  [SMALL_STATE(60)] = 2529,
  [SMALL_STATE(61)] = 2560,
  [SMALL_STATE(62)] = 2574,
  [SMALL_STATE(63)] = 2588,
  [SMALL_STATE(64)] = 2604,
  [SMALL_STATE(65)] = 2620,
  [SMALL_STATE(66)] = 2637,
  [SMALL_STATE(67)] = 2646,
  [SMALL_STATE(68)] = 2655,
  [SMALL_STATE(69)] = 2672,
  [SMALL_STATE(70)] = 2689,
  [SMALL_STATE(71)] = 2701,
  [SMALL_STATE(72)] = 2713,
  [SMALL_STATE(73)] = 2725,
  [SMALL_STATE(74)] = 2737,
  [SMALL_STATE(75)] = 2749,
  [SMALL_STATE(76)] = 2761,
  [SMALL_STATE(77)] = 2770,
  [SMALL_STATE(78)] = 2778,
  [SMALL_STATE(79)] = 2784,
  [SMALL_STATE(80)] = 2794,
  [SMALL_STATE(81)] = 2801,
  [SMALL_STATE(82)] = 2808,
  [SMALL_STATE(83)] = 2815,
  [SMALL_STATE(84)] = 2820,
  [SMALL_STATE(85)] = 2827,
  [SMALL_STATE(86)] = 2834,
  [SMALL_STATE(87)] = 2838,
  [SMALL_STATE(88)] = 2842,
  [SMALL_STATE(89)] = 2846,
  [SMALL_STATE(90)] = 2850,
  [SMALL_STATE(91)] = 2854,
  [SMALL_STATE(92)] = 2858,
  [SMALL_STATE(93)] = 2862,
  [SMALL_STATE(94)] = 2866,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_root, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 1),
  [9] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 1),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__pat, 1),
  [13] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__pat, 1),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ident, 1),
  [17] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ident, 1),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_lit, 3, .production_id = 14),
  [21] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_lit, 3, .production_id = 14),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_lit, 3, .production_id = 13),
  [25] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_lit, 3, .production_id = 13),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if, 4, .production_id = 24),
  [29] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if, 4, .production_id = 24),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(44),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [45] = {.entry = {.count = 1, .reusable = false}}, SHIFT(45),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 3, .production_id = 21),
  [49] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 3, .production_id = 21),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 3, .production_id = 20),
  [53] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 3, .production_id = 20),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bin_op, 3, .production_id = 19),
  [57] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bin_op, 3, .production_id = 19),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_lit, 2),
  [61] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_lit, 2),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 3, .production_id = 16),
  [65] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 3, .production_id = 16),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if, 6, .production_id = 31),
  [69] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if, 6, .production_id = 31),
  [71] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [81] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [83] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [85] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 4, .production_id = 25),
  [91] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 4, .production_id = 25),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 2, .production_id = 7),
  [95] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 2, .production_id = 7),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 3),
  [99] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 3),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_true, 1),
  [103] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_true, 1),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_false, 1),
  [107] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_false, 1),
  [109] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1),
  [111] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 1),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 2, .production_id = 8),
  [115] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 2, .production_id = 8),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 1, .production_id = 4),
  [119] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 1, .production_id = 4),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block, 1, .production_id = 4),
  [123] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block, 1, .production_id = 4),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [127] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 15), SHIFT_REPEAT(3),
  [130] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 15), SHIFT_REPEAT(42),
  [133] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 15), SHIFT_REPEAT(93),
  [136] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 15), SHIFT_REPEAT(26),
  [139] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 15),
  [141] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 15), SHIFT_REPEAT(33),
  [144] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 15), SHIFT_REPEAT(20),
  [147] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 15), SHIFT_REPEAT(21),
  [150] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 15), SHIFT_REPEAT(22),
  [153] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [155] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 26), SHIFT_REPEAT(3),
  [158] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 26), SHIFT_REPEAT(42),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 26),
  [163] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 26), SHIFT_REPEAT(93),
  [166] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 26), SHIFT_REPEAT(26),
  [169] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 26), SHIFT_REPEAT(33),
  [172] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 26), SHIFT_REPEAT(20),
  [175] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 26), SHIFT_REPEAT(21),
  [178] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 26), SHIFT_REPEAT(22),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__call_args_list, 1, .production_id = 18),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [185] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 1, .production_id = 17),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 1, .production_id = 17),
  [191] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 1, .production_id = 6),
  [193] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 1, .production_id = 6),
  [197] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_var_decl, 3, .production_id = 22),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_decl, 3, .production_id = 22),
  [201] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_var_decl, 5, .production_id = 29),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_decl, 5, .production_id = 29),
  [205] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_decl, 8, .production_id = 30),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_decl, 5, .production_id = 9),
  [209] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_decl, 6, .production_id = 23),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_var_decl, 5, .production_id = 12),
  [213] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_var_decl, 3, .production_id = 3),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_decl, 7, .production_id = 27),
  [217] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [219] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [221] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 6),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 6),
  [225] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 17),
  [227] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 17),
  [229] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_root_repeat1, 2),
  [231] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_root_repeat1, 2), SHIFT_REPEAT(3),
  [234] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_root, 1),
  [236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [238] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 3, .production_id = 11),
  [240] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 5, .production_id = 28),
  [242] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_func_decl_repeat1, 2, .production_id = 5), SHIFT_REPEAT(3),
  [245] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_decl_repeat1, 2, .production_id = 5),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [251] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_param, 1, .production_id = 2),
  [253] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [255] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_decl_repeat1, 1, .production_id = 1),
  [257] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [259] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_param, 3, .production_id = 10),
  [261] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [263] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [265] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [267] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [269] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [271] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [273] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [275] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [277] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [279] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_decl_repeat1, 2, .production_id = 1),
  [281] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [283] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [285] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [287] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [289] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [291] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [293] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [295] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [297] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [299] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [301] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
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
