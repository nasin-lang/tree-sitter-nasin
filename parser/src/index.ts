import { stdout, stdin } from "bun"
import { command, run, positional } from "cmd-ts"

import { parseAst } from "./parser"
import * as ast from "./proto/ast"

const cli = command({
    name: "torvo-parser",
    description:
        "Parse a file content as a module. This command will write the AST as binary to stdout.",
    args: {
        file: positional({
            displayName: "file",
            description: "Path to the file that will be parser",
        }),
        name: positional({
            displayName: "name",
            description: "Name of the module that will be parser",
        }),
    },
    handler: async ({ file, name }) => {
        try {
            const astNode = parseAst(file, name, await stdin.text())
            const data = ast.Module.encode(astNode).finish()
            await Bun.write(stdout, data)
        } catch (e) {
            console.error(e instanceof Error ? e.message : e)
            process.exit(1)
        }
    },
})

await run(cli, process.argv.slice(2))
