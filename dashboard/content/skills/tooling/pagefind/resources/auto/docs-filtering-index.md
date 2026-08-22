+++
title = "docs-filtering-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "pagefind"
+++

\> \*\*Important: Pagefind 1.5.0 introduces the Component UI, which
replaces the \> Default UI (pagefind-ui.js / PagefindUI). It includes a
search modal, better \> accessibility and customization.\*\* Full
component guide: \> https://pagefind.app/llms-component-ui.txt \#
Setting up filters To configure filters in Pagefind, pages are
associated to filter keys and values using data attributes. \##
Capturing a filter value from an element \`\`\`html

# My Blog Post

Author: + bglw

\`\`\` An element tagged with \`data-pagefind-filter\` will associate
that page with the filter name, and capture the contents of the element
as the filter value. In the above example, the page would be tagged as
\`author: \["bglw"\]\`. Filters can have multiple values per page, so
the following is also valid: \`\`\`html

# Hello World

Authors: + Pagefind and + Liam Bigelow

\`\`\` Which produces: \`author: \["Pagefind", "Liam Bigelow"\]\`. \##
Capturing a filter value from an attribute If the data you want to
filter on exists as an attribute, you can use the syntax
\`filter_name\[html_attribute\]\` \`\`\`html

\`\`\` This will capture the filter value from the attribute specified,
in this case producing \`author: \["Pagefind"\]\`. \## Specifying a
filter inline If your value doesn't already exist on the page, you can
use the syntax \`filter_name:value\`: \`\`\`html

# Hello World

\`\`\` This will tag this page as \`author: \["bglw"\]\`. The element
this is set on does not matter, meaning this attribute can be located
anywhere that is convenient in your site templating. \## Specifying
multiple filters on a single element Filter captures may be comma
separated and all will apply. The exception is specifying a filter
inline, which may only be the last item in a list. For example:
\`\`\`html

# Hello World

\`\`\` This will produce the filter values for the page: \`\`\`json {
"heading": \["Hello World"\], "tag": \["Documentation", "Article"\],
"author": \["Freeform text, captured to the end"\] } \`\`\` \## Notes \>
The \`data-pagefind-filter\` attribute does not need to be within the \`

\`, or the \`data-pagefind-body\` tag. \> The \`data-pagefind-filter\`
attribute will still apply if set on or within a
\`data-pagefind-ignore\` element. \> The keys \`any\`, \`all\`,
\`none\`, and \`not\` are reserved and can't be used as filter keys.

