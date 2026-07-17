import {describe, it, expect, mock, afterEach, beforeEach} from "bun:test"
import fs from "fs-extra"
import path from "path"
import os from "os"

// Set up temp home and AGENT_SKILLS_HOME
const tempHome = path.join(
    os.tmpdir(),
    `agent-skills-downloader-test-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`
)
fs.ensureDirSync(tempHome)

const fakeRepo = path.join(tempHome, "repo")
fs.ensureDirSync(fakeRepo)

// We want to dynamically import downloader.ts after setting env vars
process.env.AGENT_SKILLS_HOME = fakeRepo

const {downloadAction, getDomainPrefix} = await import("./downloader.ts")

describe("downloader unit tests", () => {
    let originalFetch: typeof globalThis.fetch

    beforeEach(() => {
        originalFetch = globalThis.fetch
    })

    afterEach(() => {
        globalThis.fetch = originalFetch
        try {
            fs.removeSync(tempHome)
        } catch {
            // ignore
        }
    })

    it("should detect HTML responses from candidate URLs and convert them", async () => {
        const mockSkills = [
            {
                path: "skills/engineering/test-skill/SKILL.md",
                dirName: "test-skill",
                category: "engineering",
                promoted: false,
                metadata: {
                    name: "test-skill",
                    description: "Test skill description",
                    resources: ["https://antigravity.google/docs/llms.txt"],
                },
                content: "",
            },
        ]

        // Mock fetch
        globalThis.fetch = mock(async (url: any) => {
            const urlStr = String(url)
            if (urlStr.endsWith("/llms.txt")) {
                // Return index content pointing to subpage
                return new Response(
                    `- [subpage](https://antigravity.google/docs/subpage)`,
                    {
                        status: 200,
                        headers: {"content-type": "text/plain"},
                    }
                )
            }
            if (urlStr.includes("/assets/docs/")) {
                // This is the candidate raw asset URL
                return new Response(
                    `<!DOCTYPE html><html><body><h1>404 Not Found (SPA fallback)</h1><p>This is some extra paragraphs to ensure that the content is long enough to pass the min content bytes check of fifty bytes.</p></body></html>`,
                    {
                        status: 200,
                        headers: {"content-type": "text/html"},
                    }
                )
            }
            // For other requests (main URL), return markdown (though candidate will be hit first)
            return new Response(
                `# Main URL Content\nThis is the markdown from main URL.`,
                {
                    status: 200,
                    headers: {"content-type": "text/markdown"},
                }
            )
        }) as any

        // Run downloadAction
        await downloadAction(mockSkills, fakeRepo)

        // Verify the file was downloaded, converted to markdown (not saved as raw HTML)
        const resourcesDir = path.join(
            fakeRepo,
            "skills/engineering/test-skill/resources"
        )
        const files = await fs.readdir(resourcesDir)
        // The downloaded index and its linked resources should be saved
        expect(files.length).toBe(2)
        expect(files).toContain("docs-llms.txt")
        expect(files).toContain("subpage.md")

        const indexContent = await fs.readFile(
            path.join(resourcesDir, "docs-llms.txt"),
            "utf-8"
        )
        expect(indexContent).toBe(
            "- [subpage](https://antigravity.google/docs/subpage)"
        )

        const subpageContent = await fs.readFile(
            path.join(resourcesDir, "subpage.md"),
            "utf8"
        )
        // It shouldn't contain raw HTML tags because we ran convertHtmlToMarkdown
        expect(subpageContent).not.toContain("<!DOCTYPE html>")
        expect(subpageContent).not.toContain("<html>")
        // And it should have the converted text (via pandoc)
        expect(subpageContent).toContain("404 Not Found (SPA fallback)")
        expect(subpageContent).toContain("This is some extra paragraphs")
    })

    describe("getDomainPrefix unit tests", () => {
        it("should correctly handle standard TLDs", () => {
            expect(getDomainPrefix("https:/" + "/example.com/bar")).toBe(
                "example"
            )
            expect(getDomainPrefix("https:/" + "/sub.domain.org/path")).toBe(
                "domain"
            )
        })

        it("should correctly handle multi-part TLDs", () => {
            expect(getDomainPrefix("https:/" + "/foo.co.uk/bar")).toBe("foo")
            expect(getDomainPrefix("https:/" + "/sub.example.co.uk/bar")).toBe(
                "example"
            )
            expect(getDomainPrefix("https:/" + "/example.com.cn/bar")).toBe(
                "example"
            )
            expect(getDomainPrefix("https:/" + "/another.org.uk/test")).toBe(
                "another"
            )
        })

        it("should correctly handle IP addresses", () => {
            expect(getDomainPrefix("https:/" + "/127.0.0.1/bar")).toBe(
                "127.0.0.1"
            )
            expect(getDomainPrefix("https:/" + "/[::1]/bar")).toBe("[::1]")
        })

        it("should return empty string or fallback on invalid URLs", () => {
            expect(getDomainPrefix("invalid-url")).toBe("")
        })
    })
})
