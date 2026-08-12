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
    stripReaderMetadata,
    isLlmsIndexUrl,
    isNonTextUrl,
    isNonTextContentType,
    decodeHtmlEntities,
    escapeOrphanTypeBrackets,
    protectZolaDelimiters,
    sanitizeControlCharacters,
    normalizeMarkdownFormatting,
} = await import("./downloader.ts")

describe("isNonTextUrl & isNonTextContentType", () => {
    it("identifies binary and media file extensions", () => {
        expect(isNonTextUrl("https://v2.tauri.app/_astro/image.webp")).toBe(
            true
        )
        expect(isNonTextUrl("https://example.com/logo.png")).toBe(true)
        expect(isNonTextUrl("https://example.com/diagram.svg")).toBe(true)
        expect(isNonTextUrl("https://example.com/doc.pdf")).toBe(true)
        expect(isNonTextUrl("https://example.com/archive.zip")).toBe(true)
        expect(isNonTextUrl("https://example.com/font.woff2")).toBe(true)
        expect(isNonTextUrl("https://v2.tauri.app/start/frontend")).toBe(false)
        expect(isNonTextUrl("https://v2.tauri.app/llms.txt")).toBe(false)
        expect(isNonTextUrl("https://example.com/guide.md")).toBe(false)
    })

    it("identifies non-text MIME Content-Types", () => {
        expect(isNonTextContentType("image/webp")).toBe(true)
        expect(isNonTextContentType("image/png")).toBe(true)
        expect(isNonTextContentType("image/svg+xml")).toBe(true)
        expect(isNonTextContentType("application/pdf")).toBe(true)
        expect(isNonTextContentType("application/zip")).toBe(true)
        expect(isNonTextContentType("application/octet-stream")).toBe(true)
        expect(isNonTextContentType("text/html; charset=utf-8")).toBe(false)
        expect(isNonTextContentType("text/markdown")).toBe(false)
        expect(isNonTextContentType("text/plain")).toBe(false)
    })
})

describe("Markdown Post-Processing Helpers", () => {
    it("decodes HTML entities in text and code blocks", () => {
        expect(decodeHtmlEntities("array&lt;string&gt;")).toBe("array<string>")
        expect(
            decodeHtmlEntities("&quot;hello&quot; &amp; &apos;world&apos;")
        ).toBe("\"hello\" & 'world'")
        expect(decodeHtmlEntities("double &amp;#x26; entity")).toBe(
            "double &#x26; entity"
        )
    })

    it("escapes orphan type annotation brackets outside code fences", () => {
        const input = "List of (array<string>, required)\n```\n<string>\n```"
        const output = escapeOrphanTypeBrackets(input)
        expect(output).toBe(
            "List of (array\\<string\\>, required)\n```\n<string>\n```"
        )
    })

    it("wraps raw Zola template tags in {% raw %} blocks", () => {
        expect(protectZolaDelimiters("Hello {{ name }}")).toBe(
            "{% raw %}\nHello {{ name }}\n{% endraw %}"
        )
        expect(
            protectZolaDelimiters("{% raw %}Hello {{ name }}{% endraw %}")
        ).toBe("{% raw %}Hello {{ name }}{% endraw %}")
        expect(protectZolaDelimiters("Plain text")).toBe("Plain text")
    })

    it("strips null bytes, control characters, and U+FFFD", () => {
        expect(sanitizeControlCharacters("hello\x00world\uFFFD\x07")).toBe(
            "helloworld"
        )
    })

    it("normalizes markdown formatting and empty link syntax", () => {
        const input =
            "Text [link]() and [anchor](#)\n```js\nconst x = 1\n```\nMore text"
        const expected =
            "Text link and anchor\n```js\nconst x = 1\n```\nMore text"
        expect(normalizeMarkdownFormatting(input)).toBe(expected)
    })
})

describe("isLlmsIndexUrl", () => {
    it("correctly identifies llms.txt index and sub-index patterns", () => {
        expect(isLlmsIndexUrl("https://v2.tauri.app/llms.txt")).toBe(true)
        expect(isLlmsIndexUrl("https://v2.tauri.app/_llms.txt")).toBe(true)
        expect(
            isLlmsIndexUrl("https://v2.tauri.app/_llms-txt/guides.txt")
        ).toBe(true)
        expect(
            isLlmsIndexUrl("https://v2.tauri.app/_llms-txt/reference.txt")
        ).toBe(true)
        expect(
            isLlmsIndexUrl("https://pagefind.app/llms-component-ui.txt")
        ).toBe(true)
        expect(isLlmsIndexUrl("https://example.com/docs/page.md")).toBe(false)
        expect(isLlmsIndexUrl("https://cli.github.com/manual/gh_pr")).toBe(
            false
        )
    })
})

