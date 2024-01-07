import { NumLiteral, type Ast, BinOp } from "@/parser"

export type IrProgram = IrStmt[]

export type IrStmt = IrAssign

export class IrAssign {
    constructor(
        public ref: IrVar,
        public value: IrExpr
    ) {}

    toString() {
        return `${this.ref.toString()} := ${this.value.toString()}`
    }

    toJSON() {
        return { Assign: { ref: this.ref, value: this.value } }
    }
}

export type IrExpr = IrValue | IrBinOp

export type IrValue = IrVar | NumConst

export class IrVar {
    constructor(public name: string | number) {}

    toString() {
        return typeof this.name === "string" ? this.name : `$${this.name}`
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

export class IrBinOp {
    constructor(
        public op: "+" | "-" | "*" | "/" | "%" | "^",
        public left: IrValue,
        public right: IrValue
    ) {}

    toString() {
        return `(${this.left.toString()} ${this.op} ${this.right.toString()})`
    }

    toJSON() {
        return { BinOp: { op: this.op, left: this.left, right: this.right } }
    }
}

export function astToIr(ast: Ast): IrProgram {
    const program: IrProgram = []
    let varc = 0

    const astToIrRec = (ast: Ast): IrValue => {
        switch (true) {
            case ast instanceof NumLiteral:
                return new NumConst(ast.value)
            case ast instanceof BinOp: {
                const left = astToIrRec(ast.left)
                const right = astToIrRec(ast.right)

                const ref = new IrVar(++varc)

                program.push(new IrAssign(ref, new IrBinOp(ast.op, left, right)))

                return ref
            }
            default:
                throw new Error("unreachable")
        }
    }

    astToIrRec(ast)

    return program
}
