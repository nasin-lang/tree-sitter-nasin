import { $ } from "bun"
import { readdir, readFile, writeFile } from "node:fs/promises"
import { join } from "node:path"
import { tmpdir } from "node:os"

const TESTS_DIR = "../tests"

const testFiles = await readdir(TESTS_DIR)

const results = await Promise.all(testFiles.map(runTest))

const nPassed = results.filter((s) => s).length
const nFailed = results.length - nPassed

console.log(`\n${nPassed} tests passed, ${nFailed} tests failed`)

if (nFailed > 0) {
    process.exit(1)
}

async function runTest(file: string, index: number) {
    const buf = await readFile(join(TESTS_DIR, file))
    const content = buf.toString("utf-8")

    // File layout:
    //
    // Source code
    // ---
    // Expected stdout
    // ---
    // Expected stderr

    const [src, expedtedStdout = "", expectedStderr = ""] = content.split(/^---+\n*/gm)

    const outName = `torvo-test-${file.replace(/\.\w+$/, "")}-${index}`
    const srcPath = join(tmpdir(), `${outName}.torv`)
    const outPath = join(tmpdir(), outName)

    await writeFile(srcPath, src)

    const res =
        await $`RUST_BACKTRACE=1 ../bin/torvo b -s -o ${outPath} ${srcPath} && ${outPath}`
            .quiet()
            .nothrow()

    const stdout = res.stdout.toString("utf-8")
    const stderr = res.stderr.toString("utf-8")

    if (stdout !== expedtedStdout || stderr !== expectedStderr) {
        console.log(` ✗ ${file}: failed`)
        if (stdout !== expedtedStdout) {
            console.log("  stdout:")
            printDiff(expedtedStdout, stdout)
        }
        if (stderr !== expectedStderr) {
            console.log("  stderr:")
            printDiff(expectedStderr, stderr)
        }
        return false
    } else {
        console.log(` ✓ ${file}: passed`)
        return true
    }
}

function printDiff(expected: string, found: string) {
    console.log(
        expected
            .split("\n")
            .map((line) => `  - ${line}`)
            .join("\n"),
    )
    console.log(
        found
            .split("\n")
            .map((line) => `  + ${line}`)
            .join("\n"),
    )
}
