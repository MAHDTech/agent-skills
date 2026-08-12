import {describe, it, expect, afterEach} from "bun:test"
import fs from "fs-extra"
import path from "path"
import os from "os"
import {checkDuplicateSkills, checkResourcesLayout} from "./lint.ts"
import type {Skill} from "./lib.ts"

describe("checkDuplicateSkills", () => {
    it("should return no errors or warnings for unique skill names", () => {
        const skills: Skill[] = [
            {
                path: "skills/engineering/skill-a/SKILL.md",
                dirName: "skill-a",
                category: "engineering",
                promoted: true,
                metadata: {name: "skill-a", description: "Desc a"},
                content: "",
                raw: "",
            },
            {
                path: "skills/tooling/skill-b/SKILL.md",
                dirName: "skill-b",
                category: "tooling",
                promoted: true,
                metadata: {name: "skill-b", description: "Desc b"},
                content: "",
                raw: "",
            },
        ]

        const result = checkDuplicateSkills(skills)
        expect(result.errors).toHaveLength(0)
        expect(result.warnings).toHaveLength(0)
    })

    it("should return an error when two promoted skills share a dirName", () => {
        const skills: Skill[] = [
            {
                path: "skills/engineering/duplicate-skill/SKILL.md",
                dirName: "duplicate-skill",
                category: "engineering",
                promoted: true,
                metadata: {name: "duplicate-skill", description: "Desc a"},
                content: "",
                raw: "",
            },
            {
                path: "skills/tooling/duplicate-skill/SKILL.md",
                dirName: "duplicate-skill",
                category: "tooling",
                promoted: true,
                metadata: {name: "duplicate-skill", description: "Desc b"},
                content: "",
                raw: "",
            },
        ]

        const result = checkDuplicateSkills(skills)
        expect(result.errors).toHaveLength(1)
        expect(result.errors[0]).toContain(
            "Duplicate skill name 'duplicate-skill' detected in multiple promoted categories"
        )
        expect(result.warnings).toHaveLength(0)
    })

    it("should return a warning when duplicate involves a lifecycle skill", () => {
        const skills: Skill[] = [
            {
                path: "skills/engineering/duplicate-skill/SKILL.md",
                dirName: "duplicate-skill",
                category: "engineering",
                promoted: true,
                metadata: {name: "duplicate-skill", description: "Desc a"},
                content: "",
                raw: "",
            },
            {
                path: "skills/in-progress/duplicate-skill/SKILL.md",
                dirName: "duplicate-skill",
                category: "in-progress",
                promoted: false,
                metadata: {name: "duplicate-skill", description: "Desc b"},
                content: "",
                raw: "",
            },
        ]

        const result = checkDuplicateSkills(skills)
        expect(result.errors).toHaveLength(0)
        expect(result.warnings).toHaveLength(1)
        expect(result.warnings[0]).toContain(
            "Duplicate skill name 'duplicate-skill' detected in lifecycle/promoted categories"
        )
    })
})

describe("checkResourcesLayout", () => {
    const tmp = path.join(
        os.tmpdir(),
        `agent-skills-lint-resources-test-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`
    )

    afterEach(async () => {
        await fs.remove(tmp)
    })

    it("returns no errors when resources/ does not exist", async () => {
        const errors = await checkResourcesLayout(path.join(tmp, "resources"))
        expect(errors).toEqual([])
    })

    it("returns no errors for a valid auto/ + manual/ layout", async () => {
        const resDir = path.join(tmp, "resources")
        await fs.ensureDir(path.join(resDir, "auto"))
        await fs.ensureDir(path.join(resDir, "manual"))
        expect(await checkResourcesLayout(resDir)).toEqual([])
    })

    it("flags a bare file placed directly in resources/", async () => {
        const resDir = path.join(tmp, "resources")
        await fs.ensureDir(path.join(resDir, "auto"))
        await fs.writeFile(path.join(resDir, "stray.md"), "x")
        const errors = await checkResourcesLayout(resDir)
        expect(errors).toHaveLength(1)
        expect(errors[0]).toContain("must not contain files directly")
        expect(errors[0]).toContain("stray.md")
    })

    it("flags a disallowed subdirectory under resources/", async () => {
        const resDir = path.join(tmp, "resources")
        await fs.ensureDir(path.join(resDir, "docs"))
        const errors = await checkResourcesLayout(resDir)
        expect(errors).toHaveLength(1)
        expect(errors[0]).toContain("only contain 'auto/' and 'manual/'")
        expect(errors[0]).toContain("docs/")
    })

    it("flags index.md inside resources/ subdirectories due to Zola collisions", async () => {
        const resDir = path.join(tmp, "resources")
        await fs.ensureDir(path.join(resDir, "auto"))
        await fs.writeFile(path.join(resDir, "auto", "index.md"), "x")
        const errors = await checkResourcesLayout(resDir)
        expect(errors).toHaveLength(1)
        expect(errors[0]).toContain("disallowed inside resources/")
        expect(errors[0]).toContain("index.md")
    })
})
