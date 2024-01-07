import { type BunFile } from "bun"
import * as c from "chunky-parser"
import { basename, resolve } from "node:path"

export type Ast = Expr

export type Expr = NumLiteral | BinOp

export class NumLiteral {
    constructor(
        public loc: c.LocationRange,
        public value: string
    ) {}

    toString(): string {
        return `(NumLiteral ${this.value})`
    }

    toJSON() {
        return { NumLiteral: { loc: this.loc, value: this.value } }
    }
}

export class BinOp {
    constructor(
        public op: "+" | "-" | "*" | "/" | "%" | "^",
        public loc: c.LocationRange,
        public left: Expr,
        public right: Expr
    ) {}

    toString(indent = ""): string {
        return (
            `(BinOp ${this.op}\n` +
            `${indent}  ${this.left.toString(indent + "  ")}\n` +
            `${indent}  ${this.right.toString(indent + "  ")})`
        )
    }

    toJSON() {
        return { BinOp: { loc: this.loc, op: this.op, left: this.left, right: this.right } }
    }
}

const nl = c.re(/[\r\n]+/)
const ws = c.raw(c.many1(c.oneOf(c.re(/\s+/), nl, c.seq(c.str("#")))))
const comment = c.raw(c.seq(c.str("#"), c.many0(c.not(nl))))

// FIXME: whitespaces and comments should be collect so the linter and the formatter can use them
const skp = c.map(c.many0(c.oneOf(ws, comment)), () => null)

const numLiteral = c.map(
    c.re(/(\d[\d_]*)?\.?[\d_]*\d/),
    (res): NumLiteral => new NumLiteral(res.loc, res.value.replace(/_/g, ""))
)

const pow = c.map(
    c.seq(
        () => expr,
        skp,
        c.str("^"),
        skp,
        () => expr
    ),
    (res): BinOp => new BinOp("^", res.loc, res.value[0], res.value[4])
)

const mulDivMod = c.map(
    c.seq(
        () => expr,
        skp,
        c.oneOf(c.str("*"), c.str("/"), c.str("%")),
        skp,
        () => expr
    ),
    (res): BinOp => new BinOp(res.value[2] as "*" | "/" | "%", res.loc, res.value[0], res.value[4])
)

const addSub = c.map(
    c.seq(
        () => expr,
        skp,
        c.oneOf(c.str("+"), c.str("-")),
        skp,
        () => expr
    ),
    (res): BinOp => new BinOp(res.value[2] as "+" | "-", res.loc, res.value[0], res.value[4])
)

export const expr: c.Parser<Expr> = c.named(
    "Expression",
    c.oneOf(
        addSub,
        mulDivMod,
        pow,
        numLiteral,
        c.map(
            c.seq(c.str("("), skp, () => expr, skp, c.str(")")),
            (res) => res.value[2]
        )
    )
)

export async function parseAst(file: BunFile) {
    // Fixme: use better parser
    const ast = c.parse(
        expr,
        {
            name: file.name ? basename(file.name) : "unknown",
            path: file.name ? resolve(file.name) : "unknown",
            content: await file.text(),
        },
        {}
    )
    return ast
}
