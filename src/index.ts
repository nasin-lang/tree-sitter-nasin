import { type BunFile, stdout } from "bun"
import { command, run, positional, subcommands, type Type, flag } from "cmd-ts"
import { inspect } from "node:util"

import { astToIr } from "./ir"
import { parseAst } from "./parser"
import * as ast from "./proto/ast"
import * as ir from "./proto/m_ir"

const filename: Type<string, BunFile> = {
    async from(value) {
        return Bun.file(value)
    },
}

const showAst = command({
    name: "ast",
    description: "Show the AST of a file",
    args: {
        file: positional({
            displayName: "file",
            description: "Path to the file to show AST of",
            type: filename,
        }),
        json: flag({
            long: "json",
            description: "Print the AST as JSON",
        }),
        pretty: flag({
            long: "pretty",
            description: "Print the AST in a pretty format",
        }),
    },
    handler: async ({ file, json, pretty }) => {
        try {
            const astNode = await parseAst(file)

            if (json) {
                console.log(JSON.stringify(ast.Module.toJSON(astNode)))
            } else if (pretty) {
                console.log(inspect(ast.Module.toJSON(astNode), { depth: Infinity, colors: true }))
            } else {
                const data = ast.Module.encode(astNode).finish()
                await Bun.write(stdout, data)
            }
        } catch (e) {
            console.error(e instanceof Error ? e.message : e)
            process.exit(1)
        }
    },
})

const showIr = command({
    name: "ir",
    description: "Show the IR of a file",
    args: {
        file: positional({
            displayName: "file",
            description: "Path to the file to show IR of",
            type: filename,
        }),
        json: flag({
            long: "json",
            description: "Print the IR as JSON",
        }),
        pretty: flag({
            long: "pretty",
            description: "Print the AST in a pretty format",
        }),
    },
    handler: async ({ file, json, pretty }) => {
        try {
            const astNode = await parseAst(file)
            const irNode = astToIr(astNode)

            if (json) {
                console.log(JSON.stringify(ir.Module.toJSON(irNode)))
            } else if (pretty) {
                console.log(inspect(ir.Module.toJSON(irNode), { depth: Infinity, colors: true }))
            } else {
                const data = ir.Module.encode(irNode).finish()
                await Bun.write(stdout, data)
            }
        } catch (e) {
            console.error(e instanceof Error ? e.message : e)
            process.exit(1)
        }
    },
})

const show = subcommands({
    name: "show",
    description: "Show artifacts of compilation",
    cmds: { ast: showAst, ir: showIr },
})

const app = subcommands({
    name: "torvo",
    description:
        "Torvo is a pure functional language with a easy syntax and a strong expressive type system",
    cmds: { show },
})

await run(app, process.argv.slice(2))
