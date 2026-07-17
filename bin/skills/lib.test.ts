import {describe, it, expect, afterAll} from "bun:test"
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
        it("should successfully clean up stale symlinks pointing to agent-skills repository", async () => {
            const cleanDir = path.join(tempHome, "clean-dir")
            const testRepo = path.join(tempHome, "test-repo")
            await fs.ensureDir(cleanDir)
            await fs.ensureDir(testRepo)

            await fs.writeJson(path.join(testRepo, "package.json"), {
                name: "agent-skills",
            })

            const skillTargetDir = path.join(
                testRepo,
                "skills",
                "engineering",
                "my-skill"
            )
            await fs.ensureDir(skillTargetDir)

            const staleLinkPath = path.join(cleanDir, "my-skill")
            await fs.ensureSymlink(skillTargetDir, staleLinkPath)

            const nonAgentRepo = path.join(tempHome, "non-agent-repo")
            await fs.ensureDir(nonAgentRepo)
            await fs.writeJson(path.join(nonAgentRepo, "package.json"), {
                name: "other-package",
            })
            const nonAgentTarget = path.join(
                nonAgentRepo,
                "skills",
                "engineering",
                "other-skill"
            )
            await fs.ensureDir(nonAgentTarget)
            const nonAgentLinkPath = path.join(cleanDir, "other-skill")
            await fs.ensureSymlink(nonAgentTarget, nonAgentLinkPath)

            const regularFilePath = path.join(cleanDir, "regular.txt")
            await fs.writeFile(regularFilePath, "hello")

            const removedCount = await cleanStaleLinks(cleanDir)
            expect(removedCount).toBe(1)

            expect(await fs.pathExists(staleLinkPath)).toBe(false)
            expect(await fs.pathExists(nonAgentLinkPath)).toBe(true)
            expect(await fs.pathExists(regularFilePath)).toBe(true)
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
})
