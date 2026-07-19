import {
    describe,
    it,
    expect,
    afterAll,
    beforeAll,
    beforeEach,
    spyOn,
} from "bun:test"
import fs from "fs-extra"
import fsPromises from "node:fs/promises"
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
    linkSkills,
    rewriteSkillLinks,
    registerAntigravity,
    ANTIGRAVITY_LOCK_FILE,
    ANTIGRAVITY_CONFIG_DIR,
    ANTIGRAVITY_SKILLS_JSON,
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

        it("should not release the lock file if it belongs to a different pid", async () => {
            await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)
            await fs.writeFile(ANTIGRAVITY_LOCK_FILE, "99999")
            await releaseLock()
            expect(await fs.pathExists(ANTIGRAVITY_LOCK_FILE)).toBe(true)
            await fs.remove(ANTIGRAVITY_LOCK_FILE)
        })

        it("should not break a stale lock file if it was replaced with a new lock in the meantime", async () => {
            await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)
            await fs.writeFile(ANTIGRAVITY_LOCK_FILE, "12345")
            const staleTime = new Date(Date.now() - 15000)
            await fs.utimes(ANTIGRAVITY_LOCK_FILE, staleTime, staleTime)

            let readCallCount = 0
            const spy = spyOn(fs, "readFile")
            spy.mockImplementation((async (path: any, options?: any) => {
                if (path === ANTIGRAVITY_LOCK_FILE) {
                    readCallCount++
                    if (readCallCount === 1) {
                        return "12345"
                    } else {
                        return "67890"
                    }
                }
                return fsPromises.readFile(path, options)
            }) as any)

            try {
                // Shorten the timeout for testing purposes by setting a mock or let it timeout.
                // Wait, if it takes 10 seconds to timeout, that is too slow for unit tests!
                // Let's temporarily change the timeout in process.env or stub it?
                // Wait! We can mock Date.now() or just check that it did not delete the lock file if we catch the timeout error,
                // but 10 seconds is too long. Let's make it timeout quickly by changing Date.now or similar?
                // Actually, wait! The loop checks: while (Date.now() - start < timeout)
                // If we mock Date.now() to return a time that advances instantly, we can make it timeout in 1 tick!
                const originalDateNow = Date.now
                let tick = 0
                Date.now = () => {
                    tick++
                    if (tick > 3) {
                        return originalDateNow() + 15000 // Force timeout
                    }
                    return originalDateNow()
                }

                try {
                    await expect(acquireLock()).rejects.toThrow()
                    // The lock file should NOT have been removed because we returned a different verify content
                    expect(await fs.pathExists(ANTIGRAVITY_LOCK_FILE)).toBe(
                        true
                    )
                } finally {
                    Date.now = originalDateNow
                }
            } finally {
                spy.mockRestore()
                await fs.remove(ANTIGRAVITY_LOCK_FILE)
            }
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

            // Helper to check if file/symlink physically exists on disk (does not follow symlink)
            async function fileExistsRaw(p: string) {
                try {
                    await fs.lstat(p)
                    return true
                } catch {
                    return false
                }
            }

            // Assertions
            expect(removedCount).toBe(2) // validSymlink and brokenSymlink should be deleted
            expect(await fs.pathExists(normalFile)).toBe(true)
            expect(await fs.pathExists(validSymlink)).toBe(false)
            expect(await fileExistsRaw(brokenSymlink)).toBe(false)
            expect(await fs.pathExists(invalidLayoutSymlink)).toBe(true)
            expect(await fs.pathExists(diffSymlink)).toBe(true)

            // Clean up temp directories
            await fs.remove(tempTestDir)
        })

        it("should preserve healthy symlink to foreign repo without package.json, but remove broken ones", async () => {
            await fs.remove(tempTestDir)
            await fs.ensureDir(tempTestDir)

            // Helper to check if file/symlink physically exists on disk (does not follow symlink)
            async function existsRaw(p: string) {
                try {
                    await fs.lstat(p)
                    return true
                } catch {
                    return false
                }
            }

            // 1. Healthy foreign symlink with no package.json
            const healthyForeignRepo = path.join(tempTestDir, "healthy-foreign")
            const healthyForeignSkillPath = path.join(
                healthyForeignRepo,
                "skills",
                "engineering",
                "foreign-skill"
            )
            await fs.ensureDir(healthyForeignSkillPath)
            const healthyForeignSymlink = path.join(
                tempTestDir,
                "healthy-foreign-symlink"
            )
            await fs.symlink(healthyForeignSkillPath, healthyForeignSymlink)

            // 2. Broken symlink with no package.json
            const brokenRepo = path.join(tempTestDir, "broken-repo")
            const brokenSkillPath = path.join(
                brokenRepo,
                "skills",
                "engineering",
                "broken-skill"
            )
            // Target is not created on disk
            const brokenSymlink = path.join(tempTestDir, "broken-symlink")
            await fs.symlink(brokenSkillPath, brokenSymlink)

            // Run cleanup
            const removed = await cleanStaleLinks(tempTestDir)

            expect(removed).toBe(1)
            expect(await existsRaw(healthyForeignSymlink)).toBe(true)
            expect(await existsRaw(brokenSymlink)).toBe(false)

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
                super(`Exit called with ${code}`)
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

        afterAll(async () => {
            process.exit = originalExit
            const clack = await import("@clack/prompts")
            clack.log.error = originalLogError
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

        it("should fail linting when skill content contains absolute file:/// link", async () => {
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
Some content with [link](file:///home/user/path)`
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
                loggedErrors.some(
                    (err) =>
                        err.includes("contains absolute file:/// URL") &&
                        err.includes("CRITICAL LINKING RULE")
                )
            ).toBe(true)
        })

        it("should pass linting when file:/// is in a code block but not as a markdown link", async () => {
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
Some code block:
\`\`\`rust
let path = "file:///foo/bar";
\`\`\`
`
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

        it("should fail linting and record error for missing frontmatter delimiters", async () => {
            const skillPath = path.join(
                SKILLS_DIR,
                "engineering",
                "malformed-skill",
                "SKILL.md"
            )
            await fs.ensureDir(path.dirname(skillPath))
            await fs.writeFile(
                skillPath,
                `name: malformed-skill
description: A skill missing delimiters
---
Some content`
            )

            // Verify getSkills directly
            const {getSkills} = await import("./lib.ts")
            const skills = await getSkills()
            const malformed = skills.find(
                (s) => s.dirName === "malformed-skill"
            )
            expect(malformed).toBeDefined()
            expect(malformed!.yamlError).toBe(
                "Missing or malformed YAML frontmatter delimiters"
            )

            // Verify lint fails
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
                    err.includes(
                        "YAML frontmatter syntax error: Missing or malformed YAML frontmatter delimiters"
                    )
                )
            ).toBe(true)
        })
    })

    describe("registerAntigravity", () => {
        beforeEach(async () => {
            await fs.remove(ANTIGRAVITY_SKILLS_JSON)
            await fs.remove(`${ANTIGRAVITY_SKILLS_JSON}.bak`)
        })

        it("should parse valid JSON config successfully", async () => {
            await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)
            const initialConfig = {entries: [], otherKey: "value"}
            await fs.writeJson(ANTIGRAVITY_SKILLS_JSON, initialConfig)

            await registerAntigravity()

            const updatedConfig = await fs.readJson(ANTIGRAVITY_SKILLS_JSON)
            expect(updatedConfig.$schema).toBe(
                "https://skills.sh/schemas/skills.sh.schema.json"
            )
            expect(Array.isArray(updatedConfig.entries)).toBe(true)
            expect(await fs.pathExists(`${ANTIGRAVITY_SKILLS_JSON}.bak`)).toBe(
                false
            )
        })

        it("should backup and reset config for null config", async () => {
            await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)
            await fs.writeFile(ANTIGRAVITY_SKILLS_JSON, "null")

            await registerAntigravity()

            const updatedConfig = await fs.readJson(ANTIGRAVITY_SKILLS_JSON)
            expect(updatedConfig.$schema).toBe(
                "https://skills.sh/schemas/skills.sh.schema.json"
            )
            expect(Array.isArray(updatedConfig.entries)).toBe(true)

            expect(await fs.pathExists(`${ANTIGRAVITY_SKILLS_JSON}.bak`)).toBe(
                true
            )
            const backupContent = await fs.readFile(
                `${ANTIGRAVITY_SKILLS_JSON}.bak`,
                "utf-8"
            )
            expect(backupContent).toBe("null")
        })

        it("should backup and reset config for primitive values", async () => {
            await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)

            // Test string value
            await fs.writeFile(ANTIGRAVITY_SKILLS_JSON, `"string value"`)
            await registerAntigravity()
            expect(await fs.pathExists(`${ANTIGRAVITY_SKILLS_JSON}.bak`)).toBe(
                true
            )
            expect(
                await fs.readFile(`${ANTIGRAVITY_SKILLS_JSON}.bak`, "utf-8")
            ).toBe(`"string value"`)
            await fs.remove(`${ANTIGRAVITY_SKILLS_JSON}.bak`)

            // Test number value
            await fs.writeFile(ANTIGRAVITY_SKILLS_JSON, `12345`)
            await registerAntigravity()
            expect(await fs.pathExists(`${ANTIGRAVITY_SKILLS_JSON}.bak`)).toBe(
                true
            )
            expect(
                await fs.readFile(`${ANTIGRAVITY_SKILLS_JSON}.bak`, "utf-8")
            ).toBe(`12345`)
            await fs.remove(`${ANTIGRAVITY_SKILLS_JSON}.bak`)

            // Test boolean value
            await fs.writeFile(ANTIGRAVITY_SKILLS_JSON, `true`)
            await registerAntigravity()
            expect(await fs.pathExists(`${ANTIGRAVITY_SKILLS_JSON}.bak`)).toBe(
                true
            )
            expect(
                await fs.readFile(`${ANTIGRAVITY_SKILLS_JSON}.bak`, "utf-8")
            ).toBe(`true`)
        })

        it("should backup and reset config for an array config", async () => {
            await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)
            await fs.writeFile(ANTIGRAVITY_SKILLS_JSON, "[]")

            await registerAntigravity()

            const updatedConfig = await fs.readJson(ANTIGRAVITY_SKILLS_JSON)
            expect(updatedConfig.$schema).toBe(
                "https://skills.sh/schemas/skills.sh.schema.json"
            )
            expect(Array.isArray(updatedConfig.entries)).toBe(true)

            expect(await fs.pathExists(`${ANTIGRAVITY_SKILLS_JSON}.bak`)).toBe(
                true
            )
            const backupContent = await fs.readFile(
                `${ANTIGRAVITY_SKILLS_JSON}.bak`,
                "utf-8"
            )
            expect(backupContent).toBe("[]")
        })

        it("should backup and reset config when entries is not an array", async () => {
            await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)
            const invalidConfig = {entries: "not-an-array", otherKey: "value"}
            await fs.writeJson(ANTIGRAVITY_SKILLS_JSON, invalidConfig)

            await registerAntigravity()

            const updatedConfig = await fs.readJson(ANTIGRAVITY_SKILLS_JSON)
            expect(updatedConfig.$schema).toBe(
                "https://skills.sh/schemas/skills.sh.schema.json"
            )
            expect(Array.isArray(updatedConfig.entries)).toBe(true)

            expect(await fs.pathExists(`${ANTIGRAVITY_SKILLS_JSON}.bak`)).toBe(
                true
            )
            const backupConfig = await fs.readJson(
                `${ANTIGRAVITY_SKILLS_JSON}.bak`
            )
            expect(backupConfig.entries).toBe("not-an-array")
        })
    })

    describe("sync", () => {
        beforeEach(async () => {
            await fs.remove(SKILLS_DIR)
            await fs.ensureDir(SKILLS_DIR)
            const {AGENTS_FILE} = await import("./lib.ts")
            await fs.ensureDir(path.dirname(AGENTS_FILE))
            await fs.writeFile(AGENTS_FILE, "---\nlayout: default\n---\n")
        })

        it("should reject sync if a skill is missing description", async () => {
            const skillPath = path.join(
                SKILLS_DIR,
                "engineering",
                "missing-desc-skill",
                "SKILL.md"
            )
            await fs.ensureDir(path.dirname(skillPath))
            await fs.writeFile(
                skillPath,
                `---
name: missing-desc-skill
---
Some content`
            )

            const {sync} = await import("./sync.ts")
            expect(sync()).rejects.toThrow(
                "Cannot sync: One or more skills are missing mandatory 'name' or 'description' fields."
            )
        })

        it("should reject sync if a skill is missing name", async () => {
            const skillPath = path.join(
                SKILLS_DIR,
                "engineering",
                "missing-name-skill",
                "SKILL.md"
            )
            await fs.ensureDir(path.dirname(skillPath))
            await fs.writeFile(
                skillPath,
                `---
description: Missing name skill
---
Some content`
            )

            const {sync} = await import("./sync.ts")
            expect(sync()).rejects.toThrow(
                "Cannot sync: One or more skills are missing mandatory 'name' or 'description' fields."
            )
        })

        it("should succeed sync if all skills are valid", async () => {
            const skillPath = path.join(
                SKILLS_DIR,
                "engineering",
                "valid-skill",
                "SKILL.md"
            )
            await fs.ensureDir(path.dirname(skillPath))
            await fs.writeFile(
                skillPath,
                `---
name: valid-skill
description: A perfectly valid skill
---
Some content`
            )

            const {sync} = await import("./sync.ts")
            const {AGENTS_FILE, README_FILE} = await import("./lib.ts")
            await sync()

            const readmeContent = await fs.readFile(README_FILE, "utf-8")
            expect(readmeContent).toContain("A perfectly valid skill")

            const agentsContent = await fs.readFile(AGENTS_FILE, "utf-8")
            expect(agentsContent).toContain("A perfectly valid skill")
            expect(agentsContent).not.toContain("undefined")
        })
    })

    describe("linkSkills", () => {
        const claudeDir = path.join(tempHome, ".claude")
        const claudeSkillsDir = path.join(claudeDir, "skills")
        const testSkillDir = path.join(
            SKILLS_DIR,
            "engineering",
            "conflict-skill"
        )
        const testSkillFile = path.join(testSkillDir, "SKILL.md")

        beforeEach(async () => {
            // Clean up and recreate test environment
            await fs.remove(claudeDir)
            await fs.remove(path.join(tempHome, ".gemini"))
            await fs.remove(path.join(tempHome, ".config"))
            await fs.remove(SKILLS_DIR)
            await fs.ensureDir(claudeDir)
            await fs.ensureDir(testSkillDir)
            await fs.writeFile(
                testSkillFile,
                `---\nname: conflict-skill\ndescription: A conflict test skill\n---\nTest Content`
            )
        })

        it("should link skills when no conflict exists", async () => {
            const res = await linkSkills()
            expect(res.linked).toBe(1)
            expect(res.conflicts).toBe(0)

            const destPath = path.join(claudeSkillsDir, "conflict-skill")
            const stat = await fs.lstat(destPath)
            expect(stat.isSymbolicLink()).toBe(true)
        })

        it("should preserve real directory at destination and report conflict", async () => {
            const destPath = path.join(claudeSkillsDir, "conflict-skill")
            await fs.ensureDir(destPath)

            const canaryFile = path.join(destPath, "canary.txt")
            await fs.writeFile(canaryFile, "do-not-delete")

            const res = await linkSkills()
            expect(res.linked).toBe(0)
            expect(res.conflicts).toBe(1)

            // Verify real directory and its files are still there
            const stat = await fs.lstat(destPath)
            expect(stat.isSymbolicLink()).toBe(false)
            expect(stat.isDirectory()).toBe(true)
            expect(await fs.pathExists(canaryFile)).toBe(true)
        })
    })
})
