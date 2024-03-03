import {
    oneOf,
    seq,
    TokenParser,
    many0,
    not,
    map,
    type Parser,
    optional,
    named,
    parse,
    ParseInput,
    noLog,
    withPrecedence,
    ParserWithPrecedence,
    type LocationRange,
} from "chunky-parser"

import { BinOpType, Expr, Loc, Pat, FnArg, Stmt, Module, type Type } from "./proto/ast"

function fromParsedLoc(parsedLoc: LocationRange): Loc {
    return { start: parsedLoc[0], end: parsedLoc[1] }
}

function tokenParserOnce<T extends string>(name: string, pattern: T) {
    const escaped = pattern.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")
    const reg = new RegExp(`${escaped}(?!${escaped})`)
    return new TokenParser<T>(name, reg)
}

const nl = new TokenParser("new line", /[\s\r\n]*[\r\n]/)
const ws = new TokenParser("whitespace", /[ \t]+/)
const comment = new TokenParser("comment", /\/\/[^\r\n]*/)
const eof = new TokenParser("end of file", /$/)
const num = new TokenParser("number", /(0x|0b)?(\d(_?\d)*)?\.?\d(_?\d)*/)
const name = new TokenParser("name", /[\p{L}_][\p{L}\p{Nd}_]*/u)
const plus = tokenParserOnce("plus", "+")
const minus = tokenParserOnce("minus", "-")
const asterisk = tokenParserOnce("asterisk", "*")
const dAsterisk = tokenParserOnce("double asterisk", "**")
const slash = tokenParserOnce("slash", "/")
const percent = tokenParserOnce("percent", "%")
const equal = tokenParserOnce("equal", "=")
const fn = new TokenParser("`fn`", "fn")
const lparen = new TokenParser("left parenthesis", "(")
const rparen = new TokenParser("right parenthesis", ")")
const comma = new TokenParser("comma", ",")
const colon = new TokenParser("colon", ":")
const semicolon = new TokenParser("semicolon", ";")

// FIXME: whitespaces and comments should be collect so the linter and the formatter can use them
const _ = noLog(map(many0(oneOf(nl, ws, comment)), () => null))

function sepBy<T>(
    parser: Parser<T>,
    sep: Parser<unknown>,
    last: "required" | "optional" | "none" = "none"
): Parser<T[]> {
    sep = seq(_, sep)

    return map(
        seq(
            parser,
            many0(map(seq(sep, _, parser), ({ value }) => value[2])),
            { required: sep, optional: optional(sep), none: not(sep) }[last]
        ),
        ({ value }) => [value[0], ...value[1]]
    )
}

export const expr: ParserWithPrecedence<Expr> = withPrecedence(
    map(
        seq(lparen, _, () => expr, _, rparen),
        ({ value }) => value[2]
    ),
    () => numLiteral,
    () => ident,
    () => fnCall,
    () => pow,
    () => mulDivMod,
    () => addSub,
    () => block,
    () => fnExpr
)

const numLiteral = map(
    num,
    ({ value, loc }): Expr => ({ loc: fromParsedLoc(loc), num: value.text })
)

const ident = named(
    "Identifier",
    map(name, ({ value, loc }): Expr => ({ loc: fromParsedLoc(loc), ident: value.text }))
)

const pow = map(
    seq(expr.left, _, dAsterisk, _, expr.right),
    ({ value, loc }): Expr => ({
        loc: fromParsedLoc(loc),
        binOp: {
            op: BinOpType.POW,
            left: value[0],
            right: value[4],
        },
    })
)

const mulDivMod = map(
    seq(expr.left, _, oneOf(asterisk, slash, percent), _, expr.right),
    ({ value, loc }): Expr => ({
        loc: fromParsedLoc(loc),
        binOp: {
            op: ({ "*": BinOpType.MUL, "/": BinOpType.DIV, "%": BinOpType.MOD } as const)[
                value[2].text
            ],
            left: value[0],
            right: value[4],
        },
    })
)

