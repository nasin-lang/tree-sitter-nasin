#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 95
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 37
#define ALIAS_COUNT 0
#define TOKEN_COUNT 16
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 13
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 24

enum ts_symbol_identifiers {
  sym__ident = 1,
  anon_sym_fn = 2,
  anon_sym_LPAREN = 3,
  anon_sym_COMMA = 4,
  anon_sym_RPAREN = 5,
  anon_sym_COLON = 6,
  anon_sym_EQ = 7,
  anon_sym_SEMI = 8,
  sym_plus = 9,
  sym_minus = 10,
  sym_star = 11,
  sym_double_star = 12,
  sym_slash = 13,
  sym_percent = 14,
  aux_sym_number_token1 = 15,
  sym_root = 16,
  sym__module_stmt = 17,
  sym_fn_decl = 18,
  sym_fn_param = 19,
  sym_global_var_decl = 20,
  sym_var_decl = 21,
  sym__expr = 22,
  sym_bin_op = 23,
  sym_call = 24,
  sym__call_args_list = 25,
  sym_block = 26,
  sym__block = 27,
  sym__block_clause = 28,
  sym__block_stmt = 29,
  sym__type_expr = 30,
  sym__pat = 31,
  sym_ident = 32,
  sym_number = 33,
  aux_sym_root_repeat1 = 34,
  aux_sym_fn_decl_repeat1 = 35,
  aux_sym__call_args_list_repeat1 = 36,
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
  [sym_block] = "block",
  [sym__block] = "_block",
  [sym__block_clause] = "_block_clause",
  [sym__block_stmt] = "_block_stmt",
  [sym__type_expr] = "_type_expr",
  [sym__pat] = "_pat",
  [sym_ident] = "ident",
  [sym_number] = "number",
  [aux_sym_root_repeat1] = "root_repeat1",
  [aux_sym_fn_decl_repeat1] = "fn_decl_repeat1",
  [aux_sym__call_args_list_repeat1] = "_call_args_list_repeat1",
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
  [sym_block] = sym_block,
  [sym__block] = sym__block,
  [sym__block_clause] = sym__block_clause,
  [sym__block_stmt] = sym__block_stmt,
  [sym__type_expr] = sym__type_expr,
  [sym__pat] = sym__pat,
  [sym_ident] = sym_ident,
  [sym_number] = sym_number,
  [aux_sym_root_repeat1] = aux_sym_root_repeat1,
  [aux_sym_fn_decl_repeat1] = aux_sym_fn_decl_repeat1,
  [aux_sym__call_args_list_repeat1] = aux_sym__call_args_list_repeat1,
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
};

