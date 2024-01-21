import { type BunFile } from "bun"
import {
    type LocationRange,
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
} from "chunky-parser"
import { basename, resolve } from "node:path"

export type AstNode = Module | ModuleStmt | BlockStmt | Expr

export type ModuleStmt = VarDecl | FnDecl

export type BlockStmt = VarDecl | FnDecl

export type Expr = NumLiteral | VarName | FnCall | BinOp | Block | FnExpr

export class Module {
    constructor(
        public loc: LocationRange,
        public name: string,
        public body: ModuleStmt[]
    ) {}

    toString(indent = ""): string {
        return (
            `(Module ${this.name}\n` +
            this.body.map((stmt) => `${indent}  ${stmt.toString(indent + "  ")}`).join("\n") +
            `)`
        )
    }

    toJSON() {
        return { Module: { loc: this.loc, name: this.name, body: this.body } }
    }
}

export class VarDecl {
    constructor(
        public loc: LocationRange,
        public pat: Pat,
        public value: Expr
    ) {}

    toString(indent = ""): string {
        return (
            `(VarDecl ${this.pat.toString()}\n` +
            `${indent}  ${this.value.toString(indent + "  ")})`
        )
    }

    toJSON() {
        return { VarDecl: { loc: this.loc, pat: this.pat, value: this.value } }
    }
}

export type Pat = NamePat

export class NamePat {
    constructor(
        public loc: LocationRange,
        public name: string
    ) {}

    toString(): string {
        return `(NamePat ${this.name})`
    }

    toJSON() {
        return { NamePat: { loc: this.loc, name: this.name } }
    }
}

export class NumLiteral {
    constructor(
        public loc: LocationRange,
        public value: string
    ) {}

    toString(): string {
        return `(NumLiteral ${this.value})`
    }

    toJSON() {
        return { NumLiteral: { loc: this.loc, value: this.value } }
    }
}

export class VarName {
    constructor(
        public loc: LocationRange,
        public name: string
    ) {}

    toString(): string {
        return `(VarName ${this.name})`
    }

    toJSON() {
        return { VarName: { loc: this.loc, name: this.name } }
    }
}

export class BinOp {
    constructor(
        public op: "+" | "-" | "*" | "/" | "%" | "^",
        public loc: LocationRange,
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

export class Block {
    constructor(
        public loc: LocationRange,
        public body: BlockStmt[],
        public ret: Expr
    ) {}

    toString(indent = ""): string {
        return (
            `(Block\n` +
            this.body.map((stmt) => `${indent}  ${stmt.toString(indent + "  ")}\n`).join("") +
            `${indent}  ${this.ret.toString(indent + "  ")})`
        )
    }

    toJSON() {
        return { Block: { loc: this.loc, body: this.body, ret: this.ret } }
    }
}

export class FnDecl {
    constructor(
        public loc: LocationRange,
        public pat: NamePat,
        public def: FnExpr
    ) {}

    toString(indent = ""): string {
        return (
            `(FnDecl (${this.pat.toString()})\n` + `${indent}  ${this.def.toString(indent + "  ")})`
        )
    }

    toJSON() {
        return { FnDecl: { loc: this.loc, pat: this.pat, def: this.def } }
    }
}

export class FnExpr {
    constructor(
        public loc: LocationRange,
        public args: FnArg[],
        public ret: Expr
    ) {}

    toString(indent = ""): string {
        return (
            `(FnExpr [${this.args.join(" ")}]\n` + `${indent}  ${this.ret.toString(indent + "  ")})`
        )
    }

    toJSON() {
        return { FnExpr: { loc: this.loc, args: this.args, ret: this.ret } }
    }
}

export class FnArg {
    constructor(
        public loc: LocationRange,
        public pat: Pat
    ) {}

    toString(): string {
        return `(FnArg ${this.pat.toString()})`
    }

    toJSON() {
        return { FnArg: { loc: this.loc, pat: this.pat } }
    }
}

export class FnCall {
    constructor(
        public loc: LocationRange,
        public fn: Expr,
        public args: Expr[]
    ) {}

    toString(indent = ""): string {
        return (
            `(FnCall ${this.fn.toString(indent)}\n` +
            this.args.map((arg) => `${indent}  ${arg.toString(indent + "  ")}`).join("\n") +
            `)`
        )
    }

    toJSON() {
        return { FnCall: { loc: this.loc, fn: this.fn, args: this.args } }
    }
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
            many0(map(seq(sep, _, parser), (res) => res.value[2])),
            {
                required: sep,
                optional: optional(sep),
                none: not(sep),
            }[last]
        ),
        (res) => [res.value[0], ...res.value[1]]
    )
}
export const expr: ParserWithPrecedence<Expr> = withPrecedence(
    map(
        seq(lparen, _, () => expr, _, rparen),
        (res) => res.value[2]
    ),
    () => numLiteral,
    () => varName,
    () => fnCall,
    () => pow,
    () => mulDivMod,
    () => addSub,
    () => block,
    () => fnExpr
)