const addSub = map(
    seq(expr.left, _, oneOf(plus, minus), _, expr.right),
    ({ value, loc }): Expr => ({
        loc: fromParsedLoc(loc),
        binOp: {
            op: ({ "+": BinOpType.ADD, "-": BinOpType.SUB } as const)[value[2].text],
            left: value[0],
            right: value[4],
        },
    })
)

const namePat = map(name, ({ value, loc }): Pat => ({ loc: fromParsedLoc(loc), name: value.text }))

const pat = named("Pattern", oneOf(namePat))

const typeExpr: ParserWithPrecedence<Type> = withPrecedence(
    map(
        seq(lparen, _, () => typeExpr, _, rparen),
        ({ value }) => value[2]
    ),
    () => typeIdent
)

const typeIdent = named(
    "Type Identifier",
    map(name, ({ value, loc }): Type => ({ loc: fromParsedLoc(loc), ident: value.text }))
)

const fnArg = map(
    seq(pat, optional(seq(_, colon, _, typeExpr))),
    ({ value, loc }): FnArg => ({ loc: fromParsedLoc(loc), pat: value[0], type: value[1]?.[3] })
)

const fnExpr = map(
    seq(
        fn,
        _,
        lparen,
        _,
        optional(sepBy(fnArg, optional(comma), "required")),
        _,
        rparen,
        _,
        equal,
        _,
        expr
    ),
    ({ value, loc }): Expr => ({
        loc: fromParsedLoc(loc),
        fnExpr: { args: value[4] || [], ret: value[10] },
    })
)

const fnCall = named(
    "Function Call",
    map(
        seq(
            oneOf(
                map(seq(lparen, _, expr, _, rparen), ({ value }) => value[2]),
                ident
            ),
            optional(ws),
            lparen,
            _,
            sepBy(expr, optional(comma), "required"),
            _,
            rparen
        ),
        ({ value, loc }): Expr => ({
            loc: fromParsedLoc(loc),
            fnCall: { callee: value[0], args: value[4] },
        })
    )
)

const fnDecl = named(
    "Function Declaration",
    map(
        seq(
            fn,
            _,
            name,
            _,
            lparen,
            _,
            optional(sepBy(fnArg, optional(comma), "required")),
            _,
            rparen,
            optional(seq(_, colon, _, typeExpr)),
            _,
            equal,
            _,
            expr
        ),
        ({ value, loc }): Stmt => ({
            loc: fromParsedLoc(loc),
            fn: {
                name: value[2].text,
                args: value[6] || [],
                retType: value[9]?.[3],
                ret: value[13],
            },
        })
    )
)

const varDecl = named(
    "Variable Declaration",
    map(
        seq(pat, optional(seq(_, colon, _, typeExpr)), _, equal, _, expr),
        ({ value, loc }): Stmt => ({
            loc: fromParsedLoc(loc),
            var: { pat: value[0], type: value[1]?.[3], value: value[5] },
        })
    )
)

const blockStmt = oneOf(fnDecl, varDecl)

const moduleStmt = oneOf(fnDecl, varDecl)

const block = map(
    seq(sepBy(blockStmt, optional(semicolon), "required"), _, expr),
    ({ loc, value }): Expr => ({
        loc: fromParsedLoc(loc),
        block: { body: value[0], ret: value[2] },
    })
)

const muduleBody = map(
    optional(sepBy(moduleStmt, optional(semicolon), "required")),
    ({ value }) => value || []
)

export function parseAst(path: string, name: string, content: string): Module {
    const input = new ParseInput(path, content, {})
    const moduleBodyAst = parse(
        map(seq(_, muduleBody, _, eof), ({ value }) => value[1]),
        input
    )

    return {
        loc: {
            start: moduleBodyAst[0].loc!.start,
            end: moduleBodyAst[moduleBodyAst.length - 1].loc!.end,
        },
        name,
        path,
        body: moduleBodyAst,
    }
}
