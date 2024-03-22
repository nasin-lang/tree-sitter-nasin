const PREC = {
    BLOCK: 0,
    BLOCK_CLAUSE: 1,
    SUM: 2,
    MUL: 3,
    POW: 4,
    ATOM: 5,
    CALL: 6,
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

        _module_stmt: ($) => choice($.fn_decl, $.global_var_decl),

        fn_decl: ($) =>
            seq(
                "fn",
                field("name", $.ident),
                "(",
                repeat(seq(field("params", $.fn_param), optional(","))),
                ")",
                optional(seq(":", field("ret_type", $._type_expr))),
                "=",
                field("return", $._expr),
            ),

        fn_param: ($) =>
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
                $.ident,
                $.number,
                $.call,
                $.bin_op,
                $.block,
            ),

        bin_op: ($) =>
            choice(
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

        _type_expr: ($) => choice($.ident),

        _pat: ($) => choice($.ident),

        plus: () => "+",
        minus: () => "-",
        star: () => "*",
        double_star: () => "**",
        slash: () => "/",
        percent: () => "%",

        ident: ($) => prec(PREC.ATOM, $._ident),
        _ident: () => /[\p{L}_][\p{L}\p{Nd}_]*/,

        number: () => prec(PREC.ATOM, /(\d(_?\d)*)?\.?\d(_?\d)*/),
    },
})