describe("stripReaderMetadata", () => {
    it("strips the jina reader preamble including the volatile Published Time", () => {
        const input =
            "Title: GitHub CLI\n\nURL Source: source-page\n\nPublished Time: Thu, 09 Jul 2026 15:15:41 GMT\n\nMarkdown Content:\n# gh issue\n\nManage issues.\n"
        expect(stripReaderMetadata(input)).toBe(
            "# gh issue\n\nManage issues.\n"
        )
    })
    it("strips the preamble when Published Time is absent", () => {
        const input =
            "Title: T\n\nURL Source: source-page\n\nMarkdown Content:\nBody here.\n"
        expect(stripReaderMetadata(input)).toBe("Body here.\n")
    })
    it("leaves non-jina content untouched", () => {
        const input = "# A normal doc\n\nNo preamble here.\n"
        expect(stripReaderMetadata(input)).toBe(input)
    })
    it("leaves content that starts with Title: but has no Markdown Content marker", () => {
        const input = "Title: not jina\n\nsome body\n"
        expect(stripReaderMetadata(input)).toBe(input)
    })
})

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
            "skills/engineering/test-skill/resources/auto"
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

    it("should fall back to Jina proxy when a URL returns 403 Forbidden", async () => {
        const mockSkills = [
            {
                path: "skills/engineering/test-skill/SKILL.md",
                dirName: "test-skill",
                category: "engineering",
                promoted: false,
                metadata: {
                    name: "test-skill",
                    description: "Test skill description",
                    resources: ["https://blocked-site.com/docs/page.md"],
                },
                content: "",
            },
        ]

        globalThis.fetch = mock(async (url: any) => {
            const urlStr = String(url)
            if (urlStr.startsWith("https://r.jina.ai/")) {
                return new Response(
                    "Title: Blocked Page\n\nURL Source: https://blocked-site.com/docs/page.md\n\nMarkdown Content:\n# Recovered Content\nThis content was recovered via proxy fallback and is long enough.",
                    {status: 200, headers: {"content-type": "text/markdown"}}
                )
            }
            return new Response("Forbidden", {
                status: 403,
                statusText: "Forbidden",
                headers: {"content-type": "text/html"},
            })
        }) as any

        await downloadAction(mockSkills, fakeRepo)

        const resourcesDir = path.join(
            fakeRepo,
            "skills/engineering/test-skill/resources/auto"
        )
        const fileContent = await fs.readFile(
            path.join(resourcesDir, "docs-page.md"),
            "utf-8"
        )
        expect(fileContent).toContain("# Recovered Content")
    })

    it("should skip dead links discovered from an llms.txt index without failing the run", async () => {
        const mockSkills = [
            {
                path: "skills/engineering/test-skill/SKILL.md",
                dirName: "test-skill",
                category: "engineering",
                promoted: false,
                metadata: {
                    name: "test-skill",
                    description: "Test skill description",
                    resources: ["https://example.com/llms.txt"],
                },
                content: "",
            },
        ]

        globalThis.fetch = mock(async (url: any) => {
            const urlStr = String(url)
            if (urlStr.endsWith("/llms.txt")) {
                return new Response(
                    `- [good](https://example.com/good)\n- [dead](https://example.com/dead)`,
                    {status: 200, headers: {"content-type": "text/plain"}}
                )
            }
            if (urlStr.endsWith("/dead")) {
                // A link the upstream index still lists but that now 404s
                return new Response("Not Found", {
                    status: 404,
                    headers: {"content-type": "text/plain"},
                })
            }
            return new Response(
                "# Good page\nThis is a good page with plenty of content to pass the fifty byte minimum size check.",
                {status: 200, headers: {"content-type": "text/markdown"}}
            )
        }) as any

        // Must NOT throw despite the dead discovered child link
        await downloadAction(mockSkills, fakeRepo)

        const autoDir = path.join(
            fakeRepo,
            "skills/engineering/test-skill/resources/auto"
        )
        const files = await fs.readdir(autoDir)
        expect(files).toContain("good.md")
        expect(files).not.toContain("dead.md")
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

        it("should fall back to overview-index.md for empty or index slug to avoid Zola collisions", () => {
            expect(smartSlugify("https:/" + "/example.com/日本語/")).toBe(
                "overview-index.md"
            )
            expect(smartSlugify("https:/" + "/example.com/index.html")).toBe(
                "overview-index.md"
            )
            expect(smartSlugify("https:/" + "/example.com/")).toBe(
                "overview-index.md"
            )
        })

        it("should strip hash fragments (#...) and query strings (?...) before slugifying", () => {
            expect(
                smartSlugify(
                    "https:/" + "/docs.x.ai/llms.txt#audio-transport",
                    "https:/" + "/docs.x.ai/"
                )
            ).toBe("llms.txt")
            expect(
                smartSlugify(
                    "https:/" +
                        "/docs.x.ai/developers/model-capabilities/text/generate-text#adding-encrypted-thinking-content",
                    "https:/" + "/docs.x.ai/"
                )
            ).toBe("developers-model-capabilities-text-generate-text.md")
            expect(
                smartSlugify(
                    "https:/" + "/example.com/docs/page?version=2#section"
                )
            ).toBe("docs-page.md")
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
            "skills/engineering/test-skill/resources/auto"
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
            "skills/engineering/test-skill/resources/auto"
        )
        const files = await fs.readdir(resourcesDir)

        // They should have resolved to distinct files since both originally mapped to overview-index.md
        expect(files.length).toBe(2)

        // Each file name should be overview-index-[hash].md
        for (const file of files) {
            expect(file).toMatch(/^overview-index-[a-f0-9]{8}\.md$/)
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
            "skills/engineering/test-skill/resources/auto"
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
            "skills/engineering/test-skill/resources/auto"
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

    describe("downloadAction and cleanAction configuration and filtering", () => {
        const mockSkills = [
            {
                path: "skills/engineering/skill-a/SKILL.md",
                dirName: "skill-a",
                category: "engineering",
                promoted: false,
                metadata: {
                    name: "skill-a",
                    description: "Skill A description",
                    resources: ["https://example.com/a.txt"],
                },
                content: "",
            },
            {
                path: "skills/tooling/skill-b/SKILL.md",
                dirName: "skill-b-dir",
                category: "tooling",
                promoted: false,
                metadata: {
                    name: "skill-b-name",
                    description: "Skill B description",
                    resources: ["https://example.com/b.txt"],
                },
                content: "",
            },
        ]

        beforeEach(async () => {
            globalThis.fetch = mock(async () => {
                return new Response(
                    "Some mock resource content which is longer than fifty bytes to satisfy the size check.",
                    {
                        status: 200,
                        headers: {"content-type": "text/plain"},
                    }
                )
            }) as any
        })

        it("should filter by skill name (dirName or metadata.name) in downloadAction", async () => {
            // Filter by dirName
            await downloadAction(mockSkills, fakeRepo, {skill: "skill-a"})
            expect(
                await fs.pathExists(
                    path.join(
                        fakeRepo,
                        "skills/engineering/skill-a/resources/auto/a.txt"
                    )
                )
            ).toBe(true)
            expect(
                await fs.pathExists(
                    path.join(
                        fakeRepo,
                        "skills/tooling/skill-b/resources/auto/b.txt"
                    )
                )
            ).toBe(false)

            // Cleanup
            await fs.remove(path.join(fakeRepo, "skills"))

            // Filter by metadata.name
            await downloadAction(mockSkills, fakeRepo, {skill: "skill-b-name"})
            expect(
                await fs.pathExists(
                    path.join(
                        fakeRepo,
                        "skills/engineering/skill-a/resources/auto/a.txt"
                    )
                )
            ).toBe(false)
            expect(
                await fs.pathExists(
                    path.join(
                        fakeRepo,
                        "skills/tooling/skill-b/resources/auto/b.txt"
                    )
                )
            ).toBe(true)
        })

        it("should filter by category in downloadAction", async () => {
            await downloadAction(mockSkills, fakeRepo, {category: "tooling"})
            expect(
                await fs.pathExists(
                    path.join(
                        fakeRepo,
                        "skills/engineering/skill-a/resources/auto/a.txt"
                    )
                )
            ).toBe(false)
            expect(
                await fs.pathExists(
                    path.join(
                        fakeRepo,
                        "skills/tooling/skill-b/resources/auto/b.txt"
                    )
                )
            ).toBe(true)
        })

        it("should filter by skill name and category in cleanAction", async () => {
            // Create resources/auto dirs first (cleanAction removes only auto/)
            const dirA = path.join(
                fakeRepo,
                "skills/engineering/skill-a/resources/auto"
            )
            const dirB = path.join(
                fakeRepo,
                "skills/tooling/skill-b/resources/auto"
            )
            await fs.ensureDir(dirA)
            await fs.ensureDir(dirB)

            // Clean only skill-a
            const {cleanAction} = await import("./downloader.ts")
            await cleanAction(mockSkills, fakeRepo, {skill: "skill-a"})
            expect(await fs.pathExists(dirA)).toBe(false)
            expect(await fs.pathExists(dirB)).toBe(true)

            // Reset and Clean only category tooling
            await fs.ensureDir(dirA)
            await cleanAction(mockSkills, fakeRepo, {category: "tooling"})
            expect(await fs.pathExists(dirA)).toBe(true)
            expect(await fs.pathExists(dirB)).toBe(false)
        })

        it("should propagate custom timeout to fetch signal", async () => {
            let capturedTimeoutSignal
            globalThis.fetch = mock(async (_url, init) => {
                capturedTimeoutSignal = init?.signal
                return new Response(
                    "Some mock resource content which is longer than fifty bytes to satisfy the size check.",
                    {
                        status: 200,
                        headers: {"content-type": "text/plain"},
                    }
                )
            }) as any

            await downloadAction(mockSkills, fakeRepo, {
                skill: "skill-a",
                timeout: 45,
            })
            expect(capturedTimeoutSignal).toBeDefined()

            const {download} = await import("./downloader.ts")
            const originalAbortSignalTimeout = AbortSignal.timeout
            let passedTimeoutMs = 0
            AbortSignal.timeout = (ms) => {
                passedTimeoutMs = ms
                return originalAbortSignalTimeout(ms)
            }
            try {
                await download("https://example.com/a.txt", 45)
            } catch {
                // ignore
            } finally {
                AbortSignal.timeout = originalAbortSignalTimeout
            }
            expect(passedTimeoutMs).toBe(45000)
        })

        it("should respect custom concurrency limit", async () => {
            const manyResourcesSkill = [
                {
                    path: "skills/engineering/skill-many/SKILL.md",
                    dirName: "skill-many",
                    category: "engineering",
                    promoted: false,
                    metadata: {
                        name: "skill-many",
                        description: "Many resources",
                        resources: [
                            "https://example.com/1.txt",
                            "https://example.com/2.txt",
                            "https://example.com/3.txt",
                            "https://example.com/4.txt",
                            "https://example.com/5.txt",
                        ],
                    },
                    content: "",
                },
            ]

            let activeConnections = 0
            let maxActiveConnections = 0

            globalThis.fetch = mock(async () => {
                activeConnections++
                maxActiveConnections = Math.max(
                    maxActiveConnections,
                    activeConnections
                )
                await new Promise((r) => setTimeout(r, 10))
                activeConnections--
                return new Response(
                    "Some mock resource content which is longer than fifty bytes to satisfy the size check.",
                    {
                        status: 200,
                        headers: {"content-type": "text/plain"},
                    }
                )
            }) as any

            await downloadAction(manyResourcesSkill, fakeRepo, {concurrency: 2})
            expect(maxActiveConnections).toBeLessThanOrEqual(2)

            maxActiveConnections = 0
            activeConnections = 0
            await downloadAction(manyResourcesSkill, fakeRepo, {concurrency: 1})
            expect(maxActiveConnections).toBe(1)
        })

        it("should remove resources/auto (self-healing) and keep resources/manual when a skill has no configured resources", async () => {
            const base = "skills/engineering/skill-empty"
            const autoDir = path.join(fakeRepo, base, "resources/auto")
            const manualDir = path.join(fakeRepo, base, "resources/manual")
            await fs.ensureDir(autoDir)
            await fs.ensureDir(manualDir)
            await fs.writeFile(path.join(autoDir, "stale.md"), "old downloaded")
            await fs.writeFile(path.join(manualDir, "keep.md"), "hand-authored")

            const emptySkill = [
                {
                    path: `${base}/SKILL.md`,
                    dirName: "skill-empty",
                    category: "engineering",
                    promoted: false,
                    metadata: {
                        name: "skill-empty",
                        description: "No configured resources",
                        resources: [],
                    },
                    content: "",
                },
            ]

            await downloadAction(emptySkill, fakeRepo)

            expect(await fs.pathExists(autoDir)).toBe(false)
            expect(await fs.pathExists(path.join(manualDir, "keep.md"))).toBe(
                true
            )
        })

        it("should remove only resources/auto and preserve resources/manual in cleanAction", async () => {
            const base = "skills/engineering/skill-a"
            const autoDir = path.join(fakeRepo, base, "resources/auto")
            const manualDir = path.join(fakeRepo, base, "resources/manual")
            await fs.ensureDir(autoDir)
            await fs.ensureDir(manualDir)
            await fs.writeFile(path.join(autoDir, "downloaded.md"), "x")
            await fs.writeFile(path.join(manualDir, "authored.md"), "y")

            const {cleanAction} = await import("./downloader.ts")
            await cleanAction(mockSkills, fakeRepo, {skill: "skill-a"})

            expect(await fs.pathExists(autoDir)).toBe(false)
            expect(
                await fs.pathExists(path.join(manualDir, "authored.md"))
            ).toBe(true)
        })
    })
})
