import { type BunFile } from "bun"
import { command, run, positional, subcommands, type Type, flag } from "cmd-ts"

import { astToIr } from "./ir"
import { parseAst } from "./parser"

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
    },
    handler: async ({ file, json }) => {
        try {
            const ast = await parseAst(file)

            if (json) {
                console.log(JSON.stringify(ast))
            } else {
                console.log(ast.toString())
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
    },
    handler: async ({ file, json }) => {
        const ast = await parseAst(file)
        const ir = astToIr(ast)

        if (json) {
            console.log(JSON.stringify(ir))
        } else {
            console.log(ir.toString())
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
