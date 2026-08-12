import {$} from "bun"
import path from "node:path"
import crypto from "node:crypto"
import fs from "fs-extra"
import {log} from "@clack/prompts"

export const HTTP_TIMEOUT = 60 // Timeout in seconds
export const MIN_CONTENT_BYTES = 50

interface SkillMetadata {
    name?: string
    description?: string
    resources?: string[]
    [key: string]: any
}

interface Skill {
    path: string
    dirName: string
    category: string
    promoted: boolean
    metadata: SkillMetadata
    content: string
}

interface DownloadJob {
    url: string
    destFile: string
    skillName: string
    candidates?: string[]
    // True for links auto-discovered from an llms.txt index (vs a directly
    // configured resources: URL). A dead discovered link is upstream churn,
    // not a config error, so it is skipped rather than failing the run.
    discovered?: boolean
}

export function stripReaderProxy(urlStr: string): string {
    const proxies = ["https://r.jina.ai/"]
    for (const proxy of proxies) {
        if (urlStr.startsWith(proxy)) {
            return urlStr.slice(proxy.length)
        }
    }
    return urlStr
}

// The r.jina.ai reader prepends a metadata preamble to fetched docs:
//   Title: …\n\nURL Source: …\n\n[Published Time: …\n\n]Markdown Content:\n<doc>
// The Published Time value jitters between fetches even when the page is
// unchanged, which makes downloads non-idempotent. Strip the whole preamble so
// re-downloading an unchanged page produces an identical file.
export function stripReaderMetadata(content: string): string {
    if (!content.startsWith("Title:")) return content
    const marker = "\nMarkdown Content:\n"
    const idx = content.indexOf(marker)
    if (idx === -1) return content
    return content.slice(idx + marker.length).replace(/^\n+/, "")
}

// Concurrency limit for proxy fallback to prevent hitting proxy rate limits (HTTP 429)
let activeProxyCalls = 0
const MAX_CONCURRENT_PROXY_CALLS = 2
const proxyQueue: Array<() => void> = []

async function acquireProxySlot(): Promise<void> {
    if (activeProxyCalls < MAX_CONCURRENT_PROXY_CALLS) {
        activeProxyCalls++
        return
    }
    return new Promise((resolve) => {
        proxyQueue.push(() => {
            activeProxyCalls++
            resolve()
        })
    })
}

function releaseProxySlot(): void {
    activeProxyCalls--
    const next = proxyQueue.shift()
    if (next) {
        next()
    }
}

async function fetchWithRetry(
    url: string,
    options: RequestInit,
    maxRetries = 4,
    initialDelayMs = 1000
): Promise<Response> {
    let delay = initialDelayMs
    let lastResponse: Response | undefined

    for (let attempt = 1; attempt <= maxRetries; attempt++) {
        try {
            const res = await fetch(url, options)
            lastResponse = res
            // Retry on 429 (Too Many Requests), 502, 503, 504
            if (
                (res.status === 429 ||
                    res.status === 502 ||
                    res.status === 503 ||
                    res.status === 504) &&
                attempt < maxRetries
            ) {
                const retryAfterHeader = res.headers.get("retry-after")
                const parsedMs = retryAfterHeader
                    ? parseInt(retryAfterHeader, 10) * 1000
                    : 0
                const waitMs = parsedMs && !isNaN(parsedMs) ? parsedMs : delay
                await new Promise((r) => setTimeout(r, waitMs))
                delay *= 2
                continue
            }
            return res
        } catch (err) {
            if (attempt >= maxRetries) throw err
            await new Promise((r) => setTimeout(r, delay))
            delay *= 2
        }
    }
    return lastResponse!
}

