/// <reference types="./node_modules/tree-sitter-cli/dsl.d.ts" />

let iota = 0
const PREC = {
    IF: iota++,
    BLOCK: iota++,
    BLOCK_BODY: iota++,
    LOGICAL: iota++,
    SUM: iota++,
    MUL: iota++,
    POW: iota++,
    ATOM: iota++,
    GET_PROP: iota++,
    KEYWORD: iota++,
    CALL: iota++,
    INSTANCE: iota++,
}

module.exports = grammar({
    name: "nasin",
    word: ($) => $._ident,
    extras: ($) => [$._whitespace],
    rules: {
        root: ($) => seq(optional($._newline), sep($._newline, $._module_stmt)),

        _module_stmt: ($) => choice($.func_decl, $.global_decl, $.type_decl),

        func_decl: ($) =>
            seq(
                field("name", $.ident),
                $._func_params,
                optional($._func_ret_type),
                repeat(field("directives", $.directive)),
                optional(
                    seq(
                        token_with_nl("="),
                        optional($._newline),
                        field("return", $._expr),
                    ),
                ),
            ),
        _func_params: ($) =>
            seq(
                "(",
                optional($._newline),
                sep(or_nl(",", $._newline), field("params", $.func_param)),
                ")",
            ),
        _func_ret_type: ($) =>
            seq(
                token_with_nl(":"),
                optional($._newline),
                field("ret_type", $._type_expr),
            ),

        func_param: ($) =>
            seq(
                field("pat", $._pat),
                optional(seq(":", optional($._newline), field("type", $._type_expr))),
            ),

        global_decl: ($) =>
            seq(
                field("name", $.ident),
                optional(
                    seq(
                        token_with_nl(":"),
                        optional($._newline),
                        field("type", $._type_expr),
                    ),
                ),
                token_with_nl("="),
                optional($._newline),
                field("value", $._expr),
            ),

        let_stmt: ($) =>
            seq(
                "let",
                optional($._newline),
                field("pat", $._pat),
                optional(
                    seq(
                        token_with_nl(":"),
                        optional($._newline),
                        field("type", $._type_expr),
                    ),
                ),
                token_with_nl("="),
                optional($._newline),
                field("value", $._expr),
            ),

        directive: ($) =>
            seq(token_with_nl("@"), field("name", $.ident), optional($._directive_args)),
        _directive_args: ($) =>
            seq(
                "(",
                optional($._newline),
                sep(or_nl(",", $._newline), field("args", $._directive_arg)),
                ")",
            ),
        _directive_arg: ($) =>
            choice(
                prec(
                    PREC.ATOM,
                    seq(
                        "(",
                        optional($._newline),
                        $._directive_arg,
                        optional($._newline),
                        ")",
                    ),
                ),
                $.true,
                $.false,
                $.ident,
                $.number,
                $.string_lit,
                $.array_lit,
            ),

        _expr: ($) =>
            choice(
                prec(
                    PREC.ATOM,
                    seq("(", optional($._newline), $._expr, optional($._newline), ")"),
                ),
                $.true,
                $.false,
                $.ident,
                $.get_prop,
                $.number,
                $.string_lit,
                $.array_lit,
                $.call,
                $.macro,
                $.record_lit,
                $.bin_op,
                $.block,
                $.if,
            ),

        bin_op: ($) =>
            choice(
                bin_op(PREC.LOGICAL, seq($.double_eq, optional($._newline)), $._expr),
                bin_op(PREC.LOGICAL, seq($.not_eq, optional($._newline)), $._expr),
                bin_op(PREC.LOGICAL, seq($.gt, optional($._newline)), $._expr),
                bin_op(PREC.LOGICAL, seq($.lt, optional($._newline)), $._expr),
                bin_op(PREC.LOGICAL, seq($.gt_eq, optional($._newline)), $._expr),
                bin_op(PREC.LOGICAL, seq($.lt_eq, optional($._newline)), $._expr),
                bin_op(PREC.SUM, seq($.plus, optional($._newline)), $._expr),
                bin_op(PREC.SUM, seq($.minus, optional($._newline)), $._expr),
                bin_op(PREC.MUL, seq($.star, optional($._newline)), $._expr),
                bin_op(PREC.MUL, seq($.slash, optional($._newline)), $._expr),
                bin_op(PREC.MUL, seq($.percent, optional($._newline)), $._expr),
                bin_op(PREC.POW, seq($.double_star, optional($._newline)), $._expr),
            ),

        call: ($) => prec.left(PREC.CALL, seq(field("callee", $._expr), $._call_args)),
        _call_args: ($) =>
            seq(
                "(",
                optional($._newline),
                sep(or_nl(",", $._newline), field("args", $._expr)),
                ")",
            ),

        macro: ($) =>
            prec.left(PREC.CALL, seq("@", field("name", $.ident), $._call_args)),

        get_prop: ($) =>
            prec.left(
                PREC.GET_PROP,
                seq(
                    field("parent", $._expr),
                    $.dot,
                    optional($._newline),
                    field("prop_name", $.ident),
                ),
            ),

        string_lit: ($) =>
            prec(
                PREC.ATOM,
                seq('"', field("content", $.string_lit_content), token.immediate('"')),
            ),
        string_lit_content: () => token.immediate(/(?:\\"|[^"])+/),

        array_lit: ($) =>
            prec(
                PREC.ATOM,
                seq(
                    "[",
                    optional($._newline),
                    sep(or_nl(",", $._newline), field("items", $._expr)),
                    "]",
                ),
            ),

        record_lit: ($) =>
            prec(
                PREC.INSTANCE,
                seq(
                    "{",
                    optional($._newline),
                    sep(or_nl(",", $._newline), field("fields", $.record_lit_field)),
                    "}",
                ),
            ),
        record_lit_field: ($) =>
            seq(
                field("name", $.ident),
                optional($._newline),
                "=",
                optional($._newline),
                field("value", $._expr),
            ),

        block: ($) => prec(PREC.BLOCK, $._block),
        _block: ($) =>
            prec.right(
                PREC.BLOCK_BODY,
                seq(
                    field("body", $._block_stmt),
                    $._newline,
                    choice($._block, field("value", $._expr)),
                ),
            ),

        _block_stmt: ($) => choice($.let_stmt),

        if: ($) =>
            prec.right(
                PREC.IF,
                seq(
                    "if",
                    optional($._newline),
                    field("cond", $._expr),
                    optional($._newline),
                    "then",
                    optional($._newline),
                    field("then", $._expr),
                    optional(
                        seq(
                            optional($._newline),
                            "else",
                            optional($._newline),
                            field("else", $._expr),
                        ),
                    ),
                ),
            ),

        _type_expr: ($) => choice($.ident, $.array_type, $.generic_type),

        array_type: ($) =>
            prec(
                PREC.ATOM,
                seq(
                    "[",
                    optional($._newline),
                    field("item_type", $._type_expr),
                    optional(
                        seq(
                            optional($._newline),
                            ";",
                            optional($._newline),
                            field("length", $.number),
                        ),
                    ),
                    optional($._newline),
                    "]",
                ),
            ),

        generic_type: ($) => prec(PREC.CALL, seq(field("name", $.ident), $._type_args)),
        _type_args: ($) =>
            seq(
                "(",
                optional($._newline),
                sep(or_nl(",", $._newline), field("args", $._type_expr)),
                ")",
            ),

        _pat: ($) => choice($.ident),

        type_decl: ($) =>
            seq("type", field("name", $.ident), field("body", $._type_decl_body)),
        _type_decl_body: ($) => choice($.record_type),

        record_type: ($) =>
            seq(
                "{",
                optional($._newline),
                sep(
                    or_nl(",", $._newline),
                    choice(
                        field("fields", $.record_type_field),
                        field("methods", $.method),
                    ),
                ),
                "}",
            ),
        record_type_field: ($) =>
            seq(
                field("name", $.ident),
                token_with_nl(":"),
                optional($._newline),
                field("type", $._type_expr),
            ),
        method: ($) =>
            seq(
                field("name", $.ident),
                $._func_params,
                optional($._func_ret_type),
                token_with_nl("="),
                optional($._newline),
                field("return", $._expr),
            ),

        plus: () => token_with_nl("+"),
        minus: () => token_with_nl("-"),
        star: () => token_with_nl("*"),
        double_star: () => token_with_nl("**"),
        slash: () => token_with_nl("/"),
        percent: () => token_with_nl("%"),
        double_eq: () => token_with_nl("=="),
        not_eq: () => token_with_nl("!="),
        gt: () => token_with_nl(">"),
        lt: () => token_with_nl("<"),
        gt_eq: () => token_with_nl(">="),
        lt_eq: () => token_with_nl("<="),
        dot: () => token_with_nl("."),

        ident: ($) => prec(PREC.ATOM, $._ident),
        _ident: () => /[\p{L}_][\p{L}\p{Nd}_]*/u,

        true: () => prec(PREC.KEYWORD, "true"),
        false: () => prec(PREC.KEYWORD, "false"),

        number: () => prec(PREC.ATOM, /(\d(_?\d)*)?\.?\d(_?\d)*/),

        _whitespace: () => /[ \t]+/,
        _newline: () => /(\r?\n)+/,
    },
})

