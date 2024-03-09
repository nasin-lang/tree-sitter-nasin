import type { Simplify } from "type-fest"

function bin_op(operator: Rule, operand: Rule) {
    return seq(field("left", operand), operator, field("right", operand))
}

module.exports = grammar({
    name: "torvo",
    word: ($) => $._ident,
    conflicts: ($) => [
        [$.fn_call, $.var_decl],
        [$.fn_call, $._fn_args_list],
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

        fn_call: ($) =>
            prec.left(
                "call",
                seq(field("callee", $._expr), "(", optional($._fn_args_list), ")"),
            ),
        _fn_args_list: ($) =>
            prec(1, repeat1(seq(field("args", $._expr), optional(",")))),

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
                $._wrapped_expr,
                $.ident,
                $.number,
                $.fn_call,
                $.pow,
                $.mul,
                $.div,
                $.mod,
                $.sum,
                $.sub,
                $.block,
            ),
        _wrapped_expr: ($) => prec("atom", seq("(", $._expr, ")")),

        sum: ($) => prec.left("sum-sub", bin_op("+", $._expr)),
        sub: ($) => prec.left("sum-sub", bin_op("-", $._expr)),
        mul: ($) => prec.left("mul-div", bin_op("*", $._expr)),
        div: ($) => prec.left("mul-div", bin_op("/", $._expr)),
        mod: ($) => prec.left("mul-div", bin_op("%", $._expr)),
        pow: ($) => prec.left("pow", bin_op("**", $._expr)),

        block: ($) => prec("block", $._block),
        _block: ($) => prec.left($._block_clause),
        _block_clause: ($) =>
            prec.right(
                1,
                seq(
                    field("body", $._block_stmt),
                    optional(";"),
                    choice(field("value", $._expr), $._block_clause),
                ),
            ),
        _block_stmt: ($) => choice($.var_decl),

        _type_expr: ($) => choice($.ident),

        _pat: ($) => choice($.ident),

        ident: ($) => prec("atom", $._ident),
        _ident: () => /[\p{L}_][\p{L}\p{Nd}_]*/,

        number: () => prec("atom", /(\d(_?\d)*)?\.?\d(_?\d)*/),
    },
    precedences: () => [["call", "atom", "pow", "mul-div", "sum-sub", "block"]],
})

