import * as ast from "@/parser"

export type Module = Context

export type Stmt = Assign | FnDecl | FnRet

export class Context {
    constructor(
        public parent: Context | null,
        public names: Name[] = [],
        public block: Stmt[] = []
    ) {}

    countEqualNames(originalName?: string) {
        let count = this.names.reduce((acc, name) => {
            if (name.name === originalName || name.originalName === originalName) {
                return acc + 1
            }
            return acc
        }, 0)

        if (this.parent) {
            count += this.parent.countEqualNames(originalName)
        }

        return count
    }

    getName(originalName: string): Name | null {
        for (let i = this.names.length - 1; i >= 0; i--) {
            if (this.names[i].originalName === originalName) {
                return this.names[i]
            }
        }

        if (this.parent) {
            return this.parent.getName(originalName)
        }

        return null
    }

    addName(originalName?: string): Name {
        const count = this.countEqualNames(originalName)

        const name = new Name(
            originalName ? (count ? `${originalName}_${count}` : originalName) : `$${count + 1}`,
            originalName
        )

        this.names.push(name)
        return name
    }

    addAstStmt(node: ast.BlockStmt | ast.ModuleStmt): Value {
        switch (true) {
            case node instanceof ast.VarDecl: {
                const name = this.addName(node.pat.name)
                const value = this.addAstExpr(node.value)

                this.block.push(new Assign(name, value))

                return name
            }
            case node instanceof ast.FnDecl: {
                const name = this.addName(node.pat.name)

                const fnCtx = new Context(this)

                const def = node.def
                const args = def.args.map((arg) => fnCtx.addName(arg.pat.name))

                const ret = fnCtx.addAstValue(def.ret)
                fnCtx.block.push(new FnRet(ret))

                this.block.push(new FnDecl(name, args, fnCtx))

                return name
            }
            default:
                throw new Error(`Unknown AST node: ${String(node)}`)
        }
    }

    addAstExpr(expr: ast.Expr): Expr {
        switch (true) {
            case expr instanceof ast.NumLiteral:
                return new NumConst(expr.value)
            case expr instanceof ast.BinOp: {
                const left = this.addAstValue(expr.left)
                const right = this.addAstValue(expr.right)
                return new BinOp(expr.op, left, right)
            }
            case expr instanceof ast.VarName: {
                const name = this.getName(expr.name)
                // TODO: improve error handling
                if (!name) {
                    throw new Error(`Unknown variable: ${expr.name}`)
                }
                return name
            }
            case expr instanceof ast.FnCall: {
                const args = []
                for (const arg of expr.args) {
                    args.push(this.addAstValue(arg))
                }

                const callee = this.addAstValue(expr.fn)

                if (!(callee instanceof Name) || typeof callee.name !== "string") {
                    // TODO: improve error handling
                    throw new Error(`Cannot call ${callee.toString()}`)
                }

                return new FnCall(callee.name, args)
            }
            case expr instanceof ast.Block: {
                for (const stmt of expr.body) {
                    this.addAstStmt(stmt)
                }

                return this.addAstExpr(expr.ret)
            }
            default:
                throw new Error(`Unknown AST node: ${expr.toString()}`)
        }
    }

    addAstValue(expr: ast.Expr): Value {
        const value = this.addAstExpr(expr)

        if (value instanceof Name || value instanceof NumConst) {
            return value
        } else {
            const name = this.addName()
            this.block.push(new Assign(name, value))
            return name
        }
    }

    toString(indent = "") {
        return this.block.map((stmt): string => `${indent}${stmt.toString(indent)}\n`).join("")
    }

    toJSON() {
        return { Context: { names: this.names, block: this.block } }
    }
}

export class Assign {
    constructor(
        public name: Name,
        public value: Expr
    ) {}

    toString() {
        return `${this.name.toString()} := ${this.value.toString()}`
    }

    toJSON() {
        return { Assign: { name: this.name, value: this.value } }
    }
}

export class FnDecl {
    constructor(
        public name: Name,
        public params: Name[],
        public context: Context
    ) {}

    toString(indent = "") {
        return (
            `${this.name.toString()}(${this.params.join(", ")}) => \n` +
            this.context.toString(indent + "  ")
        )
    }

    toJSON() {
        return { FnDecl: { name: this.name, params: this.params, context: this.context } }
    }
}

export class FnRet {
    constructor(public value: Expr) {}

    toString() {
        return `return ${this.value.toString()}`
    }

    toJSON() {
        return { Ret: this.value }
    }
}

export type Expr = Value | BinOp | FnCall

export type Value = Name | NumConst

export class Name {
    constructor(
        public name: string,
        public originalName?: string
    ) {}

    toString() {
        return this.name
    }

    toJSON() {
        return { Var: this.name }
    }
}

export class NumConst {
    constructor(public value: string) {}

    toString() {
        return this.value
    }

    toJSON() {
        return { Num: this.value }
    }
}

export class BinOp {
    constructor(
        public op: "+" | "-" | "*" | "/" | "%" | "^",
        public left: Value,
        public right: Value
    ) {}

    toString() {
        return `${this.left.toString()} ${this.op} ${this.right.toString()}`
    }

    toJSON() {
        return { BinOp: { op: this.op, left: this.left, right: this.right } }
    }
}

export class FnCall {
    constructor(
        public name: string,
        public args: Value[]
    ) {}

    toString() {
        return `${this.name}(${this.args.join(", ")})`
    }

    toJSON() {
        return { FnCall: { name: this.name, args: this.args } }
    }
}

export function astToIr(astModule: ast.Module): Module {
    const context = new Context(null)

    for (const stmt of astModule.body) {
        context.addAstStmt(stmt)
    }

    return context
}