// Download content using Bun-native fetch with modern browser headers and proxy fallback
export async function download(
    url: string,
    timeout?: number
): Promise<{content: string; isHtml: boolean; skipped?: boolean}> {
    if (isNonTextUrl(url)) {
        log.info(`  Skipping non-text resource URL: ${url}`)
        return {content: "", isHtml: false, skipped: true}
    }

    const timeoutSeconds = timeout !== undefined ? timeout : HTTP_TIMEOUT
    let response: Response | undefined

    const browserHeaders = {
        "User-Agent":
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
        Accept: "text/markdown, text/plain, text/html;q=0.9, application/xhtml+xml, application/xml;q=0.8, */*;q=0.7",
        "Accept-Language": "en-US,en;q=0.9",
        "Sec-Ch-Ua":
            '"Not/A)Brand";v="8", "Chromium";v="126", "Google Chrome";v="126"',
        "Sec-Ch-Ua-Mobile": "?0",
        "Sec-Ch-Ua-Platform": '"Windows"',
        "Sec-Fetch-Dest": "document",
        "Sec-Fetch-Mode": "navigate",
        "Sec-Fetch-Site": "none",
        "Sec-Fetch-User": "?1",
    }

    try {
        response = await fetchWithRetry(url, {
            headers: browserHeaders,
            signal: AbortSignal.timeout(timeoutSeconds * 1000),
        })

        if (!response.ok) {
            response = await fetchWithRetry(url, {
                headers: {
                    "User-Agent": browserHeaders["User-Agent"],
                },
                signal: AbortSignal.timeout(timeoutSeconds * 1000),
            })
        }
    } catch {
        // Direct fetch network error / timeout, proceed to fallback attempt
    }

    if (response && response.ok) {
        const contentType = response.headers.get("content-type") || ""
        if (isNonTextContentType(contentType)) {
            log.info(`  Skipping non-text resource (${contentType}): ${url}`)
            return {content: "", isHtml: false, skipped: true}
        }
        const isHtml = contentType.includes("text/html")
        const content = await response.text()
        return {content, isHtml, skipped: false}
    }

    // Proxy fallback via Jina AI Reader ONLY for WAF / anti-bot / 403 / network errors (NOT for explicit 404/410)
    if (
        (!response || (response.status !== 404 && response.status !== 410)) &&
        !url.startsWith("https://r.jina.ai/")
    ) {
        const cleanTarget = stripReaderProxy(url)
        const proxyUrl = `https://r.jina.ai/${cleanTarget}`
        await acquireProxySlot()
        try {
            const proxyResponse = await fetchWithRetry(
                proxyUrl,
                {
                    headers: {
                        "User-Agent": browserHeaders["User-Agent"],
                        Accept: "text/markdown, text/plain, */*",
                    },
                    signal: AbortSignal.timeout(timeoutSeconds * 1000),
                },
                4,
                1000
            )

            if (proxyResponse.ok) {
                const contentType =
                    proxyResponse.headers.get("content-type") || ""
                if (isNonTextContentType(contentType)) {
                    log.info(
                        `  Skipping non-text resource (${contentType}): ${url}`
                    )
                    return {content: "", isHtml: false, skipped: true}
                }
                const rawContent = await proxyResponse.text()
                const content = stripReaderMetadata(rawContent)
                const isHtml = contentType.includes("text/html")
                return {content, isHtml, skipped: false}
            }
        } catch {
            // Proxy failed, fall back to throwing original status or error
        } finally {
            releaseProxySlot()
        }
    }

    if (response) {
        throw new Error(
            `HTTP Status ${response.status} (${response.statusText})`
        )
    }

    throw new Error(`Failed to fetch ${url}: Network or connection error`)
}

// Clean HTML content by stripping scripts, styles, layout noise, and SVGs using HTMLRewriter
export async function cleanHtml(htmlContent: string): Promise<string> {
    const rewriter = new HTMLRewriter()
        .on("script", {
            element(el) {
                el.remove()
            },
        })
        .on("style", {
            element(el) {
                el.remove()
            },
        })
        .on("svg", {
            element(el) {
                el.remove()
            },
        })
        .on("nav", {
            element(el) {
                el.remove()
            },
        })
        .on("header", {
            element(el) {
                el.remove()
            },
        })
        .on("footer", {
            element(el) {
                el.remove()
            },
        })
        .on("aside", {
            element(el) {
                el.remove()
            },
        })
        .on(".sidebar", {
            element(el) {
                el.remove()
            },
        })
        .on("#sidebar", {
            element(el) {
                el.remove()
            },
        })
        .on("a.anchor, a.hash-link, a.anchor-link, a.sl-anchor-link", {
            element(el) {
                el.remove()
            },
        })
        .on(
            ".toc, #toc, .table-of-contents, .on-this-page, .breadcrumbs, .pagination, .edit-on-github, .edit-this-page, .sidebar-nav",
            {
                element(el) {
                    el.remove()
                },
            }
        )

    const response = new Response(htmlContent, {
        headers: {"Content-Type": "text/html; charset=utf-8"},
    })
    const transformedResponse = rewriter.transform(response)
    return await transformedResponse.text()
}

// Convert HTML content directly to Markdown using Pandoc via stdin/stdout after cleaning
export async function convertHtmlToMarkdown(
    htmlContent: string
): Promise<string> {
    try {
        const cleaned = await cleanHtml(htmlContent)
        const mdContent =
            await $`pandoc -f html -t gfm-raw_html < ${new Response(cleaned)}`.text()
        return mdContent
    } catch (err) {
        throw new Error(
            `Pandoc conversion failed: ${err instanceof Error ? err.message : String(err)}`,
            {cause: err}
        )
    }
}

// Check if a path is outside of resourcesDir to prevent path traversal vulnerabilities
export function checkPathTraversal(
    resourcesDir: string,
    filename: string
): string {
    const destFile = path.resolve(resourcesDir, filename)
    const relative = path.relative(resourcesDir, destFile)
    const isTraversal =
        relative.startsWith(".." + path.sep) || relative === ".."
    if (isTraversal || path.isAbsolute(relative)) {
        throw new Error(
            `Path traversal detected: ${destFile} is outside of ${resourcesDir}`
        )
    }
    return destFile
}

