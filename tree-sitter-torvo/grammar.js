const PREC = {
    CALL: 0,
    CALL_ARGS: 1,
    ATOM: 2,
    POW: 3,
    MUL: 4,
    SUM: 5,
    BLOCK: 6,
    BLOCK_CLAUSE: 7,
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
    conflicts: ($) => [
        [$.call, $.var_decl],
        [$.call, $._call_args_list],
    ],
    rules: {
        source_file: ($) => repeat($._module_stmt),

        _module_stmt: ($) => choice($.fn_decl, $.global_decl),

        fn_decl: ($) =>
            seq(
                "fn",
                field("name", $.ident),
                "(",
                repeat(seq(field("param", $.fn_param), optional(","))),
                ")",
                optional(seq(":", field("ret_type", $._type_expr))),
                "=",
                field("ret", $._expr),
            ),

        fn_param: ($) =>
            seq(field("pat", $._pat), optional(seq(":", field("type", $._type_expr)))),

        global_decl: ($) =>
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
                bin_op(PREC.SUM, "+", $._expr),
                bin_op(PREC.SUM, "-", $._expr),
                bin_op(PREC.MUL, "*", $._expr),
                bin_op(PREC.MUL, "/", $._expr),
                bin_op(PREC.MUL, "%", $._expr),
                bin_op(PREC.POW, "**", $._expr),
            ),

        call: ($) =>
            prec.left(
                PREC.CALL,
                seq(field("callee", $._expr), "(", optional($._call_args_list), ")"),
            ),
        _call_args_list: ($) =>
            prec(PREC.CALL_ARGS, repeat1(seq(field("args", $._expr), optional(",")))),

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

        ident: ($) => prec(PREC.ATOM, $._ident),
        _ident: () => /[\p{L}_][\p{L}\p{Nd}_]*/,

        number: () => prec(PREC.ATOM, /(\d(_?\d)*)?\.?\d(_?\d)*/),
    },
})
