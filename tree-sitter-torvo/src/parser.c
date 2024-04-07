#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 109
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 42
#define ALIAS_COUNT 0
#define TOKEN_COUNT 18
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 15
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 28

enum ts_symbol_identifiers {
  sym__ident = 1,
  anon_sym_fn = 2,
  anon_sym_LPAREN = 3,
  anon_sym_COMMA = 4,
  anon_sym_RPAREN = 5,
  anon_sym_COLON = 6,
  anon_sym_EQ = 7,
  anon_sym_LBRACK = 8,
  anon_sym_RBRACK = 9,
  anon_sym_SEMI = 10,
  sym_plus = 11,
  sym_minus = 12,
  sym_star = 13,
  sym_double_star = 14,
  sym_slash = 15,
  sym_percent = 16,
  aux_sym_number_token1 = 17,
  sym_root = 18,
  sym__module_stmt = 19,
  sym_fn_decl = 20,
  sym_fn_param = 21,
  sym_global_var_decl = 22,
  sym_var_decl = 23,
  sym__expr = 24,
  sym_bin_op = 25,
  sym_call = 26,
  sym__call_args_list = 27,
  sym_array_lit = 28,
  sym_block = 29,
  sym__block = 30,
  sym__block_clause = 31,
  sym__block_stmt = 32,
  sym__type_expr = 33,
  sym_array_type = 34,
  sym__pat = 35,
  sym_ident = 36,
  sym_number = 37,
  aux_sym_root_repeat1 = 38,
  aux_sym_fn_decl_repeat1 = 39,
  aux_sym__call_args_list_repeat1 = 40,
  aux_sym_array_lit_repeat1 = 41,
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
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_SEMI] = ";",
  [sym_plus] = "plus",
  [sym_minus] = "minus",
  [sym_star] = "star",
  [sym_double_star] = "double_star",
  [sym_slash] = "slash",
  [sym_percent] = "percent",
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
  [sym_array_lit] = "array_lit",
  [sym_block] = "block",
  [sym__block] = "_block",
  [sym__block_clause] = "_block_clause",
  [sym__block_stmt] = "_block_stmt",
  [sym__type_expr] = "_type_expr",
  [sym_array_type] = "array_type",
  [sym__pat] = "_pat",
  [sym_ident] = "ident",
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
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [sym_plus] = sym_plus,
  [sym_minus] = sym_minus,
  [sym_star] = sym_star,
  [sym_double_star] = sym_double_star,
  [sym_slash] = sym_slash,
  [sym_percent] = sym_percent,
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
  [sym_array_lit] = sym_array_lit,
  [sym_block] = sym_block,
  [sym__block] = sym__block,
  [sym__block_clause] = sym__block_clause,
  [sym__block_stmt] = sym__block_stmt,
  [sym__type_expr] = sym__type_expr,
  [sym_array_type] = sym_array_type,
  [sym__pat] = sym__pat,
  [sym_ident] = sym_ident,
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
  field_item_type = 4,
  field_items = 5,
  field_left = 6,
  field_name = 7,
  field_op = 8,
  field_params = 9,
  field_pat = 10,
  field_ret_type = 11,
  field_return = 12,
  field_right = 13,
  field_type = 14,
  field_value = 15,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_args] = "args",
  [field_body] = "body",
  [field_callee] = "callee",
  [field_item_type] = "item_type",
  [field_items] = "items",
  [field_left] = "left",
  [field_name] = "name",
  [field_op] = "op",
  [field_params] = "params",
  [field_pat] = "pat",
  [field_ret_type] = "ret_type",
  [field_return] = "return",
  [field_right] = "right",
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
  [12] = {.index = 19, .length = 2},
  [13] = {.index = 21, .length = 1},
  [14] = {.index = 22, .length = 1},
  [15] = {.index = 23, .length = 1},
  [16] = {.index = 24, .length = 3},
  [17] = {.index = 27, .length = 2},
  [18] = {.index = 29, .length = 3},
  [19] = {.index = 32, .length = 2},
  [20] = {.index = 34, .length = 2},
  [21] = {.index = 36, .length = 2},
  [22] = {.index = 38, .length = 2},
  [23] = {.index = 40, .length = 2},
  [24] = {.index = 42, .length = 3},
  [25] = {.index = 45, .length = 3},
  [26] = {.index = 48, .length = 3},
  [27] = {.index = 51, .length = 4},
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
    {field_items, 1, .inherited = true},
  [19] =
    {field_items, 0, .inherited = true},
    {field_items, 1, .inherited = true},
  [21] =
    {field_callee, 0},
  [22] =
    {field_args, 0},
  [23] =
    {field_args, 0, .inherited = true},
  [24] =
    {field_left, 0},
    {field_op, 1},
    {field_right, 2},
  [27] =
    {field_body, 0},
    {field_value, 2},
  [29] =
    {field_body, 0},
    {field_body, 2, .inherited = true},
    {field_value, 2, .inherited = true},
  [32] =
    {field_pat, 0},
    {field_value, 2},
  [34] =
    {field_name, 1},
    {field_return, 5},
  [36] =
    {field_pat, 0},
    {field_type, 2},
  [38] =
    {field_args, 2, .inherited = true},
    {field_callee, 0},
  [40] =
    {field_args, 0, .inherited = true},
    {field_args, 1, .inherited = true},
  [42] =
    {field_name, 1},
    {field_params, 3, .inherited = true},
    {field_return, 6},
  [45] =
    {field_pat, 0},
    {field_type, 2},
    {field_value, 4},
  [48] =
    {field_name, 1},
    {field_ret_type, 5},
    {field_return, 7},
  [51] =
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
  [7] = 4,
  [8] = 8,
  [9] = 9,
  [10] = 6,
  [11] = 11,
  [12] = 11,
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
  [23] = 22,
  [24] = 13,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 16,
  [31] = 25,
  [32] = 26,
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
  [48] = 21,
  [49] = 29,
  [50] = 50,
  [51] = 51,
  [52] = 47,
  [53] = 38,
  [54] = 43,
  [55] = 44,
  [56] = 37,
  [57] = 35,
  [58] = 36,
  [59] = 46,
  [60] = 40,
  [61] = 34,
  [62] = 33,
  [63] = 45,
  [64] = 64,
  [65] = 39,
  [66] = 41,
  [67] = 42,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 76,
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
  [95] = 95,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 100,
  [108] = 108,
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
      if (eof) ADVANCE(3);
      if (lookahead == '%') ADVANCE(17);
      if (lookahead == '(') ADVANCE(4);
      if (lookahead == ')') ADVANCE(6);
      if (lookahead == '*') ADVANCE(14);
      if (lookahead == '+') ADVANCE(12);
      if (lookahead == ',') ADVANCE(5);
      if (lookahead == '-') ADVANCE(13);
      if (lookahead == '.') ADVANCE(2);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == ':') ADVANCE(7);
      if (lookahead == ';') ADVANCE(11);
      if (lookahead == '=') ADVANCE(8);
      if (lookahead == '[') ADVANCE(9);
      if (lookahead == ']') ADVANCE(10);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(19);
      if (sym__ident_character_set_1(lookahead)) ADVANCE(18);
      END_STATE();
    case 1:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(19);
      END_STATE();
    case 2:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(20);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym_plus);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_minus);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_star);
      if (lookahead == '*') ADVANCE(15);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_double_star);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym_slash);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym_percent);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym__ident);
      if (sym__ident_character_set_2(lookahead)) ADVANCE(18);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '.') ADVANCE(2);
      if (lookahead == '_') ADVANCE(1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(19);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '_') ADVANCE(2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(20);
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
      if (lookahead == 'f') ADVANCE(1);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == 'n') ADVANCE(2);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(anon_sym_fn);
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
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
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
  [52] = {.lex_state = 0},
  [53] = {.lex_state = 0},
  [54] = {.lex_state = 0},
  [55] = {.lex_state = 0},
  [56] = {.lex_state = 0},
  [57] = {.lex_state = 0},
  [58] = {.lex_state = 0},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 0},
  [62] = {.lex_state = 0},
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
  [91] = {.lex_state = 0},
  [92] = {.lex_state = 0},
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
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [sym_plus] = ACTIONS(1),
    [sym_minus] = ACTIONS(1),
    [sym_star] = ACTIONS(1),
    [sym_double_star] = ACTIONS(1),
    [sym_slash] = ACTIONS(1),
    [sym_percent] = ACTIONS(1),
    [aux_sym_number_token1] = ACTIONS(1),
  },
  [1] = {
    [sym_root] = STATE(108),
    [sym__module_stmt] = STATE(75),
    [sym_fn_decl] = STATE(75),
    [sym_global_var_decl] = STATE(75),
    [sym_ident] = STATE(99),
    [aux_sym_root_repeat1] = STATE(75),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym__ident] = ACTIONS(5),
    [anon_sym_fn] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 13,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(13), 1,
      anon_sym_RPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    STATE(5), 1,
      aux_sym__call_args_list_repeat1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(107), 1,
      sym__call_args_list,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(50), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [46] = 13,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    ACTIONS(19), 1,
      anon_sym_RPAREN,
    STATE(5), 1,
      aux_sym__call_args_list_repeat1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(100), 1,
      sym__call_args_list,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(50), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [92] = 12,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    ACTIONS(21), 1,
      anon_sym_RBRACK,
    STATE(10), 1,
      aux_sym_array_lit_repeat1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(51), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [135] = 12,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    ACTIONS(23), 1,
      anon_sym_RPAREN,
    STATE(8), 1,
      aux_sym__call_args_list_repeat1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(50), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [178] = 12,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    ACTIONS(25), 1,
      anon_sym_RBRACK,
    STATE(9), 1,
      aux_sym_array_lit_repeat1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(51), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [221] = 12,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    ACTIONS(27), 1,
      anon_sym_RBRACK,
    STATE(6), 1,
      aux_sym_array_lit_repeat1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(51), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [264] = 12,
    ACTIONS(29), 1,
      sym__ident,
    ACTIONS(32), 1,
      anon_sym_LPAREN,
    ACTIONS(35), 1,
      anon_sym_RPAREN,
    ACTIONS(37), 1,
      anon_sym_LBRACK,
    ACTIONS(40), 1,
      aux_sym_number_token1,
    STATE(8), 1,
      aux_sym__call_args_list_repeat1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(50), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [307] = 12,
    ACTIONS(43), 1,
      sym__ident,
    ACTIONS(46), 1,
      anon_sym_LPAREN,
    ACTIONS(49), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_RBRACK,
    ACTIONS(54), 1,
      aux_sym_number_token1,
    STATE(9), 1,
      aux_sym_array_lit_repeat1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(51), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [350] = 12,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    ACTIONS(57), 1,
      anon_sym_RBRACK,
    STATE(9), 1,
      aux_sym_array_lit_repeat1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(51), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [393] = 11,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    ACTIONS(59), 1,
      anon_sym_SEMI,
    STATE(29), 1,
      sym_ident,
    STATE(37), 1,
      sym__block,
    STATE(41), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(39), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [433] = 11,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(67), 1,
      anon_sym_SEMI,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(66), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(65), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [473] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(61), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [510] = 10,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(64), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [547] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(73), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [584] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(77), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [621] = 10,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(68), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [658] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(74), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [695] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(70), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [732] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(69), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [769] = 2,
    ACTIONS(73), 1,
      sym_star,
    ACTIONS(71), 15,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [790] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(63), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(62), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [827] = 10,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    STATE(29), 1,
      sym_ident,
    STATE(37), 1,
      sym__block,
    STATE(45), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(33), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [864] = 10,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(34), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [901] = 10,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(40), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [938] = 10,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_LBRACK,
    ACTIONS(17), 1,
      aux_sym_number_token1,
    STATE(29), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(37), 1,
      sym__block,
    STATE(95), 1,
      sym__pat,
    STATE(11), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(46), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [975] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(71), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [1012] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(72), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [1049] = 3,
    ACTIONS(79), 1,
      sym_star,
    ACTIONS(77), 2,
      anon_sym_COLON,
      anon_sym_EQ,
    ACTIONS(75), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1072] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(76), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [1109] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(60), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [1146] = 10,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(63), 1,
      anon_sym_LPAREN,
    ACTIONS(65), 1,
      anon_sym_LBRACK,
    ACTIONS(69), 1,
      aux_sym_number_token1,
    STATE(49), 1,
      sym_ident,
    STATE(56), 1,
      sym__block,
    STATE(57), 1,
      sym__block_clause,
    STATE(95), 1,
      sym__pat,
    STATE(12), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(59), 6,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_array_lit,
      sym_block,
      sym_number,
  [1183] = 6,
    ACTIONS(83), 1,
      anon_sym_LPAREN,
    ACTIONS(87), 1,
      sym_star,
    ACTIONS(89), 1,
      sym_double_star,
    ACTIONS(85), 2,
      sym_plus,
      sym_minus,
    ACTIONS(91), 2,
      sym_slash,
      sym_percent,
    ACTIONS(81), 7,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym__ident,
      aux_sym_number_token1,
  [1210] = 3,
    ACTIONS(83), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 1,
      sym_star,
    ACTIONS(93), 12,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1231] = 2,
    ACTIONS(99), 1,
      sym_star,
    ACTIONS(97), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1250] = 2,
    ACTIONS(103), 1,
      sym_star,
    ACTIONS(101), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1269] = 2,
    ACTIONS(107), 1,
      sym_star,
    ACTIONS(105), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1288] = 2,
    ACTIONS(111), 1,
      sym_star,
    ACTIONS(109), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1307] = 6,
    ACTIONS(83), 1,
      anon_sym_LPAREN,
    ACTIONS(87), 1,
      sym_star,
    ACTIONS(89), 1,
      sym_double_star,
    ACTIONS(85), 2,
      sym_plus,
      sym_minus,
    ACTIONS(91), 2,
      sym_slash,
      sym_percent,
    ACTIONS(113), 7,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym__ident,
      aux_sym_number_token1,
  [1334] = 4,
    ACTIONS(83), 1,
      anon_sym_LPAREN,
    ACTIONS(89), 1,
      sym_double_star,
    ACTIONS(95), 1,
      sym_star,
    ACTIONS(93), 11,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1357] = 2,
    ACTIONS(117), 1,
      sym_star,
    ACTIONS(115), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1376] = 2,
    ACTIONS(121), 1,
      sym_star,
    ACTIONS(119), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1395] = 2,
    ACTIONS(125), 1,
      sym_star,
    ACTIONS(123), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1414] = 2,
    ACTIONS(129), 1,
      sym_star,
    ACTIONS(127), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1433] = 2,
    ACTIONS(133), 1,
      sym_star,
    ACTIONS(131), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1452] = 5,
    ACTIONS(83), 1,
      anon_sym_LPAREN,
    ACTIONS(87), 1,
      sym_star,
    ACTIONS(89), 1,
      sym_double_star,
    ACTIONS(91), 2,
      sym_slash,
      sym_percent,
    ACTIONS(93), 9,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym__ident,
      aux_sym_number_token1,
  [1477] = 2,
    ACTIONS(137), 1,
      sym_star,
    ACTIONS(135), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1496] = 2,
    ACTIONS(73), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(71), 11,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_RBRACK,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1515] = 3,
    ACTIONS(77), 2,
      anon_sym_COLON,
      anon_sym_EQ,
    ACTIONS(79), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(75), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1535] = 7,
    ACTIONS(83), 1,
      anon_sym_LPAREN,
    ACTIONS(87), 1,
      sym_star,
    ACTIONS(89), 1,
      sym_double_star,
    ACTIONS(141), 1,
      anon_sym_COMMA,
    ACTIONS(85), 2,
      sym_plus,
      sym_minus,
    ACTIONS(91), 2,
      sym_slash,
      sym_percent,
    ACTIONS(139), 4,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      sym__ident,
      aux_sym_number_token1,
  [1562] = 7,
    ACTIONS(83), 1,
      anon_sym_LPAREN,
    ACTIONS(87), 1,
      sym_star,
    ACTIONS(89), 1,
      sym_double_star,
    ACTIONS(145), 1,
      anon_sym_COMMA,
    ACTIONS(85), 2,
      sym_plus,
      sym_minus,
    ACTIONS(91), 2,
      sym_slash,
      sym_percent,
    ACTIONS(143), 4,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      sym__ident,
      aux_sym_number_token1,
  [1589] = 2,
    ACTIONS(137), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(135), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1605] = 2,
    ACTIONS(111), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(109), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1621] = 2,
    ACTIONS(125), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(123), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1637] = 2,
    ACTIONS(129), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(127), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1653] = 2,
    ACTIONS(107), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(105), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1669] = 2,
    ACTIONS(99), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(97), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1685] = 2,
    ACTIONS(103), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(101), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1701] = 6,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(95), 2,
      anon_sym_fn,
      sym__ident,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(93), 4,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
  [1725] = 4,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(95), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(93), 6,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_slash,
      sym_percent,
  [1745] = 3,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(95), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(93), 7,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1763] = 7,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(81), 2,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(155), 2,
      anon_sym_fn,
      sym__ident,
    ACTIONS(157), 2,
      sym_plus,
      sym_minus,
  [1789] = 2,
    ACTIONS(133), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(131), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1805] = 6,
    ACTIONS(83), 1,
      anon_sym_LPAREN,
    ACTIONS(87), 1,
      sym_star,
    ACTIONS(89), 1,
      sym_double_star,
    ACTIONS(85), 2,
      sym_plus,
      sym_minus,
    ACTIONS(91), 2,
      sym_slash,
      sym_percent,
    ACTIONS(159), 4,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      sym__ident,
      aux_sym_number_token1,
  [1829] = 7,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(113), 2,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(157), 2,
      sym_plus,
      sym_minus,
    ACTIONS(161), 2,
      anon_sym_fn,
      sym__ident,
  [1855] = 2,
    ACTIONS(117), 3,
      anon_sym_fn,
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
  [1871] = 2,
    ACTIONS(121), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(119), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1887] = 6,
    ACTIONS(83), 1,
      anon_sym_LPAREN,
    ACTIONS(87), 1,
      sym_star,
    ACTIONS(89), 1,
      sym_double_star,
    ACTIONS(85), 2,
      sym_plus,
      sym_minus,
    ACTIONS(91), 2,
      sym_slash,
      sym_percent,
    ACTIONS(163), 4,
      anon_sym_LBRACK,
      anon_sym_SEMI,
      sym__ident,
      aux_sym_number_token1,
  [1911] = 7,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(165), 1,
      ts_builtin_sym_end,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(157), 2,
      sym_plus,
      sym_minus,
    ACTIONS(167), 2,
      anon_sym_fn,
      sym__ident,
  [1936] = 7,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(169), 1,
      ts_builtin_sym_end,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(157), 2,
      sym_plus,
      sym_minus,
    ACTIONS(171), 2,
      anon_sym_fn,
      sym__ident,
  [1961] = 7,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(173), 1,
      ts_builtin_sym_end,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(157), 2,
      sym_plus,
      sym_minus,
    ACTIONS(175), 2,
      anon_sym_fn,
      sym__ident,
  [1986] = 7,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(177), 1,
      ts_builtin_sym_end,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(157), 2,
      sym_plus,
      sym_minus,
    ACTIONS(179), 2,
      anon_sym_fn,
      sym__ident,
  [2011] = 7,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(181), 1,
      ts_builtin_sym_end,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(157), 2,
      sym_plus,
      sym_minus,
    ACTIONS(183), 2,
      anon_sym_fn,
      sym__ident,
  [2036] = 7,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(185), 1,
      ts_builtin_sym_end,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(157), 2,
      sym_plus,
      sym_minus,
    ACTIONS(187), 2,
      anon_sym_fn,
      sym__ident,
  [2061] = 5,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(7), 1,
      anon_sym_fn,
    ACTIONS(189), 1,
      ts_builtin_sym_end,
    STATE(99), 1,
      sym_ident,
    STATE(78), 4,
      sym__module_stmt,
      sym_fn_decl,
      sym_global_var_decl,
      aux_sym_root_repeat1,
  [2080] = 6,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(191), 1,
      anon_sym_RPAREN,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(157), 2,
      sym_plus,
      sym_minus,
  [2101] = 6,
    ACTIONS(147), 1,
      anon_sym_LPAREN,
    ACTIONS(149), 1,
      sym_star,
    ACTIONS(151), 1,
      sym_double_star,
    ACTIONS(193), 1,
      anon_sym_RPAREN,
    ACTIONS(153), 2,
      sym_slash,
      sym_percent,
    ACTIONS(157), 2,
      sym_plus,
      sym_minus,
  [2122] = 5,
    ACTIONS(195), 1,
      ts_builtin_sym_end,
    ACTIONS(197), 1,
      sym__ident,
    ACTIONS(200), 1,
      anon_sym_fn,
    STATE(99), 1,
      sym_ident,
    STATE(78), 4,
      sym__module_stmt,
      sym_fn_decl,
      sym_global_var_decl,
      aux_sym_root_repeat1,
  [2141] = 5,
    ACTIONS(203), 1,
      sym__ident,
    ACTIONS(206), 1,
      anon_sym_RPAREN,
    STATE(79), 1,
      aux_sym_fn_decl_repeat1,
    STATE(93), 1,
      sym_fn_param,
    STATE(91), 2,
      sym__pat,
      sym_ident,
  [2158] = 5,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(208), 1,
      anon_sym_RPAREN,
    STATE(79), 1,
      aux_sym_fn_decl_repeat1,
    STATE(93), 1,
      sym_fn_param,
    STATE(91), 2,
      sym__pat,
      sym_ident,
  [2175] = 5,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(210), 1,
      anon_sym_RPAREN,
    STATE(80), 1,
      aux_sym_fn_decl_repeat1,
    STATE(93), 1,
      sym_fn_param,
    STATE(91), 2,
      sym__pat,
      sym_ident,
  [2192] = 3,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(212), 1,
      anon_sym_LBRACK,
    STATE(103), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2204] = 1,
    ACTIONS(214), 5,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_RBRACK,
      sym__ident,
  [2212] = 1,
    ACTIONS(216), 5,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_LBRACK,
      sym__ident,
      aux_sym_number_token1,
  [2220] = 3,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(212), 1,
      anon_sym_LBRACK,
    STATE(104), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2232] = 3,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(212), 1,
      anon_sym_LBRACK,
    STATE(106), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2244] = 3,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(212), 1,
      anon_sym_LBRACK,
    STATE(102), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2256] = 3,
    ACTIONS(61), 1,
      sym__ident,
    ACTIONS(212), 1,
      anon_sym_LBRACK,
    STATE(101), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2268] = 1,
    ACTIONS(218), 5,
      anon_sym_LPAREN,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      sym__ident,
      aux_sym_number_token1,
  [2276] = 3,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(212), 1,
      anon_sym_LBRACK,
    STATE(92), 3,
      sym__type_expr,
      sym_array_type,
      sym_ident,
  [2288] = 2,
    ACTIONS(222), 1,
      anon_sym_COLON,
    ACTIONS(220), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      sym__ident,
  [2297] = 1,
    ACTIONS(224), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      sym__ident,
  [2303] = 2,
    ACTIONS(228), 1,
      anon_sym_COMMA,
    ACTIONS(226), 2,
      anon_sym_RPAREN,
      sym__ident,
  [2311] = 2,
    ACTIONS(61), 1,
      sym__ident,
    STATE(105), 1,
      sym_ident,
  [2318] = 2,
    ACTIONS(230), 1,
      anon_sym_COLON,
    ACTIONS(232), 1,
      anon_sym_EQ,
  [2325] = 2,
    ACTIONS(234), 1,
      anon_sym_COLON,
    ACTIONS(236), 1,
      anon_sym_EQ,
  [2332] = 2,
    ACTIONS(238), 1,
      anon_sym_COLON,
    ACTIONS(240), 1,
      anon_sym_EQ,
  [2339] = 1,
    ACTIONS(242), 2,
      anon_sym_RPAREN,
      sym__ident,
  [2344] = 2,
    ACTIONS(244), 1,
      anon_sym_COLON,
    ACTIONS(246), 1,
      anon_sym_EQ,
  [2351] = 1,
    ACTIONS(248), 1,
      anon_sym_RPAREN,
  [2355] = 1,
    ACTIONS(250), 1,
      anon_sym_EQ,
  [2359] = 1,
    ACTIONS(252), 1,
      anon_sym_RBRACK,
  [2363] = 1,
    ACTIONS(254), 1,
      anon_sym_EQ,
  [2367] = 1,
    ACTIONS(256), 1,
      anon_sym_EQ,
  [2371] = 1,
    ACTIONS(258), 1,
      anon_sym_LPAREN,
  [2375] = 1,
    ACTIONS(260), 1,
      anon_sym_EQ,
  [2379] = 1,
    ACTIONS(262), 1,
      anon_sym_RPAREN,
  [2383] = 1,
    ACTIONS(264), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 46,
  [SMALL_STATE(4)] = 92,
  [SMALL_STATE(5)] = 135,
  [SMALL_STATE(6)] = 178,
  [SMALL_STATE(7)] = 221,
  [SMALL_STATE(8)] = 264,
  [SMALL_STATE(9)] = 307,
  [SMALL_STATE(10)] = 350,
  [SMALL_STATE(11)] = 393,
  [SMALL_STATE(12)] = 433,
  [SMALL_STATE(13)] = 473,
  [SMALL_STATE(14)] = 510,
  [SMALL_STATE(15)] = 547,
  [SMALL_STATE(16)] = 584,
  [SMALL_STATE(17)] = 621,
  [SMALL_STATE(18)] = 658,
  [SMALL_STATE(19)] = 695,
  [SMALL_STATE(20)] = 732,
  [SMALL_STATE(21)] = 769,
  [SMALL_STATE(22)] = 790,
  [SMALL_STATE(23)] = 827,
  [SMALL_STATE(24)] = 864,
  [SMALL_STATE(25)] = 901,
  [SMALL_STATE(26)] = 938,
  [SMALL_STATE(27)] = 975,
  [SMALL_STATE(28)] = 1012,
  [SMALL_STATE(29)] = 1049,
  [SMALL_STATE(30)] = 1072,
  [SMALL_STATE(31)] = 1109,
  [SMALL_STATE(32)] = 1146,
  [SMALL_STATE(33)] = 1183,
  [SMALL_STATE(34)] = 1210,
  [SMALL_STATE(35)] = 1231,
  [SMALL_STATE(36)] = 1250,
  [SMALL_STATE(37)] = 1269,
  [SMALL_STATE(38)] = 1288,
  [SMALL_STATE(39)] = 1307,
  [SMALL_STATE(40)] = 1334,
  [SMALL_STATE(41)] = 1357,
  [SMALL_STATE(42)] = 1376,
  [SMALL_STATE(43)] = 1395,
  [SMALL_STATE(44)] = 1414,
  [SMALL_STATE(45)] = 1433,
  [SMALL_STATE(46)] = 1452,
  [SMALL_STATE(47)] = 1477,
  [SMALL_STATE(48)] = 1496,
  [SMALL_STATE(49)] = 1515,
  [SMALL_STATE(50)] = 1535,
  [SMALL_STATE(51)] = 1562,
  [SMALL_STATE(52)] = 1589,
  [SMALL_STATE(53)] = 1605,
  [SMALL_STATE(54)] = 1621,
  [SMALL_STATE(55)] = 1637,
  [SMALL_STATE(56)] = 1653,
  [SMALL_STATE(57)] = 1669,
  [SMALL_STATE(58)] = 1685,
  [SMALL_STATE(59)] = 1701,
  [SMALL_STATE(60)] = 1725,
  [SMALL_STATE(61)] = 1745,
  [SMALL_STATE(62)] = 1763,
  [SMALL_STATE(63)] = 1789,
  [SMALL_STATE(64)] = 1805,
  [SMALL_STATE(65)] = 1829,
  [SMALL_STATE(66)] = 1855,
  [SMALL_STATE(67)] = 1871,
  [SMALL_STATE(68)] = 1887,
  [SMALL_STATE(69)] = 1911,
  [SMALL_STATE(70)] = 1936,
  [SMALL_STATE(71)] = 1961,
  [SMALL_STATE(72)] = 1986,
  [SMALL_STATE(73)] = 2011,
  [SMALL_STATE(74)] = 2036,
  [SMALL_STATE(75)] = 2061,
  [SMALL_STATE(76)] = 2080,
  [SMALL_STATE(77)] = 2101,
  [SMALL_STATE(78)] = 2122,
  [SMALL_STATE(79)] = 2141,
  [SMALL_STATE(80)] = 2158,
  [SMALL_STATE(81)] = 2175,
  [SMALL_STATE(82)] = 2192,
  [SMALL_STATE(83)] = 2204,
  [SMALL_STATE(84)] = 2212,
  [SMALL_STATE(85)] = 2220,
  [SMALL_STATE(86)] = 2232,
  [SMALL_STATE(87)] = 2244,
  [SMALL_STATE(88)] = 2256,
  [SMALL_STATE(89)] = 2268,
  [SMALL_STATE(90)] = 2276,
  [SMALL_STATE(91)] = 2288,
  [SMALL_STATE(92)] = 2297,
  [SMALL_STATE(93)] = 2303,
  [SMALL_STATE(94)] = 2311,
  [SMALL_STATE(95)] = 2318,
  [SMALL_STATE(96)] = 2325,
  [SMALL_STATE(97)] = 2332,
  [SMALL_STATE(98)] = 2339,
  [SMALL_STATE(99)] = 2344,
  [SMALL_STATE(100)] = 2351,
  [SMALL_STATE(101)] = 2355,
  [SMALL_STATE(102)] = 2359,
  [SMALL_STATE(103)] = 2363,
  [SMALL_STATE(104)] = 2367,
  [SMALL_STATE(105)] = 2371,
  [SMALL_STATE(106)] = 2375,
  [SMALL_STATE(107)] = 2379,
  [SMALL_STATE(108)] = 2383,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_root, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(48),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(94),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__call_args_list, 1, .production_id = 15),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [29] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 23), SHIFT_REPEAT(21),
  [32] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 23), SHIFT_REPEAT(30),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 23),
  [37] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 23), SHIFT_REPEAT(7),
  [40] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 23), SHIFT_REPEAT(38),
  [43] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 12), SHIFT_REPEAT(21),
  [46] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 12), SHIFT_REPEAT(30),
  [49] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 12), SHIFT_REPEAT(7),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 12),
  [54] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 12), SHIFT_REPEAT(38),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ident, 1),
  [73] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ident, 1),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 1),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__pat, 1),
  [79] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 1),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 3, .production_id = 17),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [87] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bin_op, 3, .production_id = 16),
  [95] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bin_op, 3, .production_id = 16),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block, 1, .production_id = 2),
  [99] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block, 1, .production_id = 2),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_lit, 2),
  [103] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_lit, 2),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 1, .production_id = 2),
  [107] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 1, .production_id = 2),
  [109] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1),
  [111] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 1),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 2, .production_id = 6),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 2, .production_id = 7),
  [117] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 2, .production_id = 7),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 3),
  [121] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 3),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_lit, 3, .production_id = 11),
  [125] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_array_lit, 3, .production_id = 11),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 3, .production_id = 13),
  [129] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 3, .production_id = 13),
  [131] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 3, .production_id = 18),
  [133] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 3, .production_id = 18),
  [135] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 4, .production_id = 22),
  [137] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 4, .production_id = 22),
  [139] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 1, .production_id = 14),
  [141] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 1, .production_id = 5),
  [145] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [147] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [149] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [151] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [153] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [155] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 3, .production_id = 17),
  [157] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_decl, 3, .production_id = 19),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 2, .production_id = 6),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_decl, 5, .production_id = 25),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 9, .production_id = 27),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 9, .production_id = 27),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 6, .production_id = 20),
  [171] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 6, .production_id = 20),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_var_decl, 3, .production_id = 1),
  [175] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_var_decl, 3, .production_id = 1),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_var_decl, 5, .production_id = 10),
  [179] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_var_decl, 5, .production_id = 10),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 7, .production_id = 24),
  [183] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 7, .production_id = 24),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 8, .production_id = 26),
  [187] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 8, .production_id = 26),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_root, 1),
  [191] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [193] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_root_repeat1, 2),
  [197] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_root_repeat1, 2), SHIFT_REPEAT(48),
  [200] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_root_repeat1, 2), SHIFT_REPEAT(94),
  [203] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 2, .production_id = 8), SHIFT_REPEAT(21),
  [206] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 2, .production_id = 8),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [210] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [212] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [214] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type, 3, .production_id = 9),
  [216] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 14),
  [218] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_array_lit_repeat1, 2, .production_id = 5),
  [220] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_param, 1, .production_id = 4),
  [222] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [224] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_param, 3, .production_id = 21),
  [226] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 1, .production_id = 3),
  [228] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [230] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [232] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [234] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [238] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [240] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [242] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 2, .production_id = 3),
  [244] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [254] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [256] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [264] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