// Compute the common prefix of a list of URLs ending at a slash boundary
export function getCommonPrefix(urls: string[]): string {
    if (urls.length === 0) return ""

    let origin = ""
    const pathnames: string[] = []

    for (const u of urls) {
        try {
            const url = new URL(u)
            if (!origin) origin = url.origin
            pathnames.push(url.pathname)
        } catch {
            // Ignore invalid URLs
        }
    }

    if (pathnames.length === 0) return ""
    if (pathnames.length === 1) {
        try {
            const p = pathnames[0]
            if (!p) return origin + "/"
            const idx = p.lastIndexOf("/")
            return origin + p.slice(0, idx + 1)
        } catch {
            return origin + "/"
        }
    }

    const sorted = pathnames.sort()
    const first = sorted[0]
    const last = sorted[sorted.length - 1]
    if (!first || !last) return origin + "/"

    let i = 0
    while (i < first.length && i < last.length && first[i] === last[i]) {
        i++
    }

    const commonPath = first.slice(0, i)
    const lastSlash = commonPath.lastIndexOf("/")
    if (lastSlash !== -1) {
        return origin + commonPath.slice(0, lastSlash + 1)
    }

    return origin + "/"
}

// Extract a short domain-specific prefix from a URL (e.g. "secretspec" from "secretspec.dev")
export function getDomainPrefix(urlStr: string): string {
    try {
        const cleanUrl = stripReaderProxy(urlStr)
        const url = new URL(cleanUrl)
        const hostname = url.hostname.toLowerCase()

        // Check for IPv4 address (e.g. 127.0.0.1)
        if (/^(\d{1,3}\.){3}\d{1,3}$/.test(hostname)) {
            return url.hostname
        }

        // Check for IPv6 address (e.g. [::1])
        if (hostname.startsWith("[") && hostname.endsWith("]")) {
            return url.hostname
        }

        const parts = hostname.split(".")
        if (parts.length >= 2) {
            const lastPart = parts[parts.length - 1]
            const secondToLastPart = parts[parts.length - 2]

            if (lastPart && secondToLastPart) {
                // Common second-level labels used with country-code TLDs (which are 2 letters)
                const COMMON_SUFFIXES = new Set([
                    "co",
                    "com",
                    "org",
                    "gov",
                    "edu",
                    "net",
                    "ac",
                    "asn",
                    "id",
                    "ne",
                    "or",
                    "pe",
                    "ltd",
                    "me",
                    "sch",
                    "plc",
                    "nom",
                    "gen",
                    "mil",
                    "ind",
                    "web",
                    "info",
                ])

                if (
                    lastPart.length === 2 &&
                    COMMON_SUFFIXES.has(secondToLastPart)
                ) {
                    if (parts.length >= 3) {
                        return parts[parts.length - 3] || ""
                    }
                }
                return secondToLastPart
            }
        }
        return url.hostname
    } catch {
        return ""
    }
}

// Check if a URL has a non-text / binary extension
export function isNonTextUrl(urlStr: string): boolean {
    const cleanUrl = stripReaderProxy(urlStr)
    let pathname = cleanUrl
    try {
        const url = new URL(cleanUrl)
        pathname = url.pathname
    } catch {}

    const NON_TEXT_EXT_RE =
        /\.(zip|pdf|png|jpg|jpeg|gif|webp|svg|ico|bmp|avif|tar|gz|tgz|bz2|xz|exe|dmg|iso|bin|mp4|mp3|wav|ogg|webm|mov|avi|flv|m4a|m4v|woff|woff2|ttf|eot|otf|xml|rss)$/i
    return NON_TEXT_EXT_RE.test(pathname)
}

// Check if an HTTP Content-Type header indicates non-text / binary media content
export function isNonTextContentType(contentType: string): boolean {
    const type = contentType.toLowerCase().split(";")[0]?.trim() || ""
    if (!type) return false
    if (
        type.startsWith("image/") ||
        type.startsWith("audio/") ||
        type.startsWith("video/") ||
        type.startsWith("font/")
    ) {
        return true
    }
    if (
        type === "application/pdf" ||
        type === "application/zip" ||
        type === "application/octet-stream" ||
        type === "application/x-tar" ||
        type === "application/gzip" ||
        type === "application/xml" ||
        type === "application/rss+xml" ||
        type === "application/atom+xml"
    ) {
        return true
    }
    return false
}

