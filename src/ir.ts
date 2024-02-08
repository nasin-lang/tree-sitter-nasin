import * as ast from "@/proto/ast"
import { Scope, Value, Module, BinOpType, type Instr } from "@/proto/m_ir"

export function astToIr(astModule: ast.Module): Module {
    const builder = new ScopeBuilder()

    for (const stmt of astModule.body) {
        builder.addStmt(stmt)
    }

    return { name: astModule.name, scope: builder.finish() }
}

class ScopeBuilder {
    private identMap: Map<string, string[]> = new Map()
    private names: string[] = []
    private body: Instr[] = []

    constructor(readonly parent?: ScopeBuilder) {}

    finish(): Scope {
        return { names: this.names, body: this.body }
    }

    addName(originalName?: string): string {
        const ident = originalName ?? ""
        const count = this.countIdent(ident)

        const name = originalName
            ? count
                ? `${originalName}_${count}`
                : originalName
            : `v${count + 1}`

        let identNames = this.identMap.get(ident)
        if (!identNames) {
            identNames = []
            this.identMap.set(ident, identNames)
        }

        this.names.push(name)
        identNames.push(name)

        return name
    }

    addStmt(node: ast.Stmt): Value {
        switch (true) {
            case node.var != null: {
                const name = this.addName(node.var.pat!.name!.name)
                return this.addExpr(node.var.value!, name)
            }
            case node.fn != null: {
                const name = this.addName(node.fn.name)

                const fnBuilder = new ScopeBuilder()
                const args = node.fn.args.map((arg) => fnBuilder.addName(arg.pat!.name!.name))
                const ret = fnBuilder.addExpr(node.fn.ret!)
                fnBuilder.body.push({ fnReturn: { value: ret } })

                this.body.push({ fnDecl: { name, args, scope: fnBuilder.finish() } })

                return { ident: name }
            }
            default:
                throw new Error(`Unknown AST node: ${String(node)}`)
        }
    }

    addExpr(expr: ast.Expr, name?: string): Value {
        switch (true) {
            case expr.num != null: {
                const value = { num: { value: expr.num.value } }
                if (name) {
                    this.body.push({ assign: { name, value } })
                    return { ident: name }
                }
                return value
            }
            case expr.binOp != null: {
                const left = this.addExpr(expr.binOp.left!)
                const right = this.addExpr(expr.binOp.right!)
                name ??= this.addName()
                this.body.push({
                    binOp: {
                        name,
                        left,
                        right,
                        op: {
                            [ast.BinOpType.ADD]: BinOpType.ADD,
                            [ast.BinOpType.SUB]: BinOpType.SUB,
                            [ast.BinOpType.MOD]: BinOpType.MOD,
                            [ast.BinOpType.MUL]: BinOpType.MUL,
                            [ast.BinOpType.DIV]: BinOpType.DIV,
                            [ast.BinOpType.POW]: BinOpType.POW,
                            [ast.BinOpType.UNRECOGNIZED]: BinOpType.UNRECOGNIZED,
                        }[expr.binOp.op],
                    },
                })
                return { ident: name }
            }
            case expr.ident != null: {
                const value = { ident: expr.ident.name }
                if (name) {
                    this.body.push({ assign: { name, value } })
                    return { ident: name }
                }
                return value
            }
            case expr.fnCall != null: {
                const args = []
                for (const arg of expr.fnCall.args) {
                    args.push(this.addExpr(arg))
                }

                const callee = this.addExpr(expr.fnCall.callee!)
                if (callee.ident == null) {
                    // TODO: improve error handling
                    throw new Error(`Cannot call ${JSON.stringify(callee)}`)
                }

                name ??= this.addName()

                this.body.push({ fnCall: { name, callee: callee.ident, args } })

                return { ident: name }
            }
            case expr.block != null: {
                for (const stmt of expr.block.body) {
                    this.addStmt(stmt)
                }

                return this.addExpr(expr.block.ret!, name)
            }
            default:
                throw new Error(`Unknown AST node: ${JSON.stringify(expr)}`)
        }
    }

    private countIdent(ident: string) {
        let count = this.identMap.get(ident)?.length ?? 0

        if (this.parent) {
            count += this.parent.countIdent(ident)
        }

        return count
    }
}
