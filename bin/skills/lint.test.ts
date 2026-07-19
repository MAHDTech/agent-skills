import {describe, it, expect} from "bun:test"
import {checkDuplicateSkills} from "./lint.ts"
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