// Type defs based on description from
// https://tree-sitter.github.io/tree-sitter/creating-parsers#the-grammar-dsl
declare global {
    /**
     * Opaque type representing a Tree-sitter grammar rule. Should not be read
     * directly.
     */
    class RuleObject {
        private rule: unknown
    }
    /**
     * Anything that can be accessed by `$.<name>`. Should not be read
     * directly.
     */
    class GrammarSymbol {
        private symbol: unknown
    }

    type Rule = string | RegExp | RuleObject | GrammarSymbol

    type GrammarFunc<T extends string, R> = ($: Simplify<Record<T, GrammarSymbol>>) => R

    /**
     * Defines a Tree-sitter grammar.
     */
    function grammar<T extends string>(args: {
        name: string
        /**
         * Every grammar rule is written as a JavaScript function that takes a
         * parameter conventionally called $. The syntax $.identifier is how
         * you refer to another grammar symbol within a rule. Names starting
         * with $.MISSING or $.UNEXPECTED should be avoided as they have
         * special meaning for the tree-sitter test command.
         *
         * The terminal symbols in a grammar are described using JavaScript
         * strings and regular expressions. Of course during parsing,
         * Tree-sitter does not actually use JavaScript’s regex engine to
         * evaluate these regexes; it generates its own regex-matching logic as
         * part of each parser. Regex literals are just used as a convenient
         * way of writing regular expressions in your grammar.
         *
         * Currently, only a subset of the Regex engine is actually supported.
         * This is due to certain features like lookahead and lookaround
         * assertions not feasible to use in an LR(1) grammar, as well as
         * certain flags being unnecessary for tree-sitter.
         */
        rules: Record<T, GrammarFunc<T, Rule>>
        /**
         * an array of tokens that may appear anywhere in the language. This is
         * often used for whitespace and comments. The default value of extras
         * is to accept whitespace. To control whitespace explicitly, specify
         * extras: $ => [] in your grammar.
         */
        extras?: GrammarFunc<T, Rule[]>
        /**
         * an array of rule names that should be automatically removed from the
         * grammar by replacing all of their usages with a copy of their
         * definition. This is useful for rules that are used in multiple
         * places but for which you don’t want to create syntax tree nodes at
         * runtime.
         */
        inline?: NoInfer<T>[]
        /**
         * an array of arrays of rule names. Each inner array represents a set
         * of rules that’s involved in an LR(1) conflict that is intended to
         * exist in the grammar. When these conflicts occur at runtime,
         * Tree-sitter will use the GLR algorithm to explore all of the
         * possible interpretations. If multiple parses end up succeeding,
         * Tree-sitter will pick the subtree whose corresponding rule has the
         * highest total dynamic precedence.
         */
        conflicts?: GrammarFunc<T, GrammarSymbol[][]>
        /**
         * an array of token names which can be returned by an external
         * scanner. External scanners allow you to write custom C code which
         * runs during the lexing process in order to handle lexical rules
         * (e.g. Python’s indentation tokens) that cannot be described by
         * regular expressions.
         */
        externals?: NoInfer<T>[]
        /**
         * an array of array of strings, where each array of strings defines
         * named precedence levels in descending order. These names can be used
         * in the prec functions to define precedence relative only to other
         * names in the array, rather than globally. Can only be used with
         * parse precedence, not lexical precedence.
         */
        precedences?: GrammarFunc<T, string[][]>
        /**
         * the name of a token that will match keywords for the purpose of the
         * keyword extraction optimization.
         */
        word?: GrammarFunc<T, GrammarSymbol>
        /**
         * an array of hidden rule names which should be considered to be
         * ‘supertypes’ in the generated node types file.
         */
        supertypes?: NoInfer<T>[]
    }): unknown

    /**
     * This function creates a rule that matches any number of other rules, one
     * after another. It is analogous to simply writing multiple symbols next
     * to each other in EBNF notation.
     */
    function seq(...rules: Rule[]): Rule

    /**
     * This function creates a rule that matches one of a set of possible
     * rules. The order of the arguments does not matter. This is analogous to
     * the | (pipe) operator in EBNF notation.
     */
    function choice(...rulers: Rule[]): Rule

    /**
     * This function creates a rule that matches zero-or-more occurrences of a
     * given rule. It is analogous to the {x} (curly brace) syntax in EBNF
     * notation.
     */
    function repeat(rule: Rule): Rule

    /**
     * This function creates a rule that matches one-or-more occurrences of a
     * given rule. The repeat rule is implemented in terms of repeat1 but is
     * included because it is very commonly used.
     */
    function repeat1(rule: Rule): Rule

    /**
     * This function creates a rule that matches zero or one occurrence of a
     * given rule. It is analogous to the [x] (square bracket) syntax in EBNF
     * notation.
     */
    function optional(rule: Rule): Rule

    const prec: {
        /**
         * This function marks the given rule with a numerical precedence which
         * will be used to resolve LR(1) Conflicts at parser-generation time.
         * When two rules overlap in a way that represents either a true
         * ambiguity or a local ambiguity given one token of lookahead,
         * Tree-sitter will try to resolve the conflict by matching the rule
         * with the higher precedence. The default precedence of all rules is
         * zero. This works similarly to the precedence directives in Yacc
         * grammars.
         */
        (n: number | string, rule: Rule): Rule
        /**
         * This function marks the given rule as left-associative (and
         * optionally applies a numerical precedence). When an LR(1) conflict
         * arises in which all of the rules have the same numerical precedence,
         * Tree-sitter will consult the rules’ associativity. If there is a
         * left-associative rule, Tree-sitter will prefer matching a rule that
         * ends earlier. This works similarly to associativity directives in
         * Yacc grammars.
         */
        left: {
            (rule: Rule): Rule
            (n: number | string, rule: Rule): Rule
        }
        /**
         * This function marks the given rule as right-associative (and
         * optionally applies a numerical precedence). When an LR(1) conflict
         * arises in which all of the rules have the same numerical precedence,
         * Tree-sitter will consult the rules’ associativity. If there is a
         * right-associative rule, Tree-sitter will prefer matching a rule that
         * ends later. This works similarly to associativity directives in
         * Yacc grammars.
         */
        right: {
            (rule: Rule): Rule
            (n: number | string, rule: Rule): Rule
        }
        /**
         * This function is similar to prec, but the given numerical precedence
         * is applied at runtime instead of at parser generation time. This is
         * only necessary when handling a conflict dynamically using the
         * conflicts field in the grammar, and when there is a genuine
         * ambiguity: multiple rules correctly match a given piece of code. In
         * that event, Tree-sitter compares the total dynamic precedence
         * associated with each rule, and selects the one with the highest
         * total. This is similar to dynamic precedence directives in Bison
         * grammars.
         */
        dynamic: (n: number | string, rule: Rule) => Rule
    }

    const token: {
        /**
         * This function marks the given rule as producing only a single token.
         * Tree-sitter’s default is to treat each String or RegExp literal in
         * the grammar as a separate token. Each token is matched separately by
         * the lexer and returned as its own leaf node in the tree. The token
         * function allows you to express a complex rule using the functions
         * described above (rather than as a single regular expression) but
         * still have Tree-sitter treat it as a single token. The token
         * function will only accept terminal rules, so token($.foo) will not
         * work. You can think of it as a shortcut for squashing complex rules
         * of strings or regexes down to a single token.
         */
        (rule: Rule): Rule
        /**
         * Usually, whitespace (and any other extras, such as comments) is
         * optional before each token. This function means that the token will
         * only match if there is no whitespace.
         */
        immediate: (rule: Rule) => Rule
    }

    /**
     * This function assigns a field name to the child node(s) matched by the
     * given rule. In the resulting syntax tree, you can then use that field
     * name to access specific children.
     */
    function field(name: string, rule: Rule): Rule
}

type NoInfer<T> = T extends infer U ? U : never
