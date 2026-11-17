#!/usr/bin/env bash
# Builds the dashboard into a scratch directory and checks the rendered site.
# Nothing is written to dashboard/public, so it is safe beside `devenv up`.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

site="$(mktemp -d)"
trap 'rm -rf "$site"' EXIT

fail() {
	echo "dashboard test failed: $*" >&2
	exit 1
}

tera-templates dashboard/themes

tailwindcss -i dashboard/css/input.css -o dashboard/static/build/css/generated.css --minify 2>/dev/null
# Zola prints thousands of anchor warnings for downloaded reference docs, so
# its output is only shown when the build fails.
if ! zola --root dashboard build --force -o "$site/public" >"$site/zola.log" 2>&1; then
	grep -v 'anchor in the link' "$site/zola.log" >&2 || true
	fail "zola build failed"
fi
pagefind --site "$site/public" --quiet
public="$site/public"

# The pages and assets every deployment must contain.
for file in \
	index.html \
	404.html \
	CNAME \
	skills/index.html \
	skills/engineering/index.html \
	skills/tooling/agent-guidelines/index.html \
	skills/archive/index.html \
	build/css/generated.css \
	js/site.js \
	images/logo.svg \
	favicon.png \
	fonts/Inter-Variable.woff2 \
	fonts/JetBrainsMono-Variable.woff2 \
	pagefind/pagefind.js \
	pagefind/pagefind-component-ui.js; do
	[ -s "$public/$file" ] || fail "missing or empty: $file"
done

for font in Inter-Variable JetBrainsMono-Variable; do
	[ "$(head -c 4 "$public/fonts/$font.woff2")" = "wOF2" ] || fail "$font.woff2 is not a WOFF2 file"
done

expect() {
	local file="$1" pattern="$2"
	grep -q -- "$pattern" "$public/$file" || fail "$file does not contain: $pattern"
}

# Shared chrome: theme toggle, search, skip link and navigation on every layout.
for page in index.html 404.html skills/index.html skills/tooling/agent-guidelines/index.html; do
	expect "$page" '<meta name="color-scheme" content="light dark">'
	expect "$page" 'data-theme-toggle'
	expect "$page" 'data-search-open'
	expect "$page" 'href="#main-content"'
	expect "$page" '<pagefind-modal'
	expect "$page" 'aria-label="Skills"'
done

# Landing page and catalog.
expect index.html 'npx skills add MAHDTech/agent-skills'
expect index.html '<link rel="canonical" href="https://skills.mahdtech.com/">'
expect skills/index.html 'Engineering'
expect skills/engineering/index.html 'aria-label="Breadcrumb"'

# Skill page: install snippet, source link, breadcrumbs and the document body.
skill=skills/tooling/agent-guidelines/index.html
expect "$skill" 'npx skills add MAHDTech/agent-skills --skill agent-guidelines'
expect "$skill" 'blob/trunk/skills/tooling/agent-guidelines/SKILL.md'
expect "$skill" 'aria-label="Breadcrumb"'
expect "$skill" 'class="prose-mahd'
expect "$skill" 'property="og:title" content="agent-guidelines | Agent Skills"'

# Archived skills are flagged and never offer an install command.
archived="$(find "$public/skills/archive" -mindepth 3 -maxdepth 3 -name index.html | sort | head -1)"
[ -n "$archived" ] || fail "no archived skill page was rendered"
grep -q 'aria-label="Archived"' "$archived" || fail "archived skill has no archive banner: $archived"
if grep -q -- '--skill ' "$archived"; then
	fail "archived skill offers an install command: $archived"
fi

# A component call that fails to expand is left in the page as literal text.
for page in index.html skills/index.html skills/engineering/index.html; do
	if grep -q '<ui\.' "$public/$page"; then
		fail "$page contains an unrendered component call"
	fi
done

# The CSS bundle carries both palettes and the shared surface classes.
expect build/css/generated.css 'light-dark('
expect build/css/generated.css 'monolith-card'
expect build/css/generated.css 'prose-mahd'

echo "dashboard test passed: $(find "$public" -name index.html | wc -l) pages rendered" >&2
