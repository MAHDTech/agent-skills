import {describe, it, expect} from "bun:test"
import fs from "fs-extra"
import path from "path"
import {cleanStaleLinks} from "./lib.ts"

describe("cleanStaleLinks", () => {
    const tempTestDir = path.join(import.meta.dir, "temp-clean-test")

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