enum ts_field_identifiers {
  field_args = 1,
  field_body = 2,
  field_callee = 3,
  field_left = 4,
  field_name = 5,
  field_op = 6,
  field_params = 7,
  field_pat = 8,
  field_ret_type = 9,
  field_return = 10,
  field_right = 11,
  field_type = 12,
  field_value = 13,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_args] = "args",
  [field_body] = "body",
  [field_callee] = "callee",
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
  [5] = {.index = 6, .length = 2},
  [6] = {.index = 8, .length = 3},
  [7] = {.index = 11, .length = 2},
  [8] = {.index = 13, .length = 3},
  [9] = {.index = 16, .length = 1},
  [10] = {.index = 17, .length = 1},
  [11] = {.index = 18, .length = 1},
  [12] = {.index = 19, .length = 3},
  [13] = {.index = 22, .length = 2},
  [14] = {.index = 24, .length = 3},
  [15] = {.index = 27, .length = 2},
  [16] = {.index = 29, .length = 2},
  [17] = {.index = 31, .length = 2},
  [18] = {.index = 33, .length = 2},
  [19] = {.index = 35, .length = 2},
  [20] = {.index = 37, .length = 3},
  [21] = {.index = 40, .length = 3},
  [22] = {.index = 43, .length = 3},
  [23] = {.index = 46, .length = 4},
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
    {field_body, 0},
    {field_value, 1},
  [8] =
    {field_body, 0},
    {field_body, 1, .inherited = true},
    {field_value, 1, .inherited = true},
  [11] =
    {field_params, 0, .inherited = true},
    {field_params, 1, .inherited = true},
  [13] =
    {field_name, 0},
    {field_type, 2},
    {field_value, 4},
  [16] =
    {field_callee, 0},
  [17] =
    {field_args, 0},
  [18] =
    {field_args, 0, .inherited = true},
  [19] =
    {field_left, 0},
    {field_op, 1},
    {field_right, 2},
  [22] =
    {field_body, 0},
    {field_value, 2},
  [24] =
    {field_body, 0},
    {field_body, 2, .inherited = true},
    {field_value, 2, .inherited = true},
  [27] =
    {field_pat, 0},
    {field_value, 2},
  [29] =
    {field_name, 1},
    {field_return, 5},
  [31] =
    {field_pat, 0},
    {field_type, 2},
  [33] =
    {field_args, 2, .inherited = true},
    {field_callee, 0},
  [35] =
    {field_args, 0, .inherited = true},
    {field_args, 1, .inherited = true},
  [37] =
    {field_name, 1},
    {field_params, 3, .inherited = true},
    {field_return, 6},
  [40] =
    {field_pat, 0},
    {field_type, 2},
    {field_value, 4},
  [43] =
    {field_name, 1},
    {field_ret_type, 5},
    {field_return, 7},
  [46] =
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
  [7] = 6,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 14,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 22,
  [24] = 19,
  [25] = 18,
  [26] = 26,
  [27] = 21,
  [28] = 20,
  [29] = 12,
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
  [43] = 35,
  [44] = 37,
  [45] = 45,
  [46] = 30,
  [47] = 40,
  [48] = 41,
  [49] = 42,
  [50] = 31,
  [51] = 33,
  [52] = 36,
  [53] = 34,
  [54] = 32,
  [55] = 38,
  [56] = 39,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 65,
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
  [93] = 88,
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
      if (eof) ADVANCE(3);
      if (lookahead == '%') ADVANCE(15);
      if (lookahead == '(') ADVANCE(4);
      if (lookahead == ')') ADVANCE(6);
      if (lookahead == '*') ADVANCE(12);
      if (lookahead == '+') ADVANCE(10);
      if (lookahead == ',') ADVANCE(5);
      if (lookahead == '-') ADVANCE(11);
      if (lookahead == '.') ADVANCE(2);
      if (lookahead == '/') ADVANCE(14);
      if (lookahead == ':') ADVANCE(7);
      if (lookahead == ';') ADVANCE(9);
      if (lookahead == '=') ADVANCE(8);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(17);
      if (sym__ident_character_set_1(lookahead)) ADVANCE(16);
      END_STATE();
    case 1:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(17);
      END_STATE();
    case 2:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(18);
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
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_plus);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym_minus);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym_star);
      if (lookahead == '*') ADVANCE(13);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_double_star);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_slash);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_percent);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym__ident);
      if (sym__ident_character_set_2(lookahead)) ADVANCE(16);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '.') ADVANCE(2);
      if (lookahead == '_') ADVANCE(1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(17);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '_') ADVANCE(2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(18);
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
    [sym_root] = STATE(94),
    [sym__module_stmt] = STATE(67),
    [sym_fn_decl] = STATE(67),
    [sym_global_var_decl] = STATE(67),
    [sym_ident] = STATE(86),
    [aux_sym_root_repeat1] = STATE(67),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym__ident] = ACTIONS(5),
    [anon_sym_fn] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 12,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(13), 1,
      anon_sym_RPAREN,
    ACTIONS(15), 1,
      aux_sym_number_token1,
    STATE(5), 1,
      aux_sym__call_args_list_repeat1,
    STATE(20), 1,
      sym_ident,
    STATE(38), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(93), 1,
      sym__call_args_list,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(45), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [42] = 12,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      aux_sym_number_token1,
    ACTIONS(17), 1,
      anon_sym_RPAREN,
    STATE(5), 1,
      aux_sym__call_args_list_repeat1,
    STATE(20), 1,
      sym_ident,
    STATE(38), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(88), 1,
      sym__call_args_list,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(45), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [84] = 11,
    ACTIONS(19), 1,
      sym__ident,
    ACTIONS(22), 1,
      anon_sym_LPAREN,
    ACTIONS(25), 1,
      anon_sym_RPAREN,
    ACTIONS(27), 1,
      aux_sym_number_token1,
    STATE(4), 1,
      aux_sym__call_args_list_repeat1,
    STATE(20), 1,
      sym_ident,
    STATE(38), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(45), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [123] = 11,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      aux_sym_number_token1,
    ACTIONS(30), 1,
      anon_sym_RPAREN,
    STATE(4), 1,
      aux_sym__call_args_list_repeat1,
    STATE(20), 1,
      sym_ident,
    STATE(38), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(45), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [162] = 10,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(36), 1,
      anon_sym_SEMI,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(43), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(53), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [198] = 10,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      aux_sym_number_token1,
    ACTIONS(40), 1,
      anon_sym_SEMI,
    STATE(20), 1,
      sym_ident,
    STATE(35), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(34), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [234] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(63), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [267] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(60), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [300] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(57), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [333] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(62), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [366] = 2,
    ACTIONS(44), 1,
      sym_star,
    ACTIONS(42), 13,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [385] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(59), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [418] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(66), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [451] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(65), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [484] = 9,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      aux_sym_number_token1,
    STATE(20), 1,
      sym_ident,
    STATE(38), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(61), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [517] = 9,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      aux_sym_number_token1,
    STATE(20), 1,
      sym_ident,
    STATE(38), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(58), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [550] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(49), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [583] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(50), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [616] = 3,
    ACTIONS(50), 1,
      sym_star,
    ACTIONS(48), 2,
      anon_sym_COLON,
      anon_sym_EQ,
    ACTIONS(46), 11,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [637] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(48), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [670] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(52), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(51), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [703] = 9,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      aux_sym_number_token1,
    STATE(20), 1,
      sym_ident,
    STATE(36), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(33), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [736] = 9,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      aux_sym_number_token1,
    STATE(20), 1,
      sym_ident,
    STATE(38), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(31), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [769] = 9,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      aux_sym_number_token1,
    STATE(20), 1,
      sym_ident,
    STATE(38), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(42), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [802] = 9,
    ACTIONS(32), 1,
      sym__ident,
    ACTIONS(34), 1,
      anon_sym_LPAREN,
    ACTIONS(38), 1,
      aux_sym_number_token1,
    STATE(28), 1,
      sym_ident,
    STATE(55), 1,
      sym__block_clause,
    STATE(56), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(6), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(64), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [835] = 9,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      aux_sym_number_token1,
    STATE(20), 1,
      sym_ident,
    STATE(38), 1,
      sym__block_clause,
    STATE(39), 1,
      sym__block,
    STATE(84), 1,
      sym__pat,
    STATE(7), 2,
      sym_var_decl,
      sym__block_stmt,
    STATE(41), 5,
      sym__expr,
      sym_bin_op,
      sym_call,
      sym_block,
      sym_number,
  [868] = 3,
    ACTIONS(48), 2,
      anon_sym_COLON,
      anon_sym_EQ,
    ACTIONS(50), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(46), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [888] = 2,
    ACTIONS(44), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(42), 10,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_COLON,
      anon_sym_EQ,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [906] = 2,
    ACTIONS(54), 1,
      sym_star,
    ACTIONS(52), 11,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [923] = 3,
    ACTIONS(58), 1,
      anon_sym_LPAREN,
    ACTIONS(60), 1,
      sym_star,
    ACTIONS(56), 10,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [942] = 2,
    ACTIONS(64), 1,
      sym_star,
    ACTIONS(62), 11,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [959] = 6,
    ACTIONS(58), 1,
      anon_sym_LPAREN,
    ACTIONS(70), 1,
      sym_star,
    ACTIONS(72), 1,
      sym_double_star,
    ACTIONS(68), 2,
      sym_plus,
      sym_minus,
    ACTIONS(74), 2,
      sym_slash,
      sym_percent,
    ACTIONS(66), 5,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym__ident,
      aux_sym_number_token1,
  [984] = 6,
    ACTIONS(58), 1,
      anon_sym_LPAREN,
    ACTIONS(70), 1,
      sym_star,
    ACTIONS(72), 1,
      sym_double_star,
    ACTIONS(68), 2,
      sym_plus,
      sym_minus,
    ACTIONS(74), 2,
      sym_slash,
      sym_percent,
    ACTIONS(76), 5,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym__ident,
      aux_sym_number_token1,
  [1009] = 2,
    ACTIONS(80), 1,
      sym_star,
    ACTIONS(78), 11,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1026] = 2,
    ACTIONS(84), 1,
      sym_star,
    ACTIONS(82), 11,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1043] = 2,
    ACTIONS(88), 1,
      sym_star,
    ACTIONS(86), 11,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1060] = 2,
    ACTIONS(92), 1,
      sym_star,
    ACTIONS(90), 11,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1077] = 2,
    ACTIONS(96), 1,
      sym_star,
    ACTIONS(94), 11,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1094] = 2,
    ACTIONS(100), 1,
      sym_star,
    ACTIONS(98), 11,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1111] = 5,
    ACTIONS(58), 1,
      anon_sym_LPAREN,
    ACTIONS(70), 1,
      sym_star,
    ACTIONS(72), 1,
      sym_double_star,
    ACTIONS(74), 2,
      sym_slash,
      sym_percent,
    ACTIONS(56), 7,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym__ident,
      aux_sym_number_token1,
  [1134] = 4,
    ACTIONS(58), 1,
      anon_sym_LPAREN,
    ACTIONS(60), 1,
      sym_star,
    ACTIONS(72), 1,
      sym_double_star,
    ACTIONS(56), 9,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_SEMI,
      sym_plus,
      sym_minus,
      sym_slash,
      sym_percent,
      sym__ident,
      aux_sym_number_token1,
  [1155] = 2,
    ACTIONS(80), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(78), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1171] = 2,
    ACTIONS(88), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(86), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1187] = 7,
    ACTIONS(58), 1,
      anon_sym_LPAREN,
    ACTIONS(70), 1,
      sym_star,
    ACTIONS(72), 1,
      sym_double_star,
    ACTIONS(104), 1,
      anon_sym_COMMA,
    ACTIONS(68), 2,
      sym_plus,
      sym_minus,
    ACTIONS(74), 2,
      sym_slash,
      sym_percent,
    ACTIONS(102), 3,
      anon_sym_RPAREN,
      sym__ident,
      aux_sym_number_token1,
  [1213] = 2,
    ACTIONS(54), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(52), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1229] = 2,
    ACTIONS(100), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(98), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1245] = 6,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(60), 2,
      anon_sym_fn,
      sym__ident,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(56), 4,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
  [1269] = 4,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(60), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(56), 6,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_slash,
      sym_percent,
  [1289] = 3,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(60), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(56), 7,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1307] = 7,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(66), 2,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(114), 2,
      anon_sym_fn,
      sym__ident,
    ACTIONS(116), 2,
      sym_plus,
      sym_minus,
  [1333] = 2,
    ACTIONS(84), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(82), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1349] = 7,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(76), 2,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(116), 2,
      sym_plus,
      sym_minus,
    ACTIONS(118), 2,
      anon_sym_fn,
      sym__ident,
  [1375] = 2,
    ACTIONS(64), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(62), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1391] = 2,
    ACTIONS(92), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(90), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1407] = 2,
    ACTIONS(96), 3,
      anon_sym_fn,
      sym_star,
      sym__ident,
    ACTIONS(94), 8,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_plus,
      sym_minus,
      sym_double_star,
      sym_slash,
      sym_percent,
  [1423] = 7,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(120), 1,
      ts_builtin_sym_end,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(116), 2,
      sym_plus,
      sym_minus,
    ACTIONS(122), 2,
      anon_sym_fn,
      sym__ident,
  [1448] = 6,
    ACTIONS(58), 1,
      anon_sym_LPAREN,
    ACTIONS(70), 1,
      sym_star,
    ACTIONS(72), 1,
      sym_double_star,
    ACTIONS(68), 2,
      sym_plus,
      sym_minus,
    ACTIONS(74), 2,
      sym_slash,
      sym_percent,
    ACTIONS(124), 3,
      anon_sym_SEMI,
      sym__ident,
      aux_sym_number_token1,
  [1471] = 7,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(126), 1,
      ts_builtin_sym_end,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(116), 2,
      sym_plus,
      sym_minus,
    ACTIONS(128), 2,
      anon_sym_fn,
      sym__ident,
  [1496] = 7,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(130), 1,
      ts_builtin_sym_end,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(116), 2,
      sym_plus,
      sym_minus,
    ACTIONS(132), 2,
      anon_sym_fn,
      sym__ident,
  [1521] = 6,
    ACTIONS(58), 1,
      anon_sym_LPAREN,
    ACTIONS(70), 1,
      sym_star,
    ACTIONS(72), 1,
      sym_double_star,
    ACTIONS(68), 2,
      sym_plus,
      sym_minus,
    ACTIONS(74), 2,
      sym_slash,
      sym_percent,
    ACTIONS(134), 3,
      anon_sym_SEMI,
      sym__ident,
      aux_sym_number_token1,
  [1544] = 7,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(136), 1,
      ts_builtin_sym_end,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(116), 2,
      sym_plus,
      sym_minus,
    ACTIONS(138), 2,
      anon_sym_fn,
      sym__ident,
  [1569] = 7,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(140), 1,
      ts_builtin_sym_end,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(116), 2,
      sym_plus,
      sym_minus,
    ACTIONS(142), 2,
      anon_sym_fn,
      sym__ident,
  [1594] = 7,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(144), 1,
      ts_builtin_sym_end,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(116), 2,
      sym_plus,
      sym_minus,
    ACTIONS(146), 2,
      anon_sym_fn,
      sym__ident,
  [1619] = 6,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(148), 1,
      anon_sym_RPAREN,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(116), 2,
      sym_plus,
      sym_minus,
  [1640] = 6,
    ACTIONS(106), 1,
      anon_sym_LPAREN,
    ACTIONS(108), 1,
      sym_star,
    ACTIONS(110), 1,
      sym_double_star,
    ACTIONS(150), 1,
      anon_sym_RPAREN,
    ACTIONS(112), 2,
      sym_slash,
      sym_percent,
    ACTIONS(116), 2,
      sym_plus,
      sym_minus,
  [1661] = 5,
    ACTIONS(5), 1,
      sym__ident,
    ACTIONS(7), 1,
      anon_sym_fn,
    ACTIONS(152), 1,
      ts_builtin_sym_end,
    STATE(86), 1,
      sym_ident,
    STATE(68), 4,
      sym__module_stmt,
      sym_fn_decl,
      sym_global_var_decl,
      aux_sym_root_repeat1,
  [1680] = 5,
    ACTIONS(154), 1,
      ts_builtin_sym_end,
    ACTIONS(156), 1,
      sym__ident,
    ACTIONS(159), 1,
      anon_sym_fn,
    STATE(86), 1,
      sym_ident,
    STATE(68), 4,
      sym__module_stmt,
      sym_fn_decl,
      sym_global_var_decl,
      aux_sym_root_repeat1,
  [1699] = 5,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(162), 1,
      anon_sym_RPAREN,
    STATE(71), 1,
      aux_sym_fn_decl_repeat1,
    STATE(78), 1,
      sym_fn_param,
    STATE(73), 2,
      sym__pat,
      sym_ident,
  [1716] = 5,
    ACTIONS(164), 1,
      sym__ident,
    ACTIONS(167), 1,
      anon_sym_RPAREN,
    STATE(70), 1,
      aux_sym_fn_decl_repeat1,
    STATE(78), 1,
      sym_fn_param,
    STATE(73), 2,
      sym__pat,
      sym_ident,
  [1733] = 5,
    ACTIONS(9), 1,
      sym__ident,
    ACTIONS(169), 1,
      anon_sym_RPAREN,
    STATE(70), 1,
      aux_sym_fn_decl_repeat1,
    STATE(78), 1,
      sym_fn_param,
    STATE(73), 2,
      sym__pat,
      sym_ident,
  [1750] = 1,
    ACTIONS(171), 4,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym__ident,
      aux_sym_number_token1,
  [1757] = 2,
    ACTIONS(175), 1,
      anon_sym_COLON,
    ACTIONS(173), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      sym__ident,
  [1766] = 2,
    ACTIONS(32), 1,
      sym__ident,
    STATE(92), 2,
      sym__type_expr,
      sym_ident,
  [1774] = 2,
    ACTIONS(32), 1,
      sym__ident,
    STATE(90), 2,
      sym__type_expr,
      sym_ident,
  [1782] = 2,
    ACTIONS(32), 1,
      sym__ident,
    STATE(87), 2,
      sym__type_expr,
      sym_ident,
  [1790] = 2,
    ACTIONS(9), 1,
      sym__ident,
    STATE(80), 2,
      sym__type_expr,
      sym_ident,
  [1798] = 2,
    ACTIONS(179), 1,
      anon_sym_COMMA,
    ACTIONS(177), 2,
      anon_sym_RPAREN,
      sym__ident,
  [1806] = 2,
    ACTIONS(32), 1,
      sym__ident,
    STATE(89), 2,
      sym__type_expr,
      sym_ident,
  [1814] = 1,
    ACTIONS(181), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      sym__ident,
  [1820] = 1,
    ACTIONS(183), 2,
      anon_sym_RPAREN,
      sym__ident,
  [1825] = 2,
    ACTIONS(185), 1,
      anon_sym_COLON,
    ACTIONS(187), 1,
      anon_sym_EQ,
  [1832] = 2,
    ACTIONS(189), 1,
      anon_sym_COLON,
    ACTIONS(191), 1,
      anon_sym_EQ,
  [1839] = 2,
    ACTIONS(193), 1,
      anon_sym_COLON,
    ACTIONS(195), 1,
      anon_sym_EQ,
  [1846] = 2,
    ACTIONS(32), 1,
      sym__ident,
    STATE(91), 1,
      sym_ident,
  [1853] = 2,
    ACTIONS(197), 1,
      anon_sym_COLON,
    ACTIONS(199), 1,
      anon_sym_EQ,
  [1860] = 1,
    ACTIONS(201), 1,
      anon_sym_EQ,
  [1864] = 1,
    ACTIONS(203), 1,
      anon_sym_RPAREN,
  [1868] = 1,
    ACTIONS(205), 1,
      anon_sym_EQ,
  [1872] = 1,
    ACTIONS(207), 1,
      anon_sym_EQ,
  [1876] = 1,
    ACTIONS(209), 1,
      anon_sym_LPAREN,
  [1880] = 1,
    ACTIONS(211), 1,
      anon_sym_EQ,
  [1884] = 1,
    ACTIONS(213), 1,
      anon_sym_RPAREN,
  [1888] = 1,
    ACTIONS(215), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 42,
  [SMALL_STATE(4)] = 84,
  [SMALL_STATE(5)] = 123,
  [SMALL_STATE(6)] = 162,
  [SMALL_STATE(7)] = 198,
  [SMALL_STATE(8)] = 234,
  [SMALL_STATE(9)] = 267,
  [SMALL_STATE(10)] = 300,
  [SMALL_STATE(11)] = 333,
  [SMALL_STATE(12)] = 366,
  [SMALL_STATE(13)] = 385,
  [SMALL_STATE(14)] = 418,
  [SMALL_STATE(15)] = 451,
  [SMALL_STATE(16)] = 484,
  [SMALL_STATE(17)] = 517,
  [SMALL_STATE(18)] = 550,
  [SMALL_STATE(19)] = 583,
  [SMALL_STATE(20)] = 616,
  [SMALL_STATE(21)] = 637,
  [SMALL_STATE(22)] = 670,
  [SMALL_STATE(23)] = 703,
  [SMALL_STATE(24)] = 736,
  [SMALL_STATE(25)] = 769,
  [SMALL_STATE(26)] = 802,
  [SMALL_STATE(27)] = 835,
  [SMALL_STATE(28)] = 868,
  [SMALL_STATE(29)] = 888,
  [SMALL_STATE(30)] = 906,
  [SMALL_STATE(31)] = 923,
  [SMALL_STATE(32)] = 942,
  [SMALL_STATE(33)] = 959,
  [SMALL_STATE(34)] = 984,
  [SMALL_STATE(35)] = 1009,
  [SMALL_STATE(36)] = 1026,
  [SMALL_STATE(37)] = 1043,
  [SMALL_STATE(38)] = 1060,
  [SMALL_STATE(39)] = 1077,
  [SMALL_STATE(40)] = 1094,
  [SMALL_STATE(41)] = 1111,
  [SMALL_STATE(42)] = 1134,
  [SMALL_STATE(43)] = 1155,
  [SMALL_STATE(44)] = 1171,
  [SMALL_STATE(45)] = 1187,
  [SMALL_STATE(46)] = 1213,
  [SMALL_STATE(47)] = 1229,
  [SMALL_STATE(48)] = 1245,
  [SMALL_STATE(49)] = 1269,
  [SMALL_STATE(50)] = 1289,
  [SMALL_STATE(51)] = 1307,
  [SMALL_STATE(52)] = 1333,
  [SMALL_STATE(53)] = 1349,
  [SMALL_STATE(54)] = 1375,
  [SMALL_STATE(55)] = 1391,
  [SMALL_STATE(56)] = 1407,
  [SMALL_STATE(57)] = 1423,
  [SMALL_STATE(58)] = 1448,
  [SMALL_STATE(59)] = 1471,
  [SMALL_STATE(60)] = 1496,
  [SMALL_STATE(61)] = 1521,
  [SMALL_STATE(62)] = 1544,
  [SMALL_STATE(63)] = 1569,
  [SMALL_STATE(64)] = 1594,
  [SMALL_STATE(65)] = 1619,
  [SMALL_STATE(66)] = 1640,
  [SMALL_STATE(67)] = 1661,
  [SMALL_STATE(68)] = 1680,
  [SMALL_STATE(69)] = 1699,
  [SMALL_STATE(70)] = 1716,
  [SMALL_STATE(71)] = 1733,
  [SMALL_STATE(72)] = 1750,
  [SMALL_STATE(73)] = 1757,
  [SMALL_STATE(74)] = 1766,
  [SMALL_STATE(75)] = 1774,
  [SMALL_STATE(76)] = 1782,
  [SMALL_STATE(77)] = 1790,
  [SMALL_STATE(78)] = 1798,
  [SMALL_STATE(79)] = 1806,
  [SMALL_STATE(80)] = 1814,
  [SMALL_STATE(81)] = 1820,
  [SMALL_STATE(82)] = 1825,
  [SMALL_STATE(83)] = 1832,
  [SMALL_STATE(84)] = 1839,
  [SMALL_STATE(85)] = 1846,
  [SMALL_STATE(86)] = 1853,
  [SMALL_STATE(87)] = 1860,
  [SMALL_STATE(88)] = 1864,
  [SMALL_STATE(89)] = 1868,
  [SMALL_STATE(90)] = 1872,
  [SMALL_STATE(91)] = 1876,
  [SMALL_STATE(92)] = 1880,
  [SMALL_STATE(93)] = 1884,
  [SMALL_STATE(94)] = 1888,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_root, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(29),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(85),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [19] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 19), SHIFT_REPEAT(12),
  [22] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 19), SHIFT_REPEAT(14),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 19),
  [27] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 19), SHIFT_REPEAT(40),
  [30] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__call_args_list, 1, .production_id = 11),
  [32] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [34] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [36] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [40] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [42] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ident, 1),
  [44] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ident, 1),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 1),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__pat, 1),
  [50] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 1),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 3),
  [54] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 3),
  [56] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bin_op, 3, .production_id = 12),
  [58] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [60] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bin_op, 3, .production_id = 12),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 4, .production_id = 18),
  [64] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 4, .production_id = 18),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 3, .production_id = 13),
  [68] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [70] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 2, .production_id = 5),
  [78] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 2, .production_id = 6),
  [80] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 2, .production_id = 6),
  [82] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block_clause, 3, .production_id = 14),
  [84] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 3, .production_id = 14),
  [86] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 3, .production_id = 9),
  [88] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 3, .production_id = 9),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__block, 1, .production_id = 2),
  [92] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block, 1, .production_id = 2),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 1, .production_id = 2),
  [96] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 1, .production_id = 2),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1),
  [100] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 1),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 1, .production_id = 10),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [106] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [108] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [110] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [112] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [114] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 3, .production_id = 13),
  [116] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [118] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__block_clause, 2, .production_id = 5),
  [120] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 8, .production_id = 22),
  [122] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 8, .production_id = 22),
  [124] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_decl, 3, .production_id = 15),
  [126] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 6, .production_id = 16),
  [128] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 6, .production_id = 16),
  [130] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 7, .production_id = 20),
  [132] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 7, .production_id = 20),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_var_decl, 5, .production_id = 21),
  [136] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_var_decl, 3, .production_id = 1),
  [138] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_var_decl, 3, .production_id = 1),
  [140] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_decl, 9, .production_id = 23),
  [142] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fn_decl, 9, .production_id = 23),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_var_decl, 5, .production_id = 8),
  [146] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_var_decl, 5, .production_id = 8),
  [148] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [150] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [152] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_root, 1),
  [154] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_root_repeat1, 2),
  [156] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_root_repeat1, 2), SHIFT_REPEAT(29),
  [159] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_root_repeat1, 2), SHIFT_REPEAT(85),
  [162] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [164] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 2, .production_id = 7), SHIFT_REPEAT(12),
  [167] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 2, .production_id = 7),
  [169] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__call_args_list_repeat1, 2, .production_id = 10),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_param, 1, .production_id = 4),
  [175] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 1, .production_id = 3),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fn_param, 3, .production_id = 17),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fn_decl_repeat1, 2, .production_id = 3),
  [185] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [191] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [193] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [195] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [197] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [199] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [201] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [203] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [205] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [207] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [209] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [211] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [215] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
