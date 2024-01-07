import { type BunFile } from "bun"
import { command, run, positional, subcommands, type Type, flag } from "cmd-ts"

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
        const ast = await parseAst(file)

        if (json) {
            console.log(JSON.stringify(ast))
        } else {
            console.log(ast.toString())
        }
    },
})

const show = subcommands({
    name: "show",
    description: "Show artifacts of compilation",
    cmds: { ast: showAst },
})

const app = subcommands({
    name: "torvo",
    description:
        "Torvo is a pure functional language with a easy syntax and a strong expressive type system",
    cmds: { show },
})

await run(app, process.argv.slice(2))
