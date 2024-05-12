const PREC = {
    IF: 0,
    BLOCK: 1,
    BLOCK_CLAUSE: 2,
    LOGICAL: 3,
    SUM: 4,
    MUL: 5,
    POW: 6,
    ATOM: 7,
    KEYWORD: 8,
    CALL: 9,
}

function bin_op(prec_lvl, operator, operand) {
    return prec.left(
        prec_lvl,
        seq(field("left", operand), field("op", operator), field("right", operand)),
    )
}

module.exports = grammar({
    name: "torvo",
    word: ($) => $._ident,
    rules: {
        root: ($) => repeat($._module_stmt),

        _module_stmt: ($) => choice($.func_decl, $.global_var_decl),

        func_decl: ($) =>
            seq(
                field("name", $.ident),
                "(",
                repeat(seq(field("params", $.func_param), optional(","))),
                ")",
                optional(seq(":", field("ret_type", $._type_expr))),
                "=",
                field("return", $._expr),
            ),

        func_param: ($) =>
            seq(field("pat", $._pat), optional(seq(":", field("type", $._type_expr)))),

        global_var_decl: ($) =>
            seq(
                field("name", $.ident),
                optional(seq(":", field("type", $._type_expr))),
                "=",
                field("value", $._expr),
            ),

        var_decl: ($) =>
            seq(
                field("pat", $._pat),
                optional(seq(":", field("type", $._type_expr))),
                "=",
                field("value", $._expr),
            ),

        _expr: ($) =>
            choice(
                prec(PREC.ATOM, seq("(", $._expr, ")")),
                $.true,
                $.false,
                $.ident,
                $.number,
                $.string_lit,
                $.array_lit,
                $.call,
                $.bin_op,
                $.block,
                $.if,
            ),

        bin_op: ($) =>
            choice(
                bin_op(PREC.LOGICAL, $.double_eq, $._expr),
                bin_op(PREC.LOGICAL, $.not_eq, $._expr),
                bin_op(PREC.LOGICAL, $.gt, $._expr),
                bin_op(PREC.LOGICAL, $.lt, $._expr),
                bin_op(PREC.LOGICAL, $.gt_eq, $._expr),
                bin_op(PREC.LOGICAL, $.lt_eq, $._expr),
                bin_op(PREC.SUM, $.plus, $._expr),
                bin_op(PREC.SUM, $.minus, $._expr),
                bin_op(PREC.MUL, $.star, $._expr),
                bin_op(PREC.MUL, $.slash, $._expr),
                bin_op(PREC.MUL, $.percent, $._expr),
                bin_op(PREC.POW, $.double_star, $._expr),
            ),

        call: ($) =>
            prec.left(
                PREC.CALL,
                seq(field("callee", $._expr), "(", optional($._call_args_list), ")"),
            ),
        _call_args_list: ($) =>
            prec(PREC.CALL, repeat1(seq(field("args", $._expr), optional(",")))),

        string_lit: ($) =>
            prec(
                PREC.ATOM,
                seq('"', field("content", $.string_lit_content), token.immediate('"')),
            ),
        string_lit_content: () => token.immediate(/\\"|[^"]+/),

        array_lit: ($) =>
            prec(
                PREC.ATOM,
                seq("[", repeat(seq(field("items", $._expr), optional(","))), "]"),
            ),

        block: ($) => prec(PREC.BLOCK, $._block),
        _block: ($) => prec.left($._block_clause),
        _block_clause: ($) =>
            prec.right(
                PREC.BLOCK_CLAUSE,
                seq(
                    field("body", $._block_stmt),
                    optional(";"),
                    choice(field("value", $._expr), $._block_clause),
                ),
            ),
        _block_stmt: ($) => choice($.var_decl),

        if: ($) =>
            prec.right(
                PREC.IF,
                seq(
                    "if",
                    field("cond", $._expr),
                    "then",
                    field("then", $._expr),
                    optional(seq("else", field("else", $._expr))),
                ),
            ),

        _type_expr: ($) => choice($.ident, $.array_type),

        array_type: ($) =>
            prec(
                PREC.ATOM,
                seq(
                    "[",
                    field("item_type", $._type_expr),
                    optional(seq(";", field("length", $.number))),
                    "]",
                ),
            ),

        _pat: ($) => choice($.ident),

        plus: () => "+",
        minus: () => "-",
        star: () => "*",
        double_star: () => "**",
        slash: () => "/",
        percent: () => "%",
        double_eq: () => "==",
        not_eq: () => "!=",
        gt: () => ">",
        lt: () => "<",
        gt_eq: () => ">=",
        lt_eq: () => "<=",

        ident: ($) => prec(PREC.ATOM, $._ident),
        _ident: () => /[\p{L}_][\p{L}\p{Nd}_]*/,

        true: () => prec(PREC.KEYWORD, "true"),
        false: () => prec(PREC.KEYWORD, "false"),

        number: () => prec(PREC.ATOM, /(\d(_?\d)*)?\.?\d(_?\d)*/),
    },
})