const numLiteral = map(
    num,
    (res): NumLiteral => new NumLiteral(res.loc, res.value.text.replace(/_/g, ""))
)

const varName = named(
    "Variable Name",
    map(name, (res): VarName => new VarName(res.loc, res.value.text))
)

const pow = map(
    seq(expr.left, _, carret, _, expr.right),
    (res): BinOp => new BinOp("^", res.loc, res.value[0], res.value[4])
)

const mulDivMod = map(
    seq(expr.left, _, oneOf(asterisk, slash, percent), _, expr.right),
    (res): BinOp => new BinOp(res.value[2].text, res.loc, res.value[0], res.value[4])
)

const addSub = map(
    seq(expr.left, _, oneOf(plus, minus), _, expr.right),
    (res): BinOp => new BinOp(res.value[2].text as "+" | "-", res.loc, res.value[0], res.value[4])
)

const namePat = map(name, (res): NamePat => new NamePat(res.loc, res.value.text))

const pat = named("Pattern", oneOf(namePat))

const fnArg = map(pat, (res): FnArg => new FnArg(res.loc, res.value))

const fnExpr = map(
    seq(
        optional(seq(lparen, _, optional(sepBy(fnArg, optional(comma), "required")), _, rparen, _)),
        arrow,
        _,
        expr
    ),
    (res): FnExpr => new FnExpr(res.loc, res.value[0]?.[2] || [], res.value[3])
)

const fnCall = named(
    "Function Call",
    map(
        seq(
            oneOf(
                map(seq(lparen, _, expr, _, rparen), (res) => res.value[2]),
                varName
            ),
            optional(ws),
            lparen,
            _,
            sepBy(expr, optional(comma), "required"),
            _,
            rparen
        ),
        (res): FnCall => new FnCall(res.loc, res.value[0], res.value[4])
    )
)

const fnDecl = named(
    "Function Declaration",
    map(seq(namePat, _, fnExpr), (res): FnDecl => new FnDecl(res.loc, res.value[0], res.value[2]))
)

const varDecl = named(
    "Variable Declaration",
    map(
        seq(pat, _, assign, _, expr),
        (res): VarDecl => new VarDecl(res.loc, res.value[0], res.value[4])
    )
)

const blockStmt = oneOf(fnDecl, varDecl)

const moduleStmt = oneOf(fnDecl, varDecl)

const block = map(
    seq(sepBy(blockStmt, optional(semicolon), "required"), _, expr),
    (res): Block => new Block(res.loc, res.value[0], res.value[2])
)

const muduleBody = map(
    optional(sepBy(moduleStmt, optional(semicolon), "required")),
    (res) => res.value || []
)

export async function parseAst(file: BunFile): Promise<Module> {
    const path = file.name ? resolve(file.name) : "unknown"
    const name = file.name ? basename(file.name) : "main"

    const input = new ParseInput(path, await file.text(), {})
    const moduleBodyAst = parse(
        map(seq(_, muduleBody, _, eof), (res) => res.value[1]),
        input
    )

    return new Module(
        [moduleBodyAst[0].loc[0], moduleBodyAst[moduleBodyAst.length - 1].loc[1]],
        name,
        moduleBodyAst
    )
}
