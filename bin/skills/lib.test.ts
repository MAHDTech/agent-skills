import {describe, it, expect, afterAll, beforeAll, beforeEach} from "bun:test"
import fs from "fs-extra"
import path from "path"
import os from "os"

// Set environment variables BEFORE importing lib.ts
const tempHome = path.join(
    os.tmpdir(),
    `agent-skills-test-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`
)
fs.ensureDirSync(tempHome)
process.env.HOME = tempHome

const fakeRepo = path.join(tempHome, "repo")
fs.ensureDirSync(fakeRepo)
process.env.AGENT_SKILLS_HOME = fakeRepo

// Import dynamically to ensure environment variables are set before evaluation of lib.ts
const {
    acquireLock,
    releaseLock,
    cleanStaleLinks,
    rewriteSkillLinks,
    ANTIGRAVITY_LOCK_FILE,
    ANTIGRAVITY_CONFIG_DIR,
    SKILLS_DIR,
} = await import("./lib.ts")

describe("Skills unit tests", () => {
    afterAll(() => {
        try {
            fs.removeSync(tempHome)
        } catch {
            // Ignore clean up errors
        }
    })

    describe("acquireLock and releaseLock", () => {
        it("should acquire and release the lock file", async () => {
            await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)
            expect(await fs.pathExists(ANTIGRAVITY_LOCK_FILE)).toBe(false)

            await acquireLock()
            expect(await fs.pathExists(ANTIGRAVITY_LOCK_FILE)).toBe(true)

            const content = await fs.readFile(ANTIGRAVITY_LOCK_FILE, "utf8")
            expect(content).toBe(String(process.pid))

            await releaseLock()
            expect(await fs.pathExists(ANTIGRAVITY_LOCK_FILE)).toBe(false)
        })

        it("should break a stale lock file and acquire it", async () => {
            await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)

            await fs.writeFile(ANTIGRAVITY_LOCK_FILE, "12345")
            const staleTime = new Date(Date.now() - 15000) // 15 seconds ago
            await fs.utimes(ANTIGRAVITY_LOCK_FILE, staleTime, staleTime)

            await acquireLock()
            expect(await fs.pathExists(ANTIGRAVITY_LOCK_FILE)).toBe(true)

            const content = await fs.readFile(ANTIGRAVITY_LOCK_FILE, "utf8")
            expect(content).toBe(String(process.pid))

            await releaseLock()
        })
    })

    describe("cleanStaleLinks", () => {
        const tempTestDir = path.join(tempHome, "temp-clean-test")

        it("should clean stale and broken links, but preserve others", async () => {
            // Ensure clean temp directory
            await fs.remove(tempTestDir)
            await fs.ensureDir(tempTestDir)

            // 1. Create a normal file (should not be deleted)
            const normalFile = path.join(tempTestDir, "normal.txt")
            await fs.writeFile(normalFile, "hello")

            // 2. Create a valid symlink to an agent-skills repo structure
            const validRepoRoot = path.join(tempTestDir, "valid-repo")
            await fs.ensureDir(validRepoRoot)
            await fs.writeJson(path.join(validRepoRoot, "package.json"), {
                name: "agent-skills",
            })
            const validSkillPath = path.join(
                validRepoRoot,
                "skills",
                "engineering",
                "some-skill"
            )
            await fs.ensureDir(validSkillPath)

            const validSymlink = path.join(tempTestDir, "valid-symlink")
            await fs.symlink(validSkillPath, validSymlink)

            // 3. Create a broken symlink (target deleted, but conforms to category/name layout)
            const deletedRepoRoot = path.join(tempTestDir, "deleted-repo")
            const deletedSkillPath = path.join(
                deletedRepoRoot,
                "skills",
                "engineering",
                "another-skill"
            )
            // Don't actually create the target, just create the symlink pointing to it
            const brokenSymlink = path.join(tempTestDir, "broken-symlink")
            await fs.symlink(deletedSkillPath, brokenSymlink)

            // 4. Create a symlink that does NOT match category/name layout
            const invalidSkillPath = path.join(
                validRepoRoot,
                "skills",
                "engineering",
                "nested",
                "too-deep"
            )
            await fs.ensureDir(invalidSkillPath)
            const invalidLayoutSymlink = path.join(
                tempTestDir,
                "invalid-layout-symlink"
            )
            await fs.symlink(invalidSkillPath, invalidLayoutSymlink)

            // 5. Create a symlink to a different package name
            const diffRepoRoot = path.join(tempTestDir, "diff-repo")
            await fs.ensureDir(diffRepoRoot)
            await fs.writeJson(path.join(diffRepoRoot, "package.json"), {
                name: "other-package",
            })
            const diffSkillPath = path.join(
                diffRepoRoot,
                "skills",
                "engineering",
                "diff-skill"
            )
            await fs.ensureDir(diffSkillPath)
            const diffSymlink = path.join(tempTestDir, "diff-symlink")
            await fs.symlink(diffSkillPath, diffSymlink)

            // Run cleanup
            const removedCount = await cleanStaleLinks(tempTestDir)

            // Assertions
            expect(removedCount).toBe(2) // validSymlink and brokenSymlink should be deleted
            expect(await fs.pathExists(normalFile)).toBe(true)
            expect(await fs.pathExists(validSymlink)).toBe(false)
            expect(await fs.pathExists(brokenSymlink)).toBe(false)
            expect(await fs.pathExists(invalidLayoutSymlink)).toBe(true)
            expect(await fs.pathExists(diffSymlink)).toBe(true)

            // Clean up temp directories
            await fs.remove(tempTestDir)
        })
    })

    describe("rewriteSkillLinks", () => {
        it("should rewrite relative links to Zola site format if they exist", async () => {
            const srcDir = path.join(SKILLS_DIR, "engineering", "my-skill")
            await fs.ensureDir(srcDir)

            const targetSkillDir = path.join(
                SKILLS_DIR,
                "engineering",
                "target-skill"
            )
            await fs.ensureDir(targetSkillDir)
            const targetSkillFile = path.join(targetSkillDir, "SKILL.md")
            await fs.writeFile(targetSkillFile, "# Target Skill")

            const inputContent =
                "Check [this](../target-skill/SKILL.md) out and also [external](http://google.com)"

            const output = rewriteSkillLinks(
                inputContent,
                "skills/engineering/my-skill",
                srcDir
            )

            expect(output).toContain(
                "Check [this](@/skills/engineering/target-skill/_index.md) out"
            )
            expect(output).toContain("[external](http://google.com)")
        })
    })

    describe("lint", () => {
        let originalExit: typeof process.exit
        let originalLogError: any
        let loggedErrors: string[] = []
        let exitCode: number | undefined = undefined

        class ExitError extends Error {
            constructor(public code: number) {
                super(`Exit called with \${code}`)
            }
        }

        beforeAll(async () => {
            originalExit = process.exit
            process.exit = (code?: number) => {
                throw new ExitError(code ?? 0)
            }

            const clack = await import("@clack/prompts")
            originalLogError = clack.log.error
            clack.log.error = (msg: string) => {
                loggedErrors.push(msg)
            }
        })

        afterAll(() => {
            process.exit = originalExit
            import("@clack/prompts").then((clack) => {
                clack.log.error = originalLogError
            })
        })

        beforeEach(async () => {
            loggedErrors = []
            exitCode = undefined
            await fs.remove(SKILLS_DIR)
            await fs.ensureDir(SKILLS_DIR)
        })

        it("should pass linting for standard skill paths", async () => {
            const skillPath = path.join(
                SKILLS_DIR,
                "engineering",
                "my-skill",
                "SKILL.md"
            )
            await fs.ensureDir(path.dirname(skillPath))
            await fs.writeFile(
                skillPath,
                `---
name: my-skill
description: A standard test skill
---
Some content`
            )

            const {lint} = await import("./lint.ts")
            try {
                await lint()
            } catch (e) {
                if (e instanceof ExitError) {
                    exitCode = e.code
                } else {
                    throw e
                }
            }

            expect(exitCode).toBeUndefined()
            expect(loggedErrors.length).toBe(0)
        })

        it("should fail linting for nested skill paths", async () => {
            const skillPath = path.join(
                SKILLS_DIR,
                "engineering",
                "nested",
                "my-skill",
                "SKILL.md"
            )
            await fs.ensureDir(path.dirname(skillPath))
            await fs.writeFile(
                skillPath,
                `---
name: my-skill
description: A nested test skill
---
Some content`
            )

            const {lint} = await import("./lint.ts")
            try {
                await lint()
            } catch (e) {
                if (e instanceof ExitError) {
                    exitCode = e.code
                } else {
                    throw e
                }
            }

            expect(exitCode).toBe(1)
            expect(
                loggedErrors.some((err) =>
                    err.includes("must reside exactly at skills/")
                )
            ).toBe(true)
        })

        it("should fail linting for shallow skill paths", async () => {
            const skillPath = path.join(SKILLS_DIR, "SKILL.md")
            await fs.ensureDir(path.dirname(skillPath))
            await fs.writeFile(
                skillPath,
                `---
name: my-skill
description: A shallow test skill
---
Some content`
            )

            const {lint} = await import("./lint.ts")
            try {
                await lint()
            } catch (e) {
                if (e instanceof ExitError) {
                    exitCode = e.code
                } else {
                    throw e
                }
            }

            expect(exitCode).toBe(1)
            expect(
                loggedErrors.some((err) =>
                    err.includes("must reside exactly at skills/")
                )
            ).toBe(true)
        })
    })
})