// Decode HTML entities in Markdown body text and code blocks
export function decodeHtmlEntities(content: string): string {
    return content
        .replace(/&amp;(#?[a-zA-Z0-9]+;)/g, "&$1")
        .replace(/&gt;/g, ">")
        .replace(/&lt;/g, "<")
        .replace(/&quot;/g, '"')
        .replace(/&apos;/g, "'")
        .replace(/&#x20;/g, " ")
        .replace(/&nbsp;/g, " ")
        .replace(/&amp;/g, "&")
}

// Escape orphan angle brackets outside code blocks so Markdown parsers render type parameters as text
export function escapeOrphanTypeBrackets(content: string): string {
    const KNOWN_TAGS = new Set([
        "br",
        "hr",
        "img",
        "a",
        "p",
        "div",
        "span",
        "code",
        "pre",
        "b",
        "i",
        "strong",
        "em",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "ul",
        "ol",
        "li",
        "table",
        "tr",
        "td",
        "th",
        "blockquote",
        "details",
        "summary",
        "section",
        "article",
        "main",
        "header",
        "footer",
        "nav",
        "aside",
    ])

    const lines = content.split("\n")
    let inCodeFence = false
    const processedLines: string[] = []

    for (const line of lines) {
        if (line.trim().startsWith("```")) {
            inCodeFence = !inCodeFence
            processedLines.push(line)
            continue
        }
        if (inCodeFence) {
            processedLines.push(line)
            continue
        }

        const fixed = line.replace(
            /(?<!`)<([a-zA-Z0-9_\-|\s,]+)>(?!`)/g,
            (match, inner) => {
                const tagLower = inner.toLowerCase().trim()
                if (
                    KNOWN_TAGS.has(tagLower) ||
                    tagLower.startsWith("http://") ||
                    tagLower.startsWith("https://")
                ) {
                    return match
                }
                return "\\<" + inner.trim() + "\\>"
            }
        )
        processedLines.push(fixed)
    }

    return processedLines.join("\n")
}

// Wrap document content containing raw {{ or {% in Zola {% raw %} blocks to prevent Tera template build errors
export function protectZolaDelimiters(content: string): string {
    if (/\{\{|\{%/.test(content) && !content.includes("{% raw %}")) {
        return `{% raw %}\n${content}\n{% endraw %}`
    }
    return content
}

// Strip non-printable control characters, null bytes (\x00), and replacement characters (\uFFFD)
export function sanitizeControlCharacters(content: string): string {
    return (
        content
            // eslint-disable-next-line no-control-regex
            .replace(/[\x00-\x08\x0B\x0C\x0E-\x1F]/g, "")
            .replace(/\uFFFD/g, "")
    )
}

// Normalize markdown formatting and clean up empty links
export function normalizeMarkdownFormatting(content: string): string {
    return content
        .replace(/\[([^\]]*)\]\(\)/g, "$1")
        .replace(/\[([^\]]*)\]\(#\)/g, "$1")
}

// Check if a URL matches an llms.txt index or sub-index pattern
export function isLlmsIndexUrl(urlStr: string): boolean {
    if (isNonTextUrl(urlStr)) return false
    try {
        const cleanUrl = stripReaderProxy(urlStr)
        const url = new URL(cleanUrl)
        let pathname = url.pathname.toLowerCase()
        const hashIdx = pathname.indexOf("#")
        if (hashIdx !== -1) pathname = pathname.slice(0, hashIdx)
        const queryIdx = pathname.indexOf("?")
        if (queryIdx !== -1) pathname = pathname.slice(0, queryIdx)

        if (pathname.endsWith("/llms.txt") || pathname.endsWith("/_llms.txt")) {
            return true
        }
        if (
            pathname.includes("llms") &&
            (pathname.endsWith(".txt") || pathname.endsWith(".md"))
        ) {
            return true
        }
        return false
    } catch {
        return false
    }
}

// Clean and format slug names for downloaded files
export function smartSlugify(urlStr: string, commonPrefix?: string): string {
    const cleanUrl = stripReaderProxy(urlStr)

    let pathname: string
    try {
        const url = new URL(cleanUrl)
        if (commonPrefix && cleanUrl.startsWith(commonPrefix)) {
            pathname = cleanUrl.slice(commonPrefix.length)
            const hashIdx = pathname.indexOf("#")
            if (hashIdx !== -1) pathname = pathname.slice(0, hashIdx)
            const queryIdx = pathname.indexOf("?")
            if (queryIdx !== -1) pathname = pathname.slice(0, queryIdx)
        } else {
            pathname = url.pathname
        }
    } catch {
        pathname = cleanUrl
    }

    try {
        pathname = decodeURIComponent(pathname)
    } catch {}

    // Remove leading and trailing slashes
    pathname = pathname.replace(/^\/+|\/+$/g, "")

    if (!pathname) {
        return "overview-index.md"
    }

    // Determine extension and base pathname
    let ext = ".md"
    const knownExts = [".txt", ".md", ".mdx", ".html"]
    for (const kExt of knownExts) {
        if (pathname.endsWith(kExt)) {
            ext = kExt === ".txt" ? ".txt" : ".md"
            pathname = pathname.slice(0, -kExt.length)
            break
        }
    }

    // Replace slashes with dashes and clean up characters
    let slug = pathname.replace(/\//g, "-").replace(/[^a-zA-Z0-9.\-_]/g, "")

    if (!slug || slug.toLowerCase() === "index") {
        slug = "overview-index"
    }

    return `${slug}${ext}`
}

// Parse links from an llms.txt markdown file
export function parseLlmsTxtLinks(content: string, baseUrl: string): string[] {
    const urls: string[] = []
    const linkRe = /\[[^\]]*\]\(([^)]+)\)/g
    let match
    while ((match = linkRe.exec(content)) !== null) {
        const matchedUrl = match[1]
        if (matchedUrl) {
            const urlStr = matchedUrl.trim().split(/\s+/)[0]
            if (urlStr) {
                urls.push(urlStr)
            }
        }
    }

    const base = new URL(baseUrl)
    const seen = new Set<string>()
    const resolved: string[] = []

    for (const u of urls) {
        try {
            const resolvedUrl = new URL(u, base)
            resolvedUrl.hash = ""
            // Only keep URLs sharing the same origin or base second-level domain name (e.g. bun.sh and bun.com)
            const resolvedDomain = getDomainPrefix(resolvedUrl.toString())
            const baseDomain = getDomainPrefix(base.toString())
            if (
                resolvedUrl.origin !== base.origin &&
                (!resolvedDomain || resolvedDomain !== baseDomain)
            )
                continue

            const urlString = resolvedUrl
                .toString()
                .replace(/[.,]+$/, "")
                .replace(/\/+$/, "")

            // Exclude non-text binary/media extensions
            if (isNonTextUrl(urlString)) continue

            // Exclude the llms.txt URL itself
            if (urlString === baseUrl.replace(/\/+$/, "")) continue

            if (!seen.has(urlString)) {
                seen.add(urlString)
                resolved.push(urlString)
            }
        } catch {
            // Ignore invalid URLs
        }
    }

    return resolved
}

// Generate candidate raw asset markdown paths for Google Antigravity pages
function getAntigravityCandidates(urlStr: string): string[] {
    const prefix = "https://" + "antigravity.google/docs/"
    if (!urlStr.startsWith(prefix)) return []
    const docPath = urlStr.slice(prefix.length).replace(/\/+$/, "")
    if (!docPath) return []

    const dashJoined = docPath.replace(/\//g, "-")
    const firstSegment = docPath.split("/")[0] || ""
    const assetBase = "https://" + "antigravity.google/assets/docs"

    return [
        `${assetBase}/antigravity-2-0/${dashJoined}.md`,
        `${assetBase}/${firstSegment}/${dashJoined}.md`,
    ]
}

export interface DownloadOptions {
    convertHtmlToMarkdown?: (htmlContent: string) => Promise<string>
    skill?: string
    category?: string
    timeout?: number
    concurrency?: number
}

export async function downloadAction(
    skills: Skill[],
    agentSkillsHome: string,
    options?: DownloadOptions
) {
    let targetSkills = skills

    if (options?.skill) {
        targetSkills = targetSkills.filter(
            (s) =>
                s.dirName === options.skill || s.metadata.name === options.skill
        )
    }

    if (options?.category) {
        targetSkills = targetSkills.filter(
            (s) => s.category === options.category
        )
    }

    const skillsWithDocs = targetSkills.filter(
        (s) =>
            s.metadata.resources &&
            Array.isArray(s.metadata.resources) &&
            s.metadata.resources.length > 0
    )

    // Self-healing: any skill with no configured resources must not keep a
    // downloaded resources/auto directory. Removing the last URL (or the whole
    // resources key) cleans up on the next run. resources/manual is never touched.
    for (const skill of targetSkills) {
        const urls = Array.isArray(skill.metadata.resources)
            ? skill.metadata.resources
            : []
        if (urls.length > 0) continue
        const skillDir = path.dirname(path.join(agentSkillsHome, skill.path))
        const autoDir = path.join(skillDir, "resources", "auto")
        if (await fs.pathExists(autoDir)) {
            await fs.remove(autoDir)
            log.info(
                `  Removed stale resources/auto for ${skill.dirName} (no configured resources).`
            )
        }
    }

    if (skillsWithDocs.length === 0) {
        log.info("No skills with configured resources found.")
        return
    }

    log.info(`Syncing resources for ${skillsWithDocs.length} skill(s)...`)

    const concurrencyLimit =
        options?.concurrency !== undefined ? options.concurrency : 10
    let totalSuccess = 0
    let totalFailed = 0
    let totalSkipped = 0
    const failedDownloads: Array<{
        skillName: string
        url: string
        error: string
    }> = []
    // Dead links discovered from upstream llms.txt indexes — reported as
    // warnings, never fatal.
    const staleLinks: Array<{
        skillName: string
        url: string
        error: string
    }> = []

    for (const skill of skillsWithDocs) {
        const skillName = skill.dirName
        const skillDir = path.dirname(path.join(agentSkillsHome, skill.path))
        // Downloaded resources are owned by resources/auto; resources/manual
        // holds hand-authored files and is never read or written here.
        const resourcesDir = path.join(skillDir, "resources", "auto")
        await fs.ensureDir(resourcesDir)

        const urls = skill.metadata.resources || []
        const savedFiles = new Set<string>()
        const downloadJobs: DownloadJob[] = []
        let hasFailure = false

        log.step(`Processing skill: ${skillName}`)

        // Phase 1: Resolve all URLs (handle llms.txt index parsing recursively)
        const processedIndexUrls = new Set<string>()

        const processIndexUrl = async (urlStr: string) => {
            if (processedIndexUrls.has(urlStr)) return
            processedIndexUrls.add(urlStr)

            log.info(`  Fetching index: ${urlStr}`)
            try {
                const {content: indexContent} = await download(
                    urlStr,
                    options?.timeout
                )
                const childUrls = parseLlmsTxtLinks(indexContent, urlStr)
                log.info(`  Found ${childUrls.length} links in index.`)

                const indexFilename = smartSlugify(urlStr)
                const indexDestFile = checkPathTraversal(
                    resourcesDir,
                    indexFilename
                )
                downloadJobs.push({
                    url: urlStr,
                    destFile: indexDestFile,
                    skillName,
                })

                if (childUrls.length > 0) {
                    const commonPrefix = getCommonPrefix(childUrls)
                    for (const childUrl of childUrls) {
                        if (isNonTextUrl(childUrl)) continue
                        if (isLlmsIndexUrl(childUrl)) {
                            await processIndexUrl(childUrl)
                        } else {
                            const filename = smartSlugify(
                                childUrl,
                                commonPrefix
                            )
                            const destFile = checkPathTraversal(
                                resourcesDir,
                                filename
                            )
                            const candidates =
                                getAntigravityCandidates(childUrl)
                            downloadJobs.push({
                                url: childUrl,
                                destFile,
                                skillName,
                                candidates,
                                discovered: true,
                            })
                        }
                    }
                }
            } catch (err: any) {
                log.error(`  Failed to process index ${urlStr}: ${err.message}`)
                failedDownloads.push({
                    skillName,
                    url: urlStr,
                    error: err.message,
                })
                totalFailed++
                hasFailure = true
            }
        }

        for (const urlStr of urls) {
            if (isNonTextUrl(urlStr)) continue
            if (isLlmsIndexUrl(urlStr)) {
                await processIndexUrl(urlStr)
            } else {
                const filename = smartSlugify(urlStr)
                const destFile = checkPathTraversal(resourcesDir, filename)
                downloadJobs.push({url: urlStr, destFile, skillName})
            }
        }

        // Detect if there are multiple domains to avoid filename collisions
        const domainPrefixes = new Set<string>()
        for (const job of downloadJobs) {
            const prefix = getDomainPrefix(job.url)
            if (prefix) domainPrefixes.add(prefix)
        }

        if (domainPrefixes.size > 1) {
            for (const job of downloadJobs) {
                const prefix = getDomainPrefix(job.url)
                if (prefix) {
                    const filename = path.basename(job.destFile)
                    const newFilename = `${prefix}-${filename}`
                    job.destFile = checkPathTraversal(resourcesDir, newFilename)
                }
            }
        }

        // Detect and disambiguate same-domain filename collisions
        const destFileToJobs = new Map<string, DownloadJob[]>()
        for (const job of downloadJobs) {
            const list = destFileToJobs.get(job.destFile) || []
            list.push(job)
            destFileToJobs.set(job.destFile, list)
        }

        for (const [destFile, jobs] of destFileToJobs.entries()) {
            if (jobs.length > 1) {
                log.warn(
                    `  Filename collision detected for path: ${path.basename(destFile)}. Colliding URLs:`
                )
                for (const job of jobs) {
                    log.warn(`    - ${job.url}`)
                }

                for (const job of jobs) {
                    const hash = crypto
                        .createHash("sha256")
                        .update(job.url)
                        .digest("hex")
                        .slice(0, 8)
                    const filename = path.basename(job.destFile)
                    const ext = path.extname(filename)
                    const base = ext ? filename.slice(0, -ext.length) : filename
                    const newFilename = `${base}-${hash}${ext}`
                    job.destFile = checkPathTraversal(resourcesDir, newFilename)
                }
            }
        }

        // Map of target URLs to their final output file basenames for this skill
        const urlToFilenameMap = new Map<string, string>()
        const normalizeUrlForMatching = (urlStr: string): string => {
            try {
                const u = new URL(urlStr)
                let pathname = u.pathname
                if (pathname.endsWith("/")) {
                    pathname = pathname.slice(0, -1)
                }
                pathname = pathname.replace(/\.(mdx|md|html)$/i, "")
                return `${u.origin}${pathname}`
            } catch {
                return urlStr
            }
        }
        for (const job of downloadJobs) {
            urlToFilenameMap.set(
                normalizeUrlForMatching(job.url),
                path.basename(job.destFile)
            )
        }

        // Phase 2: Download jobs with concurrency limit
        const executeJob = async (job: DownloadJob) => {
            try {
                let content = ""
                let isResponseHtml = false
                let fetchedFromCandidate = false

                // Try raw asset candidates if available (e.g. for Google Antigravity SPA docs)
                if (job.candidates && job.candidates.length > 0) {
                    for (const candidate of job.candidates) {
                        try {
                            const result = await download(
                                candidate,
                                options?.timeout
                            )
                            if (result.skipped) {
                                log.info(
                                    `  [SKIP] Skipped non-text resource: ${candidate}`
                                )
                                totalSkipped++
                                return
                            }
                            if (
                                result.content.trim().length >=
                                MIN_CONTENT_BYTES
                            ) {
                                content = result.content
                                isResponseHtml = result.isHtml
                                fetchedFromCandidate = true
                                break
                            }
                        } catch {
                            // Try next candidate
                        }
                    }
                }

                // If candidates are not available or they failed, try the main URL
                if (!content) {
                    const result = await download(job.url, options?.timeout)
                    if (result.skipped) {
                        log.info(
                            `  [SKIP] Skipped non-text resource: ${job.url}`
                        )
                        totalSkipped++
                        return
                    }
                    content = result.content
                    isResponseHtml = result.isHtml
                }

                const trimmed = stripReaderMetadata(content.trim())
                if (trimmed.length < MIN_CONTENT_BYTES) {
                    log.warn(`  [SKIP] Trivial content from ${job.url}`)
                    totalSkipped++
                    return
                }

                let finalContent = trimmed
                // Detect HTML format
                const isHtml =
                    isResponseHtml ||
                    trimmed.startsWith("<!") ||
                    trimmed.startsWith("<html") ||
                    trimmed.includes("<body")

                if (isHtml && fetchedFromCandidate) {
                    log.warn(
                        `  [WARN] HTML response detected from candidate URL for ${job.url}`
                    )
                }

                if (isHtml) {
                    const converter =
                        options?.convertHtmlToMarkdown || convertHtmlToMarkdown
                    finalContent = await converter(trimmed)
                    if (finalContent.trim().length < MIN_CONTENT_BYTES) {
                        log.warn(
                            `  [SKIP] Converted HTML yielded trivial markdown for ${job.url}`
                        )
                        totalSkipped++
                        return
                    }
                }

                // Rewrite root-relative links, relative links, and relative asset paths to be absolute or resolved
                try {
                    const urlObj = new URL(job.url)
                    const origin = urlObj.origin
                    finalContent = finalContent.replace(
                        /(\]|!\[[^\]]*\])\(([^)]+)\)/g,
                        (match, prefix, linkUrl) => {
                            const trimmedLink = linkUrl.trim()
                            if (
                                /^(https?:\/\/|mailto:|tel:)/i.test(
                                    trimmedLink
                                ) ||
                                trimmedLink.startsWith("#")
                            ) {
                                return match
                            }

                            // Check if it is a relative file link (does not start with /)
                            if (
                                trimmedLink.startsWith(".") ||
                                !trimmedLink.startsWith("/")
                            ) {
                                try {
                                    const resolvedUrlObj = new URL(
                                        trimmedLink,
                                        job.url
                                    )
                                    const hash = resolvedUrlObj.hash
                                    resolvedUrlObj.hash = ""
                                    const resolvedUrlWithoutHash =
                                        resolvedUrlObj.toString()
                                    const normalizedTarget =
                                        normalizeUrlForMatching(
                                            resolvedUrlWithoutHash
                                        )

                                    const mappedFilename =
                                        urlToFilenameMap.get(normalizedTarget)
                                    if (mappedFilename) {
                                        return `${prefix}(${mappedFilename}${hash})`
                                    } else {
                                        // Fallback to absolute URL
                                        return `${prefix}(${resolvedUrlWithoutHash}${hash})`
                                    }
                                } catch {
                                    return match
                                }
                            }

                            if (
                                trimmedLink.startsWith("/") &&
                                !trimmedLink.startsWith("//")
                            ) {
                                return `${prefix}(${origin}${trimmedLink})`
                            }
                            return match
                        }
                    )
                } catch {
                    // Ignore invalid job URLs
                }

                // Apply automated Markdown scrubbing, decoding, escaping, and formatting passes
                finalContent = sanitizeControlCharacters(finalContent)
                finalContent = decodeHtmlEntities(finalContent)
                finalContent = escapeOrphanTypeBrackets(finalContent)
                finalContent = normalizeMarkdownFormatting(finalContent)
                finalContent = protectZolaDelimiters(finalContent)

                await fs.writeFile(job.destFile, finalContent, "utf-8")
                savedFiles.add(path.basename(job.destFile))
                totalSuccess++
            } catch (err: any) {
                if (job.discovered) {
                    // A link listed in an upstream llms.txt index that no
                    // longer resolves (upstream churn). Skip it without
                    // failing the run or blocking this skill's prune.
                    log.warn(
                        `  [STALE] Skipping dead index link ${job.url}: ${err.message}`
                    )
                    staleLinks.push({
                        skillName: job.skillName,
                        url: job.url,
                        error: err.message,
                    })
                    totalSkipped++
                    return
                }
                log.error(
                    `  [FAIL] Failed to download ${job.url}: ${err.message}`
                )
                failedDownloads.push({
                    skillName: job.skillName,
                    url: job.url,
                    error: err.message,
                })
                totalFailed++
                hasFailure = true
            }
        }

        // Run download jobs in parallel up to the concurrency limit
        await runWithConcurrencyLimit(
            concurrencyLimit,
            downloadJobs,
            executeJob
        )

        // Phase 3: Auto-prune orphaned files
        if (!hasFailure && (await fs.pathExists(resourcesDir))) {
            const existingFiles = await fs.readdir(resourcesDir)
            let prunedCount = 0
            for (const file of existingFiles) {
                const filePath = path.join(resourcesDir, file)
                const stats = await fs.stat(filePath)
                const isConfigured = downloadJobs.some(
                    (job) => path.basename(job.destFile) === file
                )
                if (stats.isFile() && !savedFiles.has(file) && !isConfigured) {
                    await fs.remove(filePath)
                    prunedCount++
                }
            }
            if (prunedCount > 0) {
                log.info(
                    `  Pruned ${prunedCount} orphaned file(s) from resources.`
                )
            }
        }
    }

    log.info(
        `Download completed: ${totalSuccess} succeeded, ${totalSkipped} skipped, ${totalFailed} failed.`
    )

    if (staleLinks.length > 0) {
        log.warn(
            `\n⚠️ Skipped ${staleLinks.length} dead link(s) from upstream llms.txt indexes (upstream churn, not a config error):`
        )
        const groupedStale: Record<string, typeof staleLinks> = {}
        for (const item of staleLinks) {
            let list = groupedStale[item.skillName]
            if (!list) {
                list = []
                groupedStale[item.skillName] = list
            }
            list.push(item)
        }
        for (const [skill, items] of Object.entries(groupedStale)) {
            log.warn(`  ● Skill: ${skill}`)
            for (const item of items) {
                log.warn(`    - ${item.url}`)
            }
        }
    }

    if (failedDownloads.length > 0) {
        log.error("\n❌ Failed Downloads Summary:")
        const grouped: Record<string, typeof failedDownloads> = {}
        for (const item of failedDownloads) {
            let list = grouped[item.skillName]
            if (!list) {
                list = []
                grouped[item.skillName] = list
            }
            list.push(item)
        }
        for (const [skill, items] of Object.entries(grouped)) {
            log.error(`  ● Skill: ${skill}`)
            for (const item of items) {
                log.error(`    - URL: ${item.url}`)
                log.error(`      Error: ${item.error}`)
            }
        }
        throw new Error(
            `Download resources failed: ${failedDownloads.length} file(s) failed to download.`
        )
    }
}

export interface CleanOptions {
    skill?: string
    category?: string
}

export async function cleanAction(
    skills: Skill[],
    agentSkillsHome: string,
    options?: CleanOptions
) {
    let targetSkills = skills

    if (options?.skill) {
        targetSkills = targetSkills.filter(
            (s) =>
                s.dirName === options.skill || s.metadata.name === options.skill
        )
    }

    if (options?.category) {
        targetSkills = targetSkills.filter(
            (s) => s.category === options.category
        )
    }

    let cleanedCount = 0
    for (const skill of targetSkills) {
        const skillDir = path.dirname(path.join(agentSkillsHome, skill.path))
        // Only the downloader-owned resources/auto is removed, so a subsequent
        // download-resources reproduces it cleanly; resources/manual is kept.
        const autoDir = path.join(skillDir, "resources", "auto")
        if (await fs.pathExists(autoDir)) {
            await fs.remove(autoDir)
            cleanedCount++
        }
    }
    log.info(
        `Cleaned downloaded resources (resources/auto) for ${cleanedCount} skill(s).`
    )
}

async function runWithConcurrencyLimit<T>(
    limit: number,
    items: T[],
    fn: (item: T) => Promise<void>
) {
    const pool: Promise<void>[] = []
    for (const item of items) {
        const p = fn(item).then(() => {
            pool.splice(pool.indexOf(p), 1)
        })
        pool.push(p)
        if (pool.length >= limit) {
            await Promise.race(pool)
        }
    }
    await Promise.all(pool)
}
