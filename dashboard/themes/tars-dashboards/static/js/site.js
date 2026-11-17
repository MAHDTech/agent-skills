;(() => {
    "use strict"

    // Theme: "system" (no stored value) or an explicit "light" / "dark".
    // The choice is applied through <meta name="color-scheme">; the inline
    // script in partials/head.html applies a stored choice before first paint.
    const STORAGE_KEY = "color-scheme"
    const darkQuery = window.matchMedia("(prefers-color-scheme: dark)")

    const getSystemScheme = () => (darkQuery.matches ? "dark" : "light")

    const getStoredScheme = () => {
        try {
            const val = localStorage.getItem(STORAGE_KEY)
            return val === "light" || val === "dark" ? val : null
        } catch {
            return null
        }
    }

    const applyThemeState = (override) => {
        const meta = document.querySelector('meta[name="color-scheme"]')
        if (meta) meta.content = override ?? "light dark"

        const label = override
            ? override === "dark"
                ? "Dark"
                : "Light"
            : `Auto · ${getSystemScheme() === "dark" ? "Dark" : "Light"}`

        document.querySelectorAll("[data-theme-toggle]").forEach((btn) => {
            btn.querySelector('[data-theme-icon="sun"]')?.classList.toggle(
                "hidden",
                override !== "light"
            )
            btn.querySelector('[data-theme-icon="moon"]')?.classList.toggle(
                "hidden",
                override !== "dark"
            )
            btn.querySelector('[data-theme-icon="auto"]')?.classList.toggle(
                "hidden",
                override !== null
            )
            const labelEl = btn.querySelector("[data-theme-label]")
            if (labelEl) labelEl.textContent = label
        })

        renderDiagrams()
    }

    // The header button flips to the opposite of what is shown. Landing on the
    // system preference clears the override so auto-detection resumes.
    const toggleTheme = () => {
        const rendered = getStoredScheme() ?? getSystemScheme()
        const target = rendered === "dark" ? "light" : "dark"
        const override = target === getSystemScheme() ? null : target
        try {
            if (override) localStorage.setItem(STORAGE_KEY, override)
            else localStorage.removeItem(STORAGE_KEY)
        } catch {}
        applyThemeState(override)
    }

    const initTheme = () => {
        applyThemeState(getStoredScheme())
        document
            .querySelectorAll("[data-theme-toggle]")
            .forEach((btn) => btn.addEventListener("click", toggleTheme))
        darkQuery.addEventListener("change", () =>
            applyThemeState(getStoredScheme())
        )
        window.addEventListener("storage", (event) => {
            if (event.key === STORAGE_KEY) applyThemeState(getStoredScheme())
        })
    }

    // Mermaid diagrams. The library is only loaded on pages that set
    // `extra.mermaid`. Each block keeps its source so it can be redrawn in the
    // matching palette whenever the theme changes.
    let diagramRun = 0

    const collectDiagrams = () => {
        document
            .querySelectorAll('code[data-lang="mermaid"]')
            .forEach((code) => {
                const pre = code.closest("pre") ?? code
                const source = pre.textContent
                pre.className = "mermaid"
                pre.removeAttribute("style")
                pre.removeAttribute("data-lang")
                pre.dataset.source = source
                pre.textContent = source
            })
        document
            .querySelectorAll("pre.mermaid:not([data-source])")
            .forEach((pre) => {
                pre.dataset.source = pre.textContent
            })
    }

    const renderDiagrams = async () => {
        if (typeof window.mermaid === "undefined") return
        const blocks = Array.from(
            document.querySelectorAll("pre.mermaid[data-source]")
        )
        if (blocks.length === 0) return

        const run = ++diagramRun
        // Read tokens through a probe element: a custom property's computed value
        // is the unresolved light-dark() expression, which Mermaid cannot parse.
        const probe = document.createElement("span")
        probe.hidden = true
        document.body.appendChild(probe)
        const token = (name) => {
            probe.style.color = `var(${name})`
            return getComputedStyle(probe).color
        }
        const scheme = getStoredScheme() ?? getSystemScheme()

        window.mermaid.initialize({
            startOnLoad: false,
            theme: "base",
            themeVariables: {
                darkMode: scheme === "dark",
                background: token("--color-surface-raised"),
                primaryColor: token("--color-surface-active"),
                primaryBorderColor: token("--color-accent-cyan"),
                primaryTextColor: token("--color-text-primary"),
                lineColor: token("--color-border-strong"),
                secondaryColor: token("--color-well"),
                tertiaryColor: token("--color-surface-raised"),
                fontFamily: "JetBrains Mono, ui-monospace, monospace",
            },
        })

        probe.remove()

        blocks.forEach((pre) => {
            pre.removeAttribute("data-processed")
            pre.textContent = pre.dataset.source
        })
        try {
            if (run === diagramRun) await window.mermaid.run({nodes: blocks})
        } catch (error) {
            console.error("Mermaid rendering failed:", error)
        }
    }

    // Clipboard
    const copyToClipboard = async (text) => {
        try {
            await navigator.clipboard.writeText(String(text ?? ""))
            return true
        } catch {
            return false
        }
    }

    const bindCopyFeedback = (btn, getText) => {
        const labelEl = btn.querySelector("[data-copy-label]") ?? btn
        const initial = labelEl.textContent
        let timer = null
        btn.addEventListener("click", async () => {
            labelEl.textContent = (await copyToClipboard(getText()))
                ? "Copied"
                : "Copy failed"
            clearTimeout(timer)
            timer = setTimeout(() => (labelEl.textContent = initial), 1800)
        })
    }

    const initCopyButtons = () => {
        document
            .querySelectorAll("[data-copy-text]")
            .forEach((btn) =>
                bindCopyFeedback(btn, () => btn.getAttribute("data-copy-text"))
            )

        document
            .querySelectorAll(".prose-mahd pre:not(.mermaid)")
            .forEach((pre) => {
                const source = (pre.querySelector("code") ?? pre).textContent
                const btn = document.createElement("button")
                btn.type = "button"
                btn.className =
                    "no-print absolute top-2.5 right-2.5 px-2.5 py-1 rounded-[4px] border border-slate-700 bg-slate-900/90 hover:border-sky-400 text-slate-300 hover:text-sky-300 font-mono text-[11px] cursor-pointer transition-colors"
                btn.setAttribute("aria-label", "Copy code")
                btn.textContent = "Copy"
                bindCopyFeedback(btn, () => source)
                pre.appendChild(btn)
            })
    }

    // Search: the Pagefind modal, opened from any [data-search-open] control
    // or with Ctrl/Cmd+K.
    const initSearch = () => {
        const modal = document.querySelector("pagefind-modal")
        const open = () => {
            if (modal && typeof modal.open === "function") modal.open()
        }

        const isMac = /mac|iphone|ipad|ipod/i.test(navigator.userAgent)
        document.querySelectorAll("[data-search-shortcut]").forEach((kbd) => {
            kbd.textContent = isMac ? "⌘K" : "Ctrl K"
        })
        document
            .querySelectorAll("[data-search-open]")
            .forEach((btn) => btn.addEventListener("click", open))
        document.addEventListener("keydown", (event) => {
            if (
                (event.ctrlKey || event.metaKey) &&
                event.key.toLowerCase() === "k"
            ) {
                event.preventDefault()
                open()
            }
        })
    }

    // Sidebar: a drawer below the md breakpoint, and keep the current skill
    // in view inside the scrolling tree.
    const initSidebar = () => {
        const btn = document.getElementById("mobile-menu-btn")
        const drawer = document.getElementById("sidebar-drawer")
        const backdrop = document.getElementById("sidebar-backdrop")
        const label = document.getElementById("mobile-menu-label")

        if (btn && drawer && backdrop) {
            const setOpen = (open) => {
                btn.setAttribute("aria-expanded", String(open))
                drawer.classList.toggle("hidden", !open)
                backdrop.classList.toggle("hidden", !open)
                if (label) label.textContent = open ? "Close" : "Menu"
            }

            btn.addEventListener("click", () =>
                setOpen(btn.getAttribute("aria-expanded") !== "true")
            )
            backdrop.addEventListener("click", () => setOpen(false))
            drawer.addEventListener("click", (event) => {
                if (event.target.closest("a[href]")) setOpen(false)
            })
            document.addEventListener("keydown", (event) => {
                if (
                    event.key === "Escape" &&
                    btn.getAttribute("aria-expanded") === "true"
                ) {
                    setOpen(false)
                    btn.focus()
                }
            })
        }

        document
            .querySelector("[data-sidebar-active]")
            ?.scrollIntoView({block: "center"})
    }

    document.addEventListener("DOMContentLoaded", () => {
        collectDiagrams()
        initTheme()
        initCopyButtons()
        initSearch()
        initSidebar()
    })
})()