/**
 * Creates a token that can be placed in the next line
 * @param {string} token
 */
function token_with_nl(token) {
    token = token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")
    return new RegExp(`[ \\t\\f\\n\\r]*${token}`)
}

/**
 * Creates a rule for a binary operation
 * @param {number} level
 * @param {RuleOrLiteral} operator
 * @param {RuleOrLiteral} operand
 */
function bin_op(level, operator, operand) {
    return prec.left(
        level,
        seq(field("left", operand), field("op", operator), field("right", operand)),
    )
}

/**
 * Creates a rule that can be replaced with a new line. It will always consume trailing
 * new lines.
 * @param {RuleOrLiteral} rule
 * @param {RuleOrLiteral} newline
 */
function or_nl(rule, newline) {
    const with_nl = seq(rule, optional(newline))
    return choice(with_nl, seq(newline, optional(with_nl)))
}

/**
 * Creates a rule for a list of one or more items separated by a separator. Allow repeated
 * and trailing separators.
 * @param {RuleOrLiteral} separator
 * @param {RuleOrLiteral} rule
 */
function sep1(separator, rule) {
    return seq(rule, repeat(seq(separator, optional(rule))))
}

/**
 * Creates a rule for a list of zero or more items separated by a separator. Allows
 * repeated and trailing separators.
 * @param {RuleOrLiteral} separator
 * @param {RuleOrLiteral} rule
 */
function sep(separator, rule) {
    return optional(sep1(separator, rule))
}
