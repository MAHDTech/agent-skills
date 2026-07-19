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

const {
    downloadAction,
    getDomainPrefix,
    smartSlugify,
    getCommonPrefix,
    parseLlmsTxtLinks,
    checkPathTraversal,
} = await import("./downloader.ts")

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

        // Run downloadAction with a mock HTML-to-Markdown converter to avoid calling pandoc
        await downloadAction(mockSkills, fakeRepo, {
            convertHtmlToMarkdown: async (html) => html.replace(/<[^>]+>/g, ""),
        })

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

    it("should throw an error when one or more downloads fail", async () => {
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

        // Mock fetch to fail
        globalThis.fetch = mock(async () => {
            return new Response("Internal Server Error", {
                status: 500,
                headers: {"content-type": "text/plain"},
            })
        }) as any

        // Run downloadAction and assert it rejects with the error
        await expect(downloadAction(mockSkills, fakeRepo)).rejects.toThrow(
            "Download resources failed: 1 file(s) failed to download."
        )
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

    describe("smartSlugify unit tests", () => {
        it("should format paths replacing slashes with dashes and stripping invalid characters", () => {
            expect(smartSlugify("https:/" + "/example.com/docs/a/b")).toBe(
                "docs-a-b.md"
            )
            expect(smartSlugify("https:/" + "/example.com/docs/a-b")).toBe(
                "docs-a-b.md"
            )
        })

        it("should handle differing extensions and map to .md", () => {
            expect(smartSlugify("https:/" + "/example.com/docs/a.md")).toBe(
                "docs-a.md"
            )
            expect(smartSlugify("https:/" + "/example.com/docs/a.html")).toBe(
                "docs-a.md"
            )
            expect(smartSlugify("https:/" + "/example.com/docs/a")).toBe(
                "docs-a.md"
            )
        })

        it("should fall back to index.md for empty slug from non-ASCII paths", () => {
            expect(smartSlugify("https:/" + "/example.com/日本語/")).toBe(
                "index.md"
            )
        })
    })

    it("should resolve same-domain filename collisions by appending a short hash of the URL", async () => {
        const mockSkills = [
            {
                path: "skills/engineering/test-skill/SKILL.md",
                dirName: "test-skill",
                category: "engineering",
                promoted: false,
                metadata: {
                    name: "test-skill",
                    description: "Test skill description",
                    resources: [
                        "https://example.com/docs/a/b",
                        "https://example.com/docs/a-b",
                    ],
                },
                content: "",
            },
        ]

        // Mock fetch to succeed
        globalThis.fetch = mock(async () => {
            return new Response(
                "Some mock resource content which is longer than fifty bytes to satisfy the size check.",
                {
                    status: 200,
                    headers: {"content-type": "text/plain"},
                }
            )
        }) as any

        await downloadAction(mockSkills, fakeRepo)

        const resourcesDir = path.join(
            fakeRepo,
            "skills/engineering/test-skill/resources"
        )
        const files = await fs.readdir(resourcesDir)

        // They should have resolved to distinct files since both originally mapped to docs-a-b.md
        expect(files.length).toBe(2)

        // Each file name should be docs-a-b-[hash].md
        for (const file of files) {
            expect(file).toMatch(/^docs-a-b-[a-f0-9]{8}\.md$/)
        }
    })

    it("should resolve empty-slug fallback collisions (e.g. non-ASCII paths) by appending hash to index.md", async () => {
        const mockSkills = [
            {
                path: "skills/engineering/test-skill/SKILL.md",
                dirName: "test-skill",
                category: "engineering",
                promoted: false,
                metadata: {
                    name: "test-skill",
                    description: "Test skill description",
                    resources: [
                        "https://example.com/日本語/",
                        "https://example.com/简体中文/",
                    ],
                },
                content: "",
            },
        ]

        // Mock fetch to succeed
        globalThis.fetch = mock(async () => {
            return new Response(
                "Some mock resource content which is longer than fifty bytes to satisfy the size check.",
                {
                    status: 200,
                    headers: {"content-type": "text/plain"},
                }
            )
        }) as any

        await downloadAction(mockSkills, fakeRepo)

        const resourcesDir = path.join(
            fakeRepo,
            "skills/engineering/test-skill/resources"
        )
        const files = await fs.readdir(resourcesDir)

        // They should have resolved to distinct files since both originally mapped to index.md
        expect(files.length).toBe(2)

        // Each file name should be index-[hash].md
        for (const file of files) {
            expect(file).toMatch(/^index-[a-f0-9]{8}\.md$/)
        }
    })

    it("should rewrite root-relative links and relative asset paths during downloadAction", async () => {
        const mockSkills = [
            {
                path: "skills/engineering/test-skill/SKILL.md",
                dirName: "test-skill",
                category: "engineering",
                promoted: false,
                metadata: {
                    name: "test-skill",
                    description: "Test skill description",
                    resources: ["https://antigravity.google/docs/page.md"],
                },
                content: "",
            },
        ]

        globalThis.fetch = mock(async () => {
            return new Response(
                `# Page Content
This is a [task groups](/docs/task-groups) description.
Also check ![Antigravity 2.0 UI](assets/image/docs/AGY2.0-Home.png) and ![Model Selector Drop Down](/assets/image/docs/model-selector.png).
Do not change [absolute](https://github.com/docs) or [anchor](#section).`,
                {
                    status: 200,
                    headers: {"content-type": "text/markdown"},
                }
            )
        }) as any

        await downloadAction(mockSkills, fakeRepo)

        const resourcesDir = path.join(
            fakeRepo,
            "skills/engineering/test-skill/resources"
        )
        const fileContent = await fs.readFile(
            path.join(resourcesDir, "docs-page.md"),
            "utf-8"
        )

        expect(fileContent).toContain(
            "[task groups](https://antigravity.google/docs/task-groups)"
        )
        expect(fileContent).toContain(
            "![Antigravity 2.0 UI](https://antigravity.google/docs/assets/image/docs/AGY2.0-Home.png)"
        )
        expect(fileContent).toContain(
            "![Model Selector Drop Down](https://antigravity.google/assets/image/docs/model-selector.png)"
        )
        expect(fileContent).toContain("[absolute](https://github.com/docs)")
        expect(fileContent).toContain("[anchor](#section)")
    })

    it("should resolve relative links, normalized extensions, hashes, and fall back to absolute upstream URLs", async () => {
        const mockSkills = [
            {
                path: "skills/engineering/test-skill/SKILL.md",
                dirName: "test-skill",
                category: "engineering",
                promoted: false,
                metadata: {
                    name: "test-skill",
                    description: "Test skill description",
                    resources: [
                        "https://agentclientprotocol.com/rfds/v2/overview.mdx",
                        "https://agentclientprotocol.com/rfds/v2/prompt.mdx",
                        "https://agentclientprotocol.com/rfds/proxy-chains.mdx",
                    ],
                },
                content: "",
            },
        ]

        globalThis.fetch = mock(async (url) => {
            const urlStr = url.toString()
            if (urlStr.endsWith("overview.mdx")) {
                return new Response(
                    `# Overview
Check out [Prompt](./prompt.mdx#hash), [Proxy Chains](../proxy-chains.mdx), and [Unmirrored](../not-here.mdx#section).`,
                    {
                        status: 200,
                        headers: {"content-type": "text/markdown"},
                    }
                )
            }
            return new Response(
                `# Stub response for prompt/proxy-chains that is definitely long enough to not be skipped.`,
                {
                    status: 200,
                    headers: {"content-type": "text/markdown"},
                }
            )
        }) as any

        await downloadAction(mockSkills, fakeRepo)

        const resourcesDir = path.join(
            fakeRepo,
            "skills/engineering/test-skill/resources"
        )
        const fileContent = await fs.readFile(
            path.join(resourcesDir, "rfds-v2-overview.md"),
            "utf-8"
        )

        // ./prompt.mdx#hash -> resolved and mapped to rfds-v2-prompt.md#hash
        expect(fileContent).toContain("[Prompt](rfds-v2-prompt.md#hash)")
        // ../proxy-chains.mdx -> resolved and mapped to rfds-proxy-chains.md
        expect(fileContent).toContain("[Proxy Chains](rfds-proxy-chains.md)")
        // ../not-here.mdx#section -> fallback to absolute URL
        expect(fileContent).toContain(
            "[Unmirrored](https://agentclientprotocol.com/rfds/not-here.mdx#section)"
        )
    })

    describe("getCommonPrefix unit tests", () => {
        it("should return empty string if no URLs are provided", () => {
            expect(getCommonPrefix([])).toBe("")
        })

        it("should find the common prefix matching up to slash boundary", () => {
            const urls = [
                "https://example.com/docs/a/b",
                "https://example.com/docs/a/c",
            ]
            expect(getCommonPrefix(urls)).toBe("https://example.com/docs/a/")
        })

        it("should fall back to origin/ if no common slash directory", () => {
            const urls = [
                "https://example.com/docs/a/b",
                "https://example.com/api/v1",
            ]
            expect(getCommonPrefix(urls)).toBe("https://example.com/")
        })
    })

    describe("parseLlmsTxtLinks unit tests", () => {
        it("should extract links from markdown and resolve relative URLs", () => {
            const content = `
- [link1](/docs/page1)
- [link2](page2)
- [external](https://github.com/docs)
            `
            const baseUrl = "https://example.com/docs/index"
            const result = parseLlmsTxtLinks(content, baseUrl)
            expect(result).toContain("https://example.com/docs/page1")
            expect(result).toContain("https://example.com/docs/page2")
            expect(result).not.toContain("https://github.com/docs")
        })

        it("should exclude specified binary formats", () => {
            const content = `
- [pdf](/docs/doc.pdf)
- [zip](/docs/archive.zip)
- [ok](/docs/page)
            `
            const baseUrl = "https://example.com/docs/index"
            const result = parseLlmsTxtLinks(content, baseUrl)
            expect(result).toContain("https://example.com/docs/page")
            expect(result).not.toContain("https://example.com/docs/doc.pdf")
            expect(result).not.toContain("https://example.com/docs/archive.zip")
        })
    })

    describe("checkPathTraversal unit tests", () => {
        it("should return the resolved file path if valid", () => {
            const resourcesDir = "/home/user/skills/resources"
            const filename = "docs-a.md"
            const result = checkPathTraversal(resourcesDir, filename)
            expect(result).toBe("/home/user/skills/resources/docs-a.md")
        })

        it("should throw error if filename attempts traversal", () => {
            const resourcesDir = "/home/user/skills/resources"
            const filename = "../../../etc/passwd"
            expect(() => checkPathTraversal(resourcesDir, filename)).toThrow(
                "Path traversal detected"
            )
        })
    })
})
