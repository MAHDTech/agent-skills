/* eslint-disable @typescript-eslint/no-unused-vars */
import {describe, it, expect, beforeAll, afterAll, mock} from "bun:test"
import fs from "fs"
import path from "path"
import os from "os"

const mockExecFileSync = mock(
    (cmd: string, _args?: string[], _options?: any) => {
        if (cmd === "git") {
            return "2026-07-17\n"
        }
        return ""
    }
)

mock.module("child_process", () => ({
    execFileSync: mockExecFileSync,
}))

const mockLogStep = mock(() => {})
const mockLogWarn = mock(() => {})

mock.module("@clack/prompts", () => ({
    intro: () => {},
    outro: () => {},
    log: {
        step: mockLogStep,
        warn: mockLogWarn,
        info: () => {},
        error: () => {},
        success: () => {},
        message: () => {},
    },
}))

describe("Dashboard Lib Tests", () => {
    let tempRootDir: string
    let lib: any

    beforeAll(async () => {
        // Set up a temporary root directory to sandbox all tests
        tempRootDir = fs.mkdtempSync(path.join(os.tmpdir(), "dashboard-test-"))
        process.env.AGENT_SKILLS_HOME = tempRootDir

        // Pre-create the directory layout that writeSkillDates expects
        fs.mkdirSync(path.join(tempRootDir, "dashboard"), {recursive: true})
        fs.mkdirSync(path.join(tempRootDir, "skills", "category", "name"), {
            recursive: true,
        })
        fs.writeFileSync(
            path.join(tempRootDir, "skills", "category", "name", "SKILL.md"),
            "test skill content"
        )

        // Dynamically import lib.ts to ensure process.env.AGENT_SKILLS_HOME is used for ROOT definition
        lib = await import("./lib.ts")
    })

    afterAll(() => {
        // Clean up sandbox root directory
        if (tempRootDir && fs.existsSync(tempRootDir)) {
            fs.rmSync(tempRootDir, {recursive: true, force: true})
        }
    })

    describe("cleanOutputDir", () => {
        it("should successfully delete a non-empty directory with a custom path", () => {
            const testCleanDir = path.join(tempRootDir, "test-clean-dir")
            fs.mkdirSync(testCleanDir, {recursive: true})
            fs.writeFileSync(path.join(testCleanDir, "file.txt"), "hello")

            const subDir = path.join(testCleanDir, "subdir")
            fs.mkdirSync(subDir)
            fs.writeFileSync(path.join(subDir, "another.txt"), "world")

            expect(fs.existsSync(testCleanDir)).toBe(true)

            lib.cleanOutputDir(testCleanDir)

            expect(fs.existsSync(testCleanDir)).toBe(false)
        })

        it("should clean the default public directory if no path is provided", () => {
            const defaultPublicDir = path.join(
                tempRootDir,
                "dashboard",
                "public"
            )
            fs.mkdirSync(defaultPublicDir, {recursive: true})
            fs.writeFileSync(path.join(defaultPublicDir, "file.txt"), "hello")

            expect(fs.existsSync(defaultPublicDir)).toBe(true)

            lib.cleanOutputDir()

            expect(fs.existsSync(defaultPublicDir)).toBe(false)
        })
    })

    describe("writeSkillDates", () => {
        it("should successfully generate skill_dates.toml from git dates", () => {
            mockExecFileSync.mockClear()
            mockLogWarn.mockClear()

            lib.writeSkillDates()

            // Verify writeSkillDates called git log
            expect(mockExecFileSync).toHaveBeenCalled()
            const gitCall = mockExecFileSync.mock.calls.find(
                (call) => call[0] === "git"
            )
            expect(gitCall).toBeDefined()
            expect(gitCall![1]).toContain("skills/category/name")

            const tomlPath = path.join(
                tempRootDir,
                "dashboard",
                "skill_dates.toml"
            )
            expect(fs.existsSync(tomlPath)).toBe(true)

            const tomlContent = fs.readFileSync(tomlPath, "utf8")
            expect(tomlContent).toContain("[dates]")
            expect(tomlContent).toContain('"category/name" = "2026-07-17"')
            expect(mockLogWarn).not.toHaveBeenCalled()
        })

        it("should log a warning if git fails, but still write an empty file", () => {
            mockExecFileSync.mockClear()
            mockLogWarn.mockClear()

            // Make git command throw an error
            mockExecFileSync.mockImplementation(() => {
                throw new Error("Git command not found or not a git repository")
            })

            try {
                lib.writeSkillDates()
            } finally {
                // Restore original mock implementation to not break subsequent tests
                mockExecFileSync.mockImplementation((cmd: string) => {
                    if (cmd === "git") {
                        return "2026-07-17\n"
                    }
                    return ""
                })
            }

            // Verify a warning was logged
            expect(mockLogWarn).toHaveBeenCalled()
            const warnCalls = mockLogWarn.mock.calls as any
            expect(warnCalls.length).toBeGreaterThan(0)

            // Check that the file was still written (and is empty)
            const tomlPath = path.join(
                tempRootDir,
                "dashboard",
                "skill_dates.toml"
            )
            expect(fs.existsSync(tomlPath)).toBe(true)

            const tomlContent = fs.readFileSync(tomlPath, "utf8")
            expect(tomlContent).toContain("[dates]")
            expect(tomlContent).not.toContain('"category/name"')
        })

        it("should log a warning if the dates map is empty", () => {
            mockExecFileSync.mockClear()
            mockLogWarn.mockClear()

            // Make git command return empty output
            mockExecFileSync.mockImplementation((cmd: string) => {
                if (cmd === "git") {
                    return ""
                }
                return ""
            })

            try {
                lib.writeSkillDates()
            } finally {
                // Restore original mock implementation
                mockExecFileSync.mockImplementation((cmd: string) => {
                    if (cmd === "git") {
                        return "2026-07-17\n"
                    }
                    return ""
                })
            }

            // Verify a warning was logged
            expect(mockLogWarn).toHaveBeenCalled()
            const warnCalls = mockLogWarn.mock.calls as any
            expect(warnCalls.length).toBe(1)
            expect(warnCalls[0][0]).toContain(
                "No last-modified dates were retrieved"
            )
        })
    })

    describe("buildCss", () => {
        it("should run tailwindcss with correct input and output paths", () => {
            mockExecFileSync.mockClear()

            lib.buildCss()

            expect(mockExecFileSync).toHaveBeenCalled()
            const tailwindCall = mockExecFileSync.mock.calls.find(
                (call) => call[0] === "tailwindcss"
            )
            expect(tailwindCall).toBeDefined()
            expect(tailwindCall![1]).toEqual([
                "-i",
                "dashboard/css/input.css",
                "-o",
                "dashboard/static/build/css/generated.css",
            ])
            expect(tailwindCall![2].cwd).toBe(tempRootDir)
        })
    })

    describe("build", () => {
        it("should run pagefind with output-subdir when serve option is not set", () => {
            mockExecFileSync.mockClear()

            lib.build()

            const pagefindCall = mockExecFileSync.mock.calls.find(
                (call) => call[0] === "pagefind"
            )
            expect(pagefindCall).toBeDefined()
            expect(pagefindCall![1]).toContain("--output-subdir")
            expect(pagefindCall![1]).toContain("pagefind")
        })

        it("should run pagefind with output-path when serve option is set to true", () => {
            mockExecFileSync.mockClear()

            lib.build({serve: true})

            const pagefindCall = mockExecFileSync.mock.calls.find(
                (call) => call[0] === "pagefind"
            )
            expect(pagefindCall).toBeDefined()
            expect(pagefindCall![1]).toContain("--output-path")
            expect(pagefindCall![1]).toContain("dashboard/static/pagefind")
        })

        it("should clean up dev pagefind directory before building Zola", () => {
            mockExecFileSync.mockClear()
            const devPagefindDir = path.join(
                tempRootDir,
                "dashboard",
                "static",
                "pagefind"
            )
            fs.mkdirSync(devPagefindDir, {recursive: true})
            fs.writeFileSync(path.join(devPagefindDir, "metadata.json"), "{}")

            expect(fs.existsSync(devPagefindDir)).toBe(true)

            lib.build()

            expect(fs.existsSync(devPagefindDir)).toBe(false)
        })
    })

    describe("run", () => {
        it("should print actionable error and exit 1 if tool is missing (ENOENT)", () => {
            const originalExit = process.exit
            const originalConsoleError = console.error

            const exitMock = mock((code?: number) => {
                throw new Error(`exit:${code}`)
            })
            const consoleErrorMock = mock((..._args: any[]) => {})

            process.exit = exitMock as any
            console.error = consoleErrorMock as any

            mockExecFileSync.mockClear()
            mockExecFileSync.mockImplementation(() => {
                const err = new Error("spawnSync ENOENT") as any
                err.code = "ENOENT"
                throw err
            })

            try {
                expect(() => lib.run("missing-tool", [])).toThrow("exit:1")
            } finally {
                process.exit = originalExit
                console.error = originalConsoleError

                // Restore default mock implementation
                mockExecFileSync.mockImplementation((cmd: string) => {
                    if (cmd === "git") {
                        return "2026-07-17\n"
                    }
                    return ""
                })
            }

            expect(consoleErrorMock).toHaveBeenCalled()
            const errCalls = consoleErrorMock.mock.calls as any
            expect(errCalls[0][0]).toContain("missing-tool")
            expect(errCalls[0][0]).toContain(
                "not found — run inside the devenv shell"
            )
        })

        it("should propagate other errors normally", () => {
            mockExecFileSync.mockClear()
            mockExecFileSync.mockImplementation(() => {
                throw new Error("Some other execution error")
            })

            try {
                expect(() => lib.run("tool", [])).toThrow(
                    "Some other execution error"
                )
            } finally {
                // Restore default mock implementation
                mockExecFileSync.mockImplementation((cmd: string) => {
                    if (cmd === "git") {
                        return "2026-07-17\n"
                    }
                    return ""
                })
            }
        })
    })
})
