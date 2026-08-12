+++
title = "docs-components-summary-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "pagefind"
+++

{% raw %}
> **Important: Pagefind 1.5.0 introduces the Component UI, which replaces the
> Default UI (pagefind-ui.js / PagefindUI). It includes a search modal, better
> accessibility and customization.** Full component guide:
> https://pagefind.app/llms-component-ui.txt

# \<pagefind-summary\>


Displays search status and result count. Shows messages like "12 results for 'search term'".

<div class="demo-box demo-box-attached" style="flex-direction: column; gap: 1rem;">
<pagefind-input instance="summary-demo"></pagefind-input>
<pagefind-summary instance="summary-demo"></pagefind-summary>
</div>

```html
<pagefind-summary></pagefind-summary>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `default-message` | string | `""` | Message shown before any search |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## States

The summary displays different messages based on search state:

| State | Display |
|-------|---------|
| No search | `default-message` attribute value |
| Searching | `Searching for {term}...` |
| Results found | `{count} result(s) for {term}` |
| Results found, only filtering | `{count} result(s)` |
| No results | `0 results for {term}` |
| Error | `Error: {message}` |

These messages are automatically translated to match the language of the website.
{% endraw %}
