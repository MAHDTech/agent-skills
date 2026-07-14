import {$} from "bun"
import path from "node:path"
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
}

// Download content using Bun-native fetch
export async function download(url: string): Promise<string> {
    const response = await fetch(url, {
        headers: {
            "User-Agent":
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        },
        signal: AbortSignal.timeout(HTTP_TIMEOUT * 1000),
    })

    if (!response.ok) {
        throw new Error(
            `HTTP Status ${response.status} (${response.statusText})`
        )
    }

    return await response.text()
}

// Convert HTML content directly to Markdown using Pandoc via stdin/stdout
export async function convertHtmlToMarkdown(
    htmlContent: string
): Promise<string> {
    try {
        const mdContent =
            await $`pandoc -f html -t gfm < ${new Response(htmlContent)}`.text()
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
        const url = new URL(urlStr)
        const parts = url.hostname.split(".")
        if (parts.length >= 2) {
            const domain = parts[parts.length - 2]
            return domain || ""
        }
        return url.hostname
    } catch {
        return ""
    }
}

// Clean and format slug names for downloaded files
export function smartSlugify(urlStr: string, commonPrefix?: string): string {
    let pathStr = urlStr
    if (commonPrefix && urlStr.startsWith(commonPrefix)) {
        pathStr = urlStr.slice(commonPrefix.length)
    } else {
        try {
            const url = new URL(urlStr)
            pathStr = url.pathname
        } catch {
            // Fallback
        }
    }

    // Remove leading/trailing slashes
    pathStr = pathStr.replace(/^\/+|\/+$/g, "")

    if (!pathStr) {
        return "index.md"
    }

    // Determine extension
    let ext = ".md"
    if (pathStr.endsWith(".txt")) {
        ext = ".txt"
        pathStr = pathStr.slice(0, -4)
    } else if (pathStr.endsWith(".md")) {
        ext = ".md"
        pathStr = pathStr.slice(0, -3)
    } else if (pathStr.endsWith(".html")) {
        ext = ".md"
        pathStr = pathStr.slice(0, -5)
    }

    // Replace slashes with dashes and clean up characters
    let slug = pathStr.replace(/\//g, "-")
    slug = slug.replace(/[^a-zA-Z0-9.\-_]/g, "")

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

            // Exclude binary extensions
            if (/\.(zip|pdf|png|jpg|jpeg|gif|tar|gz|exe|dmg)$/i.test(urlString))
                continue

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

export async function downloadAction(skills: Skill[], agentSkillsHome: string) {
    const skillsWithDocs = skills.filter(
        (s) =>
            s.metadata.resources &&
            Array.isArray(s.metadata.resources) &&
            s.metadata.resources.length > 0
    )

    if (skillsWithDocs.length === 0) {
        log.info("No skills with configured resources found.")
        return
    }

    log.info(`Syncing resources for ${skillsWithDocs.length} skill(s)...`)

    const concurrencyLimit = 10
    let totalSuccess = 0
    let totalFailed = 0
    let totalSkipped = 0

    for (const skill of skillsWithDocs) {
        const skillName = skill.dirName
        const skillDir = path.dirname(path.join(agentSkillsHome, skill.path))
        const resourcesDir = path.join(skillDir, "resources")
        await fs.ensureDir(resourcesDir)

        const urls = skill.metadata.resources || []
        const savedFiles = new Set<string>()
        const downloadJobs: DownloadJob[] = []

        log.step(`Processing skill: ${skillName}`)

        // Phase 1: Resolve all URLs (handle llms.txt index parsing)
        for (const urlStr of urls) {
            const isIndex = urlStr.endsWith("/llms.txt")

            if (isIndex) {
                log.info(`  Fetching index: ${urlStr}`)
                try {
                    const indexContent = await download(urlStr)
                    const childUrls = parseLlmsTxtLinks(indexContent, urlStr)
                    log.info(`  Found ${childUrls.length} links in index.`)

                    if (childUrls.length > 0) {
                        const commonPrefix = getCommonPrefix(childUrls)
                        for (const childUrl of childUrls) {
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
                            })
                        }
                    }
                } catch (err: any) {
                    log.error(
                        `  Failed to process index ${urlStr}: ${err.message}`
                    )
                    totalFailed++
                }
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

        // Phase 2: Download jobs with concurrency limit
        const executeJob = async (job: DownloadJob) => {
            try {
                let content = ""
                let fetchedFromCandidate = false

                // Try raw asset candidates if available (e.g. for Google Antigravity SPA docs)
                if (job.candidates && job.candidates.length > 0) {
                    for (const candidate of job.candidates) {
                        try {
                            const res = await download(candidate)
                            if (res.trim().length >= MIN_CONTENT_BYTES) {
                                content = res
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
                    content = await download(job.url)
                }

                const trimmed = content.trim()
                if (trimmed.length < MIN_CONTENT_BYTES) {
                    log.warn(`  [SKIP] Trivial content from ${job.url}`)
                    totalSkipped++
                    return
                }

                let finalContent = trimmed
                // Detect HTML format (only if not loaded from raw markdown candidates)
                const isHtml =
                    !fetchedFromCandidate &&
                    (trimmed.startsWith("<!") ||
                        trimmed.startsWith("<html") ||
                        trimmed.includes("<body"))

                if (isHtml) {
                    finalContent = await convertHtmlToMarkdown(trimmed)
                    if (finalContent.trim().length < MIN_CONTENT_BYTES) {
                        log.warn(
                            `  [SKIP] Converted HTML yielded trivial markdown for ${job.url}`
                        )
                        totalSkipped++
                        return
                    }
                }

                await fs.writeFile(job.destFile, finalContent, "utf-8")
                savedFiles.add(path.basename(job.destFile))
                totalSuccess++
            } catch (err: any) {
                log.error(
                    `  [FAIL] Failed to download ${job.url}: ${err.message}`
                )
                totalFailed++
            }
        }

        // Run download jobs in parallel up to the concurrency limit
        await runWithConcurrencyLimit(
            concurrencyLimit,
            downloadJobs,
            executeJob
        )

        // Phase 3: Auto-prune orphaned files
        if (await fs.pathExists(resourcesDir)) {
            const existingFiles = await fs.readdir(resourcesDir)
            let prunedCount = 0
            for (const file of existingFiles) {
                const filePath = path.join(resourcesDir, file)
                const stats = await fs.stat(filePath)
                if (stats.isFile() && !savedFiles.has(file)) {
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
}

export async function cleanAction(skills: Skill[], agentSkillsHome: string) {
    let cleanedCount = 0
    for (const skill of skills) {
        if (
            !skill.metadata.resources ||
            !Array.isArray(skill.metadata.resources)
        ) {
            continue
        }
        const skillDir = path.dirname(path.join(agentSkillsHome, skill.path))
        const resourcesDir = path.join(skillDir, "resources")
        if (await fs.pathExists(resourcesDir)) {
            await fs.remove(resourcesDir)
            cleanedCount++
        }
    }
    log.info(`Cleaned resources for ${cleanedCount} skill(s).`)
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
