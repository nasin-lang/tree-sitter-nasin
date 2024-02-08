import { type BunFile } from "bun"
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
import { basename, resolve } from "node:path"

import { BinOpType, Expr, Loc, Pat, FnArg, Stmt, Module } from "./proto/ast"

function fromParsedLoc(parsedLoc: LocationRange): Loc {
    return { start: parsedLoc[0], end: parsedLoc[1] }
}

const nl = new TokenParser("new line", /[\s\r\n]*[\r\n]/)
const ws = new TokenParser("whitespace", /[ \t]+/)
const comment = new TokenParser("comment", /#[^\r\n]*/)
const eof = new TokenParser("end of file", /$/)
const num = new TokenParser("number", /(0x|0b)?(\d(_?\d)*)?\.?\d(_?\d)*/)
const name = new TokenParser("name", /[\p{L}_][\p{L}\p{Nd}_]*/u)
const plus = new TokenParser("plus", "+")
const minus = new TokenParser("minus", "-")
const asterisk = new TokenParser("asterisk", "*")
const slash = new TokenParser("slash", "/")
const percent = new TokenParser("percent", "%")
const carret = new TokenParser("carret", "^")
const assign = new TokenParser("assign", ":=")
const arrow = new TokenParser("arrow", "=>")
const lparen = new TokenParser("left parenthesis", "(")
const rparen = new TokenParser("right parenthesis", ")")
const comma = new TokenParser("comma", ",")
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
            {
                required: sep,
                optional: optional(sep),
                none: not(sep),
            }[last]
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
    ({ value, loc }): Expr => ({ loc: fromParsedLoc(loc), num: { value: value.text } })
)

const ident = named(
    "Identifier",
    map(name, ({ value, loc }): Expr => ({ loc: fromParsedLoc(loc), ident: { name: value.text } }))
)

const pow = map(
    seq(expr.left, _, carret, _, expr.right),
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

const namePat = map(
    name,
    ({ value, loc }): Pat => ({ loc: fromParsedLoc(loc), name: { name: value.text } })
)

const pat = named("Pattern", oneOf(namePat))

const fnArg = map(pat, ({ value, loc }): FnArg => ({ loc: fromParsedLoc(loc), pat: value }))

const fnExpr = map(
    seq(
        optional(seq(lparen, _, optional(sepBy(fnArg, optional(comma), "required")), _, rparen, _)),
        arrow,
        _,
        expr
    ),
    ({ value, loc }): Expr => ({
        loc: fromParsedLoc(loc),
        fnExpr: { args: value[0]?.[2] || [], ret: value[3] },
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
            name,
            _,
            optional(
                seq(lparen, _, optional(sepBy(fnArg, optional(comma), "required")), _, rparen, _)
            ),
            arrow,
            _,
            expr
        ),
        ({ value, loc }): Stmt => ({
            loc: fromParsedLoc(loc),
            fn: {
                name: value[0].text,
                args: value[2]?.[2] || [],
                ret: value[5],
            },
        })
    )
)

const varDecl = named(
    "Variable Declaration",
    map(
        seq(pat, _, assign, _, expr),
        ({ value, loc }): Stmt => ({
            loc: fromParsedLoc(loc),
            var: { pat: value[0], value: value[4] },
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

export async function parseAst(file: BunFile): Promise<Module> {
    const path = file.name ? resolve(file.name) : "unknown"
    const name = file.name ? basename(file.name) : "main"

    const input = new ParseInput(path, await file.text(), {})
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
