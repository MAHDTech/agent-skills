import {describe, it, expect} from "bun:test"
import fs from "fs"
import path from "path"
import {cleanOutputDir} from "./lib.ts"

describe("cleanOutputDir", () => {
    it("should successfully delete a non-empty directory", () => {
        const tempDir = path.join(import.meta.dir, "temp-test-dir")
        if (fs.existsSync(tempDir)) {
            fs.rmSync(tempDir, {recursive: true, force: true})
        }
        fs.mkdirSync(tempDir)
        fs.writeFileSync(path.join(tempDir, "file.txt"), "hello")

        const subDir = path.join(tempDir, "subdir")
        fs.mkdirSync(subDir)
        fs.writeFileSync(path.join(subDir, "another.txt"), "world")

        expect(fs.existsSync(tempDir)).toBe(true)

        cleanOutputDir(tempDir)

        expect(fs.existsSync(tempDir)).toBe(false)
    })
})
